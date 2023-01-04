use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

fn get_mean(input: &Vec<usize>) -> f64 {
    let mut total: usize = 0;

    for i in input.iter() {
        total += i;
    }
    return (total as f64) / (input.len() as f64);
}

fn get_median(input: &Vec<usize>) -> usize {
    let median_index = input.len() / 2;

    return *input.get(median_index).unwrap();
}

fn get_mode(input: &Vec<usize>) -> usize {
    let mut map = HashMap::new();
    for i in input.iter() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max_num = -1;

    for (key, value) in &map {
        if *value > max_num {
            max_num = *value;
            mode = **key;
        }
    }
    return mode;
}

fn main() {
    let mut input_s = String::new();
    let mut input_v: Vec<usize> = Vec::new();

    print!("Input list of integers(e.g. 1 2 3 4 ) > ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut input_s)
        .expect("Failed to read input.");

    for i in input_s.split_whitespace() {
        input_v.push(i.trim().parse().unwrap());
    }
    input_v.sort();

    let mean = get_mean(&input_v);
    let median = get_median(&input_v);
    let mode = get_mode(&input_v);

    println!("mean value is {}.", mean);
    println!("median value is {}.", median);
    println!("mode value is {}.", mode);
}
