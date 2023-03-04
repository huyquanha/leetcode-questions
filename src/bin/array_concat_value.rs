use std::collections::VecDeque;

// https://leetcode.com/contest/weekly-contest-332/problems/find-the-array-concatenation-value/
fn main() {

}

fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
  let mut deque: VecDeque<i32> = nums.into_iter().collect();
  let mut concat_total: i64 = 0;
  while deque.len() > 1 {
    let mut front = deque.pop_front().unwrap() as i64;
    let back  = deque.pop_back().unwrap();
    let mut back_remain = back;
    while back_remain > 0 {
      front *= 10;
      back_remain /= 10;
    }
    concat_total += front + back as i64;
  }
  if !deque.is_empty() {
    concat_total += deque.pop_front().unwrap() as i64;
  }
  concat_total
}