import { parse } from "../src";

function example(text: string) {
  console.log(parse(text));
}

example("null");
example("true");
example("false");
example("1");
example("+1");
example("-1");
example("-1");
example("-12");
example("Foo()");
example("Foo(1)");
example("Foo(1,false)");
example("Foo(1,false,Bar())");
example("Foo(Bar())");
example("Foo(Bar(1,2,true),false)");
