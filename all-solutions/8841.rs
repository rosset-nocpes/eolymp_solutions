use std::io::stdin;

fn main() {
    let mut line_input = String::new();
    stdin()
        .read_line(&mut line_input)
        .expect("Error!");
    let ans: i32 = line_input
        .trim()
        .parse()
        .expect("Error!");
    println!("{}", ans / 10 % 10);
}