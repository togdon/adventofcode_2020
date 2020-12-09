use itertools::Itertools;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-9.txt").expect("Unable to read file");
    let data: Vec<i64> = rawdata.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    println!("Input is {:?} lines", data.len());

    let preamble = 25;
    let mut current = preamble;
    let mut answer: i64 = 0;

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
                answer = data[current];
                break;
            }
            current += 1;
        }
    }

    println!("Answer is: {}", answer);
}
