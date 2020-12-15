use std::collections::HashMap;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-14.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let mut mask = Vec::new();
    let mut map = HashMap::new();

    for line in data {
        let fields: Vec<&str> = line.split(" = ").collect();

        if fields[0] == "mask" {
            mask = fields[1].chars().collect();
        } else {
            let address = fields[0];

            let mut mem_val: Vec<char> = format!("{:#064b}", fields[1].parse::<i64>().unwrap())
                .chars()
                .collect();
            let mut index: usize = 28;
            for char in &mask {
                match char {
                    'X' => {},
                    dig => mem_val[index] = *dig,
                }
                index += 1;
            }

            let x_masked_bin: String = mem_val.iter().collect();
            let intval = isize::from_str_radix(&x_masked_bin[2..], 2).unwrap();

            map.insert(address, intval);
        }
    }

    println!("Answer: {:?}", map.values().sum::<isize>());
}
