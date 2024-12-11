use crate::day10::domain::Direction;

pub struct Map {
    pub map: Vec<Vec<u32>>
}

impl Map {
    //returns new square (x, y, height)
    pub fn adjacent_square(&self, x: u32, y: u32, direction: Direction) -> Option<(u32, u32, u32)> {
        match direction {
            Direction::North => {
                if self.in_map(x as i32, y as i32 - 1) {
                    return Some((x, y - 1, self.map[x as usize][y as usize -1]))
                }
            },
            Direction::South => {
                if self.in_map(x as i32, y as i32 + 1) {
                    return Some((x, y + 1, self.map[x as usize][y as usize + 1]))
                }
            },
            Direction::East => {
                if self.in_map(x as i32 + 1, y as i32) {
                    return Some((x + 1, y, self.map[x as usize + 1][y as usize]))
                }
            },
            Direction::West => {
                if self.in_map(x as i32 - 1, y as i32) {
                    return Some((x - 1, y, self.map[x as usize - 1][y as usize]))
                }
            },
        }
        None
    }

    fn in_map(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.map.len() as i32 && y >= 0 && y < self.map[0].len() as i32
    }
}
