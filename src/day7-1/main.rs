use std::fs::File;
use std::io;
use std::io::Read;

enum Operation {
    Add,
    Multiply
}

fn op_from_digit(digit: char) -> Operation {
    match digit {
        '0' => Operation::Add,
        '1' => Operation::Multiply,
        _ => panic!("Invalid digit")
    }
}

impl Operation {
    fn execute(&self, a: i128, b: i128) -> i128 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b
        }
    }
}

struct Combination {
    numbers: Vec<i128>,
    result: i128
}

fn main() -> io::Result<()> {
    let mut file = File::open("input/day7/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut combinations = Vec::new();

    for line in contents.lines() {
        let s = line.split(":").collect::<Vec<&str>>();
        let result = s[0].parse::<i128>().unwrap();
        let numbers = s[1].split_whitespace().map(|n| n.parse::<i128>().unwrap()).collect::<Vec<i128>>();

        combinations.push(Combination {
            numbers: numbers.clone(),
            result: result
        });

        println!("{:?}", &numbers);
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
    for i in 0..(1 << operations_count) {

        let operations_as_str = format!("{:0width$b}", i, width = operations_count);

        let result = calculate_result(combination.numbers.clone(), operations_as_str.chars().collect());
        if result == combination.result {
            println!("Found result: {:?}", result);
            return true
        }
    }

    return false
}

fn calculate_result(numbers: Vec<i128>, operations: Vec<char>) -> i128 {
    let result = numbers.iter().skip(1).zip(operations).fold(numbers[0], |acc, (number, operation)| {
        let r = op_from_digit(operation).execute(acc, *number);
        r
    });

    result
}
