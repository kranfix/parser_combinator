use std::collections::HashMap;

use nom::{
  character::complete::char, multi::separated_list0, sequence::delimited, IResult, Parser,
};

use crate::{commons::whitespace, j_str::raw_str, j_value::JValue};

pub fn raw_obj(input: &str) -> IResult<&str, HashMap<String, JValue>> {
  let mut parse = delimited(char('{'), separated_list0(char(','), parse_kv), char('}'));

  let (input, pairs) = parse(input)?;
  Ok((input, HashMap::from_iter(pairs)))
}

fn parse_kv(input: &str) -> IResult<&str, (String, JValue)> {
  let (input, ()) = whitespace(input)?;
  let (input, key) = raw_str(input)?;
  let (input, ()) = whitespace(input)?;
  let (input, _) = char(':').parse(input)?;
  let (input, val) = JValue::parse(input)?;
  Ok((input, (key, val)))
}

#[cfg(test)]
mod test {
  use crate::j_value::JValue;

  use super::raw_obj;

  #[test]
  fn test_obj() {
    let Ok(("", obj)) = raw_obj(r#"{"a": "this123", "b": 123, "c": true, "d": false, "e": null}"#) else {
      unreachable!()
    };
    assert_eq!(
      obj.get(&"a".to_string()).unwrap(),
      &JValue::Str("this123".to_string())
    );
    assert_eq!(
      format!("{:?}", obj.get(&"b".to_string()).unwrap()),
      "Num(Dec(123))"
    );
    assert_eq!(obj.get(&"c".to_string()).unwrap(), &JValue::Bool(true));
    assert_eq!(obj.get(&"d".to_string()).unwrap(), &JValue::Bool(false));
    assert_eq!(obj.get(&"e".to_string()).unwrap(), &JValue::Null);
  }
}
