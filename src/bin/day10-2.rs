use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-10.txt").expect("Unable to read file");
    let data: Vec<i64> = rawdata.lines().map(|s| s.parse::<i64>().unwrap()).collect();
    let sorted: Vec<_> = data.iter().sorted().collect();

    let mut map = HashMap::new();
    map.insert(0, 1);

    for adapter in &sorted {
        let mut current: i64 = 0;

        if map.contains_key(&(*adapter - 1 as i64)) {
            current += map.get(&(*adapter - 1 as i64)).unwrap();
        }
        if map.contains_key(&(*adapter - 2 as i64)) {
            current += map.get(&(*adapter - 2 as i64)).unwrap();
        }
        if map.contains_key(&(*adapter - 3 as i64)) {
            current += map.get(&(*adapter - 3 as i64)).unwrap();
        }

        map.insert(**adapter, current);
    }

    println!("Answer: {:?}", map.get(&sorted.last().unwrap()).unwrap());
}
