/*
求小于 1000 的能被 3 或 5 整除的所有整数之和。
*/

pub fn solution_1() -> u32 {
  let mut sum = 0u32;
  for i in 1..1000 {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i;
    }
  }
  sum
}

pub fn solution_2() -> u32 {
  (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<u32>()
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution_1() {
    assert_eq!(solution_1(), 233168);
  }

  #[test]
  fn test_solution_2() {
    assert_eq!(solution_2(), 233168);
  }
}
