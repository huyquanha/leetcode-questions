// https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/

use std::collections::HashMap;

fn main() {
  let digits = String::from("23");
  println!("{:?}", letter_combinations(digits));

  let digits = String::from("");
  println!("{:?}", letter_combinations(digits));

  let digits = String::from("2");
  println!("{:?}", letter_combinations(digits));
}

fn letter_combinations(digits: String) -> Vec<String> {
  let mappings = HashMap::from([
    ('2', vec!["a", "b", "c"]),
    ('3', vec!["d", "e", "f"]),
    ('4', vec!["g", "h", "i"]),
    ('5', vec!["j", "k", "l"]),
    ('6', vec!["m", "n", "o"]),
    ('7', vec!["p", "q", "r", "s"]),
    ('8', vec!["t", "u", "v"]),
    ('9', vec!["w", "x", "y", "z"]),
  ]);
  let mut result: Vec<String> = Vec::new();
  let n = digits.len();
  if n == 0 {
    return result;
  }
  for c in digits.chars() {
    let letters = mappings.get(&c).unwrap().clone();
    if result.is_empty() {
      for l in letters {
        result.push(String::from(l));
      }
    } else {
      let mut next_result: Vec<String> = Vec::new();
      while !result.is_empty() {
        let cur = result.pop().unwrap();
        for &l in letters.iter() {
          next_result.push(format!("{cur}{l}"));
        }
      }
      result = next_result;
    }
  }
  result
}
