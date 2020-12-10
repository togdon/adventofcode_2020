use std::fs;

fn main() {
    let data = fs::read_to_string("data/day-01.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut answer = 0;

    println!("{:?} lines", lines.len() - 1);
    for line_first in 0..lines.len() - 1 {
        let i: i32 = lines[line_first].parse::<i32>().unwrap();
        for line_second in line_first + 1..lines.len() - 1 {
            let j: i32 = lines[line_second].parse::<i32>().unwrap();
            for line_third in line_first + 2..lines.len() - 1 {
                let k: i32 = lines[line_third].parse::<i32>().unwrap();
                if i + j + k == 2020 {
                    println!(
                        "(line {:?}) {:?} + (line {:?}) {:?} + (line {:?}) {:?} == 2020",
                        line_first, i, line_second, j, line_third, k
                    );
                    answer = i * j * k;
                    break;
                }
            }
            if answer != 0 {
                break;
            }
        }
        if answer != 0 {
            println!("answer is {:?}", answer);
            break;
        }
    }
}
