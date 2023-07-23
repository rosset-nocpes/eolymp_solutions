fn main() {
  let mut buffer = String::new();
  stdin()
    .read_line(&mut buffer)
    .unwrap();
  let a: i32 = buffer
    .trim()
    .parse()
    .expect("Not a number!");
  if a == ((a as f32).sqrt() as i32).pow(2) {
    println!("{}", (a as f32).sqrt() as i32);
  } else {
    println!("No");
  }
}
