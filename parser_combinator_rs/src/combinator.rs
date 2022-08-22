use crate::foundation::{Ctx, Result};

pub type ParserFn<T> = Box<dyn for<'a> Fn(&'a Ctx) -> Result<T>>;

pub fn any<T>(parsers: Vec<ParserFn<T>>) -> impl for<'a> Fn(&'a Ctx) -> Result<T> {
  fn _parser<T>(ctx: &Ctx, parsers: &Vec<ParserFn<T>>) -> Result<T> {
    let mut err = ctx.failure("any".to_string());

    for parser in parsers {
      let res = parser(&ctx);
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

// pub fn sequence<T: Clone>(parsers: Vec<ParserFn<T>>) -> impl for<'a> Fn(&'a Ctx) -> Result<Vec<T>> {
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

// pub fn many<T: Clone>(
//   parser: impl for<'a> Fn(&'a Ctx) -> Result<T>,
// ) -> impl for<'a> Fn(&'a Ctx) -> Result<Vec<T>> {
//   move |ctx| {
//     let mut values: Vec<T> = vec![];
//     let mut next_ctx = ctx.to_owned();
//     loop {
//       match parser(&next_ctx) {
//         Err(_) => break,
//         Ok(success) => {
//           next_ctx = success.ctx().to_owned();
//           let val = success.val();
//           values.push(val);
//         }
//       }
//     }
//     Ok(next_ctx.success(values))
//   }
// }

pub fn delimited<T: Clone, L, R>(
  left: impl for<'a> Fn(&'a Ctx) -> Result<L>,
  parser: impl for<'a> Fn(&'a Ctx) -> Result<T>,
  right: impl for<'a> Fn(&'a Ctx) -> Result<R>,
) -> impl for<'a> Fn(&'a Ctx) -> Result<T> {
  move |ctx| {
    let l_res = left(ctx)?;
    let mut next_ctx = l_res.ctx();
    let res = parser(&next_ctx)?;
    next_ctx = res.ctx();
    let r_res = right(&next_ctx)?;
    next_ctx = r_res.ctx();
    Ok(next_ctx.success(res.val()))
  }
}

pub fn separated<T: Clone>(
  separator: impl for<'a> Fn(&'a Ctx) -> Result<String>,
  parser: impl for<'a> Fn(&'a Ctx) -> Result<T>,
) -> impl for<'a> Fn(&'a Ctx) -> Result<Vec<T>> {
  move |ctx| {
    let mut values: Vec<T> = vec![];
    let mut next_ctx = ctx.to_owned();
    let mut is_first = true;

    loop {
      let mut inner_ctx = next_ctx.to_owned();
      if is_first {
        is_first = false;
      } else {
        let sep_res = match separator(&inner_ctx) {
          Err(_) => break,
          Ok(success) => success,
        };
        inner_ctx = sep_res.ctx().to_owned();
      }
      match parser(&inner_ctx) {
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

// a convenience method that will map a Success to callback, to let us do common things like build AST nodes from input strings.
//function map<A, B>(parser: Parser<A>, fn: (val: A) => B): Parser<B> {
//  return ctx => {
//    const res = parser(ctx);
//    return res.success ? success(res.ctx, fn(res.value)) : res;
//  };
//}

//fn map<T: Clone, R>(
//  parser: impl for<'a> Fn(&'a Ctx) -> Result<T>,
//  op: impl Fn(T) -> R,
//) -> impl for<'a> Fn(&'a Ctx) -> Result<R> {
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
