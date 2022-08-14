mod operations;
use regex::Regex;

// 32+15-15*11
pub fn parse(line: &str) {
    // let mut operation_idx: Vec<operations::Operation> = Vec::new();
    let split: Vec<&str> = line.split('+').collect();
    match split.iter().map(|x| evaluate(x)).reduce(|acc, curr| acc + curr) {
        Some(e) => println!("{e}"),
        None => {},
    }
}

fn evaluate(expression: &str) -> i64 {
    let re = Regex::new(r"[*-/]").unwrap();
    // if re.
    expression.parse::<i64>().expect("failed to parse")
}

