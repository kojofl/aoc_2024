use std::usize;

pub mod iter;

pub fn get_valid_antinodes(
    (y_a, x_a): (usize, usize),
    (y_b, x_b): (usize, usize),
    height: usize,
    width: usize,
) -> [Option<(i64, i64)>; 2] {
    let y_diff = y_a as i64 - y_b as i64;
    let x_diff = x_a as i64 - x_b as i64;
    let anti_a_y = y_a as i64 + y_diff;
    let anti_a_x = x_a as i64 + x_diff;
    let mut res = [None, None];
    if (0..height as i64).contains(&anti_a_y) && (0..width as i64).contains(&anti_a_x) {
        res[0] = Some((anti_a_y, anti_a_x));
    }
    let anti_b_y = y_b as i64 - y_diff;
    let anti_b_x = x_b as i64 - x_diff;
    if (0..height as i64).contains(&anti_b_y) && (0..width as i64).contains(&anti_b_x) {
        res[1] = Some((anti_b_y, anti_b_x));
    }
    res
}

pub fn get_valid_antinodes_line(
    (y_a, x_a): (usize, usize),
    (y_b, x_b): (usize, usize),
    height: usize,
    width: usize,
) -> Vec<(i64, i64)> {
    let y_diff = y_a as i64 - y_b as i64;
    let x_diff = x_a as i64 - x_b as i64;
    let mut res = vec![(y_a as i64, x_a as i64), (y_b as i64, x_b as i64)];
    for i in 1.. {
        let anti_a_y = y_a as i64 + i * y_diff;
        let anti_a_x = x_a as i64 + i * x_diff;
        if (0..height as i64).contains(&anti_a_y) && (0..width as i64).contains(&anti_a_x) {
            res.push((anti_a_y, anti_a_x));
        } else {
            break;
        }
    }
    for i in 1.. {
        let anti_b_y = y_b as i64 - i * y_diff;
        let anti_b_x = x_b as i64 - i * x_diff;
        if (0..height as i64).contains(&anti_b_y) && (0..width as i64).contains(&anti_b_x) {
            res.push((anti_b_y, anti_b_x));
        } else {
            break;
        }
    }
    res
}
