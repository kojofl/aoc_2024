use std::{collections::HashMap, usize};

// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let elements: Vec<usize> = INPUT
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut cache = HashMap::new();
    let mut res_1 = 0;
    let mut res_2 = 0;
    for stone in elements {
        res_1 += simulate(stone, 25, &mut cache);
        res_2 += simulate(stone, 75, &mut cache);
    }
    println!("Part 1: {}", res_1);
    println!("Part 2: {}", res_2);
}

fn simulate(stone: usize, times: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if times == 0 {
        return 1;
    }
    if let Some(cached) = cache.get(&(stone, times)) {
        return *cached;
    }
    let mut sum = 0;
    match stone {
        0 => sum += simulate(1, times - 1, cache),
        v @ _ => {
            let len = len(v);
            if len % 2 == 0 {
                let (a, b) = split(v);
                sum += simulate(a, times - 1, cache) + simulate(b, times - 1, cache);
            } else {
                sum += simulate(v * 2024, times - 1, cache)
            }
        }
    }
    cache.insert((stone, times), sum);
    sum
}

#[inline(always)]
fn len(val: usize) -> u32 {
    ((val as f64).log10() + 1.0).floor() as u32
}

fn split(val: usize) -> (usize, usize) {
    let len = len(val);
    debug_assert!(len % 2 == 0);
    let split_idx = 10_usize.pow(len / 2);
    (val / split_idx, val % split_idx)
}

#[test]
fn test_len() {
    let v = 1002;
    println!("{:?}", split(v));
}
