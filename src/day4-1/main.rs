use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day4/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let right_diagonals = calculate_diagonals(&contents);

    let mut reversed_content = Vec::new();

    for line in contents.lines() {
        let mut rev = line.chars().collect::<Vec<char>>();
        rev.reverse();
        reversed_content.push(rev.iter().collect::<String>());
    }

    let reversed_content_str = reversed_content.join("\n");

    let left_diagonals = calculate_diagonals(&reversed_content_str);

    let mut sum = 0;
    sum = sum + count_xmas(&contents.lines().collect());
    sum = sum + count_xmas(&calculate_columns(&contents).iter().map(|s| s.as_str()).collect());
    sum = sum + count_xmas(&right_diagonals.iter().map(|s| s.as_str()).collect());
    sum = sum + count_xmas(&left_diagonals.iter().map(|s| s.as_str()).collect());

    println!("{:?}", right_diagonals);
    println!("{:?}", left_diagonals);

    println!("{:?}", sum);
    Ok(())
}

fn calculate_columns(contents: &String) -> Vec<String> {
    let mut columns: Vec<String> = Vec::new();

    for i in 0..contents.lines().next().unwrap().len() {
        let column = contents.lines().map(|line| line.chars().nth(i).unwrap()).collect();
        columns.push(column);
    }

    columns
}

fn calculate_diagonals(contents: &String) -> Vec<String> {
    let first_line = contents.lines().next().unwrap();
    let last_column: String = contents.lines().map(|line| line.chars().last().unwrap()).collect();
    let mut right_diagonals: Vec<String> = Vec::new();

    for (i, c) in first_line.chars().enumerate() {
        let mut right_diagonal = Vec::new();
        right_diagonal.push(c);

        let mut char_below_idx: i32 = i as i32 - 1;
        let mut current_line_idx = 1;
        while char_below_idx >= 0 && current_line_idx < contents.lines().count() {
            let next_char = contents.lines().nth(current_line_idx).unwrap().chars().nth(char_below_idx as usize).unwrap();
            right_diagonal.push(next_char);
            char_below_idx = char_below_idx - 1;
            current_line_idx = current_line_idx + 1;
        }

        right_diagonals.push(right_diagonal.iter().collect());
    }

    for (i, c) in last_column.chars().enumerate() {
        // Skip first one, but we need to keep numbering consistent
        if i == 0 {
            continue;
        }

        let mut right_diagonal = Vec::new();
        right_diagonal.push(c);

        let mut current_line_idx = i + 1;
        let mut char_below_idx = first_line.len() - 2;
        while current_line_idx < contents.lines().count() {
            let next_char = contents.lines().nth(current_line_idx).unwrap().chars().nth(char_below_idx).unwrap();
            right_diagonal.push(next_char);
            char_below_idx = char_below_idx - 1;
            current_line_idx = current_line_idx + 1;
        }

        right_diagonals.push(right_diagonal.iter().collect());
    }

    right_diagonals
}

fn count_xmas(lines: &Vec<&str>) -> usize {
    let mut sum: usize = 0;
    let xmas = Regex::new(r"XMAS").unwrap();
    let samx = Regex::new(r"SAMX").unwrap();

    for line in lines {
        sum = sum + xmas.find_iter(line).count();
        sum = sum + samx.find_iter(line).count();
    }

    sum
}
