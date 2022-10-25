use crate::{
  combinator::{any, delimited, separated, ParserFn},
  foundation::{Ctx, Result},
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
pub fn parse(code: String) -> std::result::Result<Expr, String> {
  let ctx = Ctx::new(&code);
  let res = expr(&ctx);
  let success = res.map_err(|f| {
    format!(
      "Parse error, expected {} at char {}",
      f.expected(),
      f.index()
    )
  })?;
  Ok(success.val())
}

// expr = call | number_literal | boolean_literal;
fn expr(ctx: &Ctx) -> Result<Expr> {
  // [call, numberLiteral]
  let parsers: Vec<ParserFn<Expr>> = vec![
    // bool literal
    Box::new(|ctx| {
      let target = bool_literal(ctx)?;
      let val = target.val();
      Ok(target.ctx().success(Expr::Bool(val)))
    }),
    // number literal
    Box::new(|ctx| {
      let target = number_literal(ctx)?;
      let val = target.val();
      Ok(target.ctx().success(Expr::Num(val)))
    }),
    // call
    Box::new(|ctx| {
      let target = call(ctx)?;
      let val = target.val();
      Ok(target.ctx().success(Expr::Call(val)))
    }),
  ];
  any::<Expr>(parsers)(ctx)
}

// our regexp to match identifiers
fn ident(ctx: &Ctx) -> Result<String> {
  let re = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
  ctx.parse_regex(re, "identifier".to_owned())
}

// a regexp parser to match a number string
fn number_literal(ctx: &Ctx) -> Result<i32> {
  let re = regex::Regex::new(r"^[+\-]?[0-9]+(\.[0-9]*)?").unwrap();
  let success = ctx.parse_regex(re, "number".to_owned())?;
  let result = success.val().parse();
  match result {
    Ok(num) => Ok(success.ctx().success(num)),
    Err(_) => Err(success.ctx().failure("number".to_owned())),
  }
}

fn bool_literal(ctx: &Ctx) -> Result<bool> {
  any(vec![
    Box::new(|ctx| {
      let target = ctx.parse_str("true".to_string())?;
      Ok(target.ctx().success(true))
    }),
    Box::new(|ctx| {
      let target = ctx.parse_str("false".to_string())?;
      Ok(target.ctx().success(false))
    }),
  ])(ctx)
}

// args = expr ( trailingArg ) *
fn args(ctx: &Ctx) -> Result<Vec<Expr>> {
  let comma = Box::new(|ctx: &Ctx| ctx.parse_str(",".to_owned()));
  let parser = separated(comma, expr);
  parser(ctx)
}

// call = ident "(" args ")"
fn call(ctx: &Ctx) -> Result<Call> {
  let success = ident(ctx)?;
  let target = success.val();

  let delimited_args = delimited(
    |ctx| ctx.parse_str("(".to_string()),
    args,
    |ctx| ctx.parse_str(")".to_string()),
  );

  let success = delimited_args(success.ctx())?;
  let args = success.val();

  Ok(success.ctx().success(Call { target, args }))
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::foundation::Ctx;

  #[test]
  fn test_bool_literal() {
    let ctx = Ctx::new("truefalsenull");
    let res = bool_literal(&ctx).unwrap();
    assert!(res.val());
    assert_eq!(res.index(), 4);
    assert_eq!(res.ctx().text_slice(), "falsenull");

    let ctx = res.ctx();
    let res = bool_literal(ctx).unwrap();
    assert!(!res.val());
    assert_eq!(res.index(), 9);
    assert_eq!(res.ctx().text_slice(), "null");

    let ctx = res.ctx();
    let res = bool_literal(ctx);
    assert!(res.is_err());
  }

  #[test]
  fn test_ident() {
    let ctx = Ctx::new("foo");
    let res = ident(&ctx).unwrap();
    assert_eq!(res.val(), "foo");
    assert_eq!(res.index(), 3);
    assert_eq!(res.ctx().text_slice(), "");

    let ctx = res.ctx();
    let res = ident(ctx);
    assert!(res.is_err());

    let ctx = Ctx::new("foo(");
    let res = ident(&ctx).unwrap();
    assert_eq!(res.val(), "foo");
    assert_eq!(res.index(), 3);
    assert_eq!(res.ctx().text_slice(), "(");
  }

  #[test]
  fn test_call() {
    let ctx = Ctx::new("foo()");
    let success = call(&ctx).unwrap();
    assert_eq!(success.index(), 5);
    assert_eq!(success.val().target, "foo");
    assert_eq!(success.val().args.len(), 0);

    let ctx = Ctx::new("Foo(Bar(1,2,true),false)");
    let success = call(&ctx).unwrap();
    assert_eq!(success.index(), 24);
    assert_eq!(success.val().target, "Foo");
    assert_eq!(success.val().args.len(), 2);
    assert_eq!(
      success.val().args[0],
      Expr::Call(Call {
        target: "Bar".to_string(),
        args: vec![Expr::Num(1), Expr::Num(2), Expr::Bool(true)]
      })
    );
    assert_eq!(success.val().args[1], Expr::Bool(false));
  }
}
