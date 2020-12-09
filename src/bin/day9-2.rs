use itertools::Itertools;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-9.txt").expect("Unable to read file");
    let data: Vec<i64> = rawdata.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    println!("Input is {:?} lines", data.len());

    let preamble = 25;
    let mut current = preamble;
    let mut invalid: i64 = 0;
    let mut sorted: Vec<_>;
    let mut answer: i64 = 0;

    // Find our invalid number
    for window in data.windows(preamble) {
        if current < data.len() {
            let mut sums = Vec::new();
            for (a, b) in window.iter().tuple_combinations() {
                let sum = a + b;
                sums.push(sum);
            }
            if sums.contains(&data[current]) {
                // Nothing to see here
            } else {
                invalid = data[current];
                break;
            }
            current += 1;
        }
    }
    println!("Invalid number is {}", invalid);

    for window_size in 2..data.len() {
        for window in data.windows(window_size) {
            let sum = window.iter().sum1::<i64>().unwrap();
            if sum == invalid {
                sorted = window.iter().sorted().collect();
                answer = sorted[0] + sorted[window_size - 1];
                break;
            }
        }
        if answer != 0 {
            break;
        }
    }
    println!("Answer is {}", answer);
}
