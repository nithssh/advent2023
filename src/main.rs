use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut sum: u32 = 0;
    for line in BufReader::new(file).lines() {
        let first = line
            .as_ref()
            .unwrap()
            .chars()
            .find(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap());

        let last = line
            .unwrap()
            .chars()
            .rfind(|c| c.to_digit(10) != None)
            .map(|c| c.to_digit(10).unwrap());

        sum += match (first, last) {
            (Some(f), Some(l)) => f * 10 + l,
            _ => 0,
        }
    }
    println!("sum is: {}", sum);
}
