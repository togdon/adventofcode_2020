use itertools::Itertools;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-10.txt").expect("Unable to read file");
    let data: Vec<i32> = rawdata.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    let sorted: Vec<_> = data.iter().sorted().collect();

    println!("Input is {:?} lines", data.len());
    println!("Min: {} Max: {}", sorted[0], sorted[sorted.len() - 1]);

    let mut one_jolt = 0;
    let mut three_jolt = 1; // There's always one jump of 3 to the device

    match sorted[0] {
        1 => one_jolt += 1,
        3 => three_jolt += 1,
        _ => println!("{:?}", sorted[0]),
    }

    for element in 0..data.len() - 1 {
        match sorted[element + 1] - sorted[element] {
            1 => one_jolt += 1,
            3 => three_jolt += 1,
            _ => println!("{}:{} {}:{} == {}",
                element,
                sorted[element],
                element + 1,
                sorted[element + 1],
                sorted[element + 1] - sorted[element]
            ),
        }
    }

    println!(
        "one jolt jumps: {}, three jolt jumps: {}, answer: {}",
        one_jolt,
        three_jolt,
        one_jolt * three_jolt
    );
}
