use std::error::Error;
use std::fs;

pub fn day_5() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/day_5_input.txt")?;

    let mut nice_lines = 0;

    for line in input.lines() {
        if is_nice(line) {
            println!("{line}");
            nice_lines += 1;
        }
    }

    println!("Day 5: There are {nice_lines} nice lines");

    Ok(())
}

fn is_nice(s: &str) -> bool {
    if s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy") {
        return false;
    }

    let mut num_vowels = 0;
    let mut current_character: Option<char> = None;
    let mut has_repeated_character = false;

    for ch in s.chars() {
        // only look for a repeated character if we haven't already found one
        if !has_repeated_character {
            match current_character {
                None => current_character = Some(ch),
                Some(val) => {
                    if ch == val {
                        has_repeated_character = true;
                    }

                    current_character = Some(ch)
                }
            }
        }

        if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
            num_vowels += 1;
        }
    }

    num_vowels >= 3 && has_repeated_character
}
