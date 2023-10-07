fn main() {
  println!("{}", to_roman_numeral(2421));
}

fn repeat_string(string: &str, number: i32) -> String {
  let mut result = String::new();
  for _ in 0..number {
    result.push_str(string);
  }
  result
}

fn get_single_roman_numeral(number: i32) -> String {
  match number {
    1 => "I",
    5 => "V",
    10 => "X",
    50 => "L",
    100 => "C",
    500 => "D",
    1000 => "M",
    _ => panic!("{} is not a single roman numeral", number),
  }.to_string()
}

fn is_substractive_notation(number: i32) -> bool {
  matches!(number, 4 | 9 | 40 | 90 | 400 | 900)
}

fn get_subtractive_notation(number: i32) -> String {
  match number {
    4 => "IV",
    9 => "IX",
    40 =>  "XL",
    90 =>  "XC",
    400 =>  "CD",
    900 =>  "CM",
    _ => panic!("{} is not a subtractive notation", number),
  }.to_string()
}

fn get_roman_numeral_for_divisor(divided_by_divisor: i32, divisor: &i32) -> String {
  if is_substractive_notation(divided_by_divisor*divisor) {
    return get_subtractive_notation(divided_by_divisor*divisor);
  }
  let roman_numeral_for_divisor = get_single_roman_numeral(*divisor);
  return repeat_string(&roman_numeral_for_divisor, divided_by_divisor);
}

fn to_roman_numeral(number: i32) -> String {
  if is_substractive_notation(number) {
      return get_subtractive_notation(number);
  }

  const DIVISORS: [i32; 7] = [1000, 500, 100, 50, 10, 5, 1];
  for divisor in DIVISORS.iter() {
      let divided_by_divisor = number / divisor;
      if divided_by_divisor > 0 {
        let mut result = String::new();
        result.push_str(&get_roman_numeral_for_divisor(divided_by_divisor, divisor));
        let remainder = number % divisor;
        if remainder > 0 {
          result.push_str(&to_roman_numeral(remainder));
        }
        return result;
      }
  }
  String::new()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] 
  fn it_returns_i_for_1() {
      assert_eq!("I", to_roman_numeral(1));
  }

  #[test]
  fn it_returns_ii_for_2() {
    assert_eq!("II", to_roman_numeral(2));
  }

  #[test]
  fn it_returns_iii_for_3() {
    assert_eq!("III", to_roman_numeral(3));
  }

  #[test]
  fn it_returns_iv_for_4() {
    assert_eq!("IV", to_roman_numeral(4));
  }

  #[test]
  fn it_returns_v_for_5() {
    assert_eq!("V", to_roman_numeral(5));
  }

  #[test]
  fn it_returns_vi_for_6() {
    assert_eq!("VI", to_roman_numeral(6));
  }

  #[test]
  fn it_returns_vii_for_7() {
    assert_eq!("VII", to_roman_numeral(7));
  }

  #[test]
  fn it_returns_viii_for_8() {
    assert_eq!("VIII", to_roman_numeral(8));
  }

  #[test]
  fn it_returns_ix_for_9() {
    assert_eq!("IX", to_roman_numeral(9));
  }

  #[test]
  fn it_returns_x_for_10() {
    assert_eq!("X", to_roman_numeral(10));
  }

  #[test]
  fn it_returns_xi_for_11() {
    assert_eq!("XI", to_roman_numeral(11));
  }

  #[test]
  fn it_returns_xii_for_12() {
    assert_eq!("XII", to_roman_numeral(12));
  }

  #[test]
  fn it_returns_xiii_for_13() {
    assert_eq!("XIII", to_roman_numeral(13));
  }

  #[test]
  fn it_returns_xiv_for_14() {
    assert_eq!("XIV", to_roman_numeral(14));
  }

  #[test]
  fn it_returns_xv_for_15() {
    assert_eq!("XV", to_roman_numeral(15));
  }

  #[test]
  fn it_returns_xvi_for_16() {
    assert_eq!("XVI", to_roman_numeral(16));
  }

  #[test]
  fn it_returns_xix_for_19() {
    assert_eq!("XIX", to_roman_numeral(19));
  }

  #[test]
  fn it_returns_xx_for_20() {
    assert_eq!("XX", to_roman_numeral(20));
  }

  #[test]
  fn it_returns_xxx_for_30() {
    assert_eq!("XXX", to_roman_numeral(30));
  }

  #[test]
  fn it_returns_xxxix_for_39() {
    assert_eq!("XXXIX", to_roman_numeral(39));
  }

  #[test]
  fn it_returns_xl_for_40() {
    assert_eq!("XL", to_roman_numeral(40));
  }

  #[test]
  fn it_returns_ccxlvi_for_246() {
    assert_eq!("CCXLVI", to_roman_numeral(246));
  }

  #[test]
  fn it_returns_mmcdxxi_for_2421() {
    assert_eq!("MMCDXXI", to_roman_numeral(2421));
  }

}