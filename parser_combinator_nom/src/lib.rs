use std::num::ParseIntError;

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::{complete::char, complete::digit1},
  combinator::opt,
  sequence::Tuple,
  Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
  target: String,
  args: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
  Num(i32),
  Bool(bool),
  Call(Call),
}

// our top level parsing function that takes care of creating a `Ctx`, and unboxing the final AST (or throwing)
//pub fn parse(code: String) -> std::result::Result<Expr, String> {
//  let ctx = Ctx::new(&code);
//  let res = expr(&ctx);
//  let success = res.map_err(|f| {
//    format!(
//      "Parse error, expected {} at char {}",
//      f.expected(),
//      f.index()
//    )
//  })?;
//  Ok(success.val())
//}

fn bool_literal(input: &str) -> nom::IResult<&str, bool> {
  let (input, val) = alt((tag("true"), tag("false")))(input)?;
  println!("aaa: {}", input);
  if val == "true" {
    Ok((input, true))
  } else {
    Ok((input, false))
  }
}

fn expr_bool(input: &str) -> nom::IResult<&str, Expr> {
  let (input, val) = bool_literal(input)?;
  Ok((input, Expr::Bool(val)))
}

fn number_literal(input: &str) -> nom::IResult<&str, i32> {
  let mut acc = "".to_string();
  let sign = char('-').or(char('+'));
  println!("xxxxxxxxx");
  //let (input, val) = opt(sign)(input)?;
  //if let Some(s) = val {
  //  acc.push(s);
  //}

  let (input, (sign, first)) = (opt(sign), digit1).parse(input)?;
  if let Some(s) = sign {
    acc.push(s);
  }
  acc.push_str(&first);
  println!("==========={}", acc);
  let val = match acc.parse::<i32>() {
    Ok(v) => v,
    Err(_) => {
      return Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Digit,
      )))
    }
  };
  Ok((input, val))
}

fn expr_number(input: &str) -> nom::IResult<&str, Expr> {
  let (input, val) = number_literal(input)?;
  Ok((input, Expr::Num(val)))
}

#[cfg(test)]
mod test {
  use super::*;
  use nom::error::ErrorKind;

  #[test]
  fn test_bool_literal() {
    let (input, val) = bool_literal("truefalsenull").unwrap();
    assert_eq!(val, true);
    assert_eq!(input, "falsenull");

    let (input, val) = bool_literal(input).unwrap();
    assert_eq!(val, false);
    assert_eq!(input, "null");

    let err = bool_literal(input).unwrap_err();
    match err {
      nom::Err::Error(e) => {
        assert_eq!(e.input, "null");
        assert_eq!(e.code, ErrorKind::Tag);
      }
      _ => panic!("Expected error"),
    }
  }

  #[test]
  fn test_bool_number() {
    let (_, val) = number_literal("1").unwrap();
    assert_eq!(val, 1);

    let (_, val) = number_literal("+1").unwrap();
    assert_eq!(val, 1);

    let (_, val) = number_literal("+12").unwrap();
    assert_eq!(val, 12);

    let (_, val) = number_literal("-1").unwrap();
    assert_eq!(val, -1);

    let (_, val) = number_literal("-12").unwrap();
    assert_eq!(val, -12);

    let (input, val) = number_literal("-12a").unwrap();
    assert_eq!(val, -12);
    assert_eq!(input, "a");
  }
}
