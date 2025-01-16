use std::error::Error;
use std::{fs, vec};

fn get_coord_index(row: usize, col: usize, width: usize) -> usize {
    row * width + col
}

fn count_true(v: &Vec<bool>) -> usize {
    let mut count = 0;

    for b in v {
        if *b {
            count += 1;
        }
    }

    count
}

pub fn day_6() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/day_6_input.txt")?;

    // true means on
    // false means off
    // represent the grid as a single array using row major order
    // index = row * W + col        # 2D to 1D
    // row = index // W            # 1D to 2D row
    // col = index % W             # 1D to 2D column
    let mut grid: Vec<bool> = vec![];

    let num_of_coords = 1000 * 1000;

    for _ in 0..num_of_coords {
        grid.push(false);
    }

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').into_iter().collect();

        // 4 is a toggle instruction
        if parts.len() == 4 {
            let starting_point: Vec<usize> = parts[1]
                .split(",")
                .map(|val| val.parse::<usize>().expect("Could not parse coordinate"))
                .collect();

            let ending_point: Vec<usize> = parts[3]
                .split(",")
                .map(|val| val.parse::<usize>().expect("Could not parse coordinate"))
                .collect();

            let mut indices_to_flip: Vec<usize> = vec![];

            for row in starting_point[0]..(ending_point[0] + 1) {
                for col in starting_point[1]..(ending_point[1] + 1) {
                    indices_to_flip.push(row * 1000 + col);
                }
            }

            for idx in indices_to_flip {
                grid[idx] = !grid[idx];
            }
        } else {
            let starting_point: Vec<usize> = parts[2]
                .split(",")
                .map(|val| val.parse::<usize>().expect("Could not parse coordinate"))
                .collect();

            let ending_point: Vec<usize> = parts[4]
                .split(",")
                .map(|val| val.parse::<usize>().expect("Could not parse coordinate"))
                .collect();

            let mut indices_to_turn_on_or_off: Vec<usize> = vec![];

            for row in starting_point[0]..(ending_point[0] + 1) {
                for col in starting_point[1]..(ending_point[1] + 1) {
                    indices_to_turn_on_or_off.push(row * 1000 + col);
                }
            }

            if parts[1] == "on" {
                for idx in indices_to_turn_on_or_off {
                    grid[idx] = true;
                }
            } else if parts[1] == "off" {
                for idx in indices_to_turn_on_or_off {
                    grid[idx] = false;
                }
            }
        }
    }

    println!("Day 6: {} lights are on.", count_true(&grid));

    Ok(())
}
