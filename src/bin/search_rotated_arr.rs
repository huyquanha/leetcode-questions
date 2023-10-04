// https://leetcode.com/problems/search-in-rotated-sorted-array/description/
fn main() {
  let nums = vec![4,5,6,7,0,1,2];
  assert_eq!(4, search(nums, 0));

  let nums = vec![4,5,6,7,0,1,2];
  assert_eq!(-1, search(nums, 3));

  let nums = vec![1];
  assert_eq!(-1, search(nums, 0));
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
  // search the pivot. If there's one, the last element in the array will be 
  // smaller than the first element (due to all elements being unique). now
  // we need to know where it starts.
  // the array will increase, and then drop to a low number, then increase again.
  let n = nums.len();
  let v: i32 = nums[n - 1];
  if v < nums[0] {
    // pivots happens.
    let mut lo = 0;
    let mut hi = n - 1;
    while lo < hi {
      let mid = lo + (hi - lo) / 2;
      if nums[mid] > v {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    let orig_start = lo;
    let left_idx = bin_search(&nums, 0, orig_start - 1, target);
    return if left_idx != -1 { left_idx } else { bin_search(&nums, orig_start, n - 1, target)} ;
  } else {
    return bin_search(&nums, 0, n - 1, target);
  }
}

fn bin_search(nums: &Vec<i32>, lo: usize, hi: usize, target: i32) -> i32 {
  let mut lo = lo as i32;
  let mut hi = hi as i32;
  while lo <= hi {
    let mid = lo + (hi - lo) / 2;
    if nums[mid as usize] == target {
      return mid;
    } else if nums[mid as usize] < target {
      lo = mid + 1;
    } else {
      hi = mid - 1;
    }
  }
  -1
}