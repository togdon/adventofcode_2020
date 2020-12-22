use std::collections::{HashMap, HashSet};
use std::fs;

#[macro_use]
extern crate itertools;

fn main() {
    let rawdata = fs::read_to_string("data/day-17.txt").expect("Unable to read file");
    let data: Vec<&str> = rawdata.lines().collect();

    // Initialize the space
    let mut cube_map = HashSet::new();
    for (row_index, row) in data.iter().enumerate() {
        for (column_index, cube) in row.chars().enumerate() {
            if cube == '#' {
                cube_map.insert((column_index as i8, row_index as i8, 0 as i8));
            }
        }
    }
    println!("Initial active cubes: {:?}", cube_map.len());

    for cycle in 1..=6 {
        let mut neighbor_map = HashMap::new();

        for (column, row, layer) in &cube_map {
            let x = *column as i8;
            let y = *row as i8;
            let z = *layer as i8;
            for (dx, dy, dz) in iproduct!(-1..=1, -1..=1, -1..=1) {
                if (dx, dy, dz) != (0, 0, 0) {
                    let counter = neighbor_map.entry((x + dx, y + dy, z + dz)).or_insert(0);
                    *counter += 1;
                }
            }
        }

        cube_map = neighbor_map
            .iter()
            .filter(|&(position, &count)| count == 3 || (count == 2 && cube_map.contains(position)))
            .map(|(&position, _)| position)
            .collect();

        println!("Cycle {} active cubes: {:?}", cycle, cube_map.len());
    }
}
