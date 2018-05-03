use std::env;
use std::str::FromStr;

fn sum(x: i32, y: i32) -> i32 {
  x + y
}

fn main() {

  let args: Vec<_> = env::args().collect();
  let x = i32::from_str(&args[1]).expect("");
  let y = i32::from_str(&args[2]).expect("");

  println!("{:?}", sum(x, y));
}

#[test]
fn test_sum() {
  assert_eq!(sum(1, 2), 3);
  assert_eq!(sum(0, 0), 0);
}
