use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-13.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let arrival_time = data[0].parse::<i32>().unwrap();
    println!("Arrival time: {:?}", arrival_time);

    let mut winning_line = 0;
    let mut next_departure = arrival_time * 2;

    for field in data[1].split(',') {
        if field != "x" {
            let bus_line = field.parse::<i32>().unwrap();
            let multiplier = arrival_time / bus_line;
            let mut next_time = bus_line * multiplier;

            if next_time < arrival_time {
                next_time += bus_line;
            }

            if next_time <= next_departure {
                next_departure = next_time;
                winning_line = bus_line;
            }
        }
    }

    println!(
        "Winning line: {}, Departure time: {}",
        winning_line, next_departure
    );
    println!("Answer: {}", winning_line * (next_departure - arrival_time));
}
