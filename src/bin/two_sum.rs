fn main() {
  let nums = vec![150, 24, 79, 50, 88, 345, 3];
  // index array is [0, 1, 2]
  // after sorting is [1, 0, 2]
  let result = two_sum(nums, 200);
  println!("{:?}", result);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut indexes: Vec<usize> =
    nums.iter().enumerate().map(|(pos, _)| pos).collect();
  // Use 3-way quicksort on indexes
  let mut lo = 0;
  let mut hi = nums.len() - 1;
  sort(&nums, &mut indexes, lo, hi);
  // Now iterate from start to finish.
  loop {
    let sum = nums[indexes[lo]] + nums[indexes[hi]];
    if sum == target {
      return vec![indexes[lo] as i32, indexes[hi] as i32];
    } else if sum < target {
      lo += 1;
    } else {
      hi -= 1;
    }
  }
}

fn sort(nums: &Vec<i32>, indexes: &mut Vec<usize>, lo: usize, hi: usize) {
  if lo >= hi {
    return;
  }
  let mut lt = lo;
  let mut gt = hi;
  let mut i = lo + 1;
  let v = nums[indexes[lt]];
  // [lo..lt-1] will be numbers that < v.
  // [lt..gt] will be numbers that == v.
  // [gt + 1...hi] will be numbers that > v.
  while i <= gt {
    if nums[indexes[i]] < v {
      exch(indexes, i, lt);
      lt += 1;
      i += 1;
    } else if nums[indexes[i]] > v {
      exch(indexes, i, gt);
      gt -= 1;
    } else {
      i += 1;
    }
  }
  if lt > 0 {
    sort(nums, indexes, lo, lt - 1);
  }
  sort(nums, indexes, gt + 1, hi);
}

fn exch(nums: &mut Vec<usize>, i: usize, j: usize) {
  let tmp = nums[i];
  nums[i] = nums[j];
  nums[j] = tmp;
}
