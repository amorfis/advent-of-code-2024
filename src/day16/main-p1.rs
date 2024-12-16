use std::fs::File;
use std::io;
use std::io::Read;
use lib::day12::Direction;
use lib::day16::{Maze, Reindeer};


fn main() -> io::Result<()> {
    let mut file = File::open("input/day16/test_input.txt")?;
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
            .flat_map(|w| w.next_possibilities(&maze))
            .collect();
        // for w in walkers {
        //     new_walkers.extend(w.next_possibilities());
        // };
        walkers = new_walkers;
        if walkers.iter().all(|w| w.finished) {
            break;
        }
        
        // walkers.iter().for_each(|w| w.print_steps());
    }
    
    let scores = walkers.iter().filter(|w| w.finished).map(|w| w.calculate_score()).collect::<Vec<_>>();
    
    println!("Min score {}", scores.iter().min().unwrap());

    // println!("Sum of fences: {}", sum);
    Ok(())
}
