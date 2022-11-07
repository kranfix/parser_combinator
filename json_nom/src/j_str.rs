use nom::{
  character::complete::{alphanumeric0, char},
  sequence::delimited,
  IResult,
};

pub fn raw_str(input: &str) -> IResult<&str, String> {
  let mut parse = delimited(char('"'), alphanumeric0, char('"'));
  let (input, val) = parse(input)?;
  Ok((input, val.to_string()))
}

#[cfg(test)]
mod test {
  use super::raw_str;

  #[test]
  fn test_basic_string() {
    let Ok((input, result)) = raw_str(r#""this1234""#) else {
      unreachable!();
    };
    assert_eq!(input, "");
    assert_eq!(result, "this1234");

    // let Ok((input, result)) = raw_str(r#""this is an string with numbers 1234""#) else {
    //   unreachable!();
    // };
    // assert_eq!(input, "");
    // assert_eq!(result, r#"this is an string with numbers 1234"#);
  }
}
