import { expect, test } from "bun:test";
import { lPara, rPara } from "./parser";
import { Context, Result, Success } from "./foundation";
import { delimited, delimitedLeft, map, sequence } from "./combinator";

const foo = (ctx: Context): Result<string> => ctx.parse_str("foo");
const bar = (ctx: Context): Result<string> => ctx.parse_str("bar");

test("sequence", () => {
  const ctx = new Context("foobar");
  const parser = sequence([foo, bar]);
  const res = parser(ctx) as Success<[string, string]>;
  expect(res.success).toBeTruthy();
  expect(res.value).toEqual(["foo", "bar"]);
  expect(res.ctx.index).toBe(6);
});

test("map", () => {
  const ctx = new Context("true");
  const parser = map(
    (ctx: Context) => ctx.parse_str("true"),
    (val) => {
      expect(val).toBe("true");
      return true;
    }
  );
  const res = parser(ctx) as Success<true>;
  expect(res.success).toBeTruthy();
  expect(res.value).toEqual(true);
  expect(res.ctx.index).toBe(4);
});

test("delimited", () => {
  const ctx = new Context("(foo)bar");
  const parser = delimited(lPara, foo, rPara);
  const fooRes = parser(ctx) as Success<String>;
  expect(fooRes.success).toBeTruthy();
  expect(fooRes.value).toEqual("foo");
  expect(fooRes.ctx.index).toBe(5);
});

test("delimitedLeft", () => {
  const ctx = new Context("(foobar");
  const parser = delimitedLeft(lPara, foo);
  const fooRes = parser(ctx) as Success<String>;
  expect(fooRes.success).toBeTruthy();
  expect(fooRes.value).toEqual("foo");
  expect(fooRes.ctx.index).toBe(4);
});
