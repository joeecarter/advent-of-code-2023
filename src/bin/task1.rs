use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];
    println!("filepath: {}", filepath);

    let f = File::open(filepath)
        .expect("Should have been able to read the file");
    let reader = BufReader::new(f);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let actual_line = line.expect("should have line");
        sum += extract_calibration_value(actual_line);
    }

    println!("{}", sum)
}


fn extract_calibration_value(input: String) -> u32 {

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

    // NOTE: I'd have put this in the loop in main but the borrow checker doesn't let me due to this function dropping the input variable once its done.
    println!("{} -> {}", input, result);

    return result;
}