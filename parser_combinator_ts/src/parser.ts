import { any, many, map, sequence } from "./combinator";
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

const call = map(
  sequence<any>([
    identifier,
    (ctx) => ctx.parse_str("("),
    args,
    (ctx) => ctx.parse_str(")"),
  ]),
  ([fnName, _lparen, argList, _rparen]: [string, "(", Expr[], ")"]): Call => ({
    target: fnName,
    args: argList,
  })
);

export function args(ctx: Context): Result<Expr[]> {
  const first_res = expr(ctx);
  if (!first_res.success) {
    if (ctx.index === first_res.ctx.index) return ctx.success([]);
    return first_res as Failure;
  }

  const rest_res = many(trailingArg)(first_res.ctx);
  if (!rest_res.success) return rest_res as Failure;
  return rest_res.ctx.success([first_res.value, ...rest_res.value]);
}

export function trailingArg(ctx: Context): Result<Expr> {
  const comma_res = ctx.parse_str(",");
  if (!comma_res.success) return comma_res as Failure;
  return expr(comma_res.ctx);
}
