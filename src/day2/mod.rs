#[allow(unused_variables)]
use core::panic;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

pub fn solve() {
    let input = include_str!("input.txt");
    let result1: u32 = input
        .lines()
        .map(|game| (get_game_id(game), get_subsets(game).into_iter()))
        .map(|(game_id, mut subsets)| {
            let is_game_valid = subsets.all(|subset| is_subset_valid(subset));
            if is_game_valid {
                game_id
            } else {
                0
            }
        })
        .sum();

    println!("result1: {}", result1);
}

fn is_subset_valid(subset: &str) -> bool {
    subset.split(", ").all(|cube| {
        let mut it = cube.trim().split(" ");
        let count = it.next().unwrap();
        let color = it.next().unwrap();

        match color {
            "red" => count.parse::<i32>().unwrap() <= MAX_RED,
            "green" => count.parse::<i32>().unwrap() <= MAX_GREEN,
            "blue" => count.parse::<i32>().unwrap() <= MAX_BLUE,
            _ => panic!("Unknown color in input."),
        }
    })
}

fn get_subsets(game: &str) -> Vec<&str> {
    game[game.find(":").unwrap() + 1..].split(";").collect()
}

fn get_game_id(line: &str) -> u32 {
    let colon_idx = line.find(":").unwrap();
    line[5..colon_idx].parse().unwrap()
}
