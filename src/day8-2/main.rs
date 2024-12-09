use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(PartialEq, Eq, Hash, Debug, Ord, PartialOrd, Clone)]
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

    for locations in antennas.values() {
        let mut aaa = locations.iter().collect::<Vec<_>>();
        aaa.sort();
        for antenna_loc in aaa {
            let similar_antennas = locations.iter().filter(|loc| **loc != *antenna_loc).collect::<Vec<_>>();
            let vectors = similar_antennas.iter().map(|loc| Vector::from_to(&antenna_loc, loc)).collect::<Vec<_>>();

            for vector in vectors {
                let new_points = points_for_vector(&antenna_loc, &vector, &board);
                points_locations = points_locations.union(&new_points).cloned().collect();
            }
        }
    }

    println!("{:?}", points_locations.len());

    fn points_for_vector(antenna_loc: &Location, vector: &Vector, board: &Board) -> HashSet<Location> {
        let mut points = HashSet::new();
        let mut point = antenna_loc.clone();
        loop {
            point = point.add(vector);
            if board.is_within_bounds(&point) {
                points.insert(point.clone());
            } else {
                break;
            }
        }

        loop {
            point = point.add(&vector.negate());
            if board.is_within_bounds(&point) {
                points.insert(point.clone());
            } else {
                break;
            }
        }

        points
    }

    Ok(())
}
