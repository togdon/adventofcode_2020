use std::fs;

fn main() {
    let rawdata = fs::read_to_string("data/day-05.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.split("\n").collect();

    println!("Input is {:?} lines", data.len() - 1);
    for line_num in 0..data.len() - 1 {
        let row =
            isize::from_str_radix(&data[line_num][..7].replace("F", "0").replace("B", "1"), 2)
                .unwrap();
        let seat =
            isize::from_str_radix(&data[line_num][7..].replace("L", "0").replace("R", "1"), 2)
                .unwrap();
        let seat_id = (row * 8) + seat;

        println!("{:?} {:?} {:?}", seat_id, row, seat);
    }

    // for number in {91..928}; do echo "$number $(grep -c $number output)"; done | sort -k2 -rn
}
