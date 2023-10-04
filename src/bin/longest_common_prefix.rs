fn main() {
  let strs = vec!["flower", "flow", "flight"];
  let strs: Vec<String> = strs.iter().map(|s| String::from(*s)).collect();
  let prefix = longest_common_prefix(strs);
  assert_eq!(prefix, "fl");

  let strs = vec!["dog", "racecar", "car"];
  let strs: Vec<String> = strs.iter().map(|s| String::from(*s)).collect();
  let prefix = longest_common_prefix(strs);
  assert_eq!(prefix, "");
}

fn longest_common_prefix(strs: Vec<String>) -> String {
  if strs.len() == 1 {
    return String::clone(&strs[0]);
  }
  let mut result = String::from("");
  for (i, c) in strs[0].chars().enumerate() {
    for s in strs[1..].iter() {
      if i >= s.len() || s[i] != c {

      }
    }
  }
  result
}
