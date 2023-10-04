// https://leetcode.com/problems/unique-paths/ 
fn main() {
  assert_eq!(28, unique_paths(3, 7));
  assert_eq!(3, unique_paths(3, 2));
  assert_eq!(1, unique_paths(2, 1));
}

fn unique_paths(m: i32, n: i32) -> i32 {
  let downs = m - 1;
  let rights = n - 1;
  let moves = downs + rights;
  // This is basically combinatory math: given ${downs} red balls and
  // ${rights} green balls, how many ways to uniquely arrange them.
  // If the balls were all unique, there should be moves! ways.
  // However, because re-arranging red balls within themselves (downs!)
  // and green balls within themselves (rights!) don't actually change the combination,
  // we should divide them from the total number.
  if downs > rights {
    // moves! / downs! -> factorial(downs + 1..moves + 1)
    return factorial((downs + 1..moves + 1).collect(), (1..rights + 1).collect());
  } else {
    return factorial((rights + 1..moves + 1).collect(), (1..downs + 1).collect());
  }
}

fn factorial(dividends: Vec<i32>, divisors: Vec<i32>) -> i32 {
  let mut result: i64 = 1;
  for (idx, &dividend) in dividends.iter().enumerate() {
    // We need to do the division to avoid result being too big (it can even get over i64 limit).
    result = result * dividend as i64 / divisors[idx] as i64;
  }
  
  result as i32
}

