use std::{collections::VecDeque, usize};

//  static INPUT: &'static str = include_str!("../test");
static INPUT: &'static str = include_str!("../input");

fn main() {
    println!(
        "{}",
        INPUT
            .lines()
            .map(|l| {
                let (target, operands) = l.split_once(':').expect("targed: operands...");
                let operands = operands
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
                OperationFacility::new(target.parse().unwrap(), operands)
            })
            .filter(|f| f.is_solvable())
            .fold(0, |acc, f| acc + f.target)
    )
}

#[derive(Debug)]
struct OperationFacility {
    target: u64,
    operands: Vec<u64>,
}

impl OperationFacility {
    pub fn new(target: u64, operands: Vec<u64>) -> Self {
        Self { target, operands }
    }
    pub fn is_solvable(&self) -> bool {
        let mut state_queue = VecDeque::new();
        state_queue.push_back(OperationState::new(1, self.operands[0]));

        while let Some(state) = state_queue.pop_front() {
            if state.val_idx >= self.operands.len() {
                if self.target == state.state {
                    return true;
                }
            } else {
                let idx = state.val_idx;
                state.advance_state(&mut state_queue, self.operands[idx], self.target);
            }
        }
        false
    }
}

#[derive(Debug)]
struct OperationState {
    val_idx: usize,
    state: u64,
}

impl OperationState {
    fn new(idx: usize, state: u64) -> Self {
        Self {
            val_idx: idx,
            state,
        }
    }

    fn advance_state(self, queue: &mut VecDeque<OperationState>, val: u64, target: u64) {
        // Add
        let add = self.state + val;
        if add <= target {
            queue.push_back(OperationState::new(self.val_idx + 1, add));
        }
        // Mul
        let mul = self.state * val;
        if mul <= target {
            queue.push_back(OperationState::new(self.val_idx + 1, mul));
        }
        // Concat
        let mut con = self.state.to_string();
        con.push_str(val.to_string().as_str());
        let con = con.parse::<u64>().unwrap();
        if con <= target {
            queue.push_back(OperationState::new(self.val_idx + 1, con));
        }
    }
}
