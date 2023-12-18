use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];
    println!("filepath: {}", filepath);

    let f = File::open(filepath)
        .expect("should have been able to read the file");
    let reader = BufReader::new(f);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let actual_line = line.expect("should have line");

        let calibration_val = extract_calibration_value(&actual_line);
        sum += calibration_val;
        println!("{} -> {} (sum: {})", actual_line, calibration_val, sum);
    }

    println!("TOTAL: {}", sum)
}


fn extract_calibration_value(input: &String) -> u32 {
    // LEARNING NOTE: Since input is a reference this function can not mutate it.
    // e.g. this fails: input.push_str(", world"); (see https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

    let mut result: u32 = 0;

    for char in input.chars() {
        match char.to_digit(10) {
            Some(x) => {
                result = x * 10;
                break;
            }
            None => {
                continue;
            }
        }
    }

    for char in input.chars().rev() {
        match char.to_digit(10) {
            Some(x) => {
                result = result + x;
                break;
            }
            None => {
                continue;
            }
        }
    }

    return result;
}