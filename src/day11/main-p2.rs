use std::collections::{HashSet, LinkedList};
use std::io;
use std::fs::File;
use std::io::Read;
use std::thread::spawn;

struct Stones {
    stones: LinkedList<i64>,
}

impl Stones {

    fn blink_by_threads(&mut self, depth: usize) {
        if depth >= 4 {
            self.blink();
        } else {
            let count = self.stones.iter().count();
            if count > 10000 {
                let right_side = self.stones.split_off(count / 2);
                let mut right_stones = Stones { stones: right_side };
                println!("Spawning new thread at depth {}", depth);
                let child_thread = spawn(move || {
                    right_stones.blink_by_threads(depth + 1);
                    right_stones.stones
                });
                self.blink_by_threads(depth + 1);
                let right_blinked = child_thread.join();
                println!("Joined thread at depth {}", depth);

                self.stones.append(&mut right_blinked.unwrap());
            } else {
                self.blink();
            }
        }
    }

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

        let splits_indices = splits.iter().map(|(idx, _, _)| *idx).collect::<HashSet<usize>>();

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

        let mut parts = Vec::new();
        for (split_idx, left, right) in splits.iter().rev() {

            let mut right_part = self.stones.split_off(*split_idx);
            *right_part.front_mut().unwrap() = *right;

            parts.push(right_part);

            self.stones.push_back(*left);
        }

        for part in parts.iter_mut().rev() {
            self.stones.append(part);
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input/day11/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut stones: LinkedList<i64> = LinkedList::new();

    contents.trim().split(char::is_whitespace).for_each(|s| {
        let rr = s.parse();
        let number = rr.unwrap();
        stones.push_back(number);
    });

    let mut stones = Stones { stones };

    println!("Stones: {:?}", stones.stones);
    for i in 0..75 {
        println!("Blink {}, {} stones", i, stones.stones.iter().count());
        stones.blink_by_threads(0);
        // println!("Stones: {:?}", stones.stones);
    }

    println!("Stones count: {:?}", stones.stones.iter().count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    #[test]
    fn test() {
        let mut linked_list = LinkedList::new();
        linked_list.push_back(1);
        linked_list.push_back(2);
        linked_list.push_back(3);
        linked_list.push_back(4);

        let mut right_part = linked_list.split_off(2);
        *right_part.front_mut().unwrap() = 5;

        linked_list.append(&mut right_part);

        assert_eq!(linked_list.iter().collect::<Vec<&i32>>(), vec![&1, &2, &5, &4]);
    }
}
