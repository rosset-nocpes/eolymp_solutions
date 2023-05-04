use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    let baby: Vec<i32> = buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let ratio1: i32 = baby[0] / baby[1];
    print!("{}.", ratio1);
    let mut d: i32 = baby[0] % baby[1];
    for _ in 0..baby[2] {
        d *= 10;
        let ratio2: i32 = d / baby[1];
        print!("{}", ratio2);
        d %= baby[1];
    }
}