fn main() {
  // 123 = 1 * 100 + 2 * 10 + 3
  // reverse: 3 * 100 + 2 * 10 + 1
  // println!("{}", reverse_magic(123));
  println!("{}", reverse_magic(-321));
}

fn reverse(x: i32) -> i32 {
  let s = x.to_string();
  let mut iter = s.chars();
  let reverse = if x < 0 {
    iter.next().unwrap().to_string() + &iter.rev().collect::<String>()[..]
  } else {
    iter.rev().collect()
  };
  let number = reverse.parse::<i32>();
  match number {
    Ok(i) => i,
    _ => 0,
  }
}

// Another solution that's more primitive.
fn reverse_magic(x: i32) -> i32 {
  let mut rev = 0;
  let mut remain = x;
  // i32::MAX is 2_147_483_647i32
  let max_pos_rev = i32::MAX / 10;
  let max_pos_digit = 7;
  // i32::MIN is -2_147_483_648i32
  let min_neg_rev = i32::MIN / 10;
  let min_neg_digit = -8;
  while remain != 0 {
    let last_digit = remain % 10;
    remain = remain / 10;
    if x > 0 {
      // i32::MAX is 2_147_483_647i32
      // rev * 10 + last_digit could exceed i32::MAX if and only if
      // rev > i32::MAX / 10, or rev == i32::MAX / 10 && last_digit > 7
      if rev < max_pos_rev || (rev == max_pos_rev && last_digit <= max_pos_digit) {
        rev = rev * 10 + last_digit;
      } else {
        return 0;
      }
    } else {
      if rev > min_neg_rev || (rev == min_neg_rev && last_digit >= min_neg_digit) {
        rev = rev * 10 + last_digit;
      } else {
        return 0;
      }
    }
  }
  rev
}
