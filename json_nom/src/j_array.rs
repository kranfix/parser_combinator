use nom::{character::complete::char, multi::separated_list0, sequence::delimited, IResult};

use crate::j_value::JValue;

pub fn raw_array(input: &str) -> IResult<&str, Vec<JValue>> {
  let mut parse = delimited(
    char('['),
    separated_list0(char(','), JValue::parse),
    char(']'),
  );

  let (input, val) = parse(input)?;
  Ok((input, val))
}

#[cfg(test)]
mod test {
  use crate::j_value::JValue;

  use super::raw_array;

  #[test]
  fn test_array() {
    let Ok(("", vec)) = raw_array(r#"["this123", 123, true, false, null]"#) else {
      unreachable!()
    };
    assert_eq!(vec[0], JValue::Str("this123".to_string()));
    assert_eq!(format!("{:?}", vec[1]), "Num(Dec(123))");
    assert_eq!(vec[2], JValue::Bool(true));
    assert_eq!(vec[3], JValue::Bool(false));
    assert_eq!(vec[4], JValue::Null);
  }
}
