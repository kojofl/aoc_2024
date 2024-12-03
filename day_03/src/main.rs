use day_03::regex::Regex;

// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let regex = Regex::from_str(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut sum = 0;
    for op in regex.iter(INPUT) {
        let (a, b) = op.split_once(',').expect("regex to work");
        let a = a.trim_start_matches("mul(").parse::<u32>().unwrap();
        let b = b.trim_end_matches(")").parse::<u32>().unwrap();
        sum += a * b;
    }
    println!("Part 1: {sum}");

    let r = Regex::from_str(r"((mul\(\d{1,3},\d{1,3}\))|(don't\(\)))|(do\(\))").unwrap();
    let mut sum = 0;
    let mut active = true;
    for op in r.iter(INPUT) {
        match op {
            "don't()" => active = false,
            "do()" => active = true,
            _ => {
                if !active {
                    continue;
                }
                let (a, b) = op.split_once(',').expect("regex to work");
                let a = a.trim_start_matches("mul(").parse::<u32>().unwrap();
                let b = b.trim_end_matches(")").parse::<u32>().unwrap();
                sum += a * b;
            }
        }
    }
    println!("Part 2: {sum}");
}
