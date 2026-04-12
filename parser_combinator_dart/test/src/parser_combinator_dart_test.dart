// ignore_for_file: prefer_const_constructors
import 'package:parser_combinator_dart/parser_combinator_dart.dart';
import 'package:test/test.dart';

void main() {
  group('Ctx', () {
    test('parseStr success', () {
      final ctx = Ctx('Hello, world!');
      final res = ctx.parseStr('Hello');
      expect(res, isA<Ok<String>>());
      final ok = res as Ok<String>;
      expect(ok.value, 'Hello');
      expect(ok.ctx.index, 5);
    });

    test('parseStr failure', () {
      final ctx = Ctx('Hello, world!');
      final res = ctx.parseStr('world');
      expect(res, isA<Err<String>>());
      final err = res as Err<String>;
      expect(err.expected, 'world');
      expect(err.ctx.index, 0);
    });

    test('parseRegex success', () {
      final ctx = Ctx('Hello, world!');
      final res = ctx.parseRegex(RegExp('[A-Za-z]+'), 'word');
      expect(res, isA<Ok<String>>());
      final ok = res as Ok<String>;
      expect(ok.value, 'Hello');
      expect(ok.ctx.index, 5);
    });

    test('skip clamps at end', () {
      final ctx = Ctx('abc');
      expect(ctx.skip(100).index, 3);
    });
  });

  group('combinators', () {
    test('any returns first success', () {
      final p = any<String>([
        (ctx) => ctx.parseStr('foo'),
        (ctx) => ctx.parseStr('bar'),
      ]);
      final res = p(Ctx('bar'));
      expect(res, isA<Ok<String>>());
      expect((res as Ok<String>).value, 'bar');
    });

    test('many collects zero or more', () {
      final p = many((ctx) => ctx.parseStr('a'));
      final res = p(Ctx('aab'));
      expect(res, isA<Ok<List<String>>>());
      expect((res as Ok<List<String>>).value, ['a', 'a']);
    });

    test('many propagates error when input is consumed then fails', () {
      // delimited consumes '('
      // then fails to match 'x' → Err at index 1 (not 0).
      // many sees index advanced → propagates instead of stopping cleanly.
      final p = many(
        delimited(
          (ctx) => ctx.parseStr('('),
          (ctx) => ctx.parseStr('x'),
          (ctx) => ctx.parseStr(')'),
        ),
      );
      expect(p(Ctx('(y)')), isA<Err<List<String>>>());
    });

    test('sequence succeeds when all parsers match', () {
      final p = sequence<String>([
        (ctx) => ctx.parseStr('foo'),
        (ctx) => ctx.parseStr('bar'),
      ]);
      final res = p(Ctx('foobar'));
      expect(res, isA<Ok<List<String>>>());
      expect((res as Ok<List<String>>).value, ['foo', 'bar']);
    });

    test('sequence fails on first mismatch', () {
      final p = sequence<String>([
        (ctx) => ctx.parseStr('foo'),
        (ctx) => ctx.parseStr('baz'),
      ]);
      final res = p(Ctx('foobar'));
      expect(res, isA<Err<List<String>>>());
    });

    test('triple returns record of three values', () {
      final p = triple(
        (ctx) => ctx.parseStr('a'),
        (ctx) => ctx.parseStr('b'),
        (ctx) => ctx.parseStr('c'),
      );
      final res = p(Ctx('abc'));
      expect(res, isA<Ok<(String, String, String)>>());
      expect(
        (res as Ok<(String, String, String)>).value,
        ('a', 'b', 'c'),
      );
    });

    test('triple fails when any parser fails', () {
      final p = triple(
        (ctx) => ctx.parseStr('a'),
        (ctx) => ctx.parseStr('b'),
        (ctx) => ctx.parseStr('x'),
      );
      expect(p(Ctx('abc')), isA<Err<(String, String, String)>>());
    });

    test('delimitedLeft discards left and returns right', () {
      final p = delimitedLeft(
        (ctx) => ctx.parseStr('>>'),
        (ctx) => ctx.parseStr('value'),
      );
      final res = p(Ctx('>>value'));
      expect(res, isA<Ok<String>>());
      expect((res as Ok<String>).value, 'value');
    });

    test('delimitedLeft fails when left fails', () {
      final p = delimitedLeft(
        (ctx) => ctx.parseStr('>>'),
        (ctx) => ctx.parseStr('value'),
      );
      expect(p(Ctx('value')), isA<Err<String>>());
    });

    test('map transforms value', () {
      final p = map(
        (ctx) => ctx.parseStr('42'),
        int.parse,
      );
      final res = p(Ctx('42'));
      expect(res, isA<Ok<int>>());
      expect((res as Ok<int>).value, 42);
    });

    test('delimited returns middle value', () {
      final p = delimited(
        (ctx) => ctx.parseStr('('),
        (ctx) => ctx.parseStr('x'),
        (ctx) => ctx.parseStr(')'),
      );
      final res = p(Ctx('(x)'));
      expect(res, isA<Ok<String>>());
      expect((res as Ok<String>).value, 'x');
    });

    test('separated parses comma-separated values', () {
      final p = separated(
        (ctx) => ctx.parseStr(','),
        (ctx) => ctx.parseStr('a'),
      );
      final res = p(Ctx('a,a,a'));
      expect(res, isA<Ok<List<String>>>());
      expect((res as Ok<List<String>>).value, ['a', 'a', 'a']);
    });

    test('pair returns record', () {
      final p = pair(
        (ctx) => ctx.parseStr('foo'),
        (ctx) => ctx.parseStr('bar'),
      );
      final res = p(Ctx('foobar'));
      expect(res, isA<Ok<(String, String)>>());
      expect((res as Ok<(String, String)>).value, ('foo', 'bar'));
    });
  });

  group('example parser', () {
    test('parses boolean true', () {
      final result = parse('true');
      expect(result, isA<BoolExpr>());
      expect((result as BoolExpr).value, true);
    });

    test('parses boolean false', () {
      final result = parse('false');
      expect(result, isA<BoolExpr>());
      expect((result as BoolExpr).value, false);
    });

    test('parses positive number', () {
      final result = parse('42');
      expect(result, isA<NumberExpr>());
      expect((result as NumberExpr).value, 42);
    });

    test('parses call with no args', () {
      final result = parse('foo()');
      expect(result, isA<CallExpr>());
      final call = result as CallExpr;
      expect(call.target, 'foo');
      expect(call.args, isEmpty);
    });

    test('parses call with args', () {
      final result = parse('add(1,2)');
      expect(result, isA<CallExpr>());
      final call = result as CallExpr;
      expect(call.target, 'add');
      expect(call.args, hasLength(2));
      expect((call.args[0] as NumberExpr).value, 1);
      expect((call.args[1] as NumberExpr).value, 2);
    });

    test('parses nested call', () {
      final result = parse('outer(inner(true))');
      expect(result, isA<CallExpr>());
      final outer = result as CallExpr;
      expect(outer.target, 'outer');
      final inner = outer.args.first as CallExpr;
      expect(inner.target, 'inner');
      expect((inner.args.first as BoolExpr).value, true);
    });

    test('throws on invalid input', () {
      expect(() => parse('???'), throwsA(isA<FormatException>()));
    });

    test('BoolExpr.toString', () {
      expect(BoolExpr(true).toString(), 'Boolean(true)');
      expect(BoolExpr(false).toString(), 'Boolean(false)');
    });

    test('NumberExpr.toString', () {
      expect(NumberExpr(42).toString(), 'Number(42)');
    });

    test('CallExpr.toString', () {
      expect(
        CallExpr(target: 'f', args: [NumberExpr(1), BoolExpr(true)]).toString(),
        'f(Number(1), Boolean(true))',
      );
    });
  });
}
