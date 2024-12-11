use std::{collections::HashSet, env, fs::read_to_string};

fn main() {
    // solution part 1
    // walk the 2d array until the next index would leave the bounds and
    // keep track of each "distinct" position visited - the answer will
    // be the len() of the set
    //
    // solution part 2
    // given the guard starts in the same position, the distinct
    // visited positions from part 1 can be used as candidates
    // for new obstruction placement
    //
    // after adding a new obstruction to one of the known visit
    // positions, run the same check as in part one, but
    // verify that the guard never exits the grid
    //
    // we cand determine that the guard is stuck in a loop by
    // predetermining the maximum distinct visitable positions
    // on the entire grid (all the '.'s), then track how many
    // steps the guard has taken since the last "new" grid
    // position has been visited and if that number is >=
    // the max visitable positions in the grid, then the
    // guard must be stuck in a loop

    let args: Vec<String> = env::args().collect();
    let (map, start_pos) = get_input(args.get(1));
    let (row, col) = start_pos;
    let grid = Grid::new(map);

    let distinct_visited_positions = get_distinct_visited_positions(&grid, GridPos::new(row, col));
    let total_distinct_visits = distinct_visited_positions.len();
    let total_loop_obstruction_positions = get_total_loop_obstruction_positions(
        &grid,
        &distinct_visited_positions,
        GridPos::new(row, col),
    );
    println!("part 1 answer: {total_distinct_visits}");
    println!("part 2 answer: {total_loop_obstruction_positions}");
}

fn get_total_loop_obstruction_positions(
    grid: &Grid,
    distinct_visited_positions: &HashSet<GridPos>,
    start: GridPos,
) -> usize {
    let mut result: usize = 0;

    let mut test_data: Vec<Vec<char>> = Vec::new();

    for r in &grid.data {
        let mut row: Vec<char> = Vec::new();
        for c in r {
            row.push(*c);
        }
        test_data.push(row);
    }

    let mut test_grid = Grid::new(test_data);

    let total_visitable_positions = test_grid.total_visitable_positions();

    for obstruction_pos in distinct_visited_positions {
        dbg!(&obstruction_pos);
        if obstruction_pos == &start {
            continue;
        }

        test_grid.replace(obstruction_pos, '#');

        // if this exceed total_visitable_positions, we're in a loop
        let mut steps_since_last_distinct_position = 0;
        let mut pos = GridPos::new(start.row, start.col);
        let mut dir = Direction::up();
        let mut visited: HashSet<GridPos> = HashSet::new();

        // yes, this sort of duplicates what's in the fn for part 1 answer
        while let Some(next_pos) = test_grid.next(&pos, &dir) {
            let ch = test_grid.char_at(&next_pos);
            if !test_grid.is_visitable(&ch) {
                dir = dir.rotated90();
                continue;
            }

            if visited.contains(&pos) {
                steps_since_last_distinct_position += 1;
            } else {
                steps_since_last_distinct_position = 0;
            }

            if steps_since_last_distinct_position > total_visitable_positions {
                result += 1;
                break;
            }

            visited.insert(GridPos::new(next_pos.row, next_pos.col));
            pos = next_pos;
        }

        test_grid.replace(obstruction_pos, '.');
    }

    println!("{total_visitable_positions}");

    result
}
fn get_distinct_visited_positions(grid: &Grid, start: GridPos) -> HashSet<GridPos> {
    let mut pos = GridPos::new(start.row, start.col);
    let mut dir = Direction::up();
    let mut visited: HashSet<GridPos> = HashSet::new();

    visited.insert(GridPos::new(start.row, start.col));

    while let Some(next_pos) = grid.next(&pos, &dir) {
        let ch = grid.char_at(&next_pos);
        if !grid.is_visitable(&ch) {
            dir = dir.rotated90();
            continue;
        }

        visited.insert(GridPos::new(next_pos.row, next_pos.col));
        pos = next_pos;
    }

    visited
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
    fn is_visitable(&self, ch: &char) -> bool {
        ch == &'.'
    }
    fn place_obstruction(&mut self, pos: &GridPos) {
        self.replace(pos, '#');
    }
    fn remove_obstruction(&mut self, pos: &GridPos) {
        self.replace(pos, '.');
    }
    fn replace(&mut self, pos: &GridPos, ch: char) {
        let r = self.data.get_mut(pos.row).unwrap();
        r.remove(pos.col);
        r.insert(pos.col, ch);
    }
    fn total_visitable_positions(&self) -> usize {
        let mut result: usize = 0;
        for r in self.data.iter() {
            for ch in r.iter() {
                if !self.is_visitable(ch) {
                    continue;
                }
                result += 1;
            }
        }

        result
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
