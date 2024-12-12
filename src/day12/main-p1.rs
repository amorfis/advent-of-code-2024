use std::fs::File;
use std::io;
use std::io::Read;
use lib::day12::Garden;

fn main() -> io::Result<()> {
    let mut file = File::open("input/day12/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let width = contents.lines().next().unwrap().chars().count();
    let height = contents.lines().count();
    let mut raw_map: Vec<Vec<char>> = vec![vec!['.'; height]; width];

    for (y, line) in contents.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            raw_map[x][y] = ch;

        }
    }

    let garden = Garden::new(raw_map);

    // garden.print_fields();

    let fenced_fields = garden.calculate_fences();

    let mut sum = 0;

    for ff in fenced_fields.iter() {
        sum += ff.fences * ff.field.squares.len();
    }

    println!("Sum of fences: {}", sum);
    Ok(())
}
