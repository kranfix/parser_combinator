import { any, map } from "./combinator";
import { Context, Result } from "./foundation";

type Expr = boolean | number | Call;

interface Call {
  readonly target: string;
  readonly args: Expr[];
}

export function parse(text: string): Result<Expr> {
  const ctx = new Context(text);
  return any<Expr>([booleanLiteral, numberLiteral])(ctx);
}

export function booleanLiteral(ctx: Context): Result<boolean> {
  return any<boolean>([
    map(
      (ctx) => ctx.parse_str("true"),
      (_) => true
    ),
    map(
      (ctx) => ctx.parse_str("false"),
      (_) => false
    ),
  ])(ctx);
}

export function numberLiteral(ctx: Context): Result<number> {
  return map(
    (ctx) => ctx.parse_regex(/[+\-]?[0-9]+(\.[0-9]*)?/g, "number"),
    parseFloat
  )(ctx);
}
