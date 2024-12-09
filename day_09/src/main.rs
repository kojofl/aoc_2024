use core::panic;
use std::{cmp::Ordering, usize};

const INPUT: &'static str = include_str!("../test");
// const INPUT: &'static str = include_str!("../input");

fn main() {
    let mut memory = Memory(Vec::new());
    for (i, a) in INPUT.lines().flat_map(|l| l.chars()).enumerate() {
        let a = a.to_digit(10).expect("Input to be exclusively numbers");
        if i % 2 == 0 {
            memory.0.push(MemoryBlock::File(File {
                id: i / 2,
                amount: a as usize,
            }));
        } else {
            memory.0.push(MemoryBlock::Empty(Empty {
                cap: a as usize,
                used: Vec::new(),
            }));
        }
    }
    let mut part_2_mem = memory.clone();

    memory.sort_mem();

    let mut virtual_idx = 0;
    let mut sum = 0;
    for file in memory
        .0
        .into_iter()
        .map(|block| match block {
            MemoryBlock::File(file) => vec![file].into_iter(),
            MemoryBlock::Empty(empty) => empty.used.into_iter(),
        })
        .flatten()
    {
        for _ in 0..file.amount {
            sum += virtual_idx * file.id;
            virtual_idx += 1;
        }
    }
    println!("Part 1: {sum}");
    let mut virtual_idx = 0;
    let mut sum = 0;
    part_2_mem.sort_mem_no_frag();
    for file in part_2_mem.0.into_iter() {
        match file {
            MemoryBlock::File(file) => {
                for _ in 0..file.amount {
                    sum += virtual_idx * file.id;
                    virtual_idx += 1;
                }
            }
            MemoryBlock::Empty(empty) => {
                for file in empty.used {
                    for _ in 0..file.amount {
                        sum += virtual_idx * file.id;
                        virtual_idx += 1;
                    }
                }
                virtual_idx += empty.cap;
            }
        }
    }
    println!("Part 2: {sum}");
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    amount: usize,
}

#[derive(Debug, Clone)]
struct Empty {
    cap: usize,
    used: Vec<File>,
}

#[derive(Debug, Clone)]
enum MemoryBlock {
    File(File),
    Empty(Empty),
}

#[derive(Debug, Clone)]
struct Memory(Vec<MemoryBlock>);

impl Memory {
    fn sort_mem(&mut self) {
        while let Some(_) = self.fill_next_empty() {}
    }

    fn sort_mem_no_frag(&mut self) {
        for index in (1..self.0.len()).rev() {
            let MemoryBlock::File(f) = self.0[index] else {
                continue;
            };
            for mem in self.0[..index.saturating_sub(1)].iter_mut() {
                let MemoryBlock::Empty(empty) = mem else {
                    continue;
                };
                match empty.cap.cmp(&f.amount) {
                    Ordering::Greater | Ordering::Equal => {
                        empty.cap = empty.cap - f.amount;
                        empty.used.push(File {
                            id: f.id,
                            amount: f.amount,
                        });
                        self.0[index] = MemoryBlock::Empty(Empty {
                            cap: f.amount,
                            used: Vec::new(),
                        });
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    fn fill_next_empty(&mut self) -> Option<()> {
        let Some(position) = self.0.iter().position(|block| {
            let MemoryBlock::Empty(e) = block else {
                return false;
            };
            return e.cap != 0;
        }) else {
            return None;
        };
        let (a, b) = self.0.split_at_mut(position + 1);
        let MemoryBlock::Empty(empty) = &mut a[position] else {
            panic!()
        };
        let mut swapped: bool = false;
        for block in b.iter_mut().rev() {
            let MemoryBlock::File(file) = block else {
                continue;
            };
            swapped = true;
            let diff = empty.cap.abs_diff(file.amount);
            match empty.cap.cmp(&file.amount) {
                std::cmp::Ordering::Less => {
                    file.amount = diff;
                    empty.used.push(File {
                        id: file.id,
                        amount: empty.cap,
                    });
                    empty.cap = 0;
                    break;
                }
                std::cmp::Ordering::Greater => {
                    empty.cap = diff;
                    empty.used.push(File {
                        id: file.id,
                        amount: file.amount,
                    });
                    *block = MemoryBlock::Empty(Empty {
                        cap: 0,
                        used: Vec::new(),
                    })
                }
                std::cmp::Ordering::Equal => {
                    empty.cap = 0;
                    empty.used.push(File {
                        id: file.id,
                        amount: file.amount,
                    });
                    *block = MemoryBlock::Empty(Empty {
                        cap: 0,
                        used: Vec::new(),
                    });
                    break;
                }
            }
        }

        if !swapped {
            return None;
        }

        Some(())
    }
}
