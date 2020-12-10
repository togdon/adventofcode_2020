use regex::Regex;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-07.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let mut answer: i32 = 0;

    fn bag_lookup(answer: &mut i32, qty_bags: i32, outer_bag: &str, data: &Vec<&str>) {
        let re = Regex::new(r"(\d+)\s(\w+\s\w+)(\sbags?,?\.?)").unwrap();

        for lines in data {
            let bag: Vec<&str> = lines.split(" bags contain ").collect();
            if bag[0].contains(&outer_bag) {
                for cap in re.captures_iter(&bag[1]) {
                    let bag_count = cap[1].parse::<i32>().unwrap();
                    let bag_type = &cap[2];

                    *answer += bag_count * qty_bags;

                    bag_lookup(answer, bag_count * qty_bags, bag_type, data);
                }
            }
        }
    }

    bag_lookup(&mut answer, 1, "shiny gold", &data);
    println!("Answer: {:?}", answer);
}
