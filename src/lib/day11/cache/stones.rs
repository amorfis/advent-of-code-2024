use std::collections::HashMap;

pub struct Stones {
    pub stones: HashMap<usize, usize>,
}

impl Stones {
    pub fn blink(&mut self) {
        let mut new_stones = HashMap::new();

        for (stone, old_count) in self.stones.iter() {
            let blinked = Self::blink_single_number(*stone);
            for new_stone in blinked {
                let count = new_stones.entry(new_stone).or_insert(0_usize);
                *count = *count + old_count;
            }
        }

        self.stones = new_stones;
    }

    pub fn blink_single_number(number: usize) -> Vec<usize> {
        let stone_as_string = number.to_string();
        if stone_as_string.chars().count() % 2 == 0 {
            let (left, right) = (stone_as_string.chars().take(stone_as_string.chars().count() / 2), stone_as_string.chars().skip(stone_as_string.chars().count() / 2));
            vec![left.collect::<String>().parse::<usize>().unwrap(), right.collect::<String>().parse::<usize>().unwrap()]
        } else if number == 0 {
            vec![1]
        } else {
            vec![number * 2024]
        }
    }

    pub fn count_stones(&self) -> usize {
        let mut sum = 0;
        for c in self.stones.values() {
            sum = sum + c;
        }

        sum
    }
}
