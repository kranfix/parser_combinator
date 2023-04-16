export type Parser<T> = (ctx: Context) => Result<T>;

export class Context {
  text: string;
  index: number;

  constructor(text: string, index: number = 0) {
    this.text = text;
    this.index = index;
  }

  skip(n: number): Context {
    let newIdx = this.index + n;
    if (newIdx > this.text.length) newIdx = this.text.length;
    return new Context(this.text, newIdx);
  }

  success<T>(value: T): Success<T> {
    return { success: true, value, ctx: this };
  }

  failure(expected: string): Failure {
    return { success: false, expected, ctx: this };
  }

  parse_str<T extends string>(match: T): Result<T> {
    const endIdx = this.index + match.length;

    if (endIdx > this.text.length) {
      return this.failure(match);
    }

    if (this.text.substring(this.index, endIdx) === match) {
      const ctx = this.skip(match.length);
      return ctx.success(match);
    } else {
      return this.failure(match);
    }
  }

  parse_regex(re: RegExp, expected: string): Result<string> {
    re.lastIndex = this.index;
    const res = re.exec(this.text);
    if (res && res.index === this.index) {
      const ctx = this.skip(res[0].length);
      return ctx.success(res[0]);
    } else {
      return this.failure(expected);
    }
  }
}

export type Result<T> = Success<T> | Failure;

export type Success<T> = Readonly<{
  success: true;
  value: T;
  ctx: Context;
}>;

export type Failure = Readonly<{
  success: false;
  expected: string;
  ctx: Context;
}>;

export function formatFailure({ expected, ctx }: Failure) {
  const prev_text = ctx.text.substring(0, ctx.index);
  const index = ctx.index;
  return `Parse error, expected ${expected} at char ${index}: ${prev_text}${expected}`;
}
