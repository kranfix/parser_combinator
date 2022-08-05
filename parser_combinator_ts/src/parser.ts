import { any } from "./combinator";
import { Context, Result } from "./foundation";

type Expr = boolean | number | Call;

interface Call {
  readonly target: string;
  readonly args: Expr[];
}

export function parse(text: string): Result<Expr> {
  const ctx = new Context(text);
  return any<Expr>([])(ctx);
}
