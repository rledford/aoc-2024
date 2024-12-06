use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Direction {
    x: isize,
    y: isize,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Board {
    board: Vec<Vec<char>>,
}

impl Board {
    fn get_sequence(&self, pos: &Position, dir: &Direction, steps: usize) -> Option<String> {
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

const UP: Direction = Direction { x: 0, y: -1 };
const R_UP: Direction = Direction { x: 1, y: -1 };
const RIGHT: Direction = Direction { x: 1, y: 0 };
const R_DOWN: Direction = Direction { x: 1, y: 1 };
const DOWN: Direction = Direction { x: 0, y: 1 };
const L_DOWN: Direction = Direction { x: -1, y: 1 };
const LEFT: Direction = Direction { x: -1, y: 0 };
const L_UP: Direction = Direction { x: -1, y: -1 };

const DIRECTIONS: [Direction; 8] = [UP, R_UP, RIGHT, R_DOWN, DOWN, L_DOWN, LEFT, L_UP];
const X_DIRECTIONS: [Direction; 4] = [R_UP, R_DOWN, L_DOWN, L_UP];

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = get_input(args.get(1));

    let board = Board { board: input };
    let mut total_xmas: usize = 0;
    let mut total_mas_crossings: usize = 0;
    let mut mas_crossing_set = HashSet::new();

    for i in 0..board.row_count() {
        for j in 0..board.col_count() {
            for dir in DIRECTIONS {
                if let Some(seq) = board.get_sequence(&Position { row: i, col: j }, &dir, 4) {
                    if seq.eq("XMAS") {
                        total_xmas += 1;
                    }
                }
                if X_DIRECTIONS.contains(&dir) {
                    if let Some(seq) = board.get_sequence(&Position { row: i, col: j }, &dir, 3) {
                        if seq.eq("MAS") {
                            let key = format!("{},{}", i as isize + dir.y, j as isize + dir.x);
                            if !mas_crossing_set.contains(&key) {
                                mas_crossing_set.insert(key);
                            } else {
                                total_mas_crossings += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("part 1 answer: {total_xmas}");
    println!("part 2 answer: {total_mas_crossings}");
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
