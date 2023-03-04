use std::{collections::HashMap, cmp::max};

fn main() {
  let s = String::from("abcabcbb");
  println!("{}", length_of_longest_substring(s));
}

fn length_of_longest_substring(s: String) -> i32 {
  if s.is_empty() {
    return s.len().try_into().unwrap();
  }
  let mut map = HashMap::new();
  let mut max_len = 0;
  let mut start = 0;
  for (i, c) in s.chars().enumerate() {
    // If the map already contains letter `c` => a repeat, and that repeat
    // happens somewhere after `start` of the substring => the substring
    // we are considering is invalid.
    if map.contains_key(&c) && *map.get(&c).unwrap() >= start {
      max_len = max(max_len, i - start);
      // We move `start` past that repeated character.
      start = map.get(&c).unwrap() + 1;
    }
    map.insert(c, i); // updates the last occurrence of c to i.
  }
  // It could also happen that we come to the end without finding any duplicates.
  max_len = max(max_len, s.len() - start);
  max_len.try_into().unwrap()
}
