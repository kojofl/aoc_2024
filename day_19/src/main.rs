// const INPUT: &str = include_str!("../test");
const INPUT: &str = include_str!("../input");

fn main() {
    let mut lines = INPUT.lines();
    let mut towels: Vec<_> = lines
        .next()
        .expect("Valid input")
        .split(',')
        .map(|t| t.trim().as_bytes().into())
        .collect();
    towels.sort();

    lines.next();
    let goals: Vec<_> = lines
        .map(|g| Design(g.as_bytes().to_vec()))
        .map(|g| g.can_be_built_by(&towels))
        .collect();
    println!("Part 1: {}", goals.iter().filter(|g| **g > 0).count());
    println!("Part 2: {}", goals.iter().sum::<usize>());
}

struct Design(Vec<u8>);

impl Design {
    fn can_be_built_by(&self, towels: &[Towel]) -> usize {
        let mut ways = vec![0_usize; self.0.len() + 1];
        for i in 1..=self.0.len() {
            let mut count = 0;
            for j in 0..i {
                match j {
                    0 => {
                        if towels.iter().any(|t| t == &self.0[0..i]) {
                            ways[i] = 1;
                        }
                    }
                    _ => {
                        if ways[j] == 0 {
                            continue;
                        }
                        if towels.iter().any(|t| t == &self.0[j..i]) {
                            count += ways[j];
                        }
                    }
                }
            }
            ways[i] += count;
        }
        *ways.last().unwrap_or(&0)
    }
}

type Towel = Vec<u8>;
