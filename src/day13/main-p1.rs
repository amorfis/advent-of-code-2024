use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;

struct Machine {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct ButtonPresses {
    a: i32,
    b: i32,
}

impl ButtonPresses {
    fn distance(&self, a_travel: i32, b_travel: i32) -> i32 {
        (self.a * a_travel) + (self.b * b_travel)
    }
}

impl Machine {
    fn find_max_presses(travel: i32, price_position: i32) -> i32 {
        let max_or_lower = price_position / travel;
        let max_a = if max_or_lower * travel == price_position {
            max_or_lower
        } else {
            max_or_lower + 1
        };

        max_a
    }

    fn get_presses_on_x_axis_for_price(&self) -> HashSet<ButtonPresses> {
        let max_a_presses = Machine::find_max_presses(self.a.0, self.prize.0);

        let mut presses = ButtonPresses { a: max_a_presses, b: 0 };
        let mut valid_presses = HashSet::new();
        loop {
            if presses.a == -1 {
                break;
            }
            if presses.distance(self.a.0, self.b.0) == self.prize.0 {
                valid_presses.insert(presses.clone());
                presses.a = presses.a - 1;
                presses.b = presses.b + 1;
            } else if presses.distance(self.a.0, self.b.0) > self.prize.0 {
                presses.a = presses.a - 1;
            } else {
                presses.b = presses.b + 1;
            }
        }

        valid_presses
    }

    fn get_presses_on_y_axis_for_price(&self) -> HashSet<ButtonPresses> {
        let max_a_presses = Machine::find_max_presses(self.a.1, self.prize.1);

        let mut presses = ButtonPresses { a: max_a_presses, b: 0 };
        let mut valid_presses = HashSet::new();
        loop {
            if presses.a == -1 {
                break;
            }
            if presses.distance(self.a.1, self.b.1) == self.prize.1 {
                valid_presses.insert(presses.clone());
                presses.a = presses.a - 1;
                presses.b = presses.b + 1;
            } else if presses.distance(self.a.1, self.b.1) > self.prize.1 {
                presses.a = presses.a - 1;
            } else {
                presses.b = presses.b + 1;
            }
        }

        valid_presses
    }

    fn get_presses_for_prize(&self) -> Vec<ButtonPresses> {
        let x_presses = self.get_presses_on_x_axis_for_price();
        let y_presses = self.get_presses_on_y_axis_for_price();

        let int: Vec<_> = x_presses.intersection(&y_presses).cloned().collect();

        int
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input/day13/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut machines = Vec::new();

    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    re.captures_iter(contents.as_str()).for_each(|caps| {
        let ax = caps[1].parse::<i32>().unwrap();
        let ay = caps[2].parse::<i32>().unwrap();
        let bx = caps[3].parse::<i32>().unwrap();
        let by = caps[4].parse::<i32>().unwrap();
        let px = caps[5].parse::<i32>().unwrap();
        let py = caps[6].parse::<i32>().unwrap();

        machines.push(Machine { a: (ax, ay), b: (bx, by), prize: (px, py) });
    });

    let mut counter = 0;
    let mut sum = 0;
    for m in machines {
        let pos = m.get_presses_for_prize();
        println!("Machine: {}, possibilities: {}", counter, pos.len());

        for p in pos {
            let price = p.a * 3 + p.b;
            println!("{:?}, price {}", p, price);
            sum = sum + price;
        }

        counter = counter + 1;
    }

    println!("price sum {}", sum);
    Ok(())
}
