use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::Read;
use lib::day11::cache::Stones;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day11/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut init_stones = HashMap::new();

    contents.trim().split(char::is_whitespace).for_each(|s| {
        let number = s.parse().unwrap();
        let stones_count = init_stones.entry(number).or_insert(0_usize);
        *stones_count += 1;
    });


    let mut stones = Stones { stones: init_stones };

    println!("Stones initial: {:?}", stones.stones);

    for i in 0..75 {
        stones.blink();
        println!("Blinked {} times, {} stones", i+1, stones.count_stones());
        // println!("Stones: {:?}", stones.stones_states);
    }

    println!("{}", stones.count_stones());

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
