use nom::{
  branch::alt,
  bytes::complete::tag,
  character::{
    complete::char,
    complete::{alpha1, alphanumeric0, digit1},
  },
  combinator::{map, opt},
  multi::many0,
  sequence::{delimited, preceded, Tuple},
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

pub fn parse(input: &str) -> std::result::Result<Expr, String> {
  let res = expr(input);
  let (_, val) = res.map_err(|f| match f {
    nom::Err::Error(e) => format!("{}", e),
    nom::Err::Failure(e) => format!("{}", e),
    nom::Err::Incomplete(e) => match e {
      nom::Needed::Unknown => "Incomplete".to_string(),
      nom::Needed::Size(s) => format!("Incomplete: {}", s),
    },
  })?;
  Ok(val)
}

pub fn expr(input: &str) -> nom::IResult<&str, Expr> {
  let (input, val) = expr_bool.or(expr_number).or(expr_call).parse(input)?;
  Ok((input, val))
}

fn bool_literal(input: &str) -> nom::IResult<&str, bool> {
  let (input, val) = alt((tag("true"), tag("false")))(input)?;
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

  let (input, (sign, first)) = (opt(sign), digit1).parse(input)?;
  if let Some(s) = sign {
    acc.push(s);
  }
  acc.push_str(first);
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

// our regexp to match identifiers
fn ident(input: &str) -> nom::IResult<&str, String> {
  //let re = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
  //ctx.parse_regex(re, "identifier".to_owned())
  let (input, (first, second)) = alpha1.and(alphanumeric0).parse(input)?;
  let val = format!("{}{}", first, second);
  Ok((input, val))
}

fn call(input: &str) -> nom::IResult<&str, Call> {
  let trailing_arg = preceded(char(','), expr);
  let non_empty_arg_list = map(expr.and(many0(trailing_arg)), |(head, tail)| {
    let mut vec = vec![head];
    vec.extend_from_slice(&tail);
    vec
  });
  let arg_list = map(opt(non_empty_arg_list), |v| match v {
    Some(v) => v,
    None => vec![],
  });

  let (input, (target, args)) = (ident, delimited(char('('), arg_list, char(')'))).parse(input)?;
  Ok((input, Call { target, args }))
}

fn expr_call(input: &str) -> nom::IResult<&str, Expr> {
  let (input, val) = call(input)?;
  Ok((input, Expr::Call(val)))
}

#[cfg(test)]
mod test {
  use super::*;
  use nom::error::ErrorKind;

  #[test]
  fn test_bool_literal() {
    let (input, val) = bool_literal("truefalsenull").unwrap();
    assert!(val);
    assert_eq!(input, "falsenull");

    let (input, val) = bool_literal(input).unwrap();
    assert!(!val);
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

  #[test]
  fn test_call() {
    let (input, val) = call("foo()").unwrap();
    assert_eq!(input, "");
    assert_eq!(val.target, "foo");
    assert_eq!(val.args.len(), 0);

    let (input, val) = call("Foo(Bar(1,2,true),false)").unwrap();
    assert_eq!(input, "");
    assert_eq!(val.target, "Foo");
    assert_eq!(val.args.len(), 2);
    assert_eq!(
      val.args[0],
      Expr::Call(Call {
        target: "Bar".to_string(),
        args: vec![Expr::Num(1), Expr::Num(2), Expr::Bool(true)]
      })
    );
    assert_eq!(val.args[1], Expr::Bool(false));
  }
}
