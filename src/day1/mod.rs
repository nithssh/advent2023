use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("input.txt").unwrap();
    let result1: u32 = BufReader::new(file)
        .lines()
        .map(|line| get_calibration_value(line.unwrap()))
        .sum();

    let file = File::open("input.txt").unwrap();
    let result2: u32 = BufReader::new(file)
        .lines()
        .map(|line| get_actual_calibration_value(line.unwrap()))
        .sum();

    println!("result1: {}", result1);
    println!("result2: {}", result2);
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

fn get_actual_calibration_value(mut line: String) -> u32 {
    let digit_words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (idx, digit_word) in digit_words.into_iter().enumerate() {
        line = line.replace(digit_word, format!("{}{}{}", digit_word, idx, digit_word).as_str())
    }

    get_calibration_value(line)
}
