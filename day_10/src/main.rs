use std::collections::{HashSet, VecDeque};

// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut starts: Vec<(usize, usize)> = Vec::new();
    for (i, line) in INPUT.lines().enumerate() {
        map.push(Vec::new());
        for (j, c) in line.chars().enumerate() {
            let num = c.to_digit(10).expect("Input to be numbers");
            map[i].push(num);
            if num == 0 {
                starts.push((i, j));
            }
        }
    }
    let map = Map {
        inner: map,
        potential_starts: starts,
    };
    println!("Part 1: {}", map.compute(false));
    println!("Part 2: {}", map.compute(true));
}

#[derive(Debug)]
struct Map {
    inner: Vec<Vec<u32>>,
    potential_starts: Vec<(usize, usize)>,
}

impl Map {
    pub fn compute(&self, unique_paths: bool) -> usize {
        let mut curr = 0;
        for idx in self.potential_starts.as_slice() {
            let mut queue = VecDeque::from_iter(self.reachable_ideces(*idx).iter().flat_map(|i| {
                if let Some(i) = i {
                    Some((*idx, *i))
                } else {
                    None
                }
            }));
            let mut visited = HashSet::new();

            while let Some((from, to)) = queue.pop_front() {
                if visited.contains(&to) && !unique_paths {
                    continue;
                }
                if self.inner[from.0][from.1] + 1 != self.inner[to.0][to.1] {
                    continue;
                }
                visited.insert(to);
                if self.inner[to.0][to.1] == 9 {
                    curr += 1;
                } else {
                    queue.extend(self.reachable_ideces(to).into_iter().flat_map(|i| {
                        if let Some(i) = i {
                            Some((to, i))
                        } else {
                            None
                        }
                    }));
                }
            }
        }
        curr
    }

    #[inline(always)]
    fn reachable_ideces(&self, curr: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        let mut res = [None; 4];
        if curr.0 > 0 {
            res[0] = Some((curr.0 - 1, curr.1));
        }
        if curr.0 < self.inner.len() - 1 {
            res[1] = Some((curr.0 + 1, curr.1));
        }
        if curr.1 > 0 {
            res[2] = Some((curr.0, curr.1 - 1));
        }
        if curr.1 < self.inner[0].len() - 1 {
            res[3] = Some((curr.0, curr.1 + 1));
        }

        res
    }
}
