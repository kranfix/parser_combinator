import 'package:parser_combinator_dart/src/foundation.dart';

/// Tries each parser in [parsers] in order; returns the first [Ok].
/// On total failure, returns the [Err] that consumed the most input.
Parser<T> any<T>(List<Parser<T>> parsers) => (ctx) {
      if (parsers.isEmpty) return ctx.failure('any');
      Err<T>? best;
      for (final parser in parsers) {
        final res = parser(ctx);
        if (res is Ok<T>) return res;
        final e = res as Err<T>;
        if (best == null || best.ctx.index < e.ctx.index) best = e;
      }
      return best!;
    };

/// Runs [parser] zero or more times, collecting values into a list.
/// Stops (without error) when the parser fails without consuming input.
Parser<List<T>> many<T>(Parser<T> parser) => (ctx) {
      final values = <T>[];
      var nextCtx = ctx;
      while (true) {
        final res = parser(nextCtx);
        switch (res) {
          case Err<T>(:final ctx):
            if (nextCtx.index == ctx.index) return nextCtx.success(values);
            // consumed input then failed → propagate
            return res.cast<List<T>>();
          case Ok<T>(:final value, :final ctx):
            values.add(value);
            nextCtx = ctx;
        }
      }
    };

/// Runs each parser in [parsers] sequentially; all must succeed.
/// All parsers must share the same output type [T].
/// For heterogeneous sequences, use [pair] or [triple].
Parser<List<T>> sequence<T>(List<Parser<T>> parsers) => (ctx) {
      final values = <T>[];
      var nextCtx = ctx;
      for (final parser in parsers) {
        final res = parser(nextCtx);
        switch (res) {
          case Err<T>():
            return res.cast<List<T>>();
          case Ok<T>(:final value, :final ctx):
            values.add(value);
            nextCtx = ctx;
        }
      }
      return nextCtx.success(values);
    };

/// Runs [pa] then [pb]; returns a Record `(A, B)` on success.
Parser<(A, B)> pair<A, B>(Parser<A> pa, Parser<B> pb) => (ctx) {
      final ra = pa(ctx);
      if (ra is Err<A>) return ra.cast<(A, B)>();
      final rb = pb((ra as Ok<A>).ctx);
      if (rb is Err<B>) return rb.cast<(A, B)>();
      return (rb as Ok<B>).ctx.success((ra.value, rb.value));
    };

/// Runs [pa], [pb], [pc] in order; returns a Record `(A, B, C)` on success.
Parser<(A, B, C)> triple<A, B, C>(
  Parser<A> pa,
  Parser<B> pb,
  Parser<C> pc,
) =>
    (ctx) {
      final ra = pa(ctx);
      if (ra is Err<A>) return ra.cast<(A, B, C)>();
      final rb = pb((ra as Ok<A>).ctx);
      if (rb is Err<B>) return rb.cast<(A, B, C)>();
      final rc = pc((rb as Ok<B>).ctx);
      if (rc is Err<C>) return rc.cast<(A, B, C)>();
      return (rc as Ok<C>).ctx.success((ra.value, rb.value, rc.value));
    };

/// Applies [fn] to the value of [parser] on success.
Parser<B> map<A, B>(Parser<A> parser, B Function(A) fn) => (ctx) {
      final res = parser(ctx);
      return switch (res) {
        Err<A>() => res.cast<B>(),
        Ok<A>(:final value, :final ctx) => ctx.success(fn(value)),
      };
    };

/// Parses [left], [parser], [right] in order; returns only the middle value.
Parser<T> delimited<T, L, R>(
  Parser<L> left,
  Parser<T> parser,
  Parser<R> right,
) =>
    (ctx) {
      final leftRes = left(ctx);
      if (leftRes is Err<L>) return leftRes.cast<T>();
      final res = parser((leftRes as Ok<L>).ctx);
      if (res is Err<T>) return res;
      final rightRes = right((res as Ok<T>).ctx);
      if (rightRes is Err<R>) return rightRes.cast<T>();
      return (rightRes as Ok<R>).ctx.success(res.value);
    };

/// Parses [left] then [parser]; discards the left value.
Parser<T> delimitedLeft<T, L>(Parser<L> left, Parser<T> parser) => (ctx) {
      final leftRes = left(ctx);
      if (leftRes is Err<L>) return leftRes.cast<T>();
      return parser((leftRes as Ok<L>).ctx);
    };

/// Parses zero or more [parser] occurrences separated by [separator].
/// Returns an empty list when the first [parser] attempt immediately fails.
Parser<List<T>> separated<T, S>(Parser<S> separator, Parser<T> parser) =>
    (ctx) {
      final first = parser(ctx);
      if (first is Err<T>) return ctx.success(<T>[]);

      final values = [(first as Ok<T>).value];
      var nextCtx = first.ctx;

      while (true) {
        final sepRes = separator(nextCtx);
        if (sepRes is Err<S>) break;
        final res = parser((sepRes as Ok<S>).ctx);
        if (res is Err<T>) break;
        values.add((res as Ok<T>).value);
        nextCtx = res.ctx;
      }

      return nextCtx.success(values);
    };
