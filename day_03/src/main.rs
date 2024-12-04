use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = get_input(args.get(1));

    let mut slice: &str;
    let mut cursor: usize = 0;
    let mut answer_1: usize = 0;
    let mut answer_2: usize = 0;
    let mut is_mul_enabled = true;

    loop {
        slice = &input[cursor..];
        if slice.starts_with("m") {
            let (v, cur) = try_mul(slice);
            if let Some(result) = v {
                answer_1 += result;
                if is_mul_enabled {
                    answer_2 += result;
                }
            }

            cursor += cur;
        } else if slice.starts_with("d") {
            let (v, cur) = try_do_dont(slice);
            if let Some(result) = v {
                is_mul_enabled = result;
            }

            cursor += cur;
        } else {
            cursor += 1;
        }

        if cursor >= input.len() {
            break;
        }
    }

    println!("part 1 answer: {answer_1}");
    println!("part 2 answer: {answer_2}");
}

// Returns (Some(product), cursor_offset) when mul(a, b) found
// Returns (None, 1) when mul(a, b) not found
fn try_mul(slice: &str) -> (Option<usize>, usize) {
    if !slice.starts_with("mul(") {
        return (None, 1);
    }

    if let Some(closing_index) = slice.find(")") {
        let segment = &slice[4..closing_index];
        let parts: Vec<&str> = segment.split(",").collect();

        if parts.len() != 2 {
            return (None, 1);
        }

        if let Some(a) = parts.get(0).unwrap().parse::<usize>().ok() {
            if let Some(b) = parts.get(1).unwrap().parse::<usize>().ok() {
                return (Some(a * b), closing_index);
            }
        }
    }

    (None, 1)
}

/// Returns (Some(true), cursor_offset) if do() is found
/// Returns (Some(false), cursor_offset) if don't() is found
/// Returns (None, 1) otherwise
fn try_do_dont(slice: &str) -> (Option<bool>, usize) {
    if slice.starts_with("do()") {
        return (Some(true), 4);
    }

    if slice.starts_with("don't()") {
        return (Some(false), 7);
    }

    (None, 1)
}

fn get_input(path: Option<&String>) -> String {
    if let Some(p) = path {
        read_to_string(p).unwrap()
    } else {
        String::from("mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }
}
