use std::cmp::max;

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

    let result2: u32 = input
        .lines()
        .map(|game| get_subsets(game).into_iter())
        .map(|subsets_str| subsets_str.map(|subset| Subset::from(subset)))
        .map(|subsets| {
            subsets.fold(Subset::new(), |ac, x| {
                Subset(max(ac.0, x.0), max(ac.1, x.1), max(ac.2, x.2))
            })
        })
        .map(|x| x.power())
        .sum();

    println!("result1: {}", result1);
    println!("result2: {}", result2);
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

#[derive(Debug)]
struct Subset(u32, u32, u32);

impl Subset {
    fn new() -> Subset {
        Subset(0, 0, 0)
    }

    fn from(subset: &str) -> Subset {
        let mut new = Subset(0, 0, 0);
        for cube in subset.split(", ") {
            let mut it = cube.trim().split(" ");
            let count = it.next().unwrap();
            let color = it.next().unwrap();

            match color {
                "red" => new.0 = count.parse().unwrap(),
                "green" => new.1 = count.parse().unwrap(),
                "blue" => new.2 = count.parse().unwrap(),
                _ => panic!("Unknown color in input."),
            }
        }
        new
    }

    fn power(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}
