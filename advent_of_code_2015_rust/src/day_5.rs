use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Pair {
    val: String,
    indices: (usize, usize),
}

pub fn day_5() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/day_5_input.txt")?;

    let mut nice_lines = 0;

    for line in input.lines() {
        if is_nice(line) {
            nice_lines += 1;
        }
    }

    println!("Day 5: There are {nice_lines} nice lines");

    Ok(())
}

fn is_nice(s: &str) -> bool {
    let mut pairs: Vec<Pair> = vec![];

    let mut prev_prev_char: Option<char> = None;
    let mut prev_char: Option<char> = None;
    let mut prev_char_idx = 0;
    let mut has_non_overlapping_pairs = false;
    let mut has_repeat_character_with_divider = false;

    for (idx, ch) in s.chars().enumerate() {
        if let Some(val) = prev_prev_char {
            if val == ch {
                has_repeat_character_with_divider = true;
            }
        }

        if let Some(val) = prev_char {
            let new_pair = Pair {
                val: format!("{val}{ch}"),
                indices: (prev_char_idx, idx),
            };

            for pair in pairs.iter() {
                if pair.val == new_pair.val && pair.indices.1 != new_pair.indices.0 {
                    has_non_overlapping_pairs = true;
                }
            }

            pairs.push(new_pair);
        }

        if idx >= 2 {
            prev_prev_char = prev_char;
        }

        prev_char = Some(ch);
        prev_char_idx = idx;
    }

    has_non_overlapping_pairs && has_repeat_character_with_divider
}
