use std::fs;

// This crate is magic
// https://docs.rs/peg/0.6.3/peg/index.html
peg::parser! {
    grammar list_parser() for str {
        pub rule expression() -> u64 = precedence!{
                x:(@) _ "*" _ y:@ { x * y }
                --
                x:(@) _ "+" _ y:@ { x + y }
                --
                "(" _ e:expression() _ ")" { e }
                --
                n:number() { n }
        }

        rule number() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule _() = " "?
    }
}

fn main() {
    let rawdata = fs::read_to_string("data/day-18.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let mut answer = 0;
    for line in data {
        let current = list_parser::expression(line).unwrap();
        answer += current
    }

    println!("Answer: {:?}", answer);
}
