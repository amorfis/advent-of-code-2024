use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
struct Location {
    x: i32,
    y: i32
}

impl Location {
    pub fn add(&self, vector: &Vector) -> Location {
        Location{x: self.x + vector.x, y: self.y + vector.y}
    }
}

struct Vector {
    x: i32,
    y: i32
}

impl Vector {

    pub fn from_to(a: &Location, b: &Location) -> Vector {
        let x = b.x - a.x;
        let y = b.y - a.y;
        Vector{x, y}
    }

    pub fn multiply(&self, scalar: i32) -> Vector {
        Vector{x: self.x * scalar, y: self.y * scalar}
    }

    pub fn negate(&self) -> Vector {
        Vector{x: -self.x, y: -self.y}
    }
}

struct Board {
    width: usize,
    height: usize
}

impl Board {
    pub fn is_within_bounds(&self, loc: &Location) -> bool {
        loc.x >= 0 && loc.x < self.width as i32 && loc.y >= 0 && loc.y < self.height as i32
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input/day8/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let board = Board{width: contents.lines().next().unwrap().len(), height: contents.lines().count()};

    let mut antennas: HashMap<char, HashSet<Location>> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let current_localizations = antennas.entry(c).or_insert_with(HashSet::new);
                current_localizations.insert(Location {x: (x as i32), y: (y as i32)});
            }
        }
    }

    let mut points_locations = HashSet::new();

    let mut board_print_no = 0;

    for locations in antennas.values() {
        let mut aaa = locations.iter().collect::<Vec<_>>();
        aaa.sort();
        for antenna_loc in aaa {
            let similar_antennas = locations.iter().filter(|loc| **loc != *antenna_loc).collect::<Vec<_>>();
            let vectors = similar_antennas.iter().map(|loc| Vector::from_to(&antenna_loc, loc)).collect::<Vec<_>>();

            for vector in vectors {
                let point1 = antenna_loc.add(&vector.multiply(2));
                let point2 = antenna_loc.add(&vector.negate());


                println!("Board: {}", board_print_no);
                println!("Antenna: {:?}", antenna_loc);
                println!("Similar: {:?}", similar_antennas.clone());
                print_board(&antenna_loc, &vector, &point1, &point2, &board);
                board_print_no += 1;

                if board.is_within_bounds(&point1) {
                    points_locations.insert(point1);
                }
                if board.is_within_bounds(&point2) {
                    points_locations.insert(point2);
                }
            }
        }
    }

    println!("{:?}", points_locations.len());

    fn print_board(antenna: &Location, vector: &Vector, point1: &Location, point2: &Location, board: &Board) {
        let second_antenna = antenna.add(vector);
        for y in 0..board.height {
            for x in 0..board.width {
                if (x as i32, y as i32) == (antenna.x, antenna.y) {
                    print!("A");
                } else if (x as i32, y as i32) == (second_antenna.x, second_antenna.y) {
                    print!("A");
                } else if (x as i32, y as i32) == (point1.x, point1.y) {
                    print!("1");
                } else if (x as i32, y as i32) == (point2.x, point2.y) {
                    print!("2");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    Ok(())
}
