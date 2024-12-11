use std::io;
use std::fs::File;
use std::io::Read;
use lib::day10::domain::{Map, TrailHead};
use lib::day10::domain::TrailPart;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day10/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let width = contents.lines().next().unwrap().len();
    let height = contents.lines().count();

    let mut raw_map: Vec<Vec<u32>> = vec![vec![0; width]; height];
    let mut trails = Vec::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            raw_map[x][y] = c.to_digit(10).unwrap();
            if c == '0' {
                trails.push(TrailHead { start_point: TrailPart::starting_point(x as u32, y as u32) } );
            }
        }
    }

    let map = Map { map: raw_map };

    for trail_head in trails.iter_mut() {
        trail_head.start_point.walk_up(&map);
    }

    let mut sum = 0;
    for trail_head in trails.iter() {
        sum = sum + trail_head.calculate_rating();
    }

    println!("Rating is: {}", sum);
    Ok(())
}
