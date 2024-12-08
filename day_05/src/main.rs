use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    // solution steps
    // build a hashmap of hashmaps for each value in the rule sets
    // e.g. 47|53 the key would create to entries
    // {47: {53: 1}, 53: {47: -1}} representing that 47 comes before
    // 53, and 53 comes after 47
    // for each page update entry, ensure that any values before or after
    // the current value map to an ordering rule
    let (ordering_rules, page_updates) = get_input(args.get(1));
    let ordering_rule_key = create_ordering_rule_key(&ordering_rules);
    let mut valid_update_midpoint_sum: usize = 0;

    for list in page_updates {
        let mut is_valid = true;
        'outer: for i in 0..(list.len() - 1) {
            let left = list.get(i).unwrap();
            let rule_key = ordering_rule_key.get(left).unwrap(); // there should be rules for everything (or panic)
            for j in (i + 1)..list.len() {
                let right = list.get(j).unwrap();
                if let Some(v) = rule_key.get(right) {
                    if v != &1isize {
                        is_valid = false;
                        continue 'outer;
                    }
                }
            }
        }

        if is_valid {
            let center_index = (list.len() - 1) / 2;
            valid_update_midpoint_sum += list.get(center_index).unwrap();
        }
    }
    println!("part 1 answer: {valid_update_midpoint_sum}");
}

fn create_ordering_rule_key(
    ordering_rules: &Vec<(usize, usize)>,
) -> HashMap<usize, HashMap<usize, isize>> {
    let mut result: HashMap<usize, HashMap<usize, isize>> = HashMap::new();

    for &(l, r) in ordering_rules {
        if !result.contains_key(&l) {
            let rules: HashMap<usize, isize> = HashMap::new();
            result.insert(l, rules);
        }
        result.get_mut(&l).unwrap().insert(r, 1);

        if !result.contains_key(&r) {
            let rules: HashMap<usize, isize> = HashMap::new();
            result.insert(r, rules);
        }
        result.get_mut(&r).unwrap().insert(l, -1);
    }

    result
}

fn get_input(path: Option<&String>) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let fallback_path = String::from("test.txt");
    let mut ordering_rules: Vec<(usize, usize)> = Vec::new();
    let mut page_updates: Vec<Vec<usize>> = Vec::new();

    if let Some(p) = path.or(Some(&fallback_path)) {
        let lines: Vec<String> = read_to_string(p)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        for line in lines {
            if line.contains("|") {
                let parts: Vec<&str> = line.split("|").collect();
                ordering_rules.push((
                    parts.first().unwrap().parse().unwrap(),
                    parts.last().unwrap().parse().unwrap(),
                ));
            }
            if line.contains(",") {
                let parts: Vec<&str> = line.split(",").collect();
                let mut updates: Vec<usize> = Vec::new();
                for n in parts {
                    updates.push(n.parse().unwrap());
                }
                page_updates.push(updates);
            }
        }
    }

    (ordering_rules, page_updates)
}
