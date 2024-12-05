use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day3/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(contents.as_str()).for_each(|caps| {
        let a = caps[1].parse::<i32>().unwrap();
        let b = caps[2].parse::<i32>().unwrap();
        println!("{:?}", a * b);
        sum = sum + a * b;
    });

    println!("{:?}", sum);
    Ok(())
}
