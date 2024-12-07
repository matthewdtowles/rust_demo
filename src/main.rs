// 1. Given a list of ints, use a vector and return median
// and also find the mode of the list of ints

// 2. Convert strings to pig latin

// 3. Use hash map and vectors to create a text interface
// to allow a user to add employee names to a department in a company.

fn median(list: Vec<i32>) -> f64 {
    // sort list (in place if possible)
    // return middle number as a float
    // return mean of middle two numbers if vec size % 2 == 0
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
    let median = median(list);
    println!("Median: {}", median);
    assert_eq!(median, 37.0);
}
