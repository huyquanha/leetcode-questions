// https://leetcode.com/problems/merge-sorted-array/?envType=study-plan-v2&id=top-interview-150

fn main() {
  let mut nums1 = vec![1, 2, 3, 0, 0, 0];
  let mut nums2 = vec![2, 5, 6];
  merge(&mut nums1, 3, &mut nums2, 3);
  println!("{:?}", nums1);

  let mut nums1 = vec![1, 2, 5, 7, 9, 0, 0, 0, 0];
  let mut nums2 = vec![3, 4, 6, 8];
  merge(&mut nums1, 5, &mut nums2, 4);
  println!("{:?}", nums1);
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
  let mut i = m - 1; // track nums1
  let mut j = n - 1; // tracks nums2
  let mut k = m + n - 1;
  while i >= 0 && j >= 0 {
    let idx_i = i as usize;
    let idx_j = j as usize;
    let idx_k = k as usize;
    if nums1[idx_i] >= nums2[idx_j] {
      nums1[idx_k] = nums1[idx_i];
      i -= 1;
      k -= 1;
    } else {
      nums1[idx_k] = nums2[idx_j];
      j -= 1;
      k -= 1;
    }
  }
  while j >= 0 {
    nums1[k as usize] = nums2[j as usize];
    k -= 1;
    j -= 1;
  }
}
