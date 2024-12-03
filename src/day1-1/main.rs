use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day1/lists.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        list1.push(words[0].parse::<i32>().unwrap());
        list2.push(words[1].parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut sum = 0;

    for i in 0..list1.len() {
        let dist = (list1[i] - list2[i]).abs();
        sum = sum + dist;
    }

    println!("{:?}", sum);
    Ok(())
}
