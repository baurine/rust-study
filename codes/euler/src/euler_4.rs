/*
所谓回文数，就是两边读都一样的数，比如:698896。
求两个 3 位数之积最大的回文数。
*/

fn is_palindromic(n: u64) -> bool {
  let s = n.to_string();
  s.chars().rev().collect::<String>() == s
}

pub fn solution_1() -> u64 {
  let mut palindromic = 0;

  for x in (100..=999).rev() {
    for y in (100..=x).rev() {
      let prod = x * y;
      if prod <= palindromic {
        break;
      }
      if is_palindromic(prod) {
        palindromic = prod;
        println!("{} * {} = {}", x, y, prod);
      }
    }
  }
  palindromic
}

pub fn solution_2() -> u64 {
  let mut palindromic = 0;
  for x in 100..=999 {
    for y in x..=999 {
      let prod = x * y;
      if is_palindromic(prod ) && prod > palindromic {
        palindromic = prod;
        println!("{} * {} = {}", x, y, prod);
      }
    }
  }
  palindromic
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_palindromic() {
    assert!(is_palindromic(698896));
    assert!(!is_palindromic(123456));
  }

  #[test]
  fn test_solution_1() {
    assert_eq!(solution_1(), 906609);
  }

  #[test]
  fn test_solution_2() {
    assert_eq!(solution_2(), 906609);
  }
}
