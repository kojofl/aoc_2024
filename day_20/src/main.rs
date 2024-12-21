use std::{
    cell::OnceCell,
    cmp::Reverse,
    collections::{binary_heap::BinaryHeap, HashMap, HashSet},
    usize,
};

// const INPUT: &str = include_str!("../test");
const INPUT: &str = include_str!("../input");

fn main() {
    let mut lines = INPUT.lines();
    let mut map: Vec<Vec<Field>> = Vec::new();
    let pos = OnceCell::new();
    let end = OnceCell::new();
    let mut fields = Vec::new();
    for (i, line) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
        map.push(
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, b)| match b {
                    b'#' => Field::Wall,
                    b'.' => {
                        fields.push((i, j));
                        Field::Empty { portals: None }
                    }
                    b'S' => {
                        fields.push((i, j));
                        pos.set((i, j)).unwrap();
                        Field::Empty { portals: None }
                    }
                    b'E' => {
                        end.set((i, j)).unwrap();
                        Field::Goal
                    }
                    _ => panic!(),
                })
                .collect(),
        );
    }

    create_portals(&mut map, &fields, 1);

    let normal = run_normal(&mut map, *pos.get().unwrap());
    let (target, path) = normal.unwrap();
    let r = run(&mut map, *pos.get().unwrap(), target, &path, 2);
    println!("Part 1: {:?}", r);
    create_portals(&mut map, &fields, 20);
    let r = run(&mut map, *pos.get().unwrap(), target, &path, 100);
    println!("Part 2: {:?}", r);
}

fn create_portals(map: &mut [Vec<Field>], fields: &[(usize, usize)], jump_size: usize) {
    use Direction::*;
    let heigth = map.len() - 1;
    let width = map[0].len() - 1;
    for field in fields {
        let mut portals = HashSet::new();
        let mut prio: BinaryHeap<Reverse<Step>> = BinaryHeap::new();
        let mut visited = HashSet::<(usize, usize)>::from([*field]);
        let start = Step::new(*field);
        for d in [North, East, South, West] {
            let next = start.move_dir(d, heigth, width);
            if !visited.insert(next.pos) {
                continue;
            }
            if let Field::Empty { .. } | Field::Goal = map[next.pos.0][next.pos.1] {
                continue;
            }
            prio.push(Reverse(next));
        }
        while let Some(Reverse(step)) = prio.pop() {
            if let Some(Some(&Field::Goal | Field::Empty { .. })) =
                map.get(step.pos.0).map(|o| o.get(step.pos.1))
            {
                portals.insert(Portal {
                    target: step.pos,
                    cost: step.score,
                });
                continue;
            }
            for d in [North, East, South, West] {
                let next = step.move_dir(d, heigth, width);
                if !visited.insert(next.pos) {
                    continue;
                }
                if next.score > jump_size && map[next.pos.0][next.pos.1] == Field::Wall {
                    continue;
                }
                prio.push(Reverse(next));
            }
        }
        let Field::Empty { portals: p } = &mut map[field.0][field.1] else {
            panic!()
        };
        let _ = p.insert(portals);
    }
}

fn run_normal(
    map: &mut [Vec<Field>],
    start: (usize, usize),
) -> Option<(usize, HashMap<(usize, usize), usize>)> {
    use Direction::*;
    let heigth = map.len() - 1;
    let width = map[0].len() - 1;
    let mut prio: BinaryHeap<Reverse<HeavyStep>> = BinaryHeap::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    prio.push(Reverse(HeavyStep::new(start)));
    while let Some(Reverse(step)) = prio.pop() {
        if let Some(Some(&Field::Goal)) = map.get(step.pos.0).map(|o| o.get(step.pos.1)) {
            let len = step.path.len();
            let mut cost = HashMap::new();
            for (i, v) in step.path.iter().enumerate() {
                cost.insert(*v, len - i - 1);
            }
            return Some((step.score, cost));
        }
        for d in [North, East, South, West] {
            let next = step.move_dir(d, heigth, width);
            if !visited.insert(next.pos) {
                continue;
            }
            if map[next.pos.0][next.pos.1] == Field::Wall {
                continue;
            }
            prio.push(Reverse(next));
        }
    }
    None
}

fn run(
    map: &mut [Vec<Field>],
    start: (usize, usize),
    target: usize,
    path: &HashMap<(usize, usize), usize>,
    thresh: usize,
) -> Option<usize> {
    use Direction::*;
    let heigth = map.len() - 1;
    let width = map[0].len() - 1;
    let mut prio: BinaryHeap<Reverse<Step>> = BinaryHeap::new();
    let mut pre_visited = HashSet::<(usize, usize)>::from([start]);
    prio.push(Reverse(Step::new(start)));
    let mut cheated = 0;
    while let Some(Reverse(step)) = prio.pop() {
        if let Some(Some(&Field::Goal)) = map.get(step.pos.0).map(|o| o.get(step.pos.1)) {
            return Some(cheated);
        }
        for d in [North, East, South, West] {
            let next = step.move_dir(d, heigth, width);
            if !pre_visited.insert(next.pos) {
                continue;
            }
            if map[next.pos.0][next.pos.1] != Field::Wall {
                prio.push(Reverse(next));
            }
        }
        let Field::Empty { portals } = &map[step.pos.0][step.pos.1] else {
            panic!()
        };
        if let Some(portals) = portals {
            for portal in portals {
                if let Some(cost) = path.get(&portal.target) {
                    let cheated_score = step.score + cost + portal.cost;

                    if target >= cheated_score + thresh {
                        cheated += 1;
                    }
                }
            }
        }
    }
    None
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
struct HeavyStep {
    score: usize,
    pos: (usize, usize),
    path: Vec<(usize, usize)>,
}
impl HeavyStep {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            score: 0,
            pos,
            path: vec![pos],
        }
    }

    fn move_dir(&self, direction: Direction, height: usize, width: usize) -> Self {
        let pos = direction.apply(self.pos, height, width);
        let mut clone = self.path.clone();
        clone.push(pos);
        Self {
            score: self.score + 1,
            pos,
            path: clone,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Step {
    score: usize,
    pos: (usize, usize),
    cheated: Option<((usize, usize), (usize, usize))>,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Step {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            score: 0,
            pos,
            cheated: None,
        }
    }

    fn move_dir(&self, direction: Direction, height: usize, width: usize) -> Self {
        let pos = direction.apply(self.pos, height, width);
        Self {
            score: self.score + 1,
            pos,
            cheated: self.cheated,
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
    fn apply(&self, pos: (usize, usize), height: usize, width: usize) -> (usize, usize) {
        match self {
            Direction::North => (pos.0.saturating_sub(1), pos.1),
            Direction::East => (pos.0, (pos.1 + 1).min(width)),
            Direction::South => ((pos.0 + 1).min(height), pos.1),
            Direction::West => (pos.0, pos.1.saturating_sub(1)),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Field {
    Wall,
    Empty { portals: Option<HashSet<Portal>> },
    Goal,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Portal {
    target: (usize, usize),
    cost: usize,
}
