#[derive(Debug, Clone, PartialEq)]
pub struct Ctx {
  text: String,
  index: usize,
}

impl Ctx {
  pub fn new(text: &str) -> Ctx {
    Ctx {
      text: text.to_string(),
      index: 0,
    }
  }

  fn next(&self, index: usize) -> Self {
    let mut new_index = index;
    if new_index > self.text.len() {
      new_index = self.text.len();
    }

    Self {
      text: self.text.clone(),
      index: new_index,
    }
  }

  pub fn skip(&self, count: usize) -> Self {
    self.next(self.index + count)
  }

  pub fn success<T>(&self, val: T) -> Success<T> {
    Success {
      ctx: self.clone(),
      val,
    }
  }

  pub fn failure(&self, expected: String) -> Failure {
    Failure {
      ctx: self.clone(),
      expected,
    }
  }

  pub fn text_slice(&self) -> &str {
    &self.text[self.index..]
  }

  pub fn parse_str(&self, r#match: String) -> Result<String> {
    let len = r#match.len();
    let text = self.text_slice();
    if len > text.len() {
      return Err(self.failure(r#match.clone()));
    }

    if text[0..len] == r#match {
      return Ok(self.skip(len).success(r#match.to_string()));
    } else {
      return Err(self.failure(r#match));
    }
  }

  pub fn parse_regex(&self, re: regex::Regex, expected: String) -> Result<String> {
    fn get_firts_match(text: &str, re: regex::Regex) -> Option<regex::Match> {
      let captures = re.captures(text)?;
      let first = captures.get(0)?;
      if first.start() != 0 {
        return None;
      }
      Some(first)
    }

    let text = self.text_slice();
    match get_firts_match(text, re) {
      Some(first) => Ok(self.skip(first.end()).success(first.as_str().to_string())),
      None => Err(self.failure(expected)),
    }
  }
}

pub type Result<T> = std::result::Result<Success<T>, Failure>;

#[derive(Debug, PartialEq, Clone)]
pub struct Success<T> {
  ctx: Ctx,
  val: T,
}

impl<T> Success<T> {
  pub fn ctx(&self) -> &Ctx {
    &self.ctx
  }

  pub fn val(&self) -> T
  where
    T: Clone,
  {
    self.val.to_owned()
  }

  // visible for testing
  #[allow(dead_code)]
  pub fn index(&self) -> usize {
    self.ctx.index
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Failure {
  ctx: Ctx,
  expected: String,
}

impl Failure {
  pub fn expected(&self) -> String {
    self.expected.clone()
  }

  pub fn index(&self) -> usize {
    self.ctx.index
  }
}

#[cfg(test)]
mod ctx_test {
  use super::Ctx;

  #[test]
  fn test_new() {
    let ctx = Ctx::new("Hello, world!");
    assert_eq!(ctx.text, "Hello, world!");
    assert_eq!(ctx.index, 0);
  }

  #[test]
  fn test_next() {
    let ctx = Ctx::new("Hello, world!");
    let ctx = ctx.next(5);
    assert_eq!(ctx.text, "Hello, world!");
    assert_eq!(ctx.index, 5);

    let ctx = ctx.next(80);
    assert_eq!(ctx.text, "Hello, world!");
    assert_eq!(ctx.index, ctx.text.len());
  }

  #[test]
  fn test_skip() {
    let ctx = Ctx::new("Hello, world!");
    let ctx = ctx.skip(5);
    assert_eq!(ctx.text, "Hello, world!");
    assert_eq!(ctx.index, 5);

    let ctx = ctx.skip(2);
    assert_eq!(ctx.text, "Hello, world!");
    assert_eq!(ctx.index, 7);

    let ctx = ctx.skip(30);
    assert_eq!(ctx.text, "Hello, world!");
    assert_eq!(ctx.index, ctx.text.len());
  }

  #[test]
  fn test_parse_str_success() {
    let ctx = Ctx::new("Hello, world!");

    let success = ctx.parse_str("Hello".to_string()).unwrap();

    assert_eq!(success.val(), "Hello");
    assert_eq!(success.index(), 5);
  }

  #[test]
  fn test_parse_str_failure() {
    let ctx = Ctx::new("Hello, world!");

    let failure = ctx.parse_str("world".to_string()).unwrap_err();

    assert_eq!(failure.expected(), "world");
    assert_eq!(failure.index(), 0);
  }

  #[test]
  fn test_parse_regex_success() {
    let ctx = Ctx::new("Hello, world!");

    let re = regex::Regex::new(r"[A-Za-z]+").unwrap();

    let parser = |ctx: &Ctx| ctx.parse_regex(re.clone(), "A word".to_string());

    let success = parser(&ctx).unwrap();
    assert_eq!(success.val(), "Hello");
    assert_eq!(success.index(), 5);

    let ctx = success.ctx().skip(2);
    let result = parser(&ctx);
    match result {
      Ok(success) => {
        assert_eq!(success.val(), "world");
        assert_eq!(success.index(), 12);
      }
      Err(failure) => panic!("{:?}", failure),
    }
  }

  #[test]
  fn test_parse_regex_failure() {
    let ctx = Ctx::new("Hello, world!").skip(5);

    let re = regex::Regex::new(r"^[0-9]+$").unwrap();

    let failure = ctx.parse_regex(re, "A number".to_string()).unwrap_err();

    assert_eq!(failure.expected(), "A number");
    assert_eq!(failure.index(), 5);
  }

  #[test]
  fn test_ctx_compare() {
    let ctx = Ctx::new("Hello, world!");
    let ctx1 = ctx.next(5);
    let ctx2 = ctx.next(5);
    assert_eq!(ctx1, ctx2);
  }

  #[test]
  fn test_success() {
    let ctx = Ctx::new("Hello, world!");
    let ctx = ctx.next(5).success("Hello".to_string());
    assert_eq!(ctx.ctx.text, "Hello, world!");
    assert_eq!(ctx.ctx.index, 5);
    assert_eq!(ctx.val, "Hello");
  }

  #[test]
  fn test_failure() {
    let ctx = Ctx::new("Hello, world!");
    let ctx = ctx.next(5).failure("Hello".to_string());
    assert_eq!(ctx.ctx.text, "Hello, world!");
    assert_eq!(ctx.ctx.index, 5);
    assert_eq!(ctx.expected, "Hello");
  }

  #[test]
  fn test_ctx_success() {
    let ctx = Ctx::new("Hello, world!").next(5);
    let success = ctx.success("Hello".to_string());
    assert_eq!(*success.ctx(), ctx);
    assert_eq!(*success.val(), "Hello".to_string());
  }

  #[test]
  fn test_ctx_failure() {
    let ctx = Ctx::new("Hello, world!").next(5);
    let failure = ctx.failure("expected space".to_string());
    assert_eq!(failure.ctx, ctx);
    assert_eq!(failure.expected(), "expected space".to_string());
  }
}
