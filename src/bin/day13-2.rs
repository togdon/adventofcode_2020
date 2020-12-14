use std::fs;

// NOTE: ring_algorithm::chinese_remainder_theorem; doesn't work. At all.
//
// Using https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
// instead

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn main() {
    let rawdata = fs::read_to_string("data/day-13.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    let mut index = 0;
    let mut busses = Vec::new();

    for field in data[1].split(',') {
        if field != "x" {
            busses.push((index, field.parse::<i64>().unwrap()));
        }
        index += 1;
    }

    let mut modulii = Vec::new();
    let mut residues = Vec::new();

    for pair in busses {
        modulii.push(pair.1);
        residues.push(pair.1 - pair.0);
    }

    let answer = chinese_remainder(&residues[..], &modulii[..]).unwrap();
    println!("Answer: {:?}", answer);
}
