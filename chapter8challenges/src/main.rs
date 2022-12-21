use std::collections::HashMap;

fn main() {
    let my_vec: Vec<i32> = [9, 1, 3, 6, 5, 5, 5, 2, 1, 4, 22, 23, 24, 25, 47, 69, 81, 12].to_vec();
    let ans = return_median_and_mode(my_vec);
    println!("Median: {} Mode: {}", &ans.0, &ans.1);
}

fn return_median_and_mode(mut vec: Vec<i32>) -> (i32, i32) {
    vec.sort();
    let vec_hash = create_vec_hash(&vec);
    let mode = parse_vec_hash_map(vec_hash);
    let median = handle_median(vec.get(&vec.len() / 2));
    (median, mode)
}

fn handle_median(median: Option<&i32>) -> i32 {
    match median {
        Some(i) => *i,
        None => 0,
    }
}

fn parse_vec_hash_map(vec_hash: HashMap<&i32, i32>) -> i32 {
    let keys_with_max_value = vec_hash.iter().max_by_key(|entry| entry.1).unwrap();
    **keys_with_max_value.0
}

fn create_vec_hash(vec: &Vec<i32>) -> HashMap<&i32, i32> {
    let mut vec_hash = HashMap::new();
    for a in vec {
        let count = vec_hash.entry(a).or_insert(0);
        *count += 1;
    }
    vec_hash
}
