use std::error::Error;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() -> Result<(), Box<dyn Error>> {
    day_1::day_1();
    day_2::day_2()?;
    day_3::day_3()?;
    day_4::day_4();

    Ok(())
}
