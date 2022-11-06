use nom::error::{Error, ErrorKind};
use nom::Err::Failure;

use crate::{commons::ident, j_value::JValue};

pub fn raw_bool_or_null(input: &str) -> nom::IResult<&str, JValue> {
  let (input, identifier) = ident(input)?;
  let output = match identifier.as_str() {
    "true" => JValue::Bool(true),
    "false" => JValue::Bool(false),
    "null" => JValue::Null,
    _ => {
      return Err(Failure(Error::new(
        "Expected null, true or false",
        ErrorKind::Tag,
      )))
    }
  };
  Ok((input, output))
}

#[cfg(test)]
mod test {
  use crate::j_value::JValue;

  use super::raw_bool_or_null;

  #[test]
  fn test_null() {
    let Ok(("", JValue::Null)) = raw_bool_or_null("null") else {
      unreachable!();
    };
    let Ok((",", JValue::Null)) = raw_bool_or_null("null,") else {
      unreachable!();
    };
    let Err(_) = raw_bool_or_null("nulltrue") else {
      unreachable!();
    };
  }

  #[test]
  fn test_false() {
    let Ok(("", JValue::Bool(false))) = raw_bool_or_null("false") else {
      unreachable!();
    };
    let Ok((",", JValue::Bool(false))) = raw_bool_or_null("false,") else {
      unreachable!();
    };
    let Err(_) = raw_bool_or_null("falsenull") else {
      unreachable!();
    };
  }

  #[test]
  fn test_true() {
    let Ok(("", JValue::Bool(true))) = raw_bool_or_null("true") else {
      unreachable!();
    };
    let Ok((",", JValue::Bool(true))) = raw_bool_or_null("true,") else {
      unreachable!();
    };
    let Err(_) = raw_bool_or_null("truenull") else {
      unreachable!();
    };
  }
}
