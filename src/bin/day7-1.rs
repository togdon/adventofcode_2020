use itertools::Itertools;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-7.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let mut container_bags = Vec::new();
    let mut bag_lookup = vec!["shiny gold"];

    loop {
        for lookup in bag_lookup.pop() {
            for lines in &data {
                let bag: Vec<&str> = lines.split(" bags contain ").collect();
                if bag[1].contains(&lookup) {
                    bag_lookup.push(bag[0]);
                    container_bags.push(bag[0]);
                }
            }
        }
        if bag_lookup.len() == 0 {
            break;
        }
    }

    let answer: Vec<_> = container_bags.into_iter().unique().collect();
    println!("{:?} bag colors can eventually contain at least one shiny gold bag", answer.len());
}
