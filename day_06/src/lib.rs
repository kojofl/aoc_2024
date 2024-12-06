use std::usize;

pub struct Map {
    pub field: Vec<Vec<Field>>,
    pub guard: Guard,
}

impl Map {
    pub fn from_input(input: &str) -> Self {
        let mut field = Vec::new();
        let mut guard: Option<Guard> = None;
        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::with_capacity(line.len());
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' => row.push(Field::Blocked),
                    '.' => row.push(Field::Unvisited),
                    '<' => {
                        guard = Some(Guard {
                            pos: (i, j),
                            dir: Direction::Left,
                        });
                        row.push(Field::Visited(vec![Direction::Left]));
                    }
                    '>' => {
                        guard = Some(Guard {
                            pos: (i, j),
                            dir: Direction::Right,
                        });
                        row.push(Field::Visited(vec![Direction::Right]));
                    }
                    '^' => {
                        guard = Some(Guard {
                            pos: (i, j),
                            dir: Direction::Up,
                        });
                        row.push(Field::Visited(vec![Direction::Up]))
                    }
                    'v' => {
                        guard = Some(Guard {
                            pos: (i, j),
                            dir: Direction::Down,
                        });
                        row.push(Field::Visited(vec![Direction::Down]));
                    }
                    _ => unreachable!(),
                }
            }
            field.push(row);
        }
        Self {
            field,
            guard: guard.expect("input to have a guard"),
        }
    }

    // Simulate the guard and return the number of possible loops and the distinct positions a the
    // guard will visit without the loop.
    pub fn simulate(mut self) -> (usize, usize) {
        let mut loops = 0;
        let height = self.field.len();
        let width = self.field[0].len();

        loop {
            let Some(next) = self.guard.dir.walk(self.guard.pos, (height - 1, width - 1)) else {
                break;
            };
            if self.field[next.0][next.1] == Field::Unvisited {
                if Self::check_loop(&self.field, self.guard, next) {
                    loops += 1;
                }
            }
            match (&mut self.field[next.0][next.1], self.guard.dir) {
                (Field::Visited(vec), dir) => {
                    self.guard.pos = next;
                    vec.push(dir);
                }
                (Field::Unvisited, dir) => {
                    self.field[next.0][next.1] = Field::Visited(vec![dir]);
                    self.guard.pos = next;
                }
                (Field::Blocked, _) => {
                    self.guard.rotate();
                }
            }
        }
        (
            loops,
            self.field
                .into_iter()
                .flatten()
                .filter(|f| match f {
                    Field::Visited(_) => true,
                    _ => false,
                })
                .count(),
        )
    }

    fn check_loop(field: &[Vec<Field>], mut guard: Guard, new_stone: (usize, usize)) -> bool {
        let mut field = field.to_vec();
        let height = field.len();
        let width = field[0].len();
        field[new_stone.0][new_stone.1] = Field::Blocked;
        loop {
            let Some(next) = guard.dir.walk(guard.pos, (height - 1, width - 1)) else {
                return false;
            };
            if field[next.0][next.1] != Field::Blocked {
                guard.pos = next;
            } else {
                guard.rotate();
            }
            match (&mut field[guard.pos.0][guard.pos.1], guard.dir) {
                (Field::Visited(vec), dir) => {
                    if vec.contains(&dir) {
                        return true;
                    }
                    vec.push(dir);
                }
                (Field::Unvisited, dir) => {
                    field[next.0][next.1] = Field::Visited(vec![dir]);
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Guard {
    pub pos: (usize, usize),
    pub dir: Direction,
}
impl Guard {
    fn rotate(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
            Direction::Right => self.dir = Direction::Down,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn walk(&self, pos: (usize, usize), bounds: (usize, usize)) -> Option<(usize, usize)> {
        let next = match self {
            Direction::Up => (pos.0.saturating_sub(1), pos.1),
            Direction::Down => ((pos.0 + 1).min(bounds.0), pos.1),
            Direction::Left => (pos.0, pos.1.saturating_sub(1)),
            Direction::Right => (pos.0, (pos.1 + 1).min(bounds.1)),
        };
        if next == pos {
            return None;
        }
        Some(next)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Field {
    Visited(Vec<Direction>),
    Unvisited,
    Blocked,
}
