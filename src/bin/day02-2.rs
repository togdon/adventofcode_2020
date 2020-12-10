use std::fs;

fn main() {
    let data = fs::read_to_string("data/day-2.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut answer = 0;

    // println!("{:?} lines", lines.len() - 1);
    for line_num in 0..lines.len() - 1 {
        let line: Vec<&str> = lines[line_num].split(": ").collect();
        let key: Vec<&str> = line[0].split(&['-', ' '][..]).collect();

        let pos_one = key[0].parse::<usize>().unwrap()-1;
        let pos_two = key[1].parse::<usize>().unwrap()-1;
        let value = key[2].parse::<char>().unwrap();
        let val_arr: Vec<char> = line[1].chars().collect();

        if val_arr[pos_one] != value && val_arr[pos_two] != value {
            // Character is in neither position
        }
        else if val_arr[pos_one] == value && val_arr[pos_two] == value {
            // Character is in both positions
        }
        else {
            answer += 1;
        }
    }
    println!("{:?} correct passwords", answer);
}
