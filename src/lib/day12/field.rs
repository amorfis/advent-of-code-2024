use crate::day12::Direction;

#[derive(Debug)]
pub struct Field {
    pub squares: Vec<(usize, usize)>,
    pub crop: char
}

impl Field {

    pub fn print_field(&self, map: &Vec<Vec<char>>) {
        for y in 0..map[0].iter().count() {
            for x in 0..map.iter().count() {
                if self.squares.contains(&(x, y)) {
                    print!("{}", self.crop);
                } else {
                    print!(".");
                }
            }
            println!();
        }

    }

    pub fn adjacent_squares(&self, x: &i32, y: &i32) -> usize {
        let as_i = self.squares.iter().map(|(a, b)| (*a as i32, *b as i32)).collect::<Vec<(i32, i32)>>();

        let bools = vec![
            as_i.contains(&(x+1, *y)),
            as_i.contains(&(x-1, *y)),
            as_i.contains(&(*x, y+1)),
            as_i.contains(&(*x, y-1)),
        ];

        bools.iter().filter(|b| **b).count()
    }

}

pub struct FencedField<'a> {
    pub field: &'a Field,
    pub fences: usize
}
