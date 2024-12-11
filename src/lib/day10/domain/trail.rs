use std::collections::HashSet;
use crate::day10::domain::{Direction, Map};

pub struct TrailHead {
    pub start_point: TrailPart,
}

impl TrailHead {
    pub fn calculate_score(&self) -> u32 {
        self.start_point.get_leaves().iter().filter(|t| t.height == 9).collect::<HashSet<_>>().iter().count() as u32
    }

    pub fn calculate_rating(&self) -> u32 {
        self.start_point.get_leaves().iter().filter(|t| t.height == 9).count() as u32
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct TrailPart {
    height: u32,
    x: u32,
    y: u32,
    north: Option<Box<TrailPart>>,
    south: Option<Box<TrailPart>>,
    west: Option<Box<TrailPart>>,
    east: Option<Box<TrailPart>>,
}

impl TrailPart {
    pub fn starting_point(x: u32, y: u32) -> TrailPart {
        TrailPart {
            height: 0,
            x,
            y,
            north: None,
            south: None,
            west: None,
            east: None,
        }
    }

    fn maybe_next_part(&self, maybe_adjacent_square: Option<(u32, u32, u32)>) -> Option<Box<TrailPart>> {
        maybe_adjacent_square.and_then(|(x, y, square_h)| {
            if square_h == self.height + 1 {
                return Some(Box::new(TrailPart {
                    height: square_h,
                    x,
                    y,
                    north: None,
                    south: None,
                    west: None,
                    east: None,
                }))
            } else {
                None
            }
        })
    }

    pub fn walk_up(&mut self, map: &Map) {

        // North
        self.north = self.maybe_next_part(map.adjacent_square(self.x, self.y, Direction::North));

        // South
        self.south = self.maybe_next_part(map.adjacent_square(self.x, self.y, Direction::South));

        // West
        self.west = self.maybe_next_part(map.adjacent_square(self.x, self.y, Direction::West));

        // East
        self.east = self.maybe_next_part(map.adjacent_square(self.x, self.y, Direction::East));

        self.north.as_mut().map(|t| t.walk_up(map));
        self.south.as_mut().map(|t| t.walk_up(map));
        self.west.as_mut().map(|t| t.walk_up(map));
        self.east.as_mut().map(|t| t.walk_up(map));
    }

    pub fn get_leaves(&self) -> Vec<&TrailPart> {
        let mut all_leaves = Vec::new();

        self.north.as_ref().map(|t| all_leaves.extend(t.get_leaves()));
        self.south.as_ref().map(|t| all_leaves.extend(t.get_leaves()));
        self.east.as_ref().map(|t| all_leaves.extend(t.get_leaves()));
        self.west.as_ref().map(|t| all_leaves.extend(t.get_leaves()));

        if all_leaves.is_empty() {
            vec![self]
        } else {
            all_leaves
        }
    }
}
