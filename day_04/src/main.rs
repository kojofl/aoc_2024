// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let data = parse(INPUT);

    println!(
        "Part1: {}",
        data.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, el)| **el == b'X')
                    .fold(0, |acc, (j, _)| acc + check_position((i, j), &data))
            })
            .sum::<usize>()
    );
    println!(
        "Part2: {}",
        data.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(j, el)| **el == b'A' && is_x((i, *j), &data))
                    .count()
            })
            .sum::<usize>()
    );
}

fn parse(i: &'static str) -> Vec<&'static [u8]> {
    i.lines().map(|l| l.as_bytes()).collect()
}

enum Direction {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl Direction {
    fn is_possible(&self, (x, y): (usize, usize), (x_lim, y_lim): (usize, usize)) -> bool {
        match self {
            Direction::TopLeft => x >= 3 && y >= 3,
            Direction::Top => x >= 3,
            Direction::TopRight => x >= 3 && y <= y_lim - 3,
            Direction::Right => y <= y_lim - 3,
            Direction::BottomRight => x <= x_lim - 3 && y <= y_lim - 3,
            Direction::Bottom => x <= x_lim - 3,
            Direction::BottomLeft => x <= x_lim - 3 && y >= 3,
            Direction::Left => y >= 3,
        }
    }
}
const DIRECTIONS: [Direction; 8] = [
    Direction::TopLeft,
    Direction::Top,
    Direction::TopRight,
    Direction::Right,
    Direction::BottomRight,
    Direction::Bottom,
    Direction::BottomLeft,
    Direction::Left,
];

fn check_position((x, y): (usize, usize), data: &[&[u8]]) -> usize {
    let x_limit = data.len() - 1;
    let y_limit = data[0].len() - 1;

    let target = b"XMAS";
    DIRECTIONS
        .iter()
        .filter(|d| d.is_possible((x, y), (x_limit, y_limit)))
        .filter(|d| match d {
            Direction::TopLeft => {
                let b = [
                    data[x][y],
                    data[x - 1][y - 1],
                    data[x - 2][y - 2],
                    data[x - 3][y - 3],
                ];
                target == &b
            }
            Direction::Top => {
                let b = [data[x][y], data[x - 1][y], data[x - 2][y], data[x - 3][y]];
                target == &b
            }
            Direction::TopRight => {
                let b = [
                    data[x][y],
                    data[x - 1][y + 1],
                    data[x - 2][y + 2],
                    data[x - 3][y + 3],
                ];
                target == &b
            }
            Direction::Right => data[x][y..].starts_with(b"XMAS"),
            Direction::BottomRight => {
                let b = [
                    data[x][y],
                    data[x + 1][y + 1],
                    data[x + 2][y + 2],
                    data[x + 3][y + 3],
                ];
                target == &b
            }
            Direction::Bottom => {
                let b = [data[x][y], data[x + 1][y], data[x + 2][y], data[x + 3][y]];
                target == &b
            }
            Direction::BottomLeft => {
                let b = [
                    data[x][y],
                    data[x + 1][y - 1],
                    data[x + 2][y - 2],
                    data[x + 3][y - 3],
                ];
                target == &b
            }
            Direction::Left => data[x][..=y].ends_with(b"SAMX"),
        })
        .count()
}

fn is_x((x, y): (usize, usize), data: &[&[u8]]) -> bool {
    let candidate_a = [
        *data
            .get(x - 1)
            .map(|r| *r)
            .unwrap_or_default()
            .get(y - 1)
            .unwrap_or(&b'.'),
        *data
            .get(x + 1)
            .map(|r| *r)
            .unwrap_or_default()
            .get(y + 1)
            .unwrap_or(&b'.'),
    ];
    let candidate_b = [
        *data
            .get(x - 1)
            .map(|r| *r)
            .unwrap_or_default()
            .get(y + 1)
            .unwrap_or(&b'.'),
        *data
            .get(x + 1)
            .map(|r| *r)
            .unwrap_or_default()
            .get(y - 1)
            .unwrap_or(&b'.'),
    ];

    b"MS"
        .iter()
        .all(|c| candidate_a.contains(c) && candidate_b.contains(c))
}
