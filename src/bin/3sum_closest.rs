// https://leetcode.com/problems/3sum-closest/

fn main() {
  let nums = vec![-1, 2, 1, -4];
  let result = three_sum_closest(nums, 1);
  println!("{result}");
}

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
  nums.sort_unstable();
  let n = nums.len();
  let mut cur_min_diff = i32::MAX;
  let mut cur_sum = 0;
  for i in 0..n - 1 {
    let mut j = i + 1;
    let mut k = n - 1;
    while j < k {
      let sum = nums[i] + nums[j] + nums[k];
      let diff = sum - target;
      let diff_abs = i32::abs(diff);
      if diff_abs < cur_min_diff {
        cur_min_diff = diff_abs;
        cur_sum = sum;
      }
      if diff > 0 {
        // we shouldn't increase j, but should decrease k
        k -= 1;
      } else if diff < 0 {
        j += 1;
      } else {
        // can't get any smaller than 0
        return cur_sum;
      }
    }
  }
  cur_sum
}
