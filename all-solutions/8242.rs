use std::io::stdin;
use std::cmp::Ordering;

fn main() {
    let mut line_input = String::new();
    stdin()
        .read_line(&mut line_input)
        .expect("Try again!");
    let ans: i32 = line_input
        .trim()
        .parse()
        .expect("Input not number");
    match ans.cmp(&0) {
        Ordering::Equal => println!("Zero"),
        Ordering::Greater => println!("Positive"),
        Ordering::Less => println!("Negative")
    }
}