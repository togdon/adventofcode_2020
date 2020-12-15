use std::collections::HashMap;
use std::fs;

fn set_bit_to_0(address: i64, bit: usize) -> i64 {
    // println!(
    //     "address: {:#064b} ({}), setting bit: {} to 0\nshifted: {:#064b} ({})",
    //     address,
    //     address,
    //     bit,
    //     address & !(1 << bit),
    //     address & !(1 << bit)
    // );
    address & !(1 << bit)
}

fn set_bit_to_1(address: i64, bit: usize) -> i64 {
    // println!(
    //     "address: {:#064b} ({}), setting bit: {} to 1\nshifted: {:#064b} ({})",
    //     address,
    //     address,
    //     bit,
    //     address | (1 << bit),
    //     address | (1 << bit)
    // );
    address | (1 << bit)
}

fn main() {
    let rawdata = fs::read_to_string("data/day-14.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let mut mask = "000000000000000000000000000000000000"; // :sigh:
    let mut map = HashMap::new();

    for line in data {
        let fields: Vec<&str> = line.split(" = ").collect();

        if fields[0] == "mask" {
            mask = fields[1];
        } else {
            let mut address: i64 = fields[0][4..line.find(']').unwrap()].parse().unwrap();
            let value: i64 = fields[1].parse().unwrap();

            let mut floating_addr_bits = Vec::new();

            for (index, byte) in mask.bytes().rev().enumerate() {
                match byte {
                    b'0' => {}
                    b'X' => floating_addr_bits.push(index),
                    b'1' => address = address | (1 << index),
                    _ => unreachable!(),
                }
            }

            let base_address = address;
            for n in 0..1 << floating_addr_bits.len() {
                let mut address = base_address;
                for (index, float_addr_bit) in floating_addr_bits.iter().enumerate() {
                    match (n & (1 << index)) >> index {
                        0 => address = set_bit_to_0(address, *float_addr_bit),
                        1 => address = set_bit_to_1(address, *float_addr_bit),
                        _ => unreachable!(),
                    }
                }
                map.insert(address, value);
            }
        }
    }

    println!("Answer: {:?}", map.values().sum::<i64>());
}
