// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/?envType=study-plan-v2&id=top-interview-150

fn main() {
  let mut nums = vec![1, 1, 1, 2, 2, 3];
  let k = remove_duplicates(&mut nums);
  assert_eq!(k, 5);
  assert_eq!(nums[0..k as usize], vec![1, 1, 2, 2, 3]);

  let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
  let k = remove_duplicates(&mut nums);
  assert_eq!(k, 7);
  assert_eq!(nums[0..k as usize], vec![0, 0, 1, 1, 2, 3, 3]);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  let mut i = 1;
  let mut j = 1;
  let mut dup_count = 0;
  while j < nums.len() {
    if nums[j] == nums[j - 1] {
      if dup_count == 0 {
        nums[i] = nums[j];
        i += 1;
        dup_count += 1;
      }
    } else {
      nums[i] = nums[j];
      i += 1;
      dup_count = 0;
    }
    j += 1;
  }
  i as i32
}
