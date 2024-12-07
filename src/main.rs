// 2. Convert strings to pig latin

// 3. Use hash map and vectors to create a text interface
// to allow a user to add employee names to a department in a company.

use std::collections::HashMap;

fn median(list: &Vec<i32>) -> f64 {
    let mut sorted = list.clone();
    sorted.sort();
    let size: i32 = sorted.len() as i32;
    let mid: usize = (size / 2) as usize;
    match size % 2 {
        1 => sorted[mid] as f64,
        0 => mean(sorted[(mid - 1)..=mid].to_vec()),
        _ => panic!("Something went wrong"),
    }
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut mode = *list.first().unwrap();
    let mut max = 0;
    let mut mode_map = HashMap::new();
    for x in list {
        *mode_map.entry(x).or_insert(0) += 1;
    }
    for key in mode_map.keys() {
        let cur_val = mode_map.get(key).unwrap();
        if cur_val > &max {
            mode = **key;
            max = *cur_val;
        }
    }
    mode
}
fn mean(list: Vec<i32>) -> f64 {
    let mut sum = 0;
    let size = list.len();
    for x in list {
        sum += x;
    }
    sum as f64 / size as f64
}

fn main() {
    let list = vec![59, 37, 7, 23, 23, 43, 41];
    let median = median(&list);
    println!("Median: {}", median);
    assert_eq!(median, 37.0);
    let mode = mode(&list);
    println!("Mode: {}", mode);
    assert_eq!(mode, 23);
}
