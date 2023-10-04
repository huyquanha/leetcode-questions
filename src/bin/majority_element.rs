use std::collections::HashMap;

// https://leetcode.com/problems/majority-element/?envType=study-plan-v2&envId=top-interview-150
fn main() {
  let x = majority_element(vec![3, 2, 3]);
  assert_eq!(x, 3);

  let x = majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
  assert_eq!(x, 2);
}

fn majority_element(nums: Vec<i32>) -> i32 {
  let n = nums.len();
  let mut frequency: HashMap<i32, i32> = HashMap::new();
  for num in nums {
    if frequency.contains_key(&num) {
      let current_freq = frequency.get(&num).unwrap();
      if current_freq + 1 > n as i32 / 2 {
        return num;
      }
      frequency.insert(num, current_freq + 1);
    } else {
      frequency.insert(num, 1);
    }
  }
  panic!("Found nothing!")
}
