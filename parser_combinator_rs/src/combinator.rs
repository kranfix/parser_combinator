use crate::foundation::{Ctx, Result};
use std::cell::Cell;

pub type ParserFn<T> = Box<dyn Fn(&Ctx) -> Result<T>>;

pub fn any<T>(parsers: Vec<ParserFn<T>>) -> impl Fn(&Ctx) -> Result<T> {
  fn _parser<T>(ctx: &Ctx, parsers: &Vec<ParserFn<T>>) -> Result<T> {
    let mut err = ctx.failure("any".to_string());

    for parser in parsers {
      let res = parser(ctx);
      let failure = match res {
        Ok(success) => return Ok(success),
        Err(failure) => failure,
      };
      if failure.index() > err.index() {
        err = failure;
      }
    }
    Err(err)
  }

  move |ctx| _parser(ctx, &parsers)
}

// pub fn sequence<T: Clone>(parsers: Vec<ParserFn<T>>) -> impl Fn(&Ctx) -> Result<Vec<T>> {
//   fn _parser<T: Clone>(ctx: &Ctx, parsers: &Vec<ParserFn<T>>) -> Result<Vec<T>> {
//     let mut values: Vec<T> = vec![];
//     let mut next_ctx = ctx.to_owned();
//     for parser in parsers {
//       let success = parser(&next_ctx)?;
//       next_ctx = success.ctx().to_owned();
//       let val = success.val();
//       values.push(val);
//     }
//     Ok(next_ctx.success(values))
//   }
//   move |ctx| _parser(ctx, &parsers)
// }

pub fn many<T: Clone>(parser: impl Fn(&Ctx) -> Result<T>) -> impl Fn(&Ctx) -> Result<Vec<T>> {
  move |ctx| {
    let mut values: Vec<T> = vec![];
    let mut next_ctx = ctx.to_owned();
    loop {
      match parser(&next_ctx) {
        Err(_) => break,
        Ok(success) => {
          next_ctx = success.ctx().to_owned();
          let val = success.val();
          values.push(val);
        }
      }
    }
    Ok(next_ctx.success(values))
  }
}

pub fn delimited<T: Clone, L, R>(
  left: impl Fn(&Ctx) -> Result<L>,
  parser: impl Fn(&Ctx) -> Result<T>,
  right: impl Fn(&Ctx) -> Result<R>,
) -> impl Fn(&Ctx) -> Result<T> {
  move |ctx| {
    let l_res = left(ctx)?;
    let mut next_ctx = l_res.ctx();
    let res = parser(next_ctx)?;
    next_ctx = res.ctx();
    let r_res = right(next_ctx)?;
    next_ctx = r_res.ctx();
    Ok(next_ctx.success(res.val()))
  }
}

pub fn delimited_left<T: Clone, L>(
  left: impl Fn(&Ctx) -> Result<L>,
  parser: impl Fn(&Ctx) -> Result<T>,
) -> impl Fn(&Ctx) -> Result<T> {
  move |ctx| {
    let l_res = left(ctx)?;
    let next_ctx = l_res.ctx();
    parser(next_ctx)
  }
}

pub fn separated<T: Clone>(
  separator: impl Fn(&Ctx) -> Result<String>,
  parser: impl Fn(&Ctx) -> Result<T>,
) -> impl Fn(&Ctx) -> Result<Vec<T>> {
  let is_firt = Cell::new(true);
  let skip_first = move |ctx: &Ctx| {
    if is_firt.get() {
      is_firt.set(false);
      Ok(ctx.success("".to_owned()))
    } else {
      separator(ctx)
    }
  };

  many(delimited_left(skip_first, parser))
}

//fn map<T: Clone, R>(
//  parser: impl Fn(&Ctx) -> Result<T>,
//  op: impl Fn(T) -> R,
//) -> impl Fn(&Ctx) -> Result<R> {
//  move |ctx| {
//    let result = parser(ctx);
//    match result {
//      Ok(success) => {
//        let ctx = success.ctx();
//        let val = op(success.val());
//        Ok(ctx.success(val))
//      }
//      Err(failure) => Err(failure),
//    }
//  }
//}

//#[cfg(test)]
//mod test {
//  use crate::foundation::Ctx;
//}
