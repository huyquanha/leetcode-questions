// https://leetcode.com/problems/trapping-rain-water/description/
// TODO: revisit this and found a better solution.
fn main() {
  let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
  println!("{}", trap(height));
}

fn trap(height: Vec<i32>) -> i32 {
  let mut trap_total = 0;
  let mut i = 0;
  while i < height.len() - 1 && height[i + 1] >= height[i] {
    i += 1;
  }
  // Here height[i + 1] < height[i].
  while i < height.len() - 1 {
    let mut minus = 0;
    let mut j = i + 1;
    let mut max_index = j;
    let mut minus_until_max_index = 0;
    while j < height.len() && height[j] < height[i] {
      if height[j] > height[max_index] {
        max_index = j;
        minus_until_max_index = minus;
      }
      minus += height[j];
      j += 1;
    }
    if j == height.len() {
      // we run to end of array, take max_index instead.
      // min(height[i], height[max_index]) = height[max_index]
      trap_total += height[max_index] * (max_index - i - 1) as i32 - minus_until_max_index;
      i = max_index;
    } else {
      // Here height[j] >= height[i].
      trap_total += height[i] * (j - i - 1) as i32 - minus;
      i = j;
    }
  }
  trap_total
}