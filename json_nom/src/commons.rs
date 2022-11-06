use nom::{character::complete::char, multi::many0, IResult, Parser};

pub fn whitespace(input: &str) -> IResult<&str, ()> {
  let ws = char(' ').or(char('\n')).or(char('\r')).or(char('\t'));
  let (input, _) = many0(ws)(input)?;
  Ok((input, ()))
}

#[cfg(test)]
mod test {
  use super::whitespace;

  #[test]
  fn whitespace_test() {
    let msg = "whitespace never must fail";
    let (input, _) = whitespace("   \n\r  \n  \r   \t   abc").expect(msg);
    assert_eq!(input, "abc");

    let (input, _) = whitespace(input).expect(msg);
    assert_eq!(input, "abc");
  }
}
