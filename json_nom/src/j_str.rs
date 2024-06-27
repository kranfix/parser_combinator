use nom::character::complete::char;
use nom::error::{Error, ErrorKind, ParseError};
use nom::{AsChar, IResult, Needed, Parser, Slice};


pub fn raw_str<'a>(raw_input: &'a str) -> IResult<&'a str, String, nom::error::Error<&'a str>> {
  let (mut input, _) = char('"')(raw_input)?;

  let mut text = String::new();

  //let mut subtext = alphanumeric1.or(space).or(scape_char);
  let mut subtext = scape_char.map(|c| (c, true))
  .or(readbable_ascii_char.map(|c| (c, false)));

  loop {
    match subtext.parse(input) {
        Ok((rest, (val, is_scaped))) => {
            if val == '"' && !is_scaped {
              return Ok((rest, text))
            } else {
              input = rest;
              text.push(val);
            }
        },
        Err(nom::Err::Error(e)) => return Err(nom::Err::Failure(e)),
        Err(other_err) => return Err(other_err),
    }
  }
}

fn readbable_ascii_char(input: &str) -> IResult<&str, char> {
  match input.chars().next() {
    Some(c) => {
      if is_readbable_ascii_char(c) {
        Ok((input.slice(c.len()..), c))
      } else {
        Err(nom::Err::Failure(Error::from_error_kind(input, ErrorKind::IsA)))
      }
    },
    None => Err(nom::Err::Incomplete(Needed::new(1)))
  }
}

fn is_readbable_ascii_char(c: char) -> bool {
  let a = c as u32;
  a >= 0x20 && a <= 0x7E
}

fn scape_char<'a, E: ParseError<&'a str>>(raw_input: &'a str) -> IResult<&str, char, E> {
  let (input, _) = char::<_, E>('\\')(raw_input)?;

  if input.is_empty() {
    return Err(nom::Err::Incomplete(nom::Needed::new(1)))
  }

  match char::<_, E>('n')(input) {
    Ok((input, _)) => return Ok((input, '\n')),
    Err(_) => {},
  }

  match char::<_, E>('r')(input) {
    Ok((input, _)) => return Ok((input, '\r')),
    Err(_) => {},
  }

  match char::<_, E>('t')(input) {
    Ok((input, _)) => return Ok((input, '\t')),
    Err(_) => {},
  }

  match char::<_, E>('\\')(input) {
    Ok((input, _)) => return Ok((input, '\\')),
    Err(_) => {},
  }

  match char::<_, E>('"')(input) {
    Ok((input, _)) => return Ok((input, '\"')),
    Err(_) => {},
  }

  Err(nom::Err::Failure(E::from_error_kind(raw_input, ErrorKind::Escaped)))
}


#[cfg(test)]
mod test {
  use super::raw_str;

  #[test]
  fn test_basic_string() {
    let (rest, result) = raw_str(r#""""#).expect("Empty string");
    assert_eq!(rest, "");
    assert_eq!(result, "");

    let (rest, result) = raw_str(r#""  ""#).expect("String with whitespaces");
    assert_eq!(rest, "");
    assert_eq!(result, "  ");

    let (rest, result) = raw_str(r#""this 1234""#).expect("string with alphanumerics and a whitespace");
    assert_eq!(rest, "");
    assert_eq!(result, "this 1234");

    let Ok((input, result)) = raw_str(r#""this is an \"string\" with numbers 1234!""#) else {
      unreachable!()
    };
    assert_eq!(input, "");
    assert_eq!(result, r#"this is an "string" with numbers 1234!"#);
  }
}
