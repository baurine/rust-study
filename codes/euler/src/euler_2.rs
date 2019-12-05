/*
400 万之内所有偶数的斐波那契数字之和。
*/

pub fn solution_1() -> u32 {
  let mut fib_arr = vec![1, 2];
  let mut sum = 2;
  let mut i = 2;
  loop {
    let next_fib = fib_arr[i-1] + fib_arr[i-2];
    if next_fib > 400_0000 {
      break;
    }
    if next_fib % 2 == 0 {
      sum += next_fib;
    }
    fib_arr.push(next_fib);
    i += 1;
  }
  sum
}

pub fn solution_2() -> u32 {
  let mut fib_arr = vec![1, 2];
  for i in 2.. {
    let next_fib = fib_arr[i - 1] + fib_arr[i - 2];
    if next_fib > 400_0000 {
      break;
    }
    fib_arr.push(next_fib)
  }
  fib_arr.iter().filter(|&x| x % 2 == 0).sum::<u32>()
}

/////////////////////////////////////
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution_1() {
    assert_eq!(solution_1(), 4613732);
  }

  #[test]
  fn test_solution_2() {
    assert_eq!(solution_2(), 4613732);
  }
}
