use std::io::stdin;

fn main() {
    let mut line_input = String::new();
    stdin()
        .read_line(&mut line_input)
        .expect("Error!");
    let ans: f64 = line_input
        .trim()
        .parse()
        .expect("Error!");
    println!("{:.14}", (ans / (2 as f64).sqrt()));
}