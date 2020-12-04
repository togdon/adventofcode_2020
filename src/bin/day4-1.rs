use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-4.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.split("\n\n").collect();

    let mut num_valid = 0;

    println!("Input is {:?} lines", data.len() - 1);
    for line in data {
        let mut fields_present = 0;
        // println!("{:?}", line);
        let fields: Vec<&str> = line.split_whitespace().collect();
        // println!("{:?}", fields);
        for field in fields {
            // println!("{:?}", field);
            if field.starts_with("byr:") {
                fields_present +=1;
            } else if field.starts_with("iyr:") {
                fields_present +=1;
            } else if field.starts_with("eyr:") {
                fields_present +=1;
            } else if field.starts_with("hgt:") {
                fields_present +=1;
            } else if field.starts_with("hcl:") {
                fields_present +=1;
            } else if field.starts_with("ecl:") {
                fields_present +=1;
            } else if field.starts_with("pid:") {
                fields_present +=1;
            } else if field.starts_with("cid:") {
                // it's a freebie
            } else {
                println!("{:?}", field);
            }
        }
        if fields_present >= 7 {
            num_valid += 1;
            // println!("{:?}:({:?}){:?}", num_valid, fields_present, line);
        }
    }

    println!("Valid Passports {:?}", num_valid);

}
