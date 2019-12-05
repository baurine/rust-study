/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

fn is_prime(n: u32) -> bool {
  for i in 2..(n / 2 + 1) {
    if n % i == 0 {
      return false;
    }
  }
  true
}

pub fn solution() -> u32 {
  let mut order = 1;
  for i in (3..).step_by(2) {
    if is_prime(i) {
      order += 1;
      println!("{}st: {}", order, i);
      if order == 10_001 {
        return i;
      }
    }
  }
  0
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution_1() {
    assert_eq!(solution(), 104743);
  }
}
