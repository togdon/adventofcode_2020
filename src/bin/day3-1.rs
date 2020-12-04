use std::fs;

fn main() {
    let data = fs::read_to_string("data/day-3.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut answer = 0;

    println!("{:?} lines", lines.len() - 1);

    let mut position = 0;

    for line_num in 1..lines.len() - 1 {
        let line: Vec<char> = lines[line_num].chars().collect();
        if position + 3 < line.len() {
            position += 3;
        } else {
            position = position + 3 - line.len();
        }
        // println!("{:?}:{:?} is {:?}", position, line_num, line[position]);
        if line[position] == '#' {
            answer += 1;
        }
    }
    println!("{:?} trees", answer);
}
