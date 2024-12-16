use std::collections::HashMap;
use crate::day12::Direction;

#[derive(Clone)]
pub struct Maze {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    max_x: usize,
    max_y: usize
}

impl Maze {
    pub fn new(map: Vec<Vec<char>>) -> Maze {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;
        for y in 0..map[0].len() {
            for x in 0..map.len() {
                if map[x][y] == 'S' {
                    start = Some((x, y));
                } else if map[x][y] == 'E' {
                    end = Some((x, y));
                }
            }
        }
        let max_x = map.len() - 1;
        let max_y = map[0].len() - 1;
        Maze {
            map: map,
            start: start.unwrap(),
            end: end.unwrap(),
            max_y: max_y,
            max_x: max_x
        }
    }
    
    pub fn start(&self) -> (usize, usize) {
        self.start
    }
    
    pub fn get_adjacent(&self, x: usize, y: usize) -> HashMap<Direction, (usize, usize)> {
        let mut adjacent = HashMap::new();
        if y > 0 && self.map[x][y - 1] != '#' {
            adjacent.insert(Direction::North, (x, y - 1));
        }
        if y < self.max_y && self.map[x][y + 1] != '#' {
            adjacent.insert(Direction::South, (x, y + 1));
        }
        if x > 0 && self.map[x - 1][y] != '#' {
            adjacent.insert(Direction::West, (x - 1, y));
        }
        if x < self.max_x && self.map[x + 1][y] != '#' {
            adjacent.insert(Direction::East, (x + 1, y));
        }
        adjacent
    }

    fn print_map(&self) {
        for y in 0..self.map[0].len() {
            for x in 0..self.map.len() {
                print!("{}", self.map[x][y]);
            }
            println!();
        }
    }
}

#[derive(Clone)]
pub enum StepKind {
    Move,
    Turn
}

#[derive(Clone)]
pub struct Step {
    kind: StepKind,
    step_start_position: (usize, usize)
}

#[derive(Clone)]
pub struct Reindeer {
    // pub maze: Maze,
    pub position: (usize, usize),
    pub facing: Direction,
    pub steps_taken: Vec<Step>,
    pub finished: bool
}

impl Reindeer {
    fn opposite(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        }
    }
    pub fn next_possibilities(&self, maze: &Maze) -> Vec<Reindeer> {
        let adjacent_squares = maze.get_adjacent(self.position.0, self.position.1);
        
        if self.position == maze.end {
            let mut c = self.clone();
            c.finished = true;
            return vec![c]
        }
        
        let visited_positions = self.steps_taken.iter().map(|step| step.step_start_position).collect::<Vec<_>>();
        
        let new_reindeers = adjacent_squares.into_iter().map(|(direction, (x, y))| {
            if direction == self.facing {
                let mut r = self.clone();
                let step = Step {
                    kind: StepKind::Move,
                    step_start_position: self.position
                };
                r.steps_taken.push(step);
                r.position = (x, y);
                Some(r)
            } else if direction == Reindeer::opposite(&self.facing) {
                None
            } else {
                let mut r = self.clone();
                let step = Step {
                    kind: StepKind::Turn,
                    step_start_position: self.position
                };
                r.steps_taken.push(step);
                r.facing = direction;
                Some(r)
            }
        }).filter_map(|x| {
            let looped = x.as_ref().map(|rr| visited_positions.contains(&rr.position));
            match looped {
                Some(true) => None,
                _ => x
            }
        }).collect::<Vec<_>>();
        
        if new_reindeers.is_empty() {
            return Vec::new();
        }
        
        new_reindeers
    }
    
    pub fn print_steps(&self, maze: &Maze) {
        for y in 0..maze.map.len() {
            for x in 0..maze.map[0].len() {
                let step_at_position = self.steps_taken.iter().find(|step| step.step_start_position == (x, y));

                if self.position == (x, y) {
                    print!("R");
                } else if step_at_position.is_some() {
                    print!("*");
                } else {
                    print!("{}", maze.map[x][y]);
                }
            }
            println!();
        }
    }
    
    pub fn calculate_score(&self) -> usize {
        let mut score = 0;
        self.steps_taken.iter().for_each(|s| {
            match s.kind {
                StepKind::Move => score += 1,
                StepKind::Turn => score += 1000
            }
        });
        
        score
    }
}