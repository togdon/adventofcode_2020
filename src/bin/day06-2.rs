use itertools::Itertools;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-6.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.split("\n\n").collect();

    let mut sum_count = 0;

    println!("Input is {:?} lines", rawdata.len() - 1);
    println!("There are {:?} groups", data.len() - 1);

    for line in data {
        let mut yeses: Vec<char> = line.replace("\n", "").chars().unique().sorted().collect();

        for person in line.lines() {
            yeses.retain(|answer| person.contains(*answer))
        }
        sum_count += yeses.len();
    }

    println!("Sum of ubiquitous yeses {:?}", sum_count);
}
