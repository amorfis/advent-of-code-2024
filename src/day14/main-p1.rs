use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;

struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

const WIDTH: usize = 101;
// const WIDTH: usize = 11;
const HEIGHT: usize = 103;
// const HEIGHT: usize = 7;

const SECONDS: usize = 100;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day14/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut robots = Vec::new();
    
    let re = Regex::new(r"p=(\d+),(\d)\s*v=(-?\d*),(-?\d*)").unwrap();
    for line in contents.lines() {
        re.captures(line).map(|caps| {
            let px = caps[1].parse::<i32>().unwrap();
            let py = caps[2].parse::<i32>().unwrap();
            let vx = caps[3].parse::<i32>().unwrap();
            let vy = caps[4].parse::<i32>().unwrap();
            
            robots.push(Robot { pos: (px, py), velocity: (vx, vy) });
        });
    }
    
    print_robots(&robots, (0, 0), (WIDTH, HEIGHT));
    
    for r in robots.iter_mut() {
        r.pos.0 = (r.pos.0 + r.velocity.0 * SECONDS as i32) % WIDTH as i32;
        r.pos.1 = (r.pos.1 + r.velocity.1 * SECONDS as i32) % HEIGHT as i32;
        
        if r.pos.0 < 0 {
            r.pos.0 = WIDTH as i32 + r.pos.0;
        }
        if r.pos.1 < 0 {
            r.pos.1 = HEIGHT as i32 + r.pos.1;
        }
    }
    
    println!("After {} seconds:", SECONDS);
    print_robots(&robots, (0, 0), (WIDTH, HEIGHT));
    
    let q_width = WIDTH / 2;
    let q_height = HEIGHT / 2;
    
    let q1 = count_robots_in_area((0, 0), (q_width as usize, q_height as usize), &robots);
    let q2 = count_robots_in_area((q_width as usize + 1, 0), (WIDTH, q_height as usize), &robots);
    let q3 = count_robots_in_area((0, q_height as usize + 1), (q_width, HEIGHT), &robots);
    let q4 = count_robots_in_area((q_width as usize + 1, q_height as usize + 1), (WIDTH, HEIGHT), &robots);
    
    println!("Q1: {}, Q2: {}, Q3: {}, Q4: {}", q1, q2, q3, q4);
    println!("{}", q1 * q2 * q3 * q4);
    
    Ok(())
}

fn count_robots_in_area(upper_left: (usize, usize), lower_right:  (usize, usize), robots: &Vec<Robot>) -> usize {
    println!("Counting robots in area: {:?} - {:?}", upper_left, lower_right);
    print_robots(robots, upper_left, lower_right);
    robots 
        .iter()
        .filter(|r| r.pos.0 >= upper_left.0 as i32 && r.pos.0 < lower_right.0 as i32 && r.pos.1 >= upper_left.1 as i32 && r.pos.1 < lower_right.1 as i32)
        .count()
}

fn print_robots(robots: &Vec<Robot>, upper_left: (usize, usize), lower_right:  (usize, usize)) {
    for y in upper_left.1..lower_right.1 {
        for x in upper_left.0..lower_right.0 {
            let robots_on_square = robots.iter().filter(|r| r.pos.0 == x as i32 && r.pos.1 == y as i32).count();
            if robots_on_square == 0 {
                print!(".");
            } else {
                print!("{}", robots_on_square);
            }
        }
        println!();
    }
}
