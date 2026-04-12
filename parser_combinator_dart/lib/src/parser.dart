import 'package:parser_combinator_dart/src/combinator.dart';
import 'package:parser_combinator_dart/src/foundation.dart';

/// AST node for the example expression parser.
sealed class Expr {
  const Expr();
}

/// A boolean literal (`true` or `false`).
final class BoolExpr extends Expr {
  /// Creates a [BoolExpr].
  // ignore: avoid_positional_boolean_parameters
  const BoolExpr(this.value);

  /// The boolean value.
  final bool value;

  @override
  String toString() => 'Boolean($value)';
}

/// An integer literal.
final class NumberExpr extends Expr {
  /// Creates a [NumberExpr].
  const NumberExpr(this.value);

  /// The integer value.
  final int value;

  @override
  String toString() => 'Number($value)';
}

/// A function-call expression: `target(arg, ...)`.
final class CallExpr extends Expr {
  /// Creates a [CallExpr].
  const CallExpr({required this.target, required this.args});

  /// The function name.
  final String target;

  /// The argument list.
  final List<Expr> args;

  @override
  String toString() => '$target(${args.join(', ')})';
}

/// Parses [text] into an [Expr], or throws a [FormatException] on failure.
Expr parse(String text) {
  final res = expr(Ctx(text));
  return switch (res) {
    Ok(:final value) => value,
    Err() => throw FormatException(formatFailure(res)),
  };
}

/// Top-level expression parser: call | bool | number.
///
/// `call` is tried first because it starts with an identifier, which would
/// otherwise never be reached.
Result<Expr> expr(Ctx ctx) =>
    any<Expr>([_callExpr, _boolExpr, _numberExpr])(ctx);

// ── individual parsers ───────────────────────────────────────────────────────

final _boolExpr = any<Expr>([
  map((ctx) => ctx.parseStr('true'), (_) => const BoolExpr(true)),
  map((ctx) => ctx.parseStr('false'), (_) => const BoolExpr(false)),
]);

final _numberExpr = map<String, Expr>(
  (ctx) => ctx.parseRegex(RegExp(r'[+\-]?[0-9]+(\.[0-9]*)?'), 'number'),
  (s) => NumberExpr(int.parse(s)),
);

Result<String> _ident(Ctx ctx) =>
    ctx.parseRegex(RegExp('[a-zA-Z_][a-zA-Z0-9_]*'), 'identifier');

// _callExpr must be a function, not a variable, because it calls [expr]
// which calls [_callExpr] — a variable initialiser cannot refer to itself.
Result<Expr> _callExpr(Ctx ctx) {
  final identRes = _ident(ctx);
  if (identRes is Err<String>) return identRes.cast<Expr>();

  final argsRes = delimited(
    (c) => c.parseStr('('),
    separated((c) => c.parseStr(','), expr),
    (c) => c.parseStr(')'),
  )((identRes as Ok<String>).ctx);

  if (argsRes is Err<List<Expr>>) return argsRes.cast<Expr>();
  return (argsRes as Ok<List<Expr>>).ctx.success(
        CallExpr(target: identRes.value, args: argsRes.value),
      );
}
