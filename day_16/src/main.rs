use core::panic;
use std::{
    cell::OnceCell,
    cmp::Reverse,
    collections::{binary_heap::BinaryHeap, HashSet, VecDeque},
};

// const INPUT: &str = include_str!("../test");
// const INPUT: &str = include_str!("../test_2");
const INPUT: &str = include_str!("../input");
// const INPUT: &str = include_str!("../message.txt");

fn main() {
    let mut lines = INPUT.lines();
    let mut map: Vec<Vec<Field>> = Vec::new();
    let pos = OnceCell::new();
    let end = OnceCell::new();
    for (i, line) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
        map.push(
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, b)| match b {
                    b'#' => Field::Wall,
                    b'.' => Field::Empty(Vec::new()),
                    b'S' => {
                        pos.set((i, j)).unwrap();
                        Field::Empty(Vec::new())
                    }
                    b'E' => {
                        end.set((i, j)).unwrap();
                        Field::Goal(Vec::new())
                    }
                    _ => panic!(),
                })
                .collect(),
        );
    }

    let r = run(&mut map, *pos.get().unwrap(), *end.get().unwrap());
    println!("{:?}", r)
}

fn run(
    map: &mut [Vec<Field>],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(usize, usize)> {
    use Direction::*;
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut prio: BinaryHeap<Reverse<Step>> = BinaryHeap::new();
    prio.push(Reverse(Step::new(0, start, Direction::East)));
    while let Some(Reverse(step)) = prio.pop() {
        if !visited.insert(((step.pos), step.direction)) {
            if let Some(Some(&mut Field::Empty(ref mut s))) =
                map.get_mut(step.pos.0).map(|o| o.get_mut(step.pos.1))
            {
                if s.iter().all(|f| f.score == step.score) {
                    s.push(step);
                }
            }
            continue;
        }
        visited.insert(((step.pos), step.direction.invert()));
        if let Some(Some(&mut Field::Empty(ref mut s))) =
            map.get_mut(step.pos.0).map(|o| o.get_mut(step.pos.1))
        {
            s.push(step);
        }

        if let Some(Some(&mut Field::Goal(ref mut s))) =
            map.get_mut(step.pos.0).map(|o| o.get_mut(step.pos.1))
        {
            if s.iter().all(|f| f.score == step.score) {
                s.push(step);
            } else {
                break;
            }
        }
        for d in [North, East, South, West] {
            let next = step.move_dir(d);
            if map[next.pos.0][next.pos.1] != Field::Wall {
                prio.push(Reverse(next));
            }
        }
    }
    let len = calc_len(end, map);
    let Field::Goal(end) = &map[end.0][end.1] else {
        panic!()
    };
    return end.first().map(|s| (s.score, len));
}

/// Walk back from the end and count all unique fields that have been visited.
fn calc_len(start: (usize, usize), map: &[Vec<Field>]) -> usize {
    let mut queue: VecDeque<Step> = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    let Field::Goal(g) = &map[start.0][start.1] else {
        panic!()
    };
    queue.extend(g.iter());

    while let Some(step) = queue.pop_front() {
        if step.score == 0 {
            break;
        }
        let prev = step.direction.invert().apply(step.pos);
        let Field::Empty(steps) = &map[prev.0][prev.1] else {
            panic!("reverting: {:?}", step);
        };
        for st in steps {
            if st.move_dir(step.direction) == step {
                visited.insert(st.pos);
                queue.push_back(*st);
            }
        }
    }
    visited.len()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
struct Step {
    score: usize,
    pos: (usize, usize),
    direction: Direction,
}

impl Step {
    fn new(score: usize, pos: (usize, usize), direction: Direction) -> Self {
        Self {
            score,
            pos,
            direction,
        }
    }

    fn move_dir(self, direction: Direction) -> Self {
        match (self.direction, direction) {
            (Direction::North, Direction::North)
            | (Direction::South, Direction::South)
            | (Direction::West, Direction::West)
            | (Direction::East, Direction::East) => Self {
                score: self.score + 1,
                pos: direction.apply(self.pos),
                direction,
            },
            (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::East, Direction::West)
            | (Direction::West, Direction::East) => Self {
                score: self.score + 2001,
                pos: direction.apply(self.pos),
                direction,
            },
            (_, d) => Self {
                score: self.score + 1001,
                pos: d.apply(self.pos),
                direction,
            },
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0.saturating_sub(1), pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West => (pos.0, pos.1.saturating_sub(1)),
        }
    }

    fn invert(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Field {
    Wall,
    Empty(Vec<Step>),
    Goal(Vec<Step>),
}
