use std::{error::Error, fs};

// https://adventofcode.com/2015/day/2
pub struct PresentBox {
    length: u32,
    width: u32,
    height: u32,
}

impl PresentBox {
    pub fn build(s: &str) -> Result<PresentBox, &'static str> {
        let parts = s
            .split('x')
            .map(|val| val.parse::<u32>().expect("Could not parse value."));

        let parts_collection: Vec<u32> = parts.collect();

        Ok(PresentBox {
            length: parts_collection[0],
            width: parts_collection[1],
            height: parts_collection[2],
        })
    }

    pub fn total_surface_area(&self) -> u32 {
        return 2 * self.length * self.width
            + 2 * self.width * self.height
            + 2 * self.height * self.length;
    }

    pub fn smallest_side_area(&self) -> u32 {
        let a1 = self.length * self.width;
        let a2 = self.width * self.height;
        let a3 = self.height * self.length;

        let min_option = vec![a1, a2, a3].iter().min().copied();

        let min = match min_option {
            Some(val) => val,
            None => panic!("Somehow vector is empty!"),
        };

        min
    }

    pub fn total_required_wrapping_paper(&self) -> u32 {
        self.total_surface_area() + self.smallest_side_area()
    }

    pub fn smallest_side_perimeter(&self) -> u32 {
        let p1 = 2 * self.length + 2 * self.width;
        let p2 = 2 * self.width + 2 * self.height;
        let p3 = 2 * self.height + 2 * self.length;

        let min_option = vec![p1, p2, p3].iter().min().copied();

        let min = match min_option {
            Some(val) => val,
            None => panic!("Somehow vector is empty!"),
        };

        min
    }

    pub fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    pub fn total_required_ribbon(&self) -> u32 {
        self.smallest_side_perimeter() + self.volume()
    }
}

pub fn day_2() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/day_2_input.txt")?;

    let mut total_wrapping_paper_needed = 0;

    let mut total_ribbon_needed = 0;

    for dimensions in input.lines() {
        let present_box = PresentBox::build(dimensions)?;

        total_wrapping_paper_needed += present_box.total_required_wrapping_paper();
        total_ribbon_needed += present_box.total_required_ribbon()
    }

    println!(
        "Day 2: Elves need a total of {total_wrapping_paper_needed} square feet of wrapping paper"
    );
    println!("Day 2: Elves need a total of {total_ribbon_needed} feed of ribbon");

    Ok(())
}
