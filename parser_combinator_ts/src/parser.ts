import { any, map } from "./combinator";
import { Context, Failure, Result } from "./foundation";

// Expresion
type Expr = boolean | number | Call | null;

interface Call {
  readonly target: string;
  readonly args: Expr[];
}

export function parse(text: string): Expr {
  const _ctx = new Context(text);
  const res = any<Expr>([booleanLiteral, numberLiteral])(_ctx);
  if (res.success) return res.value;
  const { expected, ctx } = res as Failure;
  throw `Parse error, expected ${expected} at char ${ctx.index}`;
}

// Expresion parser
export function expr(text: string): Result<Expr> {
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
