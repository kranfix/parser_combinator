import { Failure, Parser } from "./foundation";

export function any<T>(parsers: Parser<T>[]): Parser<T> {
  return (ctx) => {
    if (parsers.length === 0) return ctx.failure("any");
    let failure: Failure | null = null;
    for (const parser of parsers) {
      const res = parser(ctx);
      if (res.success) return res;
      if (!failure || failure.ctx.index < res.ctx.index) {
        failure = res as Failure;
      }
    }
    return failure;
  };
}

export function many<T>(parser: Parser<T>): Parser<T[]> {
  return (ctx) => {
    const values: T[] = [];
    let nextCtx = ctx;
    while (true) {
      const res = parser(nextCtx);
      if (!res.success) {
        if (nextCtx.index == res.ctx.index) break;
        return res as Failure;
      }
      values.push(res.value);
      nextCtx = res.ctx;
    }
    return nextCtx.success(values);
  };
}

export function sequence<T>(parsers: Parser<T>[]): Parser<T[]> {
  return (ctx) => {
    let values: T[] = [];
    let nextCtx = ctx;
    for (const parser of parsers) {
      const res = parser(nextCtx);
      if (!res.success) return res as Failure;
      values.push(res.value);
      nextCtx = res.ctx;
    }
    return nextCtx.success(values);
  };
}

export function map<A, B>(parser: Parser<A>, fn: (val: A) => B): Parser<B> {
  return (ctx) => {
    const res = parser(ctx);
    return !res.success ? (res as Failure) : res.ctx.success(fn(res.value));
  };
}
