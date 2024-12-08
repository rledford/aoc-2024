use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    // solution steps for part 1
    // build a hashmap of hashmaps for each value in the rule sets
    // e.g. 47|53 the key would create to entries
    // {47: {53: 1}, 53: {47: -1}} representing that 47 comes before
    // 53, and 53 comes after 47
    // for each page update entry, ensure that the value after it maps
    // to a +1 in the constructed order rule hashmap
    //
    // solution steps for part 2
    // based on the numbers present in the update_list we can compute
    // their ordinality by referencing the ordering_rule_key and summing
    // up all the positive and negative 1 values we find for each
    // then by storing the sum as the key and the key as the value in a
    // hashmap we can get the keys, order them, and then create a list
    // of the values ordered by iterating over the ordered keys
    let (ordering_rules, page_updates) = get_input(args.get(1));
    let ordering_rule_key = create_ordering_rule_key(&ordering_rules);

    let mut valid_update_midpoint_sum: usize = 0;
    let mut fixed_update_midpoint_sum: usize = 0;

    for list in page_updates {
        let center_index = (list.len() - 1) / 2;
        if check_update_list(&ordering_rule_key, &list).is_some() {
            let fixed = create_ordinal_list(&ordering_rule_key, &list);
            fixed_update_midpoint_sum += fixed.get(center_index).unwrap();
        } else {
            valid_update_midpoint_sum += list.get(center_index).unwrap();
        }
    }

    println!("part 1 answer: {valid_update_midpoint_sum}");
    println!("part 2 answer: {fixed_update_midpoint_sum}");
}

/// get the ordianlity of elements in update list relative to each other
fn create_ordinal_list(
    key: &HashMap<usize, HashMap<usize, isize>>,
    update_list: &[usize],
) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut map: HashMap<isize, usize> = HashMap::new();

    for i in 0..(update_list.len()) {
        let k = update_list.get(i).unwrap();
        let mut sum: isize = 0;
        for j in 0..update_list.len() {
            if j == i {
                continue;
            }
            sum += key
                .get(k)
                .unwrap()
                .get(update_list.get(j).unwrap())
                .unwrap();
        }

        map.insert(sum, *k);
    }

    let mut sorted_keys: Vec<isize> = map.clone().into_keys().collect();
    sorted_keys.sort();
    sorted_keys.reverse();

    for k in sorted_keys {
        result.push(*map.get(&k).unwrap());
    }

    result
}

/// returns Some(index) indicating that the value ahead of the index is out of order
fn check_update_list(
    key: &HashMap<usize, HashMap<usize, isize>>,
    update_list: &[usize],
) -> Option<usize> {
    for i in 0..(update_list.len() - 1) {
        let left = update_list.get(i).unwrap();
        let rule_key = key.get(left).unwrap(); // there should be rules for everything (or panic)
        for j in (i + 1)..update_list.len() {
            let right = update_list.get(j).unwrap();
            if let Some(v) = rule_key.get(right) {
                if v != &1isize {
                    return Some(i);
                }
            }
        }
    }
    None
}

fn create_ordering_rule_key(
    ordering_rules: &Vec<(usize, usize)>,
) -> HashMap<usize, HashMap<usize, isize>> {
    let mut result: HashMap<usize, HashMap<usize, isize>> = HashMap::new();

    for &(l, r) in ordering_rules {
        result.entry(l).or_default();
        result.get_mut(&l).unwrap().insert(r, 1);

        result.entry(r).or_default();
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
