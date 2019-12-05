/*
找出能够被 1, 2, 3, ..., 20 (1 到 20 的所有数) 整除的最小整数。
*/

fn can_divide_1_to_20(n: u32) -> bool {
  for x in (2..=20).rev() {
    if n % x != 0 {
      return false;
    }
  }
  true
}

pub fn min_int_can_divide_1_to_20() -> u32 {
  let mut min_int = 100u32;
  for n in (100..).step_by(2) {
    if can_divide_1_to_20(n) {
      min_int = n;
      break;
    }
  }
  min_int
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution_1() {
    assert_eq!(min_int_can_divide_1_to_20(), 232792560);
  }
}
