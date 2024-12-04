use std::env;
use std::fs::read_to_string;

const INSTR: &str = "mul";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = get_input(args.get(1));

    let mut slice: &str; // moving window slice
    let mut cursor: usize = 0;
    let mut answer_1: usize = 0;

    loop {
        slice = &input[cursor..];
        println!("{slice}");

        if let Some(cur) = next_instruction_predicate(INSTR, slice) {
            println!("next cursor position {cur}");
            cursor += cur;
        } else {
            println!("no more instructions");
            break;
        }

        slice = &input[cursor..];

        let (operands, cur) = get_instruction_operands(slice);

        if let Some((a, b)) = operands {
            answer_1 += a * b;
        }

        cursor += cur;
    }

    println!("part 1 answer: {answer_1}");
}

/// Returns the index of the first character that follows "mul" in the provided slice
fn next_instruction_predicate(instruction: &str, slice: &str) -> Option<usize> {
    if slice.len() < 3 {
        return None;
    }

    let mut cmp: &str;
    let offset = instruction.len();

    for i in 0..(slice.len() - offset) {
        cmp = &slice[i..(i + offset)];
        if cmp.eq(instruction) {
            return Some(i + offset);
        }
    }

    None
}

/// Returns values extracted from the provided slice that appear within "(" and ")"
/// and an index offset of the last successfully parsed character. If the search was
/// stopped due to no matching sequence found, e.g. "(12,mul)" then the returned index
/// offset will be the position in the slice that the match logic stopped.
///
/// Example: "(12,mul)" would return (None, 4) since 4 is the index of the "m"
/// because the "m" was not an expected character of the operand sequence, indicating
/// that the search for the next "mul" should start at index 4 (relative to the slice)
fn get_instruction_operands(slice: &str) -> (Option<(usize, usize)>, usize) {
    println!("getting operands");
    if slice.len() < 5 || !slice.starts_with("(") {
        println!("input not long enough");
        return (None, 0);
    }

    if let Some(closing_index) = slice.find(")") {
        println!("has cloing parenthases at {closing_index}");
        let segment = &slice[1..closing_index];
        let parts: Vec<&str> = segment.split(",").collect();

        dbg!(&parts);

        if parts.len() != 2 {
            return (None, 0);
        }

        if let Some(a) = parts.get(0).unwrap().parse::<usize>().ok() {
            if let Some(b) = parts.get(1).unwrap().parse::<usize>().ok() {
                return (Some((a, b)), segment.len());
            }
        }
    } else {
        println!("no closing parenthases");
    }

    (None, 0)
}

fn get_input(path: Option<&String>) -> String {
    if let Some(p) = path {
        read_to_string(p).unwrap()
    } else {
        String::from("mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }
}
