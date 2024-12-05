use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    u32,
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
    let part_1 = lines.clone();
    println!(
        "{}",
        part_1
            .map(|page| {
                page.split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .filter(|v| is_ordered(v.as_slice(), &preconditions))
            .fold(0, |acc, valid_page| acc + valid_page[valid_page.len() / 2])
    );
    println!(
        "{}",
        lines
            .map(|page| {
                page.split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .filter(|v| !is_ordered(v.as_slice(), &preconditions))
            .map(|unordered| {
                let mut sorted = unordered
                    .iter()
                    .map(|n| Token {
                        val: *n,
                        rules: preconditions.get(&n),
                    })
                    .collect::<Vec<Token>>();
                sorted.sort();
                sorted[sorted.len() / 2].val
            })
            .sum::<u32>()
    );
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

fn is_ordered(page: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut used = HashSet::new();
    for el in page {
        if let Some(set) = rules.get(el) {
            if !set.is_disjoint(&used) {
                return false;
            }
        }
        used.insert(*el);
    }
    true
}
