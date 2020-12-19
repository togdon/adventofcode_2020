use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-16.txt").expect("Unable to read file");

    let rules_block = Regex::new(r"([a-z].*):\s([0-9]+-[0-9]+)\sor\s([0-9]+-[0-9]+)").unwrap();
    let my_ticket_re = Regex::new(r"your\sticket:\n([0-9].*)").unwrap();
    let nearby_tickets = Regex::new(r"(?ms)^nearby tickets:\n(.*)").unwrap();

    let mut rules_map = HashMap::new();
    for rules in rules_block.captures_iter(&rawdata) {
        let rule_type: String = rules[1].to_string();

        let range1_string: String = rules[2].to_string();
        let range2_string: String = rules[3].to_string();

        rules_map.insert(rule_type, (range1_string, range2_string));
    }

    let _my_ticket = &my_ticket_re.captures(&rawdata).unwrap()[1];

    let tickets_string = &nearby_tickets.captures(&rawdata).unwrap()[1];
    let tickets: Vec<&str> = tickets_string.lines().collect();

    let mut scanning_error_rate = Vec::new();

    for ticket_data in tickets {
        for data_field in ticket_data.split(",") {
            let field = data_field.parse().unwrap();

            let mut valid_rule_found = false;

            for rule in &rules_map {
                let range1_string = &rule.1 .0;
                let range2_string = &rule.1 .1;

                let range1_vec: Vec<usize> = range1_string
                    .split('-')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                let range1 = range1_vec[0]..=range1_vec[1];

                let range2_vec: Vec<usize> = range2_string
                    .split('-')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                let range2 = range2_vec[0]..=range2_vec[1];

                if range1.contains(&field) || range2.contains(&field) {
                    valid_rule_found = true;
                    break;
                }
            }
            if valid_rule_found == false {
                // println!("No valid rule matched for {:?}", field);
                scanning_error_rate.push(field);
            }
        }
    }
    println!("Answer: {}", scanning_error_rate.iter().sum::<usize>());
}
