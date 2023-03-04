use std::collections::HashSet;

use rand::Rng;

fn main() {
  let nums = vec![-1, 0, 1, 2, -1, -4];
  // sort: -4,-1,-1,0,1,2
  println!("{:?}", three_sum(nums));
  assert_eq!(vec![1, 2, 3], vec![1, 2, 3]);
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
  // Random shuffle to avoid worst case with quicksort.
  let mut nums = random_shuffle(nums);
  nums.sort_unstable(); // quicksort
  let mut result: Vec<Vec<i32>> = Vec::new();
  let mut last_i: Option<i32> = None;
  for i in 0..nums.len() - 2 {
    // Avoid repeating nums[i].
    if last_i != None && nums[i] == last_i.unwrap() {
      continue;
    }
    let mut j = i + 1;
    let mut k = nums.len() - 1;
    while j < k {
      let sum = nums[j] + nums[k];
      if sum < -nums[i] {
        j += 1;
      } else if sum > -nums[i] {
        k -= 1;
      } else {
        // to eliminate duplicates.
        result.push(vec![nums[i], nums[j], nums[k]]);
        loop {
          j += 1;
          k -= 1;
          // Avoid repeatin nums[j] and nums[k]
          if j >= k || nums[j - 1] != nums[j] || nums[k + 1] != nums[k] {
            break;
          }
        }
      }
    }
    last_i = Some(nums[i]);
  }
  result
}

fn random_shuffle(nums: Vec<i32>) -> Vec<i32> {
  let mut nums = nums.clone();
  let mut rng = rand::thread_rng();
  for i in 0..nums.len() - 1 {
    let j = rng.gen_range(i + 1..nums.len());
    swap(&mut nums, i, j);
  }
  nums
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
  let temp = nums[i];
  nums[i] = nums[j];
  nums[j] = temp;
}
