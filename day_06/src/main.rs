use std::{collections::HashSet, env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (map, start_pos) = get_input(args.get(1));

    let (row, col) = start_pos;

    let mut pos = GridPos::new(row, col);
    let mut dir = Direction::up();
    let mut visited: HashSet<GridPos> = HashSet::new();

    // solution part 1
    // walk the 2d array until the next index would leave the bounds and
    // keep track of each "distinct" position visited - the answer will
    // be the len() of the set

    visited.insert(GridPos::new(row, col));

    let grid = Grid::new(map);

    while let Some(next_pos) = grid.next(&pos, &dir) {
        let ch = grid.char_at(&next_pos);
        if !grid.is_passable(&ch) {
            dir = dir.rotated90();
            continue;
        }

        visited.insert(GridPos::new(next_pos.row, next_pos.col));
        pos = next_pos;
    }

    let total_distinct_visits = visited.len();
    println!("part 1 answer: {total_distinct_visits}");
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        Grid { data }
    }
    /// returns the next grid position in the provided direction relative
    /// to the start position
    /// returns None if the next position would be out of the grid bounds
    fn next(&self, start: &GridPos, dir: &Direction) -> Option<GridPos> {
        let next_row = (start.row as isize) + dir.y;
        let next_col = (start.col as isize) + dir.x;

        if next_row < 0 || next_row >= self.data.len() as isize {
            return None;
        }

        if next_col < 0 || next_col >= self.data.first().unwrap().len() as isize {
            return None;
        }

        Some(GridPos::new(next_row as usize, next_col as usize))
    }
    fn char_at(&self, pos: &GridPos) -> char {
        *self.data.get(pos.row).unwrap().get(pos.col).unwrap()
    }
    fn is_passable(&self, ch: &char) -> bool {
        ch != &'#'
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct GridPos {
    row: usize,
    col: usize,
}

impl GridPos {
    fn new(row: usize, col: usize) -> Self {
        GridPos { row, col }
    }
}

#[derive(Debug)]
struct Direction {
    y: isize,
    x: isize,
}

impl Direction {
    fn new(y: isize, x: isize) -> Self {
        Direction { y, x }
    }
    fn up() -> Self {
        Direction { y: -1, x: 0 }
    }
    fn rotated90(&self) -> Self {
        Direction::new(self.x, -self.y)
    }
}

fn get_input(path: Option<&String>) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut result: Vec<Vec<char>> = Vec::new();
    let fallback_path = String::from("test.txt");
    let mut start_pos: (usize, usize) = (0, 0);

    if let Some(p) = path.or(Some(&fallback_path)) {
        let lines: Vec<String> = read_to_string(p)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        for row in 0..lines.len() {
            let mut line: Vec<char> = lines.get(row).unwrap().chars().collect();

            if let Some(col) = line.iter().position(|&c| c == '^') {
                line[col] = '.';
                start_pos = (row, col);
            }

            result.push(line);
        }
    }

    (result, start_pos)
}
