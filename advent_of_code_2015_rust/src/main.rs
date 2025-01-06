use std::error::Error;

mod day_1;
mod day_2;

fn main() -> Result<(), Box<dyn Error>> {
    day_1::day_1();
    day_2::day_2()?;

    Ok(())
}
