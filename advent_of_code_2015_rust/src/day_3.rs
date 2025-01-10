use std::collections::HashMap;
use std::error::Error;
use std::fs;

// https://adventofcode.com/2015/day/3

/*
Solution:

Hash map of coordinates to number of times visited.
The keys are strings with coordinates separated by a comma.
For example 2,1 means to right from the origin and 1 up from the origin.
-7,-3 means left 7 from the origin and down 3 from the origin.
Go through the results and count the number of times each space is visited.
Then count all the entries with values greater than 1.
*/
pub fn day_3() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/day_3_input.txt")?;

    let mut coord_map = HashMap::new();

    coord_map.insert("0,0".to_string(), 1);

    let mut santa_x = 0;
    let mut santa_y = 0;

    let mut robo_santa_x = 0;
    let mut robo_santa_y = 0;

    // 0 is Santa
    // 1 is Robo Santa
    let mut whos_move = 0;

    for character in input.chars() {
        if whos_move == 0 {
            if character == '^' {
                santa_y += 1;
            }
            if character == 'v' {
                santa_y -= 1;
            }
            if character == '>' {
                santa_x += 1;
            }
            if character == '<' {
                santa_x -= 1;
            }

            let key = format!("{santa_x},{santa_y}");

            match coord_map.get(&key) {
                Some(val) => coord_map.insert(key, val + 1),
                None => coord_map.insert(key, 1),
            };

            whos_move = 1;
        } else if whos_move == 1 {
            if character == '^' {
                robo_santa_y += 1;
            }
            if character == 'v' {
                robo_santa_y -= 1;
            }
            if character == '>' {
                robo_santa_x += 1;
            }
            if character == '<' {
                robo_santa_x -= 1;
            }

            let key = format!("{robo_santa_x},{robo_santa_y}");

            match coord_map.get(&key) {
                Some(val) => coord_map.insert(key, val + 1),
                None => coord_map.insert(key, 1),
            };

            whos_move = 0;
        }
    }

    let num_houses = coord_map.keys().len();

    println!("Day 3: Santa visited {num_houses} houses at least once.");

    Ok(())
}
