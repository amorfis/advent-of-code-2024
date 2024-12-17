use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use lib::day12::Direction;
use lib::day16::{Maze, Reindeer, Visited};
use uuid::{uuid, Uuid};

fn main() -> io::Result<()> {
    let mut file = File::open("input/day16/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let width = contents.lines().next().unwrap().chars().count();
    let height = contents.lines().count();
    let mut raw_map: Vec<Vec<char>> = vec![vec!['.'; height]; width];

    for (y, line) in contents.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            raw_map[x][y] = ch;

        }
    }

    let maze = Maze::new(raw_map);
    let mut visited = Visited { visited: HashMap::new() };

    let mut walker = Reindeer {
        position: maze.start(),
        facing: Direction::East,
        steps_taken: Vec::new(),
        finished: false
    };

    let mut counter = 0;
    let mut walkers = vec![walker.clone()];
    loop {
        println!("Iteration {}, walkers {}", counter, walkers.len());
        counter += 1;
        let new_walkers: Vec<_> = walkers
            .iter_mut()
            .flat_map(|w| w.next_possibilities(&maze, &mut visited))
            .collect();
        // for w in walkers {
        //     new_walkers.extend(w.next_possibilities());
        // };
        walkers = new_walkers;
        if walkers.iter().all(|w| w.finished) {
            break;
        }

        let finished = walkers.iter().find(|w| w.finished);
        if finished.is_some() {
            println!("Found a finished walker with cost {}", finished.unwrap().calculate_score());
        }

        // walkers.iter().for_each(|w| w.print_steps());
    }

    let mut finished_ones = walkers.iter().filter(|w| w.finished).collect::<Vec<_>>();
    finished_ones.sort_by(|a, b| a.calculate_score().cmp(&b.calculate_score()));

    println!("Finished ones: {}", finished_ones.len());

    println!("Min score {}", finished_ones.iter().next().unwrap().calculate_score());

    finished_ones.iter().next().unwrap().print_steps(&maze);

    // println!("Sum of fences: {}", sum);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reindeer() {
        let mut v = vec![0, 2, 89, 2, 23, 5];
        v.sort_by(|a, b| a.cmp(b));
        assert_eq!(v, vec![0, 2, 2, 5, 23, 89]);
    }
}
