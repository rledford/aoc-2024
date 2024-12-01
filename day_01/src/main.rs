use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

// references
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

fn main() {
    let args: Vec<String> = env::args().collect();
    let (mut l, mut r) = get_input(args.get(1));
    let mut l_similarity: HashMap<usize, usize> = HashMap::new();
    let mut r_similarity: HashMap<usize, usize> = HashMap::new();

    // answer_1 solution algo
    // once we have the two lists, we want to sequentially diff the smallest numbers
    // numbers in each list. the diff should always be positive, so in stead of
    // determining which side is higher, we'll just use the absolute value of the
    // subtraction result and always do "left - right".
    // - sort the lists in ascending order
    // - iterate from 0 to list.len()
    // - result should be accumulated as "result += abs(l[i] - r[i])"
    //
    // answer_2 solution algo
    // using hashmaps for the left and right lists, we track the occurrences of values
    // in the left and right lists. for the left list, we only need to add a key entry
    // but for the right list we need to accumulate the number of occurrences. once
    // all items in left and right list have been iterated, we go through all the
    // keys in the left list, and if they also appear in the right list, we add
    // the product of "value * occurrences" to an accumulator.

    l.sort();
    r.sort();
    let mut ans_1: usize = 0;
    for i in 0..l.len() {
        let left = *l.get(i).unwrap();
        let right = *r.get(i).unwrap();
        let diff = left.abs_diff(right);
        ans_1 += diff;
        println!("left: {left}, right: {right}, diff: {diff}");

        l_similarity.entry(left).or_insert(0);
        match r_similarity.get(&right) {
            Some(count) => r_similarity.insert(right, count + 1),
            None => r_similarity.insert(right, 1),
        };
    }

    let mut ans_2: usize = 0;

    for k in l_similarity.keys() {
        if let Some(count) = r_similarity.get(k) {
            ans_2 += k * count;
            println!("{k} occured {count} times in right list")
        }
    }

    println!("part 1 answer: {ans_1}");
    println!("part 2 answer: {ans_2}");
}

fn get_input(path: Option<&String>) -> (Vec<usize>, Vec<usize>) {
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();
    if let Some(p) = path {
        // as mentioned in the read_lines rust-by-example "More efficient approach"
        // there are more efficient ways to do this with File and Buf, but
        // that includes more advanced rust than what i currently comprehend
        let lines: Vec<String> = read_to_string(p)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        for line in lines.iter() {
            let split: Vec<&str> = line.split(" ").collect();
            if split.len() != 4 {
                dbg!(split);
                println!("line split does not contain 4 things");
                continue;
            }
            // unwrapping like this feels bad
            l.push(split.first().unwrap().parse().unwrap());
            r.push(split.last().unwrap().parse().unwrap());
        }
    } else {
        // just generate some data to use
        for i in 0..10 {
            l.push(i + 1);
            r.push(10 - i);
        }
    }

    (l, r)
}
