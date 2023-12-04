use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let result1: u32 = BufReader::new(file)
        .lines()
        .map(|line| get_calibration_value(line.unwrap()))
        .sum();

    println!("result1: {}", result1);
}

fn get_calibration_value(line: String) -> u32 {
    let first = line
        .chars()
        .find(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap());

    let last = line
        .chars()
        .rfind(|c| c.to_digit(10) != None)
        .map(|c| c.to_digit(10).unwrap());

    match (first, last) {
        (Some(f), Some(l)) => f * 10 + l,
        _ => 0,
    }
}
