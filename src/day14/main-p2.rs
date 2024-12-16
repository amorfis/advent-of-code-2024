use std::fs::File;
use std::io;
use std::io::Read;
use regex::Regex;

struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const HALFLENGTH_LIMIT: usize = 2;
const LINES_DOWN: usize = 5;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day14/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut robots = Vec::new();

    let re = Regex::new(r"p=(\d+),(\d+)\s*v=(-?\d*),(-?\d*)").unwrap();
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
    
    let mut seconds = 0;
    loop {
        if is_christmas_tree(&robots) {
            break;
        }
        second_pass(&mut robots);
        seconds = seconds + 1;
        println!("{} seconds pass", seconds);
        // print_robots(&robots, (0, 0), (WIDTH, HEIGHT));
    }

    println!("After {} seconds:", seconds);
    print_robots(&robots, (0, 0), (WIDTH, HEIGHT));
    Ok(())
}

fn print_robots(robots: &Vec<Robot>, upper_left: (usize, usize), lower_right:  (usize, usize)) {
    let mut line_count = 0;
    for y in upper_left.1..lower_right.1 {
        print!("{:03}: ", line_count);
        line_count = line_count + 1;
        
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

fn second_pass(robots: &mut Vec<Robot>) {
    for r in robots.iter_mut() {
        r.pos.0 = (r.pos.0 + r.velocity.0) % WIDTH as i32;
        r.pos.1 = (r.pos.1 + r.velocity.1) % HEIGHT as i32;

        if r.pos.0 < 0 {
            r.pos.0 = WIDTH as i32 + r.pos.0;
        }
        if r.pos.1 < 0 {
            r.pos.1 = HEIGHT as i32 + r.pos.1;
        }
    }
}

fn is_christmas_tree(robots: &Vec<Robot>) -> bool {
    for robot in robots {
        if is_symmetric_down(robots, robot.pos.0 as usize, robot.pos.1 as usize, HALFLENGTH_LIMIT, LINES_DOWN) {
            println!("Symmetry found at ({}, {})", robot.pos.0, robot.pos.1);
            return true
        }
    }
    
    false
}

fn is_symmetric_down(robots: &Vec<Robot>, x: usize, y: usize, max_halflength: usize, depth: usize) -> bool {
    let halflengths = get_symmetric_robots(robots, y+1, x).into_iter().filter(|h| *h <= max_halflength).collect::<Vec<_>>();
    if halflengths.len() == 0 {
        false
    } else {
        if depth == 0 {
            true
        } else {
            is_symmetric_down(robots, x, y + 1, halflengths.into_iter().next().unwrap() + HALFLENGTH_LIMIT, depth - 1)
        }
    }
}

// fn is_symmetric_line_clean(robots_line: &Vec<bool>, symmetry_axis: usize, min_halflength: usize) -> i32 {
//     let mut halflength: i32 = -1;
//     for x in 0..(symmetry_axis + min_halflength) {
//         if robots_line[x] {
//             if has_symmetric_robot(robots_line, x, symmetry_axis) {
//                 if halflength == -1 {
//                     halflength = (symmetry_axis - x) as i32;
//                 }
//             } else {
//                 if halflength != -1 {
//                     return -1;
//                 }
//             }
//         }
//     }
//     
//     // if halflength != -1 {
//     //     println!("Symmetry found at axis {} with halflength {}", symmetry_axis, halflength);
//     //     print_line(robots_line);
//     // }
// 
//     halflength
// }

// fn is_symmetric_line(robots_line: &Vec<bool>, symmetry_axis: usize, min_halflength: usize) -> i32 {
//     let mut halflength: i32 = -1;
//     for x in 0..symmetry_axis {
//         if robots_line[x] {
//             if has_symmetric_robot(robots_line, x, symmetry_axis) {
//                 if halflength == -1 {
//                     halflength = (symmetry_axis - x) as i32;
//                 }
//             } else {
//                 if halflength != -1 {
//                     return -1;
//                 }
//             }
//         }
//     }
// 
//     // if halflength != -1 {
//     //     println!("Symmetry found at axis {} with halflength {}", symmetry_axis, halflength);
//     //     print_line(robots_line);
//     // }
// 
//     halflength
// }

fn get_symmetric_robots(robots: &Vec<Robot>, y: usize, symmetry_axis: usize) -> Vec<usize> {
    let mut symmetry_halflengths = Vec::new();
    let robots_line = robots.iter().filter(|r| r.pos.1 == y as i32).collect::<Vec<_>>();
    for x in 0..symmetry_axis {
        let robot = robots_line.iter().find(|r| r.pos.0 == x as i32);
        if robot.is_some() {
            if has_symmetric_robot(&robots_line, x, symmetry_axis) {
                // println!("Found symmetric robot at {x} axis {symmetry_axis}:");
                // print_line_with_marks(&robots_line, symmetry_axis, x);
                // has_symmetric_robot(&robots_line, x, symmetry_axis);
                
                symmetry_halflengths.push((symmetry_axis - x) as usize);
            }
        }
    }

    symmetry_halflengths
}

fn has_symmetric_robot(robots_line: &Vec<&Robot>, robot_x: usize, symmetry_axis: usize) -> bool {
    if robot_x == symmetry_axis {
        return true
    } else {
        if robot_x < symmetry_axis {
            return robots_line.iter().find(|r| r.pos.0 as usize == symmetry_axis + (symmetry_axis - robot_x)).is_some();
        }
        if robot_x > symmetry_axis {
            return robots_line.iter().find(|r| r.pos.0 as usize == symmetry_axis - (robot_x - symmetry_axis)).is_some();
        }
    }
    
    false
}

fn print_line(robots_line: &Vec<bool>) {
    println!();
    for i in 0..WIDTH {
        if robots_line[i] {
            print!("X");
        } else {
            print!(".");
        }
    }
    println!();
}

fn print_line_with_marks(robots_line: &Vec<&Robot>, axis: usize, sym_robot: usize) {
    println!();
    let robot_x = robots_line.iter().map(|r| r.pos.0 as usize).collect::<Vec<_>>();
    for i in 0..WIDTH {
        if robot_x.contains(&i) && sym_robot == i {
            print!("#");
        } else if axis == i {
            print!("|");
        } else if robot_x.contains(&i) { 
            print!("x");
        } else {
            print!(".");
        }
    }
    println!();
}
