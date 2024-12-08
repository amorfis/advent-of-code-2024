use std::fs::File;
use std::io;
use std::io::Read;
use radix_fmt::Radix;

enum Operation {
    Add,
    Multiply,
    Concatenate
}

fn op_from_digit(digit: char) -> Operation {
    match digit {
        '0' => Operation::Add,
        '1' => Operation::Multiply,
        '2' => Operation::Concatenate,
        _ => panic!("Invalid digit")
    }
}

impl Operation {
    fn execute(&self, a: u128, b: u128) -> u128 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => format!("{}{}", a, b).parse().unwrap()
        }
    }
}

struct Combination {
    numbers: Vec<u128>,
    result: u128
}

fn main() -> io::Result<()> {
    let mut file = File::open("input/day7/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut combinations = Vec::new();

    for line in contents.lines() {
        let s = line.split(":").collect::<Vec<&str>>();
        let result = s[0].parse::<u128>().unwrap();
        let numbers = s[1].split_whitespace().map(|n| n.parse::<u128>().unwrap()).collect::<Vec<u128>>();

        combinations.push(Combination {
            numbers: numbers.clone(),
            result: result
        });
    }

    let mut sum = 0;

    for combination in combinations {
        if process_combination(&combination) {
            sum = sum + combination.result;
        }
    }

    println!("{:?}", sum);
    Ok(())
}

fn process_combination(combination: &Combination) -> bool {
    let operations_count = combination.numbers.len() - 1;

    for i in 0..(3_i32.pow(operations_count as u32)) {

        let in_3 = Radix::new(i as u64, 3).to_string();
        let operations_as_str = format!("{:0>width$}", in_3, width = operations_count);

        // println!("{:?}", operations_as_str);

        let result = calculate_result(combination.numbers.clone(), operations_as_str.chars().collect());
        if result == combination.result {
            println!("Found result: {:?}", result);
            return true
        }
    }

    return false
}

fn calculate_result(numbers: Vec<u128>, operations: Vec<char>) -> u128 {
    let result = numbers.iter().skip(1).zip(operations).fold(numbers[0], |acc, (number, operation)| {
        let r = op_from_digit(operation).execute(acc, *number);
        r
    });

    result
}
