use std::io::stdin;

fn main() {
    let mut a = String::new();
    stdin()
        .read_line(&mut a)
        .expect("Sorry!");
    let ans = a.chars().count() - 1;
    println!("{}", ans);
}