// https://leetcode.com/problems/container-with-most-water/
use std::cmp::min;

fn main() {
  // Let say we already find i and j that gives us some area.
  // How do we move i and j to further maximise the result?
  // If h[i] <= h[j], decrementing j will not help us, because
  // even if h[j - 1] > h[j], min(h[i], h[j - 1]) is still h[i],
  // and the distance is one less so the area it yields will not be as big.
  // Similarly, if h[i] > h[j], we should decrement j instead of incrementing
  // i, because maximumly the min of 2 heights will be h[j] and the distance is one less.
  let height = vec![2, 3, 10, 5, 7, 8, 9];
  println!("{}", max_area(height));
}

fn max_area(height: Vec<i32>) -> i32 {
  let mut max_area = 0;
  let mut i = 0;
  let mut j = height.len() - 1;
  while i < j {
    let area = min(height[i], height[j]) * (j - i) as i32;
    if area > max_area {
      max_area = area;
    }
    if height[i] <= height[j] {
      i += 1;
    } else {
      j -= 1;
    }
  }
  max_area
}
