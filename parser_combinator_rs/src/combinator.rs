use crate::foundation::{Ctx, Result};

pub type ParserFn<T> = Box<dyn for<'a> Fn(&'a Ctx) -> Result<T>>;

pub fn any<T>(parsers: Vec<ParserFn<T>>) -> impl for<'a> Fn(&'a Ctx) -> Result<T> {
  fn _parser<T>(ctx: &Ctx, parsers: &Vec<ParserFn<T>>) -> Result<T> {
    let mut err = ctx.failure("There was no match".to_string());

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

// fn optional<T: Clone>(
//   parser: impl for<'a> Fn(&'a Ctx) -> Result<T>,
// ) -> impl for<'a> Fn(&'a Ctx) -> Result<Option<T>> {
//   move |ctx| {
//     let ok = match parser(ctx) {
//       Ok(success) => {
//         let ctx = success.ctx();
//         let val = Some(success.val());
//         ctx.success(val)
//       }
//       Err(failure) => {
//         let ctx = failure.ctx();
//         ctx.success(None)
//       }
//     };
//     Ok(ok)
//   }
// }

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

pub fn many<T: Clone>(
  parser: impl for<'a> Fn(&'a Ctx) -> Result<T>,
) -> impl for<'a> Fn(&'a Ctx) -> Result<Vec<T>> {
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
