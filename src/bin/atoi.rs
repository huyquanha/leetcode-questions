use std::collections::HashSet;

fn main() {
  let x = my_atoi(String::from("42"));
  println!("{x}");
}

fn my_atoi(s: String) -> i32 {
  let mut list: Vec<char> = Vec::new();
  let digitSet =
    HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
  let mut found_non_ws = false;
  let mut positive: Option<bool> = None;
  for c in s.chars() {
    if !found_non_ws && c == ' ' {
      continue;
    }
    // if we come here, c is not a whitespace anymore.
    found_non_ws = true;
    if positive == None {
      // If positive is still None, we require the next character to either
      // be a sign, or a digit.
      if c == '-' {
        positive = Some(false);
      } else if c == '+' {
        // +, and any immediate digits are treated as positives.
        positive = Some(true);
      } else if digitSet.contains(&c) {
        positive = Some(true);
        if c != '0' {
          list.push(c);
        }
      } else {
        break;
      }
    } else {
      // after positive is set (true or false), we require any remaining
      // characters to be a digit.
      if !digitSet.contains(&c) {
        break;
      }
      // Ignore leading 0s.
      if c == '0' && list.is_empty() {
        continue;
      }
      list.push(c);
    }
  }
  if list.is_empty() {
    return 0;
  }
  let mut result = 0;
  let multiplier: i32 = if positive.unwrap() { 1 } else { -1 };
  for c in list {
    let digit = c.to_digit(10).unwrap() as i32 * multiplier;
    if positive.unwrap()
      && (result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7))
    {
      return i32::MAX;
    }
    if !positive.unwrap()
      && (result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < -8))
    {
      return i32::MIN;
    }
    result = result * 10 + digit;
  }
  result
}
