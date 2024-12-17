use std::collections::HashMap;
use uuid::Uuid;
use crate::day12::Direction;
use crate::day16::Reindeer;

pub struct State {
    pub cost: usize,
    pub facing: Direction,
}

pub struct Visited {
    pub visited: HashMap<(usize, usize), Vec<State>>
}