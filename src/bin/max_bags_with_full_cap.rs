// https://leetcode.com/problems/maximum-bags-with-full-capacity-of-rocks/

fn main() {
  let capacity = vec![2, 3, 4, 5];
  let rocks = vec![1, 2, 4, 4];
  assert_eq!(maximum_bags(capacity, rocks, 2), 3);

  let capacity = vec![10, 2, 2];
  let rocks = vec![2, 2, 0];
  assert_eq!(maximum_bags(capacity, rocks, 100), 3);
}

fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
  let mut remain = Vec::with_capacity(capacity.len());
  for (i, cap) in capacity.iter().enumerate() {
    remain.push(cap - rocks[i]);
  }
  remain.sort_unstable();

  let mut remain_rocks = additional_rocks;
  let mut index = 0;
  while remain_rocks > 0 && index < remain.len() {
    remain_rocks -= remain[index];
    if remain_rocks >= 0 {
      index += 1;
    }
  }
  index as i32
}  