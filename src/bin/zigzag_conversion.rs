// https://leetcode.com/problems/zigzag-conversion/description/

fn main() {}

fn convert(s: String, num_rows: i32) -> String {
  // If there's just one row, our algo will fail. Also if just 1 row,
  // we can short-circuit and return the original string.
  if num_rows == 1 {
    return s;
  }
  let rows = num_rows as usize;
  let mut charsByNumber: Vec<Vec<char>> = vec![Vec::new(); rows];
  let mut i = 0;
  let mut up = true;
  for c in s.chars() {
    charsByNumber[i].push(c);
    if up {
      i += 1;
    } else {
      i -= 1;
    }
    if i == 0 || i == rows - 1 {
      up = !up;
    }
  }
  charsByNumber.into_iter().flatten().collect()
}
