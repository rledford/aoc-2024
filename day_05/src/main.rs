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
    // by finding the order_rule_key "key" that has all -1 values, we know
    // that that "key" must be last, and the first would have all 1 values
    // so we can compute each number's ordinality based on the order-rule
    //
    // after determining the ordinality, we go through that ordinal list and
    // if the current value is in the update_page list being checked, we push
    // it onto a separate result list that will be returned
    let (ordering_rules, page_updates) = get_input(args.get(1));
    let ordering_rule_key = create_ordering_rule_key(&ordering_rules);

    let mut valid_update_midpoint_sum: usize = 0;
    let mut fixed_update_midpoint_sum: usize = 0;

    for list in page_updates {
        let center_index = (list.len() - 1) / 2;
        if let Some(_) = check_update_list(&ordering_rule_key, &list) {
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
        let v = map.get(&k).unwrap();
        if update_list.contains(v) {
            result.push(*v);
        }
    }

    result
}

fn create_ordinal_list_old(key: &HashMap<usize, HashMap<usize, isize>>) -> Vec<usize> {
    let mut ordinal_map: HashMap<usize, isize> = HashMap::new();
    for k in key.keys() {
        let mut sum: isize = 0;
        for v in key.get(k).unwrap().values() {
            sum += v;
        }
        if sum != 0 {
            println!("sum is not 0 - {sum}");
        }
        ordinal_map.insert(*k, sum);
    }
    let ordinal_list: Vec<usize> = Vec::new();

    ordinal_list
}

/// returns Some(index) indicating that the value ahead of the index
/// should be swapped with the value at index
/// returns None if the list is valid
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
