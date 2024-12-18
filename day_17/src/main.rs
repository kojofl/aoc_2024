use core::panic;
use std::{collections::VecDeque, u64, usize};

// const INPUT: &str = include_str!("../test");
const INPUT: &str = include_str!("../input");
// const INPUT: &str = include_str!("../custom_test");

fn main() {
    let mut p = Process::from_text(INPUT);
    let res = p.run();
    println!("{p:?}");
    p.reset(37221261688308 * 2, 0, 0);
    println!("Part 1: {res:?}");
    println!("Part 2: {:?}", p.find_perfect_a(0, 0));
}

#[derive(Debug, Clone)]
pub struct Process {
    r_a: u64,
    r_b: u64,
    r_c: u64,
    ip: usize,
    text: Program,
}

impl Process {
    pub fn from_text(input: &str) -> Self {
        let mut iter = input.lines();
        let a = iter
            .next()
            .expect("Register A to be set")
            .split_ascii_whitespace()
            .last()
            .expect("Register A to be set")
            .parse()
            .unwrap();
        let b = iter
            .next()
            .expect("Register B to be set")
            .split_ascii_whitespace()
            .last()
            .expect("Register B to be set")
            .parse()
            .unwrap();
        let c = iter
            .next()
            .expect("Register C to be set")
            .split_ascii_whitespace()
            .last()
            .expect("Register C to be set")
            .parse()
            .unwrap();

        assert!(iter.next().unwrap().is_empty());

        let instructions = iter
            .next()
            .expect("instructions")
            .split_ascii_whitespace()
            .last()
            .expect("instructions");

        let mut instructions = instructions.split(',');

        let mut text = Vec::new();
        let mut raw_text = Vec::new();

        while let Some(opcode) = instructions.next() {
            let opcode = opcode.parse::<u8>().unwrap();
            let operand = instructions.next().unwrap().parse::<u8>().unwrap();

            text.push(unsafe {
                (
                    *(&opcode as *const u8 as *const Opcode),
                    *(&operand as *const u8 as *const Operand),
                )
            });
            raw_text.push(opcode);
            raw_text.push(operand);
        }

        Self {
            r_a: a,
            r_b: b,
            r_c: c,
            ip: 0,
            text: Program {
                code: text,
                raw: raw_text,
            },
        }
    }

    pub fn reset(&mut self, a: u64, b: u64, c: u64) {
        self.r_a = a;
        self.r_b = b;
        self.r_c = c;
        self.ip = 0;
    }

    /// We assume that the programm will have a loop that contains **all** of the computation logic.
    fn find_perfect_a(&mut self, b: u64, c: u64) -> Option<u64> {
        use Opcode::*;
        let Some((loop_start, loop_end)) = self
            .text
            .code
            .iter()
            .enumerate()
            .find(|(_, (o, _))| *o == Opcode::Jnz)
            .map(|(e, (_, s))| (*s as usize, e))
        else {
            panic!("expected loop")
        };

        let prev_man: Vec<u64> = self.text.code[..loop_start]
            .iter()
            .filter(|(o, _)| *o == Adv)
            .rev()
            .map(|(_, v)| match v {
                Operand::Zero | Operand::One | Operand::Two | Operand::Three => {
                    2_u64.pow(*v as u32)
                }
                _ => {
                    panic!("Too stupid")
                }
            })
            .collect();

        let order: Vec<u64> = self.text.code[loop_start..loop_end]
            .iter()
            .filter(|(o, _)| *o == Adv)
            .rev()
            .map(|(_, v)| match v {
                Operand::Zero | Operand::One | Operand::Two | Operand::Three => {
                    2_u64.pow(*v as u32)
                }
                _ => {
                    panic!("Too stupid")
                }
            })
            .collect();

        let step_size: u64 = order.iter().product();
        let prev_man: u64 = (prev_man.iter().product::<u64>()).max(1);

        let mut starts = VecDeque::from_iter(0..*order.first().expect("At least one manipulation"));

        for m in &order[1..] {
            let len = starts.len();
            for _ in 0..len {
                let prev = starts.pop_front().unwrap();
                starts.extend(prev * m..prev * m + m);
            }
        }

        while let Some(start) = starts.pop_front() {
            self.reset(start * prev_man, b, c);
            let res = self.run();
            if res
                .iter()
                .rev()
                .zip(self.text.raw.iter().rev())
                .all(|(a, b)| a == b)
            {
                if res.len() == self.text.raw.len() {
                    return Some(start * prev_man);
                }
                if start * step_size != start {
                    starts.extend(start * step_size..start * step_size + step_size);
                }
            }
        }
        None
    }

    pub fn run(&mut self) -> Vec<u8> {
        use Opcode::*;
        let mut output = Vec::new();
        while self.ip < self.text.code.len() {
            match self.text.code[self.ip] {
                (Adv, v) => {
                    self.r_a /= 2_u64.pow(self.to_combo(v) as u32);
                }
                (Bxl, v) => {
                    self.r_b ^= v as u64;
                }
                (Bst, v) => {
                    self.r_b = self.to_combo(v) % 8;
                }
                (Jnz, v) => {
                    if self.r_a != 0 {
                        self.ip = v as usize;
                        continue;
                    }
                }
                (Bxc, _) => {
                    self.r_b ^= self.r_c;
                }
                (Out, v) => {
                    output.push((self.to_combo(v) % 8) as u8);
                }
                (Bdv, v) => {
                    self.r_b = self.r_a / 2_u64.pow(self.to_combo(v) as u32);
                }
                (Cdv, v) => {
                    self.r_c = self.r_a / 2_u64.pow(self.to_combo(v) as u32);
                }
            }
            self.ip += 1;
        }
        output
    }

    fn to_combo(&self, op: Operand) -> u64 {
        match op {
            Operand::Zero | Operand::One | Operand::Two | Operand::Three => op as u64,
            Operand::Four => self.r_a,
            Operand::Five => self.r_b,
            Operand::Six => self.r_c,
            Operand::Seven => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    code: Vec<(Opcode, Operand)>,
    raw: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Operand {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}
