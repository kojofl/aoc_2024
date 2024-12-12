pub mod perfect_hash;
use perfect_hash::HashMap;
use std::{
    cmp::min,
    collections::{HashSet, VecDeque},
    usize,
};

// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let v: Vec<Vec<u8>> = INPUT.lines().map(|l| l.bytes().collect()).collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut map: HashMap<Vec<(usize, usize, usize)>> = HashMap::new();

    let height = v.len();
    let width = v[0].len();

    loop {
        for i in 0..height {
            for j in 0..width {
                if visited.contains(&(i, j)) {
                    continue;
                }
                let area = discover_area((i, j), &v, &mut visited);
                map.entry(v[i][j])
                    .and_modify(|v| v.push(area))
                    .or_insert(vec![area]);
            }
        }
        break;
    }

    println!(
        "Part 1: {}",
        map.iter_values()
            .map(|v| v.iter())
            .flatten()
            .fold(0, |acc, (area, perimiter, ..)| { acc + area * perimiter })
    );

    println!(
        "Part 2: {}",
        map.iter_values()
            .map(|v| v.iter())
            .flatten()
            .fold(0, |acc, (area, .., sides)| { acc + area * sides })
    );
}

fn discover_area(
    idx: (usize, usize),
    map: &[Vec<u8>],
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize, usize) {
    visited.insert(idx);
    let mut queue = VecDeque::new();
    queue.push_back(idx);

    let area_key = map[idx.0][idx.1];
    let mut area = 0;
    let mut perimiter = 0;
    let mut sides = 0;
    let mut edges = Vec::new();
    while let Some((i, j)) = queue.pop_front() {
        area += 1;
        let mut is_edge = false;
        if i != 0 && area_key == map[i - 1][j] {
            if visited.insert((i - 1, j)) {
                queue.push_back((i - 1, j))
            }
        } else {
            perimiter += 1;
            is_edge = true;
        }
        if i < map.len() - 1 && area_key == map[i + 1][j] {
            if visited.insert((i + 1, j)) {
                queue.push_back((i + 1, j))
            }
        } else {
            perimiter += 1;
            is_edge = true;
        }
        if j != 0 && area_key == map[i][j - 1] {
            if visited.insert((i, j - 1)) {
                queue.push_back((i, j - 1))
            }
        } else {
            perimiter += 1;
            is_edge = true;
        }
        if j < map[0].len() - 1 && area_key == map[i][j + 1] {
            if visited.insert((i, j + 1)) {
                queue.push_back((i, j + 1))
            }
        } else {
            perimiter += 1;
            is_edge = true;
        }
        if is_edge {
            edges.push((i as i64, j as i64));
        }
    }

    // Look for sides
    edges.sort_by(|a, b| match a.0.cmp(&b.0) {
        v @ (std::cmp::Ordering::Less | std::cmp::Ordering::Greater) => v,
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
    });

    let mut prev_top: Option<(i64, i64)> = None;
    let mut prev_bottom: Option<(i64, i64)> = None;

    for edge in edges.iter() {
        // Reset if new row
        prev_top.take_if(|p| p.0 + 1 != edge.0);
        prev_bottom.take_if(|p| p.0 - 1 != edge.0);
        let top = (edge.0 - 1, edge.1);
        let bottom = (edge.0 + 1, edge.1);
        if let Some(prev_t) = prev_top {
            if map[(edge.0 as usize).saturating_sub(1)][edge.1 as usize]
                != map[edge.0 as usize][edge.1 as usize]
                || edge.0 == 0
            {
                if prev_t.1.abs_diff(edge.1) != 1 {
                    sides += 1;
                }
                prev_top = Some(top);
            }
        } else if map[(edge.0 as usize).saturating_sub(1)][edge.1 as usize]
            != map[edge.0 as usize][edge.1 as usize]
            || edge.0 == 0
        {
            sides += 1;
            prev_top = Some(top);
        }
        if let Some(prev_b) = prev_bottom {
            if map[min(edge.0 as usize + 1, map.len() - 1)][edge.1 as usize]
                != map[edge.0 as usize][edge.1 as usize]
                || bottom.0 as usize == map.len()
            {
                if prev_b.1.abs_diff(edge.1) != 1 {
                    sides += 1;
                }
                prev_bottom = Some(bottom);
            }
        } else if map[min(edge.0 as usize + 1, map.len() - 1)][edge.1 as usize]
            != map[edge.0 as usize][edge.1 as usize]
            || bottom.0 as usize == map.len()
        {
            sides += 1;
            prev_bottom = Some(bottom);
        }
    }
    edges.sort_by(|a, b| match a.1.cmp(&b.1) {
        v @ (std::cmp::Ordering::Less | std::cmp::Ordering::Greater) => v,
        std::cmp::Ordering::Equal => a.0.cmp(&b.0),
    });
    let mut prev_left: Option<(i64, i64)> = None;
    let mut prev_right: Option<(i64, i64)> = None;

    for edge in edges.iter() {
        // Reset if new row
        prev_left.take_if(|p| p.1 + 1 != edge.1);
        prev_right.take_if(|p| p.1 - 1 != edge.1);
        let left = (edge.0, edge.1 - 1);
        let right = (edge.0, edge.1 + 1);
        if let Some(prev_l) = prev_left {
            if map[edge.0 as usize][(edge.1 as usize).saturating_sub(1)]
                != map[edge.0 as usize][edge.1 as usize]
                || edge.1 == 0
            {
                if prev_l.0.abs_diff(edge.0) != 1 {
                    sides += 1;
                }
                prev_left = Some(left);
            }
        } else if map[edge.0 as usize][(edge.1 as usize).saturating_sub(1)]
            != map[edge.0 as usize][edge.1 as usize]
            || edge.1 == 0
        {
            sides += 1;
            prev_left = Some(left);
        }
        if let Some(prev_r) = prev_right {
            if map[edge.0 as usize][min(edge.1 as usize + 1, map[0].len() - 1)]
                != map[edge.0 as usize][edge.1 as usize]
                || right.1 as usize == map[0].len()
            {
                if prev_r.0.abs_diff(edge.0) != 1 {
                    sides += 1;
                }
                prev_right = Some(right);
            }
        } else if map[edge.0 as usize][min(edge.1 as usize + 1, map[0].len() - 1)]
            != map[edge.0 as usize][edge.1 as usize]
            || right.1 as usize == map[0].len()
        {
            sides += 1;
            prev_right = Some(right);
        }
    }
    edges.sort_by(|a, b| match a.1.cmp(&b.1) {
        v @ (std::cmp::Ordering::Less | std::cmp::Ordering::Greater) => v,
        std::cmp::Ordering::Equal => a.0.cmp(&b.0),
    });
    (area, perimiter, sides)
}
