
pub fn solve() {
    let input = include_str!("test.txt");
    // let result1: u32 = input.lines()
    //     .enumerate()


    // 1. parse out the numbers in input into Vec<SchematicNumber>
    // 2. parse out the symbols in input into Vec<Symbol>
    // 3. for each schematic number, go through the symbol list and update is_part
    // 4. filter by is_part and sum
}

struct Coordinate(u32, u32);

struct SchematicNumber {
    value: u32,
    coordinates: Vec<Coordinate>,
    is_part: bool,
}

struct Symbol {
    value: char,
    coordinate: Coordinate,
}


