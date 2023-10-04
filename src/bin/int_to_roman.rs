fn main() {
  println!("{}", int_to_roman(3));
}

fn int_to_roman(mut num: i32) -> String {
  let mut result = String::from("");
  while num > 0 {
    if num >= 1000 {
      result.push_str("M");
      num -= 1000;
    } else if num >= 900 {
      result.push_str("CM");
      num -= 900;
    } else if num >= 500 {
      result.push_str("D");
      num -= 500;
    } else if num >= 400 {
      result.push_str("CD");
      num -= 400;
    } else if num >= 100 {
      result.push_str("C");
      num -= 100;
    } else if num >= 90 {
      result.push_str("XC");
      num -= 90;
    } else if num >= 50 {
      result.push_str("L");
      num -= 50;
    } else if num >= 40 {
      result.push_str("XL");
      num -= 40;
    } else if num >= 10 {
      result.push_str("X");
      num -= 10;
    } else if num >= 9 {
      result.push_str("IX");
      num -= 9;
    } else if num >= 5 {
      result.push_str("V");
      num -= 5;
    } else if num >= 4 {
      result.push_str("IV");
      num -= 4;
    } else {
      result.push_str("I");
      num -= 1;
    }
  }
  result
}