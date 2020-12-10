use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-08.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    let mut accumulator = 0;

    println!("Input is {:?} lines", data.len());

    let mut visted_instructions = Vec::new();
    let mut line_num = 0;

    loop {
        // check to see if we've been on this line before
        if visted_instructions.iter().any(|&i| i==line_num) {
            println!("Revisited line {:?}, breaking", line_num);
            break;
        } else {
            visted_instructions.push(line_num);            
        }

        let line: Vec<&str> = data[line_num].split_whitespace().collect();
        // println!("Line {}: Instruction: {} Current: {}", line_num, data[line_num], accumulator);
        match line[0] {
            "nop" => {
                line_num += 1;
            },
            "acc" => {
                if line[1].starts_with('+') {
                    accumulator += line[1].strip_prefix('+').unwrap().parse::<i32>().unwrap();
                } else {
                    accumulator -= line[1].strip_prefix('-').unwrap().parse::<i32>().unwrap();
                }
                line_num += 1;
            },
            "jmp" => {
                if line[1].starts_with('+') {
                    line_num += line[1].strip_prefix('+').unwrap().parse::<usize>().unwrap();
                } else {
                    line_num -= line[1].strip_prefix('-').unwrap().parse::<usize>().unwrap();
                }
            },
            _ => break,
        }
    }
    println!("Final accumulator before breaking: {:?}", accumulator);
}
