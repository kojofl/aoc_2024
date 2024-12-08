use day_08::{get_valid_antinodes, get_valid_antinodes_line, iter::*};
use std::{
    collections::{HashMap, HashSet},
    usize,
};

// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    for (i, line) in INPUT.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                a @ _ => {
                    map.entry(a)
                        .and_modify(|v| v.push((i, j)))
                        .or_insert(vec![(i, j)]);
                }
            }
            width = width.max(j)
        }
        height += 1
    }
    width += 1;
    println!(
        "Part 1: {}",
        map.iter()
            .map(|(_, v)| {
                CombinationsOwned::from(v.clone())
                    .map(|(a, b)| get_valid_antinodes(a, b, height, width).into_iter())
                    .flatten()
                    .filter_map(|v| v)
            })
            .flatten()
            .collect::<HashSet<(i64, i64)>>()
            .len()
    );
    println!(
        "Part 2: {}",
        map.into_iter()
            .map(|(_, v)| {
                CombinationsOwned::from(v)
                    .map(|(a, b)| get_valid_antinodes_line(a, b, height, width).into_iter())
                    .flatten()
            })
            .flatten()
            .collect::<HashSet<(i64, i64)>>()
            .len()
    );
}
