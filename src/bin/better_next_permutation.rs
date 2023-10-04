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
  // From right -> left, find the first index i where nums[i] < nums[i + 1]
  // If that does not exist, the entire list is sorted in decreasing order
  // so we just reverse the list.
  let n = nums.len();
  if n <= 1 {
    return;
  }
  let mut idx = n;
  for i in (0..n - 1).rev() {
    if nums[i] < nums[i + 1] {
      idx = i;
      break;
    }
  }
  if idx == n {
    // Nothing found, reverse the whole list.
    reverse(nums, 0, n - 1);
    return;
  }
  // Find the first number greater than nums[idx] in the right range (we know this is sorted in decreasing order).
  // We are guaranteed to find one, then swap the 2 with each other.
  let first_greater_idx = search(nums, idx + 1, n - 1, nums[idx]);
  swap(nums, first_greater_idx, idx);
  // Now reverse the right range so it's in ascending order (to be as small as possible).
  reverse(nums, idx + 1, n - 1);
}

fn search(nums: &Vec<i32>, mut lo: usize, mut hi: usize, v: i32) -> usize {
  while lo <= hi {
    let mid = lo + (hi - lo) / 2;
    if nums[mid] <= v {
      // We need to go left of mid, where the elements are bigger
      hi = mid - 1;
    } else {
      // nums[mid] already qualifies, but we want to go right (where elements are smaller)
      // to see if there's even a smaller one that is still bigger than v.
      lo = mid + 1;
    }
  }
  lo - 1
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
  let tmp = nums[i];
  nums[i] = nums[j];
  nums[j] = tmp;
}

fn reverse(nums: &mut Vec<i32>, mut lo: usize, mut hi: usize) {
  while lo < hi {
    swap(nums, lo, hi);
    lo += 1;
    hi -= 1;
  }
}