
use crate::day12::Direction;
use crate::day12::Field;
use crate::day12::field::FencedField;

#[derive(Debug)]
pub struct Garden {
    map: Vec<Vec<char>>,
    fields: Vec<Field>
}

impl Garden {
    pub fn new(map: Vec<Vec<char>>) -> Garden {
        let mut g = Garden { map: map, fields: Vec::new() };
        g.initialize_fields();
        g
    }

    fn is_in_garden(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.map.iter().count() as i32 && y >= 0 && y < self.map[x as usize].iter().count() as i32
    }

    fn adjacent_field(&self, x: usize, y: usize, direction: Direction) -> Option<&Field> {
        let unchecked_adjacent_coords = match direction {
            Direction::North => (x as i32, y as i32 - 1),
            Direction::West => (x as i32 - 1, y as i32),
            _ => panic!("Not implemented")
        };

        if !self.is_in_garden(unchecked_adjacent_coords.0, unchecked_adjacent_coords.1) {
            return None;
        }

        self.fields.iter().find(|f| f.squares.contains(&(unchecked_adjacent_coords.0 as usize, unchecked_adjacent_coords.1 as usize)))
    }

    fn adjacent_field_mut(&mut self, x: usize, y: usize, direction: Direction) -> Option<&mut Field> {
        let unchecked_adjacent_coords = match direction {
            Direction::North => (x as i32, y as i32 - 1),
            // Direction::South => (x as i32, y as i32 + 1),
            Direction::West => (x as i32 - 1, y as i32),
            // Direction::East => (x as i32 + 1, y as i32)
            _ => panic!("Not implemented")
        };

        if !self.is_in_garden(unchecked_adjacent_coords.0, unchecked_adjacent_coords.1) {
            return None;
        }

        self.fields.iter_mut().find(|f| f.squares.contains(&(unchecked_adjacent_coords.0 as usize, unchecked_adjacent_coords.1 as usize)))
    }

    fn initialize_fields(&mut self) {
        for y in 0..self.map[0].iter().count() {
            for x in 0..self.map.iter().count() {
                print!("{}", self.map[x][y]);
            }
            println!()
        }

        for y in 0..self.map[0].iter().count() {
            for x in 0..self.map.iter().count() {
                if (x, y) == (0, 0) {
                    // Initialize first field
                    self.fields.push(Field { squares: vec![(x, y)], crop: self.map[x][y] });
                    continue;
                }

                let north_field = self.adjacent_field(x, y, Direction::North);
                let west_field = self.adjacent_field(x, y, Direction::West);

                if x == 0 {
                    // First field in a row
                    if north_field.unwrap().crop == self.map[x][y] {
                        self.adjacent_field_mut(x, y, Direction::North).unwrap().squares.push((x, y));
                    } else {
                        // new field
                        self.fields.push(Field { squares: vec![(x, y)], crop: self.map[x][y] });
                    }
                } else {
                    if y != 0 {
                        if north_field.unwrap().crop == self.map[x][y] && west_field.unwrap().crop == self.map[x][y] {
                            let ni = self.fields.iter().position(|f| std::ptr::eq(f, north_field.unwrap())).unwrap();
                            let wi = self.fields.iter().position(|f| std::ptr::eq(f, west_field.unwrap())).unwrap();

                            self.merge_fields(ni, wi);
                            self.adjacent_field_mut(x, y, Direction::North).unwrap().squares.push((x, y));
                        } else if north_field.unwrap().crop == self.map[x][y] {
                            self.adjacent_field_mut(x, y, Direction::North).unwrap().squares.push((x, y));
                        } else if west_field.unwrap().crop == self.map[x][y] {
                            self.adjacent_field_mut(x, y, Direction::West).unwrap().squares.push((x, y));
                        } else {
                            // new field
                            self.fields.push(Field { squares: vec![(x, y)], crop: self.map[x][y] });
                        }
                    } else {
                        if west_field.unwrap().crop == self.map[x][y] {
                            self.adjacent_field_mut(x, y, Direction::West).unwrap().squares.push((x, y));
                        } else {
                            // new field
                            self.fields.push(Field { squares: vec![(x, y)], crop: self.map[x][y] });
                        }
                    }
                }

                // // Find and print current field
                // let f = self.fields.iter().find(|f| f.squares.contains(&(x, y))).unwrap();
                // f.print_field(&self.map);
                // println!("------------------");
            }
        }
    }

    fn merge_fields(&mut self, field1_idx: usize, field2_idx: usize) {
        if (field1_idx == field2_idx) {
            return;
        }
        // self.fields[field1_idx].print_field(&self.map);
        // self.fields[field2_idx].print_field(&self.map);

        let field2_squares = std::mem::take(&mut self.fields[field2_idx].squares);

        let field1 = self.fields.get_mut(field1_idx).unwrap();
        field1.squares.extend(field2_squares);

        self.fields.remove(field2_idx);
    }

    pub fn print_fields(&self) {
        for f in self.fields.iter() {
            f.print_field(&self.map);
            println!("------------------");
        }
    }

    pub fn calculate_fences(&self) -> Vec<FencedField>{
        let mut fenced_fields: Vec<FencedField> = Vec::new();
        for field in self.fields.iter() {
            let mut fence = 0;
            for (x, y) in field.squares.iter() {
                let adjacent_squares = field.adjacent_squares(&(*x as i32), &(*y as i32));
                fence += 4 - adjacent_squares;
            }
            fenced_fields.push(FencedField { field: field, fences: fence });
        }

        fenced_fields
    }
}
