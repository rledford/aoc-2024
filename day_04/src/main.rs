use std::env;
use std::fs::read_to_string;

struct Position {
    row: usize,
    col: usize,
}

struct Direction {
    x: isize,
    y: isize,
}

struct Board {
    board: Vec<Vec<char>>,
}

impl Board {
    fn grab_sequence(&self, pos: &Position, dir: &Direction, steps: usize) -> Option<String> {
        let mut result: String = String::new();

        for i in 0..steps {
            let r = ((i as isize * dir.y) + pos.row as isize) as usize;
            let row: &Vec<char>;

            if let Some(v) = self.board.get(r) {
                row = v;
            } else {
                return None;
            }

            let c = ((i as isize * dir.x) + pos.col as isize) as usize;

            if let Some(ch) = row.get(c) {
                result.push(*ch);
            }
        }

        Some(result)
    }
    fn row_count(&self) -> usize {
        self.board.len()
    }
    fn col_count(&self) -> usize {
        if let Some(row) = self.board.first() {
            return row.len();
        }

        0
    }
}

const DIRECTIONS: [Direction; 8] = [
    Direction { x: 0, y: -1 },  // up
    Direction { x: 1, y: -1 },  // right-up
    Direction { x: 1, y: 0 },   // right
    Direction { x: 1, y: 1 },   // right-down
    Direction { x: 0, y: 1 },   // down
    Direction { x: -1, y: 1 },  // left-down
    Direction { x: -1, y: 0 },  // left
    Direction { x: -1, y: -1 }, // left-up
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = get_input(args.get(1));

    let board = Board { board: input };
    let mut total_xmas: usize = 0;

    for i in 0..board.row_count() {
        for j in 0..board.col_count() {
            for dir in DIRECTIONS {
                if let Some(seq) = board.grab_sequence(&Position { row: i, col: j }, &dir, 4) {
                    if seq.eq("XMAS") {
                        total_xmas += 1;
                    }
                }
            }
        }
    }

    println!("answer part 1: {total_xmas}");
}

fn get_input(path: Option<&String>) -> Vec<Vec<char>> {
    let mut data: Vec<Vec<char>> = Vec::new();
    if let Some(p) = path {
        for line in read_to_string(p).unwrap().lines() {
            data.push(line.chars().collect());
        }
    }

    // just generate some data to use
    data
}
