use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-12.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let mut position = (0, 0);
    let mut waypoint = (1, 10);

    println!("Starting Position: {:?}, waypoint: {:?}", position, waypoint);

    fn move_waypoint(waypoint: &mut (i32, i32), dir: &str, distance: i32) {
        match dir {
            "N" => waypoint.0 += distance,
            "E" => waypoint.1 += distance,
            "S" => waypoint.0 -= distance,
            "W" => waypoint.1 -= distance,
            _ => unreachable!(),
        }
    }

    fn rotate_waypoint(waypoint: &mut (i32, i32), amount: i32) {
        match amount {
            0 => {}
            90 => *waypoint = (-waypoint.1, waypoint.0),
            180 => *waypoint = (-waypoint.0, -waypoint.1),
            270 => *waypoint = (waypoint.1, -waypoint.0),
            _ => unreachable!(),
        }
    }

    for line in data {
        let (action, val) = line.split_at(1);

        let amount = val.parse::<i32>().unwrap();
        match action {
            "L" => rotate_waypoint(&mut waypoint, 360 - amount),
            "R" => rotate_waypoint(&mut waypoint, amount),
            "F" => {
                position.0 += waypoint.0 * amount;
                position.1 += waypoint.1 * amount;
            },
            dir => move_waypoint(&mut waypoint, dir, amount),
        }
    }

    println!("Ending Position: {:?}, waypoint: {:?}", position, waypoint);
    println!("Manhattan distance: {}", position.0.abs()+position.1.abs());
}
