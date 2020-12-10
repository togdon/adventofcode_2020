use std::fs;

fn main() {
    let data = fs::read_to_string("data/day-02.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut answer = lines.len() - 1;

    // println!("{:?} lines", lines.len() - 1);
    for line_num in 0..lines.len() - 1 {
        let line: Vec<&str> = lines[line_num].split(": ").collect();
        let key: Vec<&str> = line[0].split(&['-', ' '][..]).collect();

        let count = line[1].matches(key[2]).count();
        if count < key[0].parse::<usize>().unwrap() || count > key[1].parse::<usize>().unwrap() {
            answer -= 1;
        }
    }
    println!("{:?} correct passwords", answer);
}
