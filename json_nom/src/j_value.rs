use std::collections::HashMap;

use crate::{
  commons::whitespace, j_array::raw_array, j_bool_null::raw_bool_or_null, j_num::Dec,
  j_obj::raw_obj, j_str::raw_str,
};
use nom::Parser;

#[derive(Debug, PartialEq, Eq)]
pub enum JValue {
  Str(String),
  Num(Dec),
  Obj(HashMap<String, JValue>),
  Array(Vec<JValue>),
  Bool(bool),
  Null,
}

impl JValue {
  pub fn parse(input: &str) -> nom::IResult<&str, JValue> {
    let (input, ()) = whitespace(input)?;
    let mut parser = parse_jvalue_str
      .or(parse_jvalue_num)
      .or(parse_jvalue_obj)
      .or(parse_jvalue_array)
      .or(raw_bool_or_null);
    let (input, parsed) = parser.parse(input)?;
    let (input, ()) = whitespace(input)?;
    Ok((input, parsed))
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

fn parse_jvalue_obj(input: &str) -> nom::IResult<&str, JValue> {
  let (input, obj) = raw_obj(input)?;
  Ok((input, JValue::Obj(obj)))
}

fn parse_jvalue_array(input: &str) -> nom::IResult<&str, JValue> {
  let (input, array) = raw_array(input)?;
  Ok((input, JValue::Array(array)))
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

  #[test]
  fn parse_complex_obj() {
    let json = r#"
      {
        "a": [
          1,
          "hola",
          {
            "x": true
          }
        ],
        "b": {
          "y": [true, false, null]
        }
      }
    "#;

    let Ok(("", JValue::Obj(obj))) = JValue::parse(json) else {
      unreachable!();
    };

    {
      let Some(JValue::Array(array)) = obj.get(&"a".to_string()) else {
        unreachable!();
      };
      assert_eq!(format!("{:?}", array[0]), "Num(Dec(1))");
      assert_eq!(array[1], JValue::Str("hola".to_string()));

      let JValue::Obj(obj1) = &array[2] else {
        unreachable!()
      };
      let Some(JValue::Bool(true)) = obj1.get(&"x".to_string()) else  {
        unreachable!()
      };
    }

    {
      let Some(JValue::Obj(b)) = obj.get(&"b".to_string()) else {
        unreachable!();
      };
      let Some(JValue::Array(array)) = b.get(&"y".to_string()) else {
        unreachable!();
      };

      use JValue::{Bool, Null};
      assert_eq!(array, &[Bool(true), Bool(false), Null]);
    }
  }
}
