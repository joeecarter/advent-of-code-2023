use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::collections::HashMap;
use std::process::exit;


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

        let calibration_val = extract_calibration_value_part2(&actual_line);
        sum += calibration_val;
        println!("{} -> {} (sum: {})", actual_line, calibration_val, sum);

        // temp
        return
    }

    println!("TOTAL: {}", sum)
}

fn extract_calibration_value_part2(input: &String) -> u32 {
    // TODO: Move this elsewhere...
    let mut forwards = HashMap::new();
    forwards.insert(String::from("one"), 1);
    forwards.insert(String::from("two"), 2);
    forwards.insert(String::from("three"), 3);
    forwards.insert(String::from("four"), 4);
    forwards.insert(String::from("five"), 5);
    forwards.insert(String::from("six"), 6);
    forwards.insert(String::from("seven"), 7);
    forwards.insert(String::from("eight"), 8);
    forwards.insert(String::from("nine"), 9);

    println!("HELLO {}", input);

    read_number(&forwards, input);
    exit(0);

    

    return 0;


}

fn read_number(lookup: &HashMap<String, u32>, input: &String) -> u32 {
    let mut current_token_index: usize = 0;
    let mut current_options: Option<Vec<&String>> = Option::None;

    for char in input.chars() {
        // first check if we can continue our option tree
        if current_options.is_some() {
            current_token_index = current_token_index + 1;
            let next_options = get_next_options(current_options.unwrap(), current_token_index, char);
            if next_options.is_some() {
                current_options = next_options;
                continue;
            }
        }

        // have we bumped into a digit?
        let digit = char.to_digit(10);
        if digit.is_some() {
            // TODO: Is there a nicer way of doing this?
            return digit.unwrap();
        }


        // check if we are at the start of an option tree
        current_token_index = 0;
        let next_options = get_next_options(lookup.keys().collect(), current_token_index, char);
        if next_options.is_some() {
            current_options = next_options;
            continue;
        }
        
        // otherwise we just drop the char...
        println!("dropping useless char: {}", char);
    }

    return 0;
}

fn get_next_options(options: Vec<&String>, index: usize, c: char) -> Option<Vec<&String>>{

    let filtered = options.into_iter().filter(|s| s.chars().nth(index)
        .map(|actual_c| actual_c == c)
        .unwrap_or(false));

    let collected: Vec<&String> = filtered.collect();

    let mut ret: Option<Vec<&String>> = Option::None;
    if !collected.is_empty() {
        ret = Option::Some(collected);
    }
    return ret;
}

fn lookup_next_tokens(lookup: &HashMap<String, i32>, c: char) -> Option<Vec<&String>> {
    let mut avail = Vec::new();

    let filtered = lookup.keys().into_iter()
        .filter(|s| s.chars().nth(0)
            .map(|actual_c| actual_c == c)
            .unwrap_or(false)
        );
    
    dbg!(&filtered);
    
    avail = filtered.collect();

    // print!("{}", filtered);


    let mut ret = Option::None;
    if !avail.is_empty() {
        ret = Option::Some(avail);
    }

    return ret;
}

fn extract_calibration_value_part1(input: &String) -> u32 {
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

