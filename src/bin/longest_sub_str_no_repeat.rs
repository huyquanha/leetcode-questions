// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/?envType=study-plan-v2&id=top-interview-150

use std::{collections::HashMap, cmp::max};

fn main() {
  let s = String::from("abcabcbb");
  let n = length_of_longest_substring(s);
  assert_eq!(n, 3);

  let s = String::from("bbbb");
  let n = length_of_longest_substring(s);
  assert_eq!(n, 1);

  let s = String::from("");
  let n = length_of_longest_substring(s);
  assert_eq!(n, 0);

  let s = String::from("wpwkew");
  let n = length_of_longest_substring(s);
  assert_eq!(n, 4);

  let s = String::from("abba");
  let n = length_of_longest_substring(s);
  assert_eq!(n, 2);
}

fn length_of_longest_substring(s: String) -> i32 {
  let mut longest: usize = 0;
  if s.is_empty() {
    return longest as i32;
  }
  let mut occurs: HashMap<char, usize> = HashMap::new();
  let mut start: usize = 0;
  for (i, c) in s.chars().enumerate() {
    // If c was encountered before (by the presence of prev_idx),
    // and that prev_idx is within the range we're considering (>= start)
    // then we need to move start forward. Otherwise if the previous
    // occurrence is before start, it doesn't really conflict with
    // our substring.
    if let Some(prev_idx) = occurs.get(&c) {
      if *prev_idx >= start {
        longest = max(longest, i - start);
        start = *prev_idx + 1;
      }
    }
    occurs.insert(c, i);
  }
  // We could come to the end of the array without finding
  // another duplicate, so we make sure to take the last
  // substring from start -> (s.len() - 1) into account.
  longest = max(longest, s.len() - start);
  longest as i32
}
