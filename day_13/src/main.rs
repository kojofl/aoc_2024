use std::ops::Add;

// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let mut machines = Vec::new();
    let mut lines: Vec<&str> = Vec::new();
    for line in INPUT.lines() {
        if line.is_empty() {
            machines.push(Machine::from_lines(lines.iter().copied()));
            lines.clear();
            continue;
        }
        lines.push(line);
    }
    machines.push(Machine::from_lines(lines.iter().copied()));
    lines.clear();
    let mut sum = 0;
    for machine in machines.iter() {
        if let Some(res) = machine.solve() {
            sum += 3 * res.0 + res.1
        }
    }
    println!("{sum}");
    sum = 0;
    for machine in machines.iter_mut() {
        machine.goal.0 += 10000000000000;
        machine.goal.1 += 10000000000000;
        if let Some(res) = machine.solve() {
            sum += 3 * res.0 + res.1
        }
    }
    println!("{sum}");
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    goal: (u64, u64),
}

impl Machine {
    // Parsing dies das
    pub fn from_lines<'a>(mut lines: impl Iterator<Item = &'a str>) -> Self {
        let (x_dir, y_dir) = lines.next().unwrap().split_once(',').unwrap();
        let x_dir = x_dir
            .split_once('+')
            .map(|(_, b)| b.parse::<u64>().unwrap());
        let y_dir = y_dir
            .split_once('+')
            .map(|(_, b)| b.parse::<u64>().unwrap());
        let a = (x_dir.unwrap(), y_dir.unwrap());
        let (x_dir, y_dir) = lines.next().unwrap().split_once(',').unwrap();
        let x_dir = x_dir
            .split_once('+')
            .map(|(_, b)| b.parse::<u64>().unwrap());
        let y_dir = y_dir
            .split_once('+')
            .map(|(_, b)| b.parse::<u64>().unwrap());
        let b = (x_dir.unwrap(), y_dir.unwrap());
        let (x_dir, y_dir) = lines.next().unwrap().split_once(',').unwrap();
        let x_dir = x_dir
            .split_once('=')
            .map(|(_, b)| b.parse::<u64>().unwrap());
        let y_dir = y_dir
            .split_once('=')
            .map(|(_, b)| b.parse::<u64>().unwrap());
        let goal = (x_dir.unwrap(), y_dir.unwrap());
        Self { a, b, goal }
    }

    fn solve(&self) -> Option<(u64, u64)> {
        let a = Formula {
            x_1: self.a.0 as f64,
            x_2: self.b.0 as f64,
            eq: self.goal.0 as f64,
        };
        let b = Formula {
            x_1: self.a.1 as f64,
            x_2: self.b.1 as f64,
            eq: self.goal.1 as f64,
        };
        let (x_1, x_2) = Self::gaussian(a, b);
        let (x_1, x_2) = (x_1.round() as u64, x_2.round() as u64);
        if self.a.0 * x_1 + self.b.0 * x_2 == self.goal.0 {
            return Some((x_1, x_2));
        }
        None
    }

    fn gaussian(a: Formula, b: Formula) -> (f64, f64) {
        let v = -(b.x_1 / a.x_1);
        let to_add = a.mul(v);
        let b = b + to_add;
        let x_2 = b.eq / b.x_2;
        let x_1 = (a.eq - (x_2 * a.x_2)) / a.x_1;
        (x_1, x_2)
    }
}

#[derive(Debug, Clone, Copy)]
struct Formula {
    x_1: f64,
    x_2: f64,
    eq: f64,
}

impl Formula {
    fn mul(&self, v: f64) -> Self {
        Self {
            x_1: self.x_1 * v,
            x_2: self.x_2 * v,
            eq: self.eq * v,
        }
    }
}

impl Add for Formula {
    type Output = Formula;

    fn add(self, rhs: Self) -> Self::Output {
        Formula {
            x_1: self.x_1 + rhs.x_1,
            x_2: self.x_2 + rhs.x_2,
            eq: self.eq + rhs.eq,
        }
    }
}

#[test]
fn test_solve() {
    let m = Machine {
        a: (26, 66),
        b: (67, 21),
        goal: (8400, 5400),
    };
    println!("{:?}", m.solve())
}
