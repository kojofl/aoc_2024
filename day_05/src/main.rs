use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let mut preconditions: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut lines = INPUT.lines();
    for cond in lines.by_ref().take_while(|l| !l.is_empty()) {
        let (cond, v) = cond.split_once('|').expect("input to be correct");
        let cond = cond.parse().unwrap();
        let v = v.parse().unwrap();
        preconditions
            .entry(cond)
            .and_modify(|c| {
                c.insert(v);
            })
            .or_insert(HashSet::from([v]));
    }

    let (part_1, part_2) = lines
        .map(|page| {
            page.split(',')
                .map(|n| {
                    let n = n.parse::<u32>().unwrap();
                    Token {
                        val: n,
                        rules: preconditions.get(&n),
                    }
                })
                .collect::<Vec<Token>>()
        })
        .fold((0, 0), |acc, mut page| {
            if is_ordered(&page) {
                (acc.0 + page[page.len() / 2].val, acc.1)
            } else {
                page.sort();
                (acc.0, acc.1 + page[page.len() / 2].val)
            }
        });

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[derive(PartialEq, Eq)]
struct Token<'a> {
    val: u32,
    rules: Option<&'a HashSet<u32>>,
}

impl Ord for Token<'_> {
    fn cmp(&self, other: &'_ Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Token<'_> {
    fn partial_cmp(&self, other: &'_ Self) -> Option<std::cmp::Ordering> {
        if self.rules.map(|m| m.contains(&other.val)).unwrap_or(false) {
            Some(Ordering::Greater)
        } else if other.rules.map(|m| m.contains(&self.val)).unwrap_or(false) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn is_ordered(page: &[Token]) -> bool {
    let mut used = HashSet::new();
    for el in page {
        if let Some(set) = el.rules {
            if !set.is_disjoint(&used) {
                return false;
            }
        }
        used.insert(el.val);
    }
    true
}
