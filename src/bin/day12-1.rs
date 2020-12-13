use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-12.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let mut position = (0, 0);
    let mut direction = 90;

    println!("Starting Position: {:?}, {}", position, direction);

    fn move_ship(position: &mut (i32, i32), dir: &str, distance: i32) {
        match dir {
            "N" => position.0 += distance,
            "E" => position.1 += distance,
            "S" => position.0 -= distance,
            "W" => position.1 -= distance,
            _ => unreachable!(),
        }
    }

    fn dir(dir: i32) -> &'static str {
        match dir {
            0 => "N",
            90 => "E",
            180 => "S",
            270 => "W",
            _ => unreachable!(),
        }
    }

    for line in data {
        let (action, val) = line.split_at(1);

        let amount = val.parse::<i32>().unwrap();
        match action {
            "L" => direction = (direction + 360 - amount) % 360,
            "R" => direction = (direction + amount) % 360,
            "F" => move_ship(&mut position, dir(direction), amount),
            dir => move_ship(&mut position, dir, amount),
        }
    }

    println!("Ending Position: {:?}, {}", position, direction);
    println!("Manhattan distance: {}", position.0.abs()+position.1.abs());
}
