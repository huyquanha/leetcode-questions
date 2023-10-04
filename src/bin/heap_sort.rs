fn main() {
  let mut nums = vec![3, 2, 5, 1, 4, 2, 6,6,9,8];
  sort(&mut nums);
  println!("{:?}", nums);
}

fn sort(nums: &mut Vec<i32>) {
  let n = nums.len();
  for i in (1..n / 2 + 1).rev() {
    sink(nums, i, n);
  }
  // Now sortdown.
  let mut k = n;
  while k > 1 {
    // swap the largest element (at 1) down to the bottom.
    swap(nums, 1, k);
    k -= 1;
    // Sink the new element at 1 down.
    sink(nums, 1, k);
  }
}

fn sink(nums: &mut Vec<i32>, mut i: usize, n: usize) {
  while i * 2 <= n {
    let mut j = i * 2;
    if j < n && less(nums, j, j + 1) {
      j += 1;
    }
    if !less(nums, i, j) {
      break;
    }
    swap(nums, i, j);
    i = j;
  }
}

fn less(nums: &mut Vec<i32>, i: usize, j: usize) -> bool {
  nums[i - 1] < nums[j - 1]
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
  let tmp = nums[i - 1];
  nums[i - 1] = nums[j - 1];
  nums[j - 1] = tmp;
}