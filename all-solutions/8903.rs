use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    let mut n: i32 = buffer
        .trim()
        .parse()
        .unwrap();
    n -= 1;
    while n % 5 != 0 {
        n -= 1;
    }
    println!("{}", n);
}