// https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&id=top-interview-150

fn main() {
  let mut nums = vec![1];
  let k = remove_duplicates(&mut nums);
  assert_eq!(nums[0..k as usize], vec![1]);
  assert_eq!(k, 1);

  let mut nums = vec![1, 2, 3, 4, 5];
  let k = remove_duplicates(&mut nums);
  assert_eq!(nums[0..k as usize], vec![1, 2, 3, 4, 5]);
  assert_eq!(k, 5);

  let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
  let k = remove_duplicates(&mut nums);
  assert_eq!(nums[0..k as usize], vec![0, 1, 2, 3, 4]);
  assert_eq!(k, 5);

  let mut nums = vec![1, 2, 2];
  let k = remove_duplicates(&mut nums);
  assert_eq!(nums[0..k as usize], vec![1, 2]);
  assert_eq!(k, 2);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  let mut i = 1;
  let mut j = 1;
  while j < nums.len() {
    if nums[j] != nums[j - 1] {
      nums[i] = nums[j];
      i += 1;
    }
    j += 1;
  }
  i as i32
}
