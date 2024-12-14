use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;
use lib::day13::Machine;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day13/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut machines = Vec::new();

    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    re.captures_iter(contents.as_str()).for_each(|caps| {
        let ax = caps[1].parse::<i64>().unwrap();
        let ay = caps[2].parse::<i64>().unwrap();
        let bx = caps[3].parse::<i64>().unwrap();
        let by = caps[4].parse::<i64>().unwrap();
        let px = caps[5].parse::<i64>().unwrap();
        let py = caps[6].parse::<i64>().unwrap();

        machines.push(Machine::new((ax, ay), (bx, by), (px + 10000000000000, py + 10000000000000)));
    });

    let mut counter = 0;
    let mut sum = 0;
    
    // let mmm = Machine { a: (1, 1), b: (1, 1), prize: (3, 4) };
    // let bw = mmm.get_best_way_to_prize();
    // println!("{:?}", bw);
    
    for m in machines.iter() {
        let way = m.get_best_way_to_prize();
        println!("Machine: {}, possible: {:?}", counter, way);
        counter = counter + 1;
    }

    println!("price sum {}", sum);
    Ok(())
}
