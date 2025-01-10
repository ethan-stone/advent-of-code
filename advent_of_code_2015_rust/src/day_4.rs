use hex;
use md5::{Digest, Md5};

pub fn day_4() {
    let input = "yzbqklnj";

    let mut start = 0;

    loop {
        let mut hasher = Md5::new();

        hasher.update(format!("{input}{start}"));

        let result = hasher.finalize();

        let result_str = hex::encode(result);

        if result_str.starts_with("000000") {
            break;
        }

        start += 1;
    }

    println!("Day 4: The number is {start}");
}
