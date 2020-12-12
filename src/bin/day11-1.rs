use std::fs;

// (-1,-1) (0,-1) (1,-1)
// (-1,0)  Moves  (1,0)
// (-1,1)  (0,1)  (1,1)

static MOVES: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let rawdata = fs::read_to_string("data/day-11.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    println!("Input is {:?} lines", data.len());

    let rows = data.len() - 1;
    let cols = data[0].len() - 1;

    let mut seatmap = Vec::new();

    for row in &data {
        let row_vec: Vec<char> = row.chars().collect();
        seatmap.push(row_vec);
    }

    let mut round = 0;
    let mut total_occupied: i32;
    let mut changes: i32;

    loop {
        total_occupied = 0;
        changes = 0;
        round += 1;

        let mut take_seat = Vec::new();
        let mut leave_seat = Vec::new();

        for row in 0..=rows {
            for col in 0..=cols {
                let mut adj_occupied = 0;

                for direction in MOVES.iter() {
                    let neigh_col = col as i8 + direction.0;
                    let neigh_row = row as i8 + direction.1;

                    if neigh_col >= 0
                        && neigh_col <= cols as i8
                        && neigh_row >= 0
                        && neigh_row <= rows as i8
                    {
                        if seatmap[neigh_row as usize][neigh_col as usize] == '#' {
                            adj_occupied += 1;
                        }
                    }
                }

                match seatmap[row][col] {
                    'L' => {
                        if adj_occupied == 0 {
                            take_seat.push((row, col));
                            changes += 1;
                        }
                    }
                    '.' => {}
                    '#' => {
                        total_occupied += 1;
                        if adj_occupied >= 4 {
                            leave_seat.push((row, col));
                            changes += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        for take in take_seat {
            seatmap[take.0][take.1] = '#';
        }
        for leave in leave_seat {
            seatmap[leave.0][leave.1] = 'L';
        }

        if changes == 0 {
            break;
        }
    }

    println!(
        "After {} rounds steady state acheived with total occupied seats == {}",
        round, total_occupied
    );
}
