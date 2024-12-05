use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day4/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let line_len = contents.lines().next().unwrap().len();
    //MAS-MAS
    let masmas = Regex::new(&format!(r"M.M.{{{}}}A.{{{}}}S.S", line_len - 2, line_len - 2)).unwrap();
    //MAS-SAM
    let massam = Regex::new(&format!(r"M.S.{{{}}}A.{{{}}}M.S", line_len - 2, line_len - 2)).unwrap();
    //SAM-MAS
    let sammas = Regex::new(&format!(r"S.M.{{{}}}A.{{{}}}S.M", line_len - 2, line_len - 2)).unwrap();
    //SAM-SAM
    let samsam = Regex::new(&format!(r"S.S.{{{}}}A.{{{}}}M.M", line_len - 2, line_len - 2)).unwrap();

    let mut sum = 0;

    for triple in contents.lines().into_iter().collect::<Vec<_>>().windows(3) {
        for t in triple.iter() {
            println!("{:?}", t);
        }
        let concatenated = triple.join("");
        [&masmas, &massam, &sammas, &samsam].iter().for_each(|re| {
            let count = count_matches(re, &concatenated);
            println!("{:?}", count);
            sum = sum + count;
        });
    }

    println!("{:?}", sum);

    Ok(())
}

fn count_matches(re: &Regex, line: &str) -> usize {
    let mut sum = 0;
    let mut start_pos = 0;

    loop {
        match re.find(&line[start_pos..]) {
            Some(m) => {
                start_pos = start_pos + m.start() + 1;
                sum = sum + 1
            }
            None => break
        }
    }

    sum
}
