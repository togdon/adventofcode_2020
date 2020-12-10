use itertools::Itertools;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-6.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.split("\n\n").collect();

    let mut sum_count = 0;

    println!("Input is {:?} lines", rawdata.len() - 1);
    println!("There are {:?} groups", data.len() - 1);
    for line in data {
        let yeses = line.replace("\n", "").chars().unique().count();
        sum_count += yeses;
    }

    println!("Sum of yeses {:?}", sum_count);
}
