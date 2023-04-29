use std::io::stdin;

fn main() {
    let mut a = String::new();
    stdin()
        .read_line(&mut a)
        .expect("Try again!");
    let char_vec: Vec<char> = a.chars().collect();
    for c in char_vec {
        print!("{} ", c);
    }
}