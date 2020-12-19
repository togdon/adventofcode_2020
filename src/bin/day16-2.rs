use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-16.txt").expect("Unable to read file");

    let rules_block = Regex::new(r"([a-z].*):\s([0-9]+-[0-9]+)\sor\s([0-9]+-[0-9]+)").unwrap();
    let my_ticket_re = Regex::new(r"your\sticket:\n([0-9].*)").unwrap();
    let nearby_tickets = Regex::new(r"(?ms)^nearby tickets:\n(.*)").unwrap();

    let mut rules_vec = Vec::new();
    let mut rule_names = Vec::new();
    for rules in rules_block.captures_iter(&rawdata) {
        let rule_type: String = rules[1].to_string();

        let range1_string: String = rules[2].to_string();
        let range2_string: String = rules[3].to_string();

        rules_vec.push((rule_type, range1_string, range2_string));
        rule_names.push(rules[1].to_string());
    }

    let my_ticket: Vec<usize> = my_ticket_re.captures(&rawdata).unwrap()[1]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let tickets_string = &nearby_tickets.captures(&rawdata).unwrap()[1];
    let tickets: Vec<&str> = tickets_string.lines().collect();
    println!("{} potential tickets", tickets.len());

    let mut valid_tickets = Vec::new();

    for ticket_data in tickets {
        let mut valid_ticket = true;

        for data_fields in ticket_data.split(",") {
            let field = data_fields.parse().unwrap();

            let mut valid_rule_found = false;

            for rule in &rules_vec {
                let range1_string = &rule.1;
                let range2_string = &rule.2;

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
                }
            }
            if valid_rule_found == false {
                valid_ticket = false;
            }
        }
        if valid_ticket == true {
            valid_tickets.push(ticket_data);
        }
    }

    let columns = valid_tickets[0].split(',').count();

    println!("{} valid tickets", valid_tickets.len());
    println!("{} ticket columns", columns);

    let mut rule_map = HashMap::new();

    while rule_map.len() < columns {
        for column in 0..columns {
            let mut valid_rules = 0;
            let mut column_rule = "None";

            for rule in &rules_vec {
                let rule_name: String = rule.0.to_owned();

                let mut valid_rule_count = 0;

                let range1_string = &rule.1;
                let range2_string = &rule.2;

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

                for ticket_data in &valid_tickets {
                    if rule_map.contains_key::<str>(&rule_name) {
                        // We've already determined the column for rule_name
                        break;
                    }

                    let field: usize = ticket_data
                        .split(",")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()[column];

                    if range1.contains(&field) || range2.contains(&field) {
                        valid_rule_count += 1;
                    }
                }
                if valid_rule_count == valid_tickets.len() {
                    valid_rules += 1;
                    column_rule = &rule.0;
                }
            }
            if valid_rules == 1 {
                rule_map.insert(column_rule, column);
            }
        }
    }
    println!("All {} column rules determined", rule_map.len());

    let mut answer = 1;
    for rule_details in rule_map {
        if rule_details.0.starts_with("departure") {
            answer *= my_ticket[rule_details.1];
        }
    }

    println!("Answer: {}", answer);
}
