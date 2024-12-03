use std::env;
use std::fs::read_to_string;

const MIN_DIFF: usize = 1;
const MAX_DIFF: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data = get_input(args.get(1));
    let mut total_reports: usize = 0;
    let mut good_report_count: usize = 0;
    let mut damp_report_count: usize = 0;

    // answer_1 solution algo
    // determine that the report has at least 2 values
    // determine asc/desc order based on first 2 items
    // begin iteration on index 2 and look backwards to
    // - ensure order matches initial order
    // - ensure the diff between current and previous are within min/max tolerance
    // if the report is good, increment good reports count
    //
    // answer_2 solution algo
    // when a bad report is encountered, considering the current index of the
    // for loop, remove the prev index from the array and retest and if that
    // is also bad remove the next index from the original array and retest
    // if both retests are bad then the Problem Dampener did not help
    //
    // OR
    //
    // when a bad report is encountered, brute force the Problem Dampener
    // by generating lists with one element removed and testing the
    // revised list
    for report in data.iter() {
        total_reports += 1;
        if is_safe_report(report) {
            good_report_count += 1;
        } else if brute_force_problem_dampener(report) {
            damp_report_count += 1;
        }
    }

    damp_report_count += good_report_count;

    println!("analyzed: {total_reports}");
    println!("answer 1: {good_report_count}");
    println!("answer 2: {damp_report_count}");
}

fn is_safe_report(report: &[usize]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let mut prev = report.first().unwrap();
    let mut next = report.get(1).unwrap();

    if prev == next {
        return false;
    }

    let is_ascending = is_ascending_order(prev, next);

    for i in 1..report.len() {
        prev = report.get(i - 1).unwrap();
        next = report.get(i).unwrap();
        if is_ascending_order(prev, next) != is_ascending {
            return false;
        }
        if !is_within_tolerance(prev, next) {
            return false;
        }
    }

    true
}

// there's definitely a better way
fn brute_force_problem_dampener(report: &[usize]) -> bool {
    let mut tmp: Vec<usize> = Vec::new();
    for i in 0..report.len() {
        tmp.clear();
        for j in 0..report.len() {
            if j != i {
                tmp.push(*report.get(j).unwrap());
            }
        }

        if is_safe_report(&tmp) {
            return true;
        }
    }

    false
}

fn is_within_tolerance(prev: &usize, next: &usize) -> bool {
    let diff = prev.abs_diff(*next);

    (MIN_DIFF..=MAX_DIFF).contains(&diff)
}

fn is_ascending_order(prev: &usize, next: &usize) -> bool {
    prev < next
}

fn get_input(path: Option<&String>) -> Vec<Vec<usize>> {
    let mut data: Vec<Vec<usize>> = Vec::new();

    if let Some(p) = path {
        let lines: Vec<String> = read_to_string(p)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        for line in lines.iter() {
            let split: Vec<&str> = line.split(' ').collect();
            let mut report: Vec<usize> = Vec::new();
            for v in split {
                report.push(v.parse().unwrap());
            }
            data.push(report);
        }
    } else {
        // just generate some data to use
        for i in 0..10 {
            let mut report = Vec::new();
            for j in 0..5 {
                report.push(i + j);
            }

            data.push(report);
        }
    }

    data
}
