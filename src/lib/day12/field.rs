use std::collections::{HashMap, HashSet};
use crate::day12::Direction;

#[derive(Debug)]
pub struct Field {
    pub squares: Vec<(usize, usize)>,
    pub crop: char
}

impl Field {

    pub fn print_field(&self, map: &Vec<Vec<char>>) {
        for y in 0..map[0].iter().count() {
            for x in 0..map.iter().count() {
                if self.squares.contains(&(x, y)) {
                    print!("{}", self.crop);
                } else {
                    print!(".");
                }
            }
            println!();
        }

    }

    pub fn adjacent_squares_count(&self, x: &i32, y: &i32) -> usize {
        self.get_adjacent(*x as usize, *y as usize).len()
    }

    pub fn get_adjacent(&self, x: usize, y: usize) -> HashMap<Direction, (usize, usize)> {
        let mut adjacent = HashMap::new();
        if y > 0 && self.squares.contains(&(x, y - 1)) {
            adjacent.insert(Direction::North, (x, y - 1));
        }
        if self.squares.contains(&(x + 1, y)) {
            adjacent.insert(Direction::East, (x + 1, y));
        }
        if self.squares.contains(&(x, y + 1)) {
            adjacent.insert(Direction::South, (x, y + 1));
        }
        if x > 0 && self.squares.contains(&(x - 1, y)) {
            adjacent.insert(Direction::West, (x - 1, y));
        }
        adjacent
    }

}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct SquareFence {
    x: isize,
    y: isize,
    direction: Direction
}

pub struct FencedField<'a> {
    pub field: &'a Field,
    pub fences: usize
}

struct Result {
    sides: usize,
    visited_fences: HashSet<SquareFence>
}

impl FencedField<'_> {
    pub fn count_sides(&self) -> usize {
        let mut visited_fences: HashSet<SquareFence> = HashSet::new();

        let mut sides = 0;

        for (sx, sy) in self.field.squares.iter() {

            for sf in self.get_all_square_fences(*sx, *sy) {
                if visited_fences.contains(&sf) {
                    continue;
                }
                let r = self.walk_fence(sf);
                visited_fences.extend(r.visited_fences);
                sides = sides + r.sides;
            }
        }

        sides
    }

    fn walk_fence(&self, start_fence: SquareFence) -> Result {
        let mut fence_visited = HashSet::new();
        fence_visited.insert(start_fence.clone());

        let mut current_fence = start_fence;
        let mut sides = 0;
        loop {
            let next_fence = self.step_along_fence(&current_fence);
            if next_fence.direction != current_fence.direction {
                sides += 1;
            }
            if fence_visited.contains(&next_fence) {
                break;
            }
            fence_visited.insert(next_fence.clone());
            current_fence = next_fence;
        }

        Result { sides, visited_fences: fence_visited }
    }

    fn get_all_square_fences(&self, x: usize, y:usize) -> Vec<SquareFence> {
        let mut all_sf = Vec::new();
        let sf = self.field.get_adjacent(x, y);


        if !sf.contains_key(&Direction::North) {
            all_sf.push(SquareFence { x: x as isize, y: y as isize, direction: Direction::North });
        }
        if !sf.contains_key(&Direction::East) {
            all_sf.push(SquareFence { x: x as isize, y: y as isize, direction: Direction::East });
        }
        if !sf.contains_key(&Direction::South) {
            all_sf.push(SquareFence { x: x as isize, y: y as isize, direction: Direction::South });
        }
        if !sf.contains_key(&Direction::West) {
            all_sf.push(SquareFence { x: x as isize, y: y as isize, direction: Direction::West });
        }

        all_sf
    }

    fn check_next_square_along<'a>(&self, possible_around_corner: &'a SquareFence, possible_straight_ahead: &'a SquareFence, possible_turn_right: &'a SquareFence) -> &'a SquareFence {
        let around_corner_exists = if possible_around_corner.x >= 0 && possible_around_corner.y >= 0 {
            self.field.squares.iter().find(|(x, y)| *x == possible_around_corner.x as usize && *y == possible_around_corner.y as usize).is_some()
        } else {
            false
        };

        let straight_ahead_exists = if possible_straight_ahead.x >= 0 && possible_straight_ahead.y >= 0 {
            self.field.squares.iter().find(|(x, y)| *x == possible_straight_ahead.x as usize && *y == possible_straight_ahead.y as usize).is_some()
        } else {
            false
        };

        if around_corner_exists {
            possible_around_corner
        } else if straight_ahead_exists {
            possible_straight_ahead
        } else {
            possible_turn_right
        }
    }

    fn step_along_fence(&self, square_fence: &SquareFence) -> SquareFence {
        let (possible_around_corner, possible_straight_along, possible_turn) = match square_fence.direction {
            Direction::North =>
                (
                    SquareFence { x: square_fence.x + 1, y: square_fence.y - 1, direction: Direction::West },
                    SquareFence { x: square_fence.x + 1, y: square_fence.y, direction: Direction::North },
                    SquareFence { x: square_fence.x, y: square_fence.y, direction: Direction::East }
                ),
                Direction::South => (
                    SquareFence { x: square_fence.x - 1, y: square_fence.y + 1, direction: Direction::East },
                    SquareFence { x: square_fence.x - 1, y: square_fence.y, direction: Direction::South },
                    SquareFence { x: square_fence.x, y: square_fence.y, direction: Direction::West }
                ),
                Direction::East => (
                    SquareFence { x: square_fence.x + 1, y: square_fence.y + 1, direction: Direction::North },
                    SquareFence { x: square_fence.x, y: square_fence.y + 1, direction: Direction::East },
                    SquareFence { x: square_fence.x, y: square_fence.y, direction: Direction::South }
                ),
                Direction::West => (
                    SquareFence { x: square_fence.x - 1, y: square_fence.y - 1, direction: Direction::South },
                    SquareFence { x: square_fence.x, y: square_fence.y - 1, direction: Direction::West },
                    SquareFence { x: square_fence.x, y: square_fence.y, direction: Direction::North }
                )
            };

        self.check_next_square_along(&possible_around_corner, &possible_straight_along, &possible_turn).clone()
    }
}
