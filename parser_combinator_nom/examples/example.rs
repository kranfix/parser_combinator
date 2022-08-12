use parser_combinator_nom::parse;

fn example(code: String) {
  let res = parse(code.as_str());
  match res {
    Ok(val) => println!("sucess: {:?}", val),
    Err(err) => println!("failure {:?}", err),
  }
}

fn main() {
  example("true".to_string());
  example("false".to_string());
  example("1".to_string());
  example("+1".to_string());
  example("+12".to_string());
  example("-1".to_string());
  example("-12".to_string());
  example("Foo()".to_string());
  example("Foo(Bar())".to_string());
  example("Foo(Bar(1,2,true),false)".to_string());
}
