// https://leetcode.com/problems/valid-parentheses/

use std::collections::HashMap;

fn main() {
  let mut s = String::from("()");
  assert_eq!(is_valid(s), true);

  s = String::from("()[]{}");
  assert_eq!(is_valid(s), true);

  s = String::from("([]){}");
  assert_eq!(is_valid(s), true);

  s = String::from("({[]})");
  assert_eq!(is_valid(s), true);

  s = String::from("(]");
  assert_eq!(is_valid(s), false);

  s = String::from("([{})]");
  assert_eq!(is_valid(s), false);

  // single open bracket without closing should return false.
  s = String::from("[");
  assert_eq!(is_valid(s), false);
}

fn is_valid(s: String) -> bool {
  // Quick check: if s only has one bracket, there's no way it's valid.
  // This can help save allocating map memory in these edge cases.
  if s.len() == 1 {
      return false;
  }

  let mut brackets: HashMap<char, char> = HashMap::new();
  brackets.insert('(', ')');
  brackets.insert('[', ']');
  brackets.insert('{', '}');

  let mut stack = vec![];
  for c in s.chars() {
    if brackets.contains_key(&c) {
      stack.push(c);
    } else {
      // Must be a closing bracket.
      let open_bracket_option = stack.pop();
      if let Some(open_bracket) = open_bracket_option {
        let expected_closing_bracket = brackets.get(&open_bracket).unwrap();
        if c != *expected_closing_bracket {
          return false;
        }
      } else {
        return false;
      }
    }
  }
  stack.is_empty()
}
