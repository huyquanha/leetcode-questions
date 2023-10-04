// https://leetcode.com/problems/next-permutation/
fn main() {
  let mut nums = vec![1, 2, 3];
  next_permutation(&mut nums);
  assert_eq!(nums, vec![1, 3, 2]);

  let mut nums = vec![3, 2, 1];
  next_permutation(&mut nums);
  assert_eq!(nums, vec![1, 2, 3]);

  let mut nums = vec![1, 1, 5];
  next_permutation(&mut nums);
  assert_eq!(nums, vec![1, 5, 1]);

  let mut nums = vec![2, 0, 3, 1];
  next_permutation(&mut nums);
  assert_eq!(nums, vec![2, 1, 0, 3]);

  let mut nums = vec![2, 3, 1];
  next_permutation(&mut nums);
  assert_eq!(nums, vec![3, 1, 2]);

  let mut nums = vec![2, 4, 2, 3, 1];
  next_permutation(&mut nums);
  assert_eq!(nums, vec![2, 4, 3, 1, 2]);

  let mut nums = vec![2,2,7,5,4,3,2,2,1];
  next_permutation(&mut nums);
  assert_eq!(nums, vec![2,3,1,2,2,2,4,5,7]);
}

fn next_permutation(nums: &mut Vec<i32>) {
  let n = nums.len();
  if n <= 1 {
    return;
  }
  let mut lo_idx = n - 1;
  let hi_idx = n - 1;
  let mut idx = n - 2;
  loop {
    let smallest_higher_idx = search(nums, nums[idx], lo_idx, hi_idx);
    if smallest_higher_idx > hi_idx {
      // Means, no numbers are higher. Move all numbers left one spot,
      // and move nums[idx] into the last position
      bubble(nums, lo_idx, hi_idx, nums[idx]);
      lo_idx = idx;
      if idx > 0 {
        idx -= 1;
      } else {
        break;
      }
    } else {
      swap(nums, idx, smallest_higher_idx);
      break;
    }
  }
}

fn search(nums: &Vec<i32>, v: i32, mut lo: usize, mut hi: usize) -> usize {
  while lo <= hi {
    let mid = lo + (hi - lo) / 2;
    if nums[mid] <= v {
      lo = mid + 1;
    } else {
      hi = mid - 1;
    }
  }
  // Found nothing, returns the one more than hi_idx.
  hi + 1
}

fn bubble(nums: &mut Vec<i32>, lo: usize, hi: usize, v: i32) {
  for i in lo..hi + 1 {
    nums[i - 1] = nums[i];
  }
  nums[hi] = v;
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
  let tmp = nums[i];
  nums[i] = nums[j];
  nums[j] = tmp;
}