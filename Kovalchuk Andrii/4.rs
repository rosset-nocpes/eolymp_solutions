use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    let baby: Vec<f32> = buffer     // x1 y1 r1 x2 y2 r2
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let ans: f32 = (((baby[1] - baby[4]).powf(2.0) + (baby[3] - baby[0]).powf(2.0)).abs()).sqrt();
    if baby[2] == baby[5] && baby[0] == baby[3] && baby[1] == baby[4] {
        println!("-1");
    } else if ans > (baby[2] + baby[5]).abs() || ans < (baby[5] - baby[2]).abs() {
        println!("0");
    } else if ans == (baby[2] + baby[5]).abs() || ans == (baby[2] - baby[5]).abs() {
        println!("1");
    } else {
        println!("2");
    }
}
