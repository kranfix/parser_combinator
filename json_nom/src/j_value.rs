//use std::collections::HashMap;

use crate::{j_bool_null::raw_bool_or_null, j_num::Dec, j_str::raw_str};
use nom::Parser;

#[derive(Debug)]
pub enum JValue {
  Str(String),
  Num(Dec),
  //Obj(HashMap<String, JValue>),
  //Array(Vec<JValue>),
  Bool(bool),
  Null,
}

impl JValue {
  pub fn parse(input: &str) -> nom::IResult<&str, JValue> {
    let mut parser = parse_jvalue_str.or(parse_jvalue_num).or(raw_bool_or_null);
    parser.parse(input)
  }
}

fn parse_jvalue_str(input: &str) -> nom::IResult<&str, JValue> {
  let (input, text) = raw_str(input)?;
  Ok((input, JValue::Str(text)))
}

fn parse_jvalue_num(input: &str) -> nom::IResult<&str, JValue> {
  let (input, num) = Dec::parse(input)?;
  Ok((input, JValue::Num(num)))
}

#[cfg(test)]
mod test {
  use super::JValue;

  #[test]
  fn parse_str() {
    let Ok(("", JValue::Str(text))) = JValue::parse(r#""this1234""#) else {
      unreachable!();
    };
    assert_eq!(text, "this1234");
  }

  #[test]
  fn parse_num() {
    let Ok(("", JValue::Num(num))) = JValue::parse("-12000") else {
      unreachable!();
    };
    assert_eq!(format!("{num:?}"), "Dec(-12000)");

    let Ok(("", JValue::Num(num))) = JValue::parse("123.01") else {
      unreachable!();
    };
    assert_eq!(format!("{num:?}"), "Dec(0.12301e3)");
  }

  #[test]
  fn parse_bool_or_null() {
    let Ok(("", JValue::Bool(true))) = JValue::parse("true") else {
      unreachable!();
    };

    let Ok(("", JValue::Bool(false))) = JValue::parse("false") else {
      unreachable!();
    };

    let Ok(("", JValue::Null)) = JValue::parse("null") else {
      unreachable!();
    };

    JValue::parse("trueish").unwrap_err();
    JValue::parse("falseish").unwrap_err();
    JValue::parse("nullish").unwrap_err();
  }
}
