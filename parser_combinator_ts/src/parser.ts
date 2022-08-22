import {
  any,
  delimited,
  delimitedLeft,
  many,
  map,
  sequence,
} from "./combinator";
import { Context, Failure, Result, formatFailure } from "./foundation";

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
  return any<Expr>([call, nullLiteral, booleanLiteral, numberLiteral])(ctx);
}

function nullLiteral(ctx: Context): Result<null> {
  let res = ctx.parse_str("null");
  if (res.success) return res.ctx.success(null);
  return ctx.failure("null");
}

export const booleanLiteral = any<boolean>([
  map(
    (ctx) => ctx.parse_str("true"),
    (_) => true
  ),
  map(
    (ctx) => ctx.parse_str("false"),
    (_) => false
  ),
]);

export const numberLiteral = map(
  (ctx) => ctx.parse_regex(/[+\-]?[0-9]+(\.[0-9]*)?/g, "number"),
  parseFloat
);

export function identifier(ctx: Context): Result<string> {
  return ctx.parse_regex(/[a-zA-Z][a-zA-Z0-9]*/g, "identifier");
}

const lPara = (ctx: Context): Result<string> => ctx.parse_str("(");
const rPara = (ctx: Context): Result<string> => ctx.parse_str(")");
const call = map(
  sequence<any>([identifier, delimited(lPara, args, rPara)]),
  ([target, args]: [string, Expr[]]): Call => ({ target, args })
);

export function args(ctx: Context): Result<Expr[]> {
  let isFirst = true;
  const skipFirstComma = (ctx) => {
    if (isFirst) isFirst = false;
    return ctx.parse_str(isFirst ? "" : ",");
  };
  const trailingArg = delimitedLeft(skipFirstComma, expr);
  return many(trailingArg)(ctx);
}
