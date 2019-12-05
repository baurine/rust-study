/*
求 1 到 100 自然数的“和的平方”与“平方和”的差。
*/

pub fn solution() -> u32 {
  let sum_of_squares = (1..=100).map(|x| x*x).sum::<u32>();
  let square_sum = (1..=100).sum::<u32>().pow(2);
  square_sum - sum_of_squares
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution_1() {
    assert_eq!(solution(), 25164150);
  }
}
