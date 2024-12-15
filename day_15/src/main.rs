use core::panic;
use std::{
    cell::OnceCell,
    collections::VecDeque,
    fmt::{Debug, Write},
};

// const INPUT: &str = include_str!("../large_example");
// const INPUT: &str = include_str!("../other_small");
// const INPUT: &str = include_str!("../small_test");
const INPUT: &str = include_str!("../input");

fn main() {
    let mut lines = INPUT.lines();
    let mut map = Vec::new();
    let robot_pos = OnceCell::new();
    for (i, line) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
        map.push(
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, b)| match b {
                    b'#' => Field::Wall,
                    b'O' => Field::Box,
                    b'.' => Field::Empty,
                    b'@' => {
                        robot_pos.set((i, j)).unwrap();
                        Field::Empty
                    }
                    _ => panic!(),
                })
                .collect(),
        );
    }

    let moves: Vec<Moves> = lines
        .flat_map(|l| {
            l.as_bytes().iter().map(|c| match c {
                b'<' => Moves::Left,
                b'>' => Moves::Right,
                b'^' => Moves::Up,
                b'v' => Moves::Down,
                _ => panic!(),
            })
        })
        .collect();

    simulate_robot(*robot_pos.get().unwrap(), &mut map, &moves);

    let mut sum = 0;
    for (i, r) in map.iter().enumerate() {
        for (j, el) in r.iter().enumerate() {
            if *el == Field::Box {
                sum += i * 100 + j;
            }
        }
    }
    println!("Part1: {sum}");

    let wide_robot_pos = OnceCell::new();
    let mut lines = INPUT.lines();
    let mut wide_map: Vec<Vec<WideField>> = Vec::new();
    for (i, line) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
        wide_map.push(
            line.as_bytes()
                .iter()
                .enumerate()
                .flat_map(|(j, b)| match b {
                    b'#' => [WideField::Wall, WideField::Wall],
                    b'O' => [WideField::BoxStart, WideField::BoxEnd],
                    b'.' => [WideField::Empty, WideField::Empty],
                    b'@' => {
                        wide_robot_pos.set((i, j * 2)).unwrap();
                        [WideField::Empty, WideField::Empty]
                    }
                    _ => panic!(),
                })
                .collect(),
        );
    }

    simulate_robot_wide(*wide_robot_pos.get().unwrap(), &mut wide_map, &moves);

    let mut sum = 0;
    for (i, r) in wide_map.iter().enumerate() {
        for (j, el) in r.iter().enumerate() {
            if *el == WideField::BoxStart {
                sum += i * 100 + j;
            }
        }
    }
    println!("Part2: {sum}");
}

fn simulate_robot(mut pos: (usize, usize), map: &mut [Vec<Field>], moves: &[Moves]) {
    for m in moves {
        let next = m.apply(pos);
        match map[next.0][next.1] {
            Field::Wall => continue,
            Field::Empty => {
                pos = next;
                continue;
            }
            Field::Box => {
                let mut behind = next;
                loop {
                    behind = m.apply(behind);
                    match map[behind.0][behind.1] {
                        Field::Wall => break,
                        Field::Empty => {
                            map[behind.0][behind.1] = Field::Box;
                            map[next.0][next.1] = Field::Empty;
                            pos = next;
                            break;
                        }
                        Field::Box => {}
                    }
                }
            }
        }
    }
}

fn simulate_robot_wide(mut pos: (usize, usize), map: &mut [Vec<WideField>], moves: &[Moves]) {
    for m in moves {
        let next = m.apply(pos);
        match map[next.0][next.1] {
            WideField::Wall => continue,
            WideField::Empty => {
                pos = next;
                continue;
            }
            WideField::BoxStart => match m {
                Moves::Up | Moves::Down => {
                    if manipulate_map((next.0, (next.1, next.1 + 1)), map, *m) {
                        pos = next;
                    }
                }
                Moves::Left => unreachable!(),
                Moves::Right => {
                    let mut behind = next;
                    loop {
                        behind = m.apply(behind);
                        match map[behind.0][behind.1] {
                            WideField::Wall => break,
                            WideField::Empty => {
                                for j in (next.1..=behind.1).rev() {
                                    map[pos.0].swap(j, j - 1);
                                }
                                pos = next;
                                break;
                            }
                            _ => continue,
                        }
                    }
                }
            },
            WideField::BoxEnd => match m {
                Moves::Up | Moves::Down => {
                    if manipulate_map((next.0, (next.1 - 1, next.1)), map, *m) {
                        pos = next;
                    }
                }
                Moves::Left => {
                    let mut behind = next;
                    loop {
                        behind = m.apply(behind);
                        match map[behind.0][behind.1] {
                            WideField::Wall => break,
                            WideField::Empty => {
                                for j in behind.1..next.1 {
                                    map[pos.0].swap(j, j + 1);
                                }
                                pos = next;
                                break;
                            }
                            _ => continue,
                        }
                    }
                }
                Moves::Right => unreachable!(),
            },
        }
    }
}

fn manipulate_map(coord: (usize, (usize, usize)), map: &mut [Vec<WideField>], m: Moves) -> bool {
    let mut to_check = VecDeque::new();
    let mut to_push = Vec::new();
    to_check.push_back(coord);
    while let Some((y, (x_1, x_2))) = to_check.pop_front() {
        let n_1 = m.apply((y, x_1));
        let n_2 = m.apply((y, x_2));

        to_push.push((y, (x_1, x_2)));
        match (map[n_1.0][n_1.1], map[n_2.0][n_2.1]) {
            (WideField::BoxStart, WideField::BoxEnd) => {
                to_check.push_back((n_1.0, (n_1.1, n_2.1)));
            }
            (WideField::BoxEnd, WideField::BoxStart) => {
                to_check.push_back((n_1.0, (n_1.1 - 1, n_1.1)));
                to_check.push_back((n_2.0, (n_2.1, n_2.1 + 1)));
            }
            (WideField::BoxEnd, WideField::Empty) => {
                to_check.push_back((n_1.0, (n_1.1 - 1, n_1.1)));
            }
            (WideField::Empty, WideField::BoxStart) => {
                to_check.push_back((n_2.0, (n_2.1, n_2.1 + 1)));
            }
            (WideField::Empty, WideField::Empty) => {}
            (WideField::BoxStart, WideField::Wall)
            | (WideField::BoxStart, WideField::Empty)
            | (WideField::BoxStart, WideField::BoxStart)
            | (WideField::Empty, WideField::BoxEnd)
            | (WideField::BoxEnd, WideField::BoxEnd) => unreachable!(),
            _ => return false,
        }
    }
    for (y, (x_1, x_2)) in to_push.into_iter().rev() {
        let n_1 = m.apply((y, x_1));
        let n_2 = m.apply((y, x_2));
        map[n_1.0][n_1.1] = WideField::BoxStart;
        map[n_2.0][n_2.1] = WideField::BoxEnd;
        map[y][x_1] = WideField::Empty;
        map[y][x_2] = WideField::Empty;
    }
    true
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Wall,
    Box,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WideField {
    Wall,
    BoxStart,
    BoxEnd,
    Empty,
}

impl Debug for WideField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WideField::Wall => f.write_char('#'),
            WideField::BoxStart => f.write_char('['),
            WideField::BoxEnd => f.write_char(']'),
            WideField::Empty => f.write_char('.'),
        }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Wall => f.write_char('#'),
            Field::Box => f.write_char('O'),
            Field::Empty => f.write_char('.'),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Moves {
    Up,
    Down,
    Left,
    Right,
}

impl Moves {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Moves::Up => (pos.0 - 1, pos.1),
            Moves::Down => (pos.0 + 1, pos.1),
            Moves::Left => (pos.0, pos.1 - 1),
            Moves::Right => (pos.0, pos.1 + 1),
        }
    }
}
