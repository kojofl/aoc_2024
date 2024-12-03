pub mod regex;

#[test]
fn test_regex() {
    use regex::Regex;
    let r = Regex::from_str(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    println!(
        "{:?}",
        r.match_str_with_rest(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        )
    );
    for entry in r.iter("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))") {
        println!("{entry}");
    }
}

#[test]
fn test_option() {
    use regex::Regex;
    let r = Regex::from_str(r"((mul\(\d{1,3},\d{1,3}\))|(don't\(\)))|(do\(\))").unwrap();
    for op in r.iter("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))") {
        println!("{op}")
    }
}
