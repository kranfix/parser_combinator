use nom::character::complete::{char, digit1};
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::{error, Parser};

use crate::j_value::JValue;

pub fn raw_num(input: &str) -> nom::IResult<&str, JValue> {
  let (input, num) = Dec::parse(input)?;
  Ok((input, JValue::Num(num)))
}

pub struct Dec {
  negative: bool,
  data: Vec<u8>,
  exponent: i32,
}

impl Dec {
  fn parse(input: &str) -> nom::IResult<&str, Dec, error::Error<&str>> {
    let (input, sign) = opt(char('-')).parse(input)?;
    let negative = sign.is_some();

    let (input, integer): (&str, Vec<u8>) = char('0')
      .map(|_| vec![0])
      .or(digit1.map(|d: &str| Vec::from(d.as_bytes())))
      .parse(input)?;

    let mut exponent = integer.len() as i32;
    let mut data: Vec<u8> = integer.into_iter().map(|c| c - b'0').collect();

    let (input, mantissa) = opt(preceded(char('.'), digit1)).parse(input)?;

    if let Some(mantissa) = mantissa {
      let mut iter = mantissa.as_bytes().iter();
      let last = iter.rfind(|d| **d != b'0');

      for d in iter {
        data.push(d - b'0');
      }
      if let Some(d) = last {
        data.push(d - b'0');
      }
    }

    while let Some(0) = data.last() {
      data.pop();
    }

    let (input, _exponent) = parse_exp(input)?;
    exponent += _exponent;

    #[rustfmt::skip]
    let num = Dec { negative, data, exponent };

    Ok((input, num))
  }

  pub fn is_int(&self) -> bool {
    self.data.len() as i32 <= self.exponent
  }
}

impl std::fmt::Display for Dec {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.negative {
      write!(f, "-")?;
    }

    if self.is_int() {
      for b in self.data.iter() {
        write!(f, "{b}")?;
      }
      let complement = (self.exponent as usize) - self.data.len();
      for _ in 0..complement {
        write!(f, "0")?;
      }
      return Ok(());
    }

    if self.data.is_empty() {
      write!(f, "0")?;
    } else {
      write!(f, "0.")?;
      for b in self.data.iter() {
        write!(f, "{b}")?;
      }
    }

    if self.exponent != 0 {
      write!(f, "e{}", self.exponent)?;
    }
    Ok(())
  }
}

impl std::fmt::Debug for Dec {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Dec({self})")
  }
}

fn parse_exp(input: &str) -> nom::IResult<&str, i32> {
  let (input, Some(_)) = opt(char('e').or(char('E'))).parse(input)? else {
    return Ok((input, 0));
  };

  let (input, sign) = opt(char('-').or(char('+'))).parse(input)?;
  let negative = sign.map_or(false, |v| v == '-');

  let (input, val) = digit1(input)?;
  let mut val: i32 = val.parse().expect("Num.exponent must have valid digits");
  if negative {
    val = -val;
  }
  Ok((input, val))
}

#[cfg(test)]
mod test {
  use super::Dec;

  #[test]
  fn test_parse_num() {
    let Ok(("", num)) = Dec::parse("-12.0e-5") else {
      unreachable!();
    };
    assert_eq!(num.to_string(), "-0.12e-3");
    assert_eq!(format!("{num:?}"), "Dec(-0.12e-3)");
    assert_eq!(num.data.len(), 2);

    let Ok((",", num)) = Dec::parse("-12.0e+3,") else {
      unreachable!();
    };
    assert_eq!(num.to_string(), "-12000");
    assert_eq!(format!("{num:?}"), "Dec(-12000)");
    assert_eq!(num.data.len(), 2);

    let Ok(("", num)) = Dec::parse("12.3e+1") else {
      unreachable!();
    };
    assert_eq!(num.to_string(), "123");
    assert_eq!(num.data.len(), 3);
  }
}
