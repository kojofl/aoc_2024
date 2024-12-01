// const INPUT: &'static str = include_str!("../test.txt");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let lines = INPUT.lines();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let mut values = line.split_ascii_whitespace();
        left.push(values.next().unwrap().parse::<i32>().unwrap());
        right.push(values.next().unwrap().parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();
    let mut curr_idx = 0;
    let mut diff = 0;
    let mut res = 0;
    let mut occurance = 0;
    let mut last_left = 0;

    for (a, b) in left.iter().zip(right.iter()) {
        diff += (a - b).abs();
        if *a != last_left {
            occurance = 0;
        }
        for el in right[curr_idx..].iter() {
            if el > a {
                break;
            }
            if el == a {
                occurance += 1;
            }
            curr_idx += 1;
        }
        last_left = *a;
        res += a * occurance;
    }
    println!("Part 1: {diff}");
    println!("Part 2: {res}")
}
