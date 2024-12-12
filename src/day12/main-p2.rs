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

    let mut price_sum = 0;

    for ff in fenced_fields.iter() {
        let field_sides = ff.count_sides();
        let price = ff.field.squares.len() * field_sides;
        println!("Field: {}, area: {}, sides: {}, price {}", ff.field.crop, ff.field.squares.len(), field_sides, ff.field.squares.len() * field_sides);

        price_sum += price
    }


    println!("Sum price: {}", price_sum);
    Ok(())
}
