use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter number");
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    println!("{}",armstrong_number(n));
}
pub fn armstrong_number(num: u32) -> bool {
  let digits: Vec<u32> = split(num);
  let length = digits.len() as u32;
  let mut sum = 0u32;
  for digit in digits {
    sum += digit.pow(length);
  }
  sum == num
}

fn split(n: u32) -> Vec<u32> {
  let mut digs = Vec::new();
  // save a copy
  let mut n = n;
  while n != 0 {
    digs.push(n % 10);
    n /= 10;
  }
  digs.reverse();
  digs
}
