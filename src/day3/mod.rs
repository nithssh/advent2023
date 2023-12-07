#![allow(dead_code)]

pub fn solve() {
    let input = include_str!("input.txt");
    let mut numbers: Vec<SchematicNumber> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| get_schematic_numbers(y, line))
        .collect();

    let symbols: Vec<Symbol> = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.bytes()
                .enumerate()
                .filter(|(_, c)| is_symbol(c))
                .map(|(col_idx, c)| Symbol {
                    value: c as char,
                    coordinate: Coordinate(row_idx.clone(), col_idx.clone()),
                })
                .collect::<Vec<Symbol>>()
        })
        .collect();

    for symbol in &symbols {
        for number in &mut numbers {
            let is_adjacent = number
                .coordinates
                .iter()
                .any(|x| symbol.coordinate.is_adjacent(x));
            if is_adjacent {
                number.is_part = true;
            }
        }
    }

    let part_numbers: Vec<&SchematicNumber> = numbers.iter().filter(|x| x.is_part).collect();

    // dbg!(&numbers);
    // dbg!(&symbols);
    // dbg!(&part_numbers);

    let result1: u32 = part_numbers.iter().map(|part| part.value).sum();

    println!("result1: {}", result1);
}

fn is_symbol(byte: &u8) -> bool {
    match *byte as char {
        '.' => false,
        '0'..='9' => false,
        _ => true,
    }
}

#[derive(Debug)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn is_adjacent(&self, other: &Coordinate) -> bool {
        if self.0.abs_diff(other.0) <= 1 && self.1.abs_diff(other.1) <= 1 {
            true
        } else {
            false
        }
    }
}

/// Get the SchematicNumbers in a given row of schematic input
fn get_schematic_numbers(row_idx: usize, row: &str) -> Vec<SchematicNumber> {
    let mut numbers = Vec::new();
    // Note: assuming single byte chars
    let mut l = 0;
    while l < row.len() {
        let digits_limit = std::cmp::min(3, row.len() - l);
        for digits in (1..=digits_limit).rev() {
            if let Ok(n) = row[l..l + digits].parse::<u32>() {
                let coordinates = (0..digits).map(|i| Coordinate(row_idx, l + i)).collect();
                numbers.push(SchematicNumber {
                    value: n,
                    coordinates,
                    is_part: false,
                });
                l += digits;
                break;
            };
        }
        l += 1;
    }
    numbers
}

#[derive(Debug)]
struct SchematicNumber {
    value: u32,
    coordinates: Vec<Coordinate>,
    is_part: bool,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    coordinate: Coordinate,
}
