// https://leetcode.com/contest/weekly-contest-332/problems/count-the-number-of-fair-pairs/
fn main() {
  //    [0,1,7,4,4,5]
  // [0,1,4,4,5,7]
  // keep 0, move 5,4,4 => 3 more.
  // keep 5, move 0 up => 1 more.
  // >= [3,2,-4,-1,-1,-2] to have sum >= 3
  // <= [6,5,-1,2,2,1] to have sum <= 6.
  // [0,1,2,3,4,5]
  // [0,1,3,4,5,2] the indexes order by how big the elements are.
  // (0,5), from here we could do either (0, 4) or (1, 5)
  // we could also do (1, 4) and try to go as low as we can while still more than upper.
  // [0, 0, 0, 0, 0, 0]
  //  i              j
  // end            start
  // - 1
  // - 2
  // ...
  // - [(start - 1) - (end + 1) + 1] = start - end - 1
  let nums = vec![3,-8,-5,-4,-1000000000,-1000000000,-1000000000,-1000000000];
  println!("{}", count_fair_pairs(nums, -10, 15));
}

fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
  // Sort the number unstably for better performance (using quicksort instead of mergesort)
  let mut nums = nums.clone();
  nums.sort_unstable();
  let mut i = 0;
  let mut j = nums.len() - 1;
  // Find the number of pairs that are smaller than lower.
  let mut less_than_lower_count: i64 = 0;
  while i < j {
    let sum = nums[i] + nums[j];
    if sum >= lower {
      j -= 1;
    } else {
      // here sum < lower
      less_than_lower_count += j as i64 - i as i64; // together with nums[i], all numbers from nums[i + 1].. nums[j] sum will < lower.
      i += 1; // try increasing i and see which upper bound j create a sum < lower.
    }
  }
  // Find the number of pairs that are <= upper.
  i = 0;
  j = nums.len() - 1;
  let mut less_than_equal_upper_count: i64 = 0;
  while i < j {
    let sum = nums[i] + nums[j];
    if sum > upper {
      j -= 1;
    } else {
      // here sum <= upper.
      less_than_equal_upper_count += j as i64 - i as i64;
      i += 1;
    }
  }
  less_than_equal_upper_count - less_than_lower_count
}

// Brute-force version.
fn count_fair_pairs_bf(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
  let mut count = 0;
  for i in 0..nums.len() - 1 {
    for j in (i+1)..nums.len() {
      let sum = nums[i] as i32 + nums[j];
      if sum >= lower && sum <= upper {
        count += 1;
      }
    }
  }
  count
}
