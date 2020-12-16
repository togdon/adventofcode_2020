use std::fs;

fn run(data: &Vec<usize>, iterations: usize) -> usize {

    let mut last_spoken: Vec<Option<usize>> = vec![None; iterations as usize];

    for (index, num) in data.iter().enumerate() {
        last_spoken[*num as usize] = Some(index);
    }

    let mut answer: usize = 0;

    for iteration in data.len()..((iterations as usize) - 1) {
        let iteration = iteration as usize;
        match last_spoken[answer] {
            None => {
                last_spoken[answer as usize] = Some(iteration);
                answer = 0;
            }
            Some(last_spot) => {
                last_spoken[answer as usize] = Some(iteration);
                answer = iteration - last_spot;
            }
        };
    }
    answer
}

fn main() {
    let rawdata = fs::read_to_string("data/day-15.txt").expect("Unable to read file");
    let data: Vec<usize> = rawdata.trim().split(',').map(|num| num.parse().unwrap()).collect();

    println!("2020 Answer: {}", run(&data, 2020));
    println!("30000000 Answer: {}", run(&data, 30000000));
}
