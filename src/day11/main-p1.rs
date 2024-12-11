use std::io;
use std::fs::File;
use std::io::Read;
use lib::day10::domain::{Map, TrailHead};
use lib::day10::domain::TrailPart;

struct Stones {
    stones: Vec<i64>,
}

impl Stones {

    fn blink(&mut self) {
        // calculate splits
        let mut splits = Vec::new();
        for (idx, stone) in self.stones.iter().enumerate() {
            let stone_as_string = stone.to_string();
            if stone_as_string.chars().count() % 2 == 0 {
                let (left, right) = (stone_as_string.chars().take(stone_as_string.chars().count() / 2), stone_as_string.chars().skip(stone_as_string.chars().count() / 2));
                splits.push((idx, left.collect::<String>().parse::<i64>().unwrap(), right.collect::<String>().parse::<i64>().unwrap()));
            }
        }

        let splits_indices = splits.iter().map(|(idx, _, _)| *idx).collect::<Vec<usize>>();

        // splits calculated, but let's leave them for now. Now do other modifications
        for (idx, stone) in self.stones.iter_mut().enumerate() {
            if splits_indices.contains(&idx) {
                // We'll split this stone later
                continue;
            }

            if *stone == 0 {
                *stone = 1;
            } else {
                *stone = *stone * 2024;
            }
        }

        let mut shift = 0;
        for (split_idx, left, right) in splits {
            self.stones.insert(split_idx + shift, left);
            self.stones[split_idx + shift + 1] = right;
            shift = shift + 1;
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input/day11/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut stones: Vec<i64> = Vec::new();

    contents.trim().split(char::is_whitespace).for_each(|s| {
        let rr = s.parse();
        let number = rr.unwrap();
        stones.push(number);
    });

    let mut stones = Stones { stones: stones };

    println!("Stones: {:?}", stones.stones);
    for i in 0..25 {
        println!("Blink {}, {} stones", i, stones.stones.iter().count());
        stones.blink();
        println!("Stones: {:?}", stones.stones);
    }

    println!("Stones count: {:?}", stones.stones.iter().count());

    Ok(())
}
