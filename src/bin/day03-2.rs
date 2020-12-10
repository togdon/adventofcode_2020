use std::fs;

fn main() {
    let data = fs::read_to_string("data/day-03.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut answer: u64 = 1;

    println!("Input is {:?} lines", lines.len() - 1);

    for moves in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        println!("Moving {:?} right, {:?} down", moves.0, moves.1);
        let mut position = 0;
        let mut trees = 0;

        for line_num in (moves.1..lines.len() - 1).step_by(moves.1) {
            let line: Vec<char> = lines[line_num].chars().collect();

            if position + moves.0 < line.len() {
                position += moves.0;
            } else {
                position = position + moves.0 - line.len();
            }

            if line[position] == '#' {
                trees += 1;
            }
        }
        answer *= trees;
        println!("Hit {:?} trees", trees);
    }

    println!("answer: {:?}", answer);
}
