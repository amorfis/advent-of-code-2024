use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day2/reports.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut list1 = Vec::new();

    for line in contents.lines() {
        let www: Vec<&str> = line.split_whitespace().collect();
        let words: Vec<i32> = www.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        list1.push(words);
    }

    let mut counter = 0;

    for i in 0..list1.len() {
        if is_report_safe(&list1[i]) {
            counter = counter + 1;
        }
    }

    println!("{:?}", counter);
    Ok(())
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut sorted_asc = report.clone();
    sorted_asc.sort();
    let mut sorted_desc = report.clone();
    sorted_desc.sort_by(|a, b| b.cmp(a));

    if *report != sorted_asc && *report != sorted_desc {
        return false;
    }

    report.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        return diff > 0 && diff < 4;
    })
}
