import { expect, test } from "bun:test";
import { Expr, args } from "./parser";
import { Context, Success } from "./foundation";

test("No args", () => {
  const ctx = new Context("");
  const res = args(ctx) as Success<Expr[]>;
  expect(res.success).toBeTruthy();
  expect(res.value).toEqual([]);
});

test("Only one arg", () => {
  const ctx = new Context("4");
  const res = args(ctx) as Success<Expr[]>;
  expect(res.success).toBeTruthy();
  expect(res.value).toEqual([4]);
});

test("Many args", () => {
  const ctx = new Context("4,true");
  const res = args(ctx) as Success<Expr[]>;
  expect(res.success).toBeTruthy();
  expect(res.value).toEqual([4, true]);
});
