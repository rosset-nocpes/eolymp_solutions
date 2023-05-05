use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    let n: u32 = buffer
        .trim()
        .parse()
        .unwrap();
    let bin = format!("{:b}", n);
    let count = bin
        .chars()
        .filter(|&c| c == '1')
        .count();
    println!("{}", count);
}
