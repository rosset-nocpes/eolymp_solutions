use std::io::stdin;

fn main() {
    let mut line_input = String::new();
    stdin()
        .read_line(&mut line_input)
        .unwrap();
    let x: f64 = line_input
        .trim()
        .parse()
        .unwrap();
    println!("{:.3}", x.powf(3.0) - (5.0*x.powf(2.0)) / 7.0 + 9.0*x - 3.0 / x + 1.0);
}