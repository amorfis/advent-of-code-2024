use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;

enum Token {
    Mul(usize, usize),
    Do,
    Dont
}

impl Token {

    fn patterns() -> Vec<Regex> {
        vec![
            Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap(),
            Regex::new(r"do\(\)").unwrap(),
            Regex::new(r"don't\(\)").unwrap()
        ]
    }

    fn find_token( token: &Regex, str: &str) -> Option<(Token, String)> {
        token.find(str).map(|m| {
            let end = m.end();
            let tail = str.chars().skip(end).collect::<String>();
            let matched_token = Token::from_str(m.as_str()).unwrap();
            (matched_token, tail)
        })
    }

    fn from_str(s: &str) -> Option<Token> {
        let mul_pattern = &Token::patterns()[0];
        let do_pattern = &Token::patterns()[1];
        let dont_pattern = &Token::patterns()[2];

        if let Some(caps) = mul_pattern.captures(s) {
            let a = caps[1].parse::<usize>().unwrap();
            let b = caps[2].parse::<usize>().unwrap();
            return Some(Token::Mul(a, b));
        }

        if do_pattern.is_match(s) {
            return Some(Token::Do);
        }

        if dont_pattern.is_match(s) {
            return Some(Token::Dont);
        }

        None
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input/day3/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    let mut tail = contents.clone();
    let mut mul_on = true;
    loop {
        match find_first_token(tail.as_str()) {
            Some((token, str)) => {
                match token {
                    Token::Mul(a, b) =>
                        if mul_on {
                            sum = sum + a * b;
                        }
                    Token::Do => mul_on = true,
                    Token::Dont => mul_on = false
                }
                tail = str.to_string();
            },
            None => break
        }
    }

    println!("{:?}", sum);
    Ok(())
}

fn find_first_token(str: &str) -> Option<(Token, String)> {
    let maybe_found: Vec<Option<(Token, String)>> = Token::patterns().iter().map(|reg| Token::find_token(reg, str)).collect();
    let mut found: Vec<(Token, String)> = maybe_found.into_iter().filter_map(|x| x).collect::<Vec<_>>();

    found.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    found.pop()
}
