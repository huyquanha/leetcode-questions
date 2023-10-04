// https://leetcode.com/problems/generate-parentheses/description/

fn main() {
  let result = generate_parentheses(1);
  println!("{:?}", result);
}

struct Bundle {
  s: String,
  open: i32,
  closed: i32,
}

impl Bundle {
  fn new(s: String, open: i32, closed: i32) -> Bundle {
    Bundle {
      s, open, closed
    }
  }
}

fn generate_parentheses(n: i32) -> Vec<String> {
  let mut result = Vec::new();
  let mut stack: Vec<Bundle> = Vec::new();
  let expected_len = n * 2;
  stack.push(Bundle::new(String::from("("), 1, 0));
  while !stack.is_empty() {
    let b = stack.pop().unwrap();
    if b.s.len() as i32 == expected_len {
      result.push(b.s);
      continue;
    }
    if b.closed < b.open {
      let mut new_s = String::clone(&b.s);
      new_s.push(')');
      stack.push(Bundle::new(new_s, b.open, b.closed + 1));
    }
    if b.open < n {
      let mut new_s = String::clone(&b.s);
      new_s.push('(');
      stack.push(Bundle::new(new_s, b.open + 1, b.closed))
    }
  }
  result
}