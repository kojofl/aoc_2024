use day_06::Map;

// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let map = Map::from_input(INPUT);
    let (part_2, part_1) = map.simulate();
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
