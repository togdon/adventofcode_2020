use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-04.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.split("\n\n").collect();

    let mut num_valid = 0;

    println!("Input is {:?} lines", data.len() - 1);
    for line in data {
        let mut valid_fields = 0;
        let fields: Vec<&str> = line.split_whitespace().collect();
        for field in fields {
            if field.starts_with("byr:") {
                let field_data: Vec<&str> = field.split(':').collect();
                let birth_year = field_data[1].parse::<i32>().unwrap();
                if birth_year >= 1920 && birth_year <= 2002 {
                    valid_fields += 1;
                }
            } else if field.starts_with("iyr:") {
                let field_data: Vec<&str> = field.split(':').collect();
                let issue_year = field_data[1].parse::<i32>().unwrap();
                if issue_year >= 2010 && issue_year <= 2020 {
                    valid_fields += 1;
                }
            } else if field.starts_with("eyr:") {
                let field_data: Vec<&str> = field.split(':').collect();
                let expr_year = field_data[1].parse::<i32>().unwrap();
                if expr_year >= 2020 && expr_year <= 2030 {
                    valid_fields += 1;
                }
            } else if field.starts_with("hgt:") {
                let field_data: Vec<&str> = field.split(':').collect();
                if field_data[1].ends_with("cm") {
                    let height = field_data[1][0..(field_data[1].len() - 2)]
                        .parse()
                        .unwrap_or(0);
                    if height >= 150 && height <= 193 {
                        valid_fields += 1;
                    }
                } else if field_data[1].ends_with("in") {
                    let height = field_data[1][0..(field_data[1].len() - 2)]
                        .parse()
                        .unwrap_or(0);
                    if height >= 59 && height <= 76 {
                        valid_fields += 1;
                    }
                } else {
                    // Invalid height data
                }
            } else if field.starts_with("hcl:") {
                let field_data: Vec<&str> = field.split(':').collect();
                if field_data[1].starts_with("#")
                    && field_data[1].len() == 7
                    && field_data[1].chars().skip(1).all(|chars| chars.is_ascii_hexdigit())
                {
                    valid_fields += 1;
                }
            } else if field.starts_with("ecl:") {
                let field_data: Vec<&str> = field.split(':').collect();
                if field_data[1] == "amb"
                    || field_data[1] == "blu"
                    || field_data[1] == "brn"
                    || field_data[1] == "gry"
                    || field_data[1] == "grn"
                    || field_data[1] == "hzl"
                    || field_data[1] == "oth"
                {
                    valid_fields += 1;
                }
            } else if field.starts_with("pid:") {
                let field_data: Vec<&str> = field.split(':').collect();
                if field_data[1].len() == 9
                    && field_data[1].chars().all(|chars| chars.is_ascii_digit())
                {
                    valid_fields += 1;
                }
            } else if field.starts_with("cid:") {
                // it's a freebie
            } else {
                println!("{:?}", field);
            }
        }
        if valid_fields >= 7 {
            num_valid += 1;
            // println!("{:?}:({:?}){:?}", num_valid, valid_fields, line);
        }
    }

    println!("Valid Passports: {:?}", num_valid);
}
