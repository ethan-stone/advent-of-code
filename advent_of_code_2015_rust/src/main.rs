#![allow(warnings)]

use std::error::Error;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn main() -> Result<(), Box<dyn Error>> {
    // day_1::day_1();
    // day_2::day_2()?;
    // day_3::day_3()?;
    // day_4::day_4();
    // day_5::day_5()?;
    day_6::day_6()?;

    Ok(())
}
