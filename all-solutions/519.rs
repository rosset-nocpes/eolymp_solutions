use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_line(&mut  buffer)
        .unwrap();
    let ab: Vec<i64> = buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", ab[0].pow(2) + ab[1].pow(2));
}
