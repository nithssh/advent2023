use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut sum: u32 = 0;
    for line in BufReader::new(file).lines() {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for char in line.unwrap().chars() {
            let Some(num) = char.to_digit(10) else {
                continue;
            };
            match (first, last) {
                (Some(_), Some(_)) => { last = Some(num); },
                (Some(_), None) =>{ last = Some(num); },
                (None, Some(_)) => { panic!("First not assigned while last is!") },
                (None, None) => { first = Some(num); },
            }
        }

        sum += match (first, last) {
            (Some(f), Some(l)) => f + l,
            _ => 0,
        }
    }
    println!("sum is: {}", sum);
}
