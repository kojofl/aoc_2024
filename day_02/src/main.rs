// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

type Level = Vec<u8>;

fn main() {
    let levels: Vec<Level> = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    println!("Part 1: {}", levels.iter().filter(|l| is_safe(l)).count());

    println!(
        "Part 2: {} ",
        levels.iter().filter(|l| is_safe_with_damper(l)).count()
    );
}

enum Change {
    Increase,
    Decrease,
}

fn is_safe(level: &Level) -> bool {
    let mut change: Option<Change> = None;
    for window in level.windows(2) {
        let (a, b) = (window[0], window[1]);
        match (b.cmp(&a), &change) {
            (std::cmp::Ordering::Greater, Some(Change::Decrease))
            | (std::cmp::Ordering::Less, Some(Change::Increase))
            | (std::cmp::Ordering::Equal, None)
            | (std::cmp::Ordering::Equal, Some(_)) => return false,
            (std::cmp::Ordering::Less, None) => change = Some(Change::Decrease),
            (std::cmp::Ordering::Greater, None) => change = Some(Change::Increase),
            _ => {}
        }
        if a.abs_diff(b) > 3 {
            return false;
        }
    }
    true
}

fn is_safe_with_damper(level: &Level) -> bool {
    if let Err(idx) = try_safety(level) {
        for to_remove in idx.saturating_sub(1)..=idx + 1 {
            if is_safe_left_right(&level[..to_remove], &level[to_remove + 1..]) {
                return true;
            }
        }
        false
    } else {
        true
    }
}

fn is_safe_left_right(left: &[u8], right: &[u8]) -> bool {
    let mut iter = left.iter().chain(right.iter()).peekable();
    let mut change: Option<Change> = None;
    while let Some(el) = iter.next() {
        let Some(b) = iter.peek() else {
            break;
        };
        match (b.cmp(&el), &change) {
            (std::cmp::Ordering::Greater, Some(Change::Decrease))
            | (std::cmp::Ordering::Less, Some(Change::Increase))
            | (std::cmp::Ordering::Equal, None)
            | (std::cmp::Ordering::Equal, Some(_)) => return false,
            (std::cmp::Ordering::Less, None) => change = Some(Change::Decrease),
            (std::cmp::Ordering::Greater, None) => change = Some(Change::Increase),
            _ => {}
        }
        if el.abs_diff(**b) > 3 {
            return false;
        }
    }
    true
}

fn try_safety(level: &Level) -> Result<(), usize> {
    let mut change: Option<Change> = None;
    for (win_idx, window) in level.windows(2).enumerate() {
        let (a, b) = (window[0], window[1]);
        match (b.cmp(&a), &change) {
            (std::cmp::Ordering::Greater, Some(Change::Decrease))
            | (std::cmp::Ordering::Less, Some(Change::Increase))
            | (std::cmp::Ordering::Equal, None)
            | (std::cmp::Ordering::Equal, Some(_)) => return Err(win_idx),
            (std::cmp::Ordering::Less, None) => change = Some(Change::Decrease),
            (std::cmp::Ordering::Greater, None) => change = Some(Change::Increase),
            _ => {}
        }
        if a.abs_diff(b) > 3 {
            return Err(win_idx);
        }
    }
    Ok(())
}
