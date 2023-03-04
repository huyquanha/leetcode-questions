fn main() {}

// abcdcba
// abbacdcab
// abbbacdcabf
fn longest_palindrome(s: String) -> String {
  let mut max_len = 0;
  let mut start = 0;
  let mut end = 0;
  let chars: Vec<char> = s.chars().collect();
  let mut i = 0;
  while i < chars.len() {
    // j will be pointing at characters equalling c.
    let mut j = i;
    while j + 1 < chars.len() && chars[j + 1] == chars[i] {
      j += 1;
    }
    let next_i = j + 1;
    // [i..j] is the centre of the palindrome.
    // chars[j+1] is different though. We extend to both ends to see if we
    // can find a similar character on both ends, and continue doing that.
    while i > 0 && j + 1 < chars.len() && chars[i - 1] == chars[j + 1] {
      i -= 1;
      j += 1;
    }
    // Now we check if j - 1 + 1 is more than max_len, and if so update start and end.
    if j - i + 1 > max_len {
      start = i;
      end = j;
      max_len = end - start + 1;
    }
    i = next_i;
  }
  String::from(s.get(start..end+1).unwrap())
}
