import { parse } from "../src";

function example(text: string) {
  const res = parse(text);
  if (res.success) {
    console.log(res.value);
  } else {
    console.log(res.expected);
  }
}

example("true");
example("false");
example("1");
example("+1");
example("-1");
example("-1");
example("-12");
example("Foo()");
example("Foo(Bar())");
example("Foo(Bar(1,2,true), false)");
