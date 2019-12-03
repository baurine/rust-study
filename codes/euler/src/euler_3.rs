/*
找出整数 600851475143 的最大素数因子。
*/

fn is_prime(num: u64) -> bool {
  for i in 2..(num / 2 + 1) {
    if num % i == 0 {
      return false;
    }
  }
  true
}

// pub fn solution_1() -> u64 {
//   let big_num = 600851475143;
//   for i in (2..=big_num).rev() {
//     if big_num % i == 0 && is_prime(i) {
//       return i;
//     }
//   }
//   1
// }

pub fn solution_2() -> u64 {
  let mut big_num = 600851475143;
  let mut max_prime_factor = 2;
  while big_num >= 2 {
    for i in 2..=big_num {
      if big_num % i == 0 && is_prime(i) {
        big_num = big_num / i;
        if i > max_prime_factor {
          max_prime_factor = i;
          break;
        }
      }
    }
  }
  max_prime_factor
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution_2() {
    assert_eq!(solution_2(), 6857);
  }
}
