// https://leetcode.com/problems/remove-element/?envType=study-plan-v2&id=top-interview-150
fn main() {
  let mut nums = vec![1, 3, 2];
  let val = 1;
  let k = remove_element(&mut nums, val);
  println!("{:?}", nums);
  println!("{}", k);

  let mut nums = vec![0,1,2,2,3,0,4,2];
  let val = 2;
  let k = remove_element(&mut nums, val);
  println!("{:?}", nums);
  println!("{}", k);

  let mut nums = vec![1];
  let val = 1;
  let k = remove_element(&mut nums, val);
  println!("{:?}", nums);
  println!("{}", k);
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
  if nums.is_empty() {
    return 0
  }
  let mut i = 0;
  let mut j: i32 = nums.len() as i32 - 1;
  while i <= j {
    let idx_i = i as usize;
    let idx_j = j as usize;
    if nums[idx_j] == val {
      j -= 1;
      continue;
    }
    if nums[idx_i] == val {
      // here, nums[j] != val, and nums[i] = val.
      // We want to switch the 2.
      nums[idx_i] = nums[idx_j];
      nums[idx_j] = val;
      i += 1;
      j -= 1;
    } else {
      // nums[i] is also not val.
      i += 1;
    }
  }
  j + 1
}
