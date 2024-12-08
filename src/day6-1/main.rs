use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Turtle {
    x: i32,
    y: i32,
    direction: Direction
}

struct Board {
    cells: Vec<Vec<char>>
}

impl Board {
    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.cells[0].len() as i32 && y >= 0 && y < self.cells.len() as i32
    }

    fn is_cell_obstructed(&self, x: i32, y: i32) -> bool {
        self.cells[x as usize][y as usize] == '#'
    }
}
enum MoveResult {
    Obstructed,
    OutOfBoard,
    Moved
}

impl Turtle {
    fn try_move(&mut self, board: &Board, new_x: i32, new_y: i32) -> MoveResult {
        if !board.is_inside(new_x, new_y) {
            MoveResult::OutOfBoard
        } else if board.is_cell_obstructed(new_x, new_y) {
            MoveResult::Obstructed
        } else {
            self.x = new_x;
            self.y = new_y;
            MoveResult::Moved
        }
    }

    fn step(&mut self, board: &Board) -> bool {
        let (new_direction_if_obstructed, new_x, new_y) = match self.direction {
            Direction::Up => {
                (Direction::Right, self.x, self.y - 1)
            }
            Direction::Down => {
                (Direction::Left, self.x, self.y + 1)
            }
            Direction::Left => {
                (Direction::Up, self.x - 1, self.y)
            }
            Direction::Right => {
                (Direction::Down, self.x + 1, self.y)
            }
        };

        match self.try_move(board, new_x, new_y) {
            MoveResult::Obstructed => {
                self.direction = new_direction_if_obstructed;
                true
            }
            MoveResult::OutOfBoard => {
                false
            }
            MoveResult::Moved => {
                self.x = new_x;
                self.y = new_y;
                true
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input/day6/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rows_count = contents.lines().count();
    let cols_count = contents.lines().next().unwrap().len();

    let mut initial_board = vec![vec!['?'; cols_count]; rows_count];
    let mut visited_cells = HashSet::new();

    let mut maybe_turtle: Option<Turtle> = None;

    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                let turtle = Turtle {
                    x: j as i32,
                    y: i as i32,
                    direction: Direction::Up
                };
                maybe_turtle = Some(turtle);

                initial_board[j][i] = '.';
            } else {
                initial_board[j][i] = c;
            }
        }
    }

    let board = Board {
        cells: initial_board
    };
    let mut turtle = maybe_turtle.unwrap();

    loop {
        visited_cells.insert((turtle.x, turtle.y));
        let still_inside_board = turtle.step(&board);
        if !still_inside_board {
            break;
        }
    }

    println!("{:?}", visited_cells.len());
    Ok(())
}
