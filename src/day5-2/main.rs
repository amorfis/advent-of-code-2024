use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day5/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rule_gen = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();

    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates = Vec::new();

    for line in contents.lines() {
        match rule_gen.captures(line) {
            Some(caps) => {
                let a = caps[1].parse::<u32>().unwrap();
                let b = caps[2].parse::<u32>().unwrap();
                rules.push((a, b));
            }
            None => {
                if !line.is_empty() {
                    updates.push(line.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
                }
            }
        }
    }

    let mut sum = 0;

    for u in updates.iter() {
        let broken_rule = find_broken_rule(u, &rules);
        if broken_rule.is_none() {
            continue;
        }

        let fixed_update = fix_update(&u, &rules);

        sum = sum + find_middle(&fixed_update)
    }

    println!("{:?}", sum);
    Ok(())
}

fn fix_update(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut working_update = update.clone();
    loop {
        let broken_rule = find_broken_rule(&working_update, rules);
        match broken_rule {
            None => {
                break;
            }
            Some((rule_before, rule_after)) => {
                let before_idx = working_update.iter().position(|&x| x == rule_before).unwrap();
                let after_idx = working_update.iter().position(|&x| x == rule_after).unwrap();

                working_update.swap(before_idx, after_idx);
            }
        }
    }

    working_update
}

fn find_broken_rule(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Option<(u32, u32)> {
    rules.iter().find(|(before, after)| {
        let before_idx = update.iter().position(|&x| x == *before);
        let after_idx = update.iter().position(|&x| x == *after);

        match (before_idx, after_idx) {
            (Some(b), Some(a)) => {
                if b > a {
                    true
                } else {
                    false
                }
            }
            (_, _) => false
        }
    }).map(|tuple| *tuple)
}

fn find_middle(update: &Vec<u32>) -> u32 {
    update.iter().skip(update.len() / 2).next().unwrap().clone()
}
