import { any, map } from "./combinator";
import { Context, Failure, Result } from "./foundation";

// Expresion
type Expr = boolean | number | Call | null;

interface Call {
  readonly target: string;
  readonly args: Expr[];
}

export function parse(text: string): Expr {
  const ctx = new Context(text);
  const res = expr(ctx);
  if (res.success) return res.value;
  throw formatFailure(res as Failure);
}

// Expresion parser
export function expr(ctx: Context): Result<Expr> {
  return any<Expr>([nullLiteral, booleanLiteral, numberLiteral])(ctx);
}

function nullLiteral(ctx: Context): Result<null> {
  let res = ctx.parse_str("null");
  if (res.success) return res.ctx.success(null);
  return ctx.failure("null");
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
