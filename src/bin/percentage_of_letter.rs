// https://leetcode.com/problems/percentage-of-letter-in-string/
fn main() {
  let s = String::from("foobar");
  let letter = 'o';
  assert_eq!(percentage_letter(s, letter), 33);

  let s = String::from("jjjj");
  let letter = 'k';
  assert_eq!(percentage_letter(s, letter), 0);
}

fn percentage_letter(s: String, letter: char) -> i32 {
  let mut count: i32 = 0;
  for c in s.chars() {
    if letter == c {
      count += 1;
    }
  }
  return (count as f64 / s.len() as f64 * 100.0) as i32;
}
