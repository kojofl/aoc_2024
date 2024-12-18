use std::{
    collections::{HashSet, VecDeque},
    usize, vec,
};

// const INPUT: &str = include_str!("../test");
const INPUT: &str = include_str!("../input");

fn main() {
    // let (height, width) = (7, 7);
    let (height, width) = (71, 71);
    let mut map = vec![vec![Field::new(); height]; width];

    for (time, line) in INPUT.lines().enumerate() {
        let Some((i, j)) = line
            .split_once(',')
            .map(|(i, j)| (j.parse::<usize>().unwrap(), i.parse::<usize>().unwrap()))
        else {
            continue;
        };
        map[i][j].obstructed_at = Some(time + 1);
    }

    let r = shortest_path_at(&map, 1024);

    println!("Part 1: {}", r.unwrap());

    for i in 1024.. {
        if let None = shortest_path_at(&map, i) {
            for (x, row) in map.iter().enumerate() {
                for (y, e) in row.iter().enumerate() {
                    if e.obstructed_at == Some(i) {
                        println!("Part 2: {y},{x}");
                        return;
                    }
                }
            }
        }
    }
}

fn shortest_path_at(map: &[Vec<Field>], time: usize) -> Option<usize> {
    use Direction::*;
    let size = map.len() - 1;
    let end = (size, size);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    queue.push_back(Step {
        num: 0,
        pos: (0, 0),
    });
    const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];
    while let Some(s) = queue.pop_front() {
        if s.pos == end {
            return Some(s.num);
        }
        for dir in DIRECTIONS {
            let (i, j) = dir.apply(s.pos, size);
            if visited.insert((i, j)) && map[i][j].obstructed_at.map(|t| t > time).unwrap_or(true) {
                queue.push_back(Step {
                    num: s.num + 1,
                    pos: (i, j),
                });
            }
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
struct Step {
    num: usize,
    pos: (usize, usize),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, pos: (usize, usize), border: usize) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0.saturating_sub(1), pos.1),
            Direction::Down => ((pos.0 + 1).min(border), pos.1),
            Direction::Left => (pos.0, pos.1.saturating_sub(1)),
            Direction::Right => (pos.0, (pos.1 + 1).min(border)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Field {
    obstructed_at: Option<usize>,
}

impl Field {
    fn new() -> Self {
        Self {
            obstructed_at: None,
        }
    }
}
