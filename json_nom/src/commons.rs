use nom::{
  character::complete::{alpha1, alphanumeric0, char},
  multi::many0,
  IResult, Parser,
};

pub fn whitespace(input: &str) -> IResult<&str, ()> {
  let ws = char(' ').or(char('\n')).or(char('\r')).or(char('\t'));
  let (input, _) = many0(ws)(input)?;
  Ok((input, ()))
}

pub fn ident(input: &str) -> nom::IResult<&str, String> {
  let (input, (first, second)) = alpha1.and(alphanumeric0).parse(input)?;
  let val = format!("{}{}", first, second);
  Ok((input, val))
}

#[cfg(test)]
mod test {
  use super::{ident, whitespace};

  #[test]
  fn whitespace_test() {
    let msg = "whitespace never must fail";
    let (input, _) = whitespace("   \n\r  \n  \r   \t   abc").expect(msg);
    assert_eq!(input, "abc");

    let (input, _) = whitespace(input).expect(msg);
    assert_eq!(input, "abc");
  }

  #[test]
  fn test_ident() {
    let (input, val) = ident("foo").unwrap();
    assert_eq!(val, "foo");
    assert_eq!(input, "");

    let res = ident(input);
    assert!(res.is_err());

    let (input, val) = ident("foo(").unwrap();
    assert_eq!(val, "foo");
    assert_eq!(input, "(");

    let (input, val) = ident("foo2A3dEz(").unwrap();
    assert_eq!(val, "foo2A3dEz");
    assert_eq!(input, "(");
  }
}
