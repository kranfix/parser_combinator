use parser_combinator::parser::parse;

fn example(code: impl AsRef<str>) {
  let res = parse(code.as_ref());
  match res {
    Ok(val) => println!("sucess: {:?}", val),
    Err(err) => println!("failure {:?}", err),
  }
}

fn main() {
  example("true");
  example("false");
  example("1");
  example("+1");
  example("+12");
  example("-1");
  example("-12");
  example("Foo()");
  example("Foo(Bar())");
  example("Foo(Bar(1,2,true),false)");
}
