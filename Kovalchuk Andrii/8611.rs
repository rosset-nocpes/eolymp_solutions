use std::io::stdin;

fn main() {
    let mut line_input = String::new();
    stdin()
        .read_line(&mut line_input)
        .expect("Error!");
    let t: i32 = line_input
        .trim()
        .parse()
        .expect("Not number!");
    if t > 0 {
        println!("Water");
    } else {
        println!("Ice");
    }
}