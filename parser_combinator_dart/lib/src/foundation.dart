/// A [Parser] is a function from [Ctx] to [Result].
typedef Parser<T> = Result<T> Function(Ctx ctx);

/// Parsing context — holds the full input text and current position.
class Ctx {
  /// Creates a [Ctx] starting at index 0.
  Ctx(String text)
      : _text = text,
        _index = 0;

  const Ctx._({required String text, required int index})
      : _text = text,
        _index = index;

  final String _text;
  final int _index;

  /// Current byte position in the input.
  int get index => _index;

  /// Full input text (exposed for [formatFailure]).
  String get text => _text;

  /// Returns a new [Ctx] advanced by [n] characters (clamped at end of input).
  Ctx skip(int n) {
    final newIndex = (_index + n).clamp(0, _text.length);
    return Ctx._(text: _text, index: newIndex);
  }

  /// Wraps [value] into an [Ok] at the current position.
  Ok<T> success<T>(T value) => Ok(value: value, ctx: this);

  /// Wraps [expected] into an [Err] at the current position.
  Err<T> failure<T>(String expected) => Err(expected: expected, ctx: this);

  /// Matches [match] literally at the current position.
  Result<String> parseStr(String match) {
    final endIdx = _index + match.length;
    if (endIdx > _text.length) return failure(match);
    if (_text.substring(_index, endIdx) == match) {
      return skip(match.length).success(match);
    }
    return failure(match);
  }

  /// Matches [re] anchored at the current position via [RegExp.matchAsPrefix].
  Result<String> parseRegex(RegExp re, String expected) {
    final m = re.matchAsPrefix(_text, _index);
    if (m == null) return failure(expected);
    return skip(m.end - _index).success(m.group(0)!);
  }
}

/// The result of a [Parser] — either [Ok] or [Err].
sealed class Result<T> {
  const Result();
}

/// A successful parse carrying [value] and the [ctx] after consuming input.
final class Ok<T> extends Result<T> {
  /// Creates an [Ok] with the parsed [value] and the resulting [ctx].
  const Ok({required this.value, required this.ctx});

  /// The parsed value.
  final T value;

  /// Context positioned right after the matched text.
  final Ctx ctx;
}

/// A failed parse carrying what was [expected] and [ctx] at the failure site.
final class Err<T> extends Result<T> {
  /// Creates an [Err] with the [expected] token and the failing [ctx].
  const Err({required this.expected, required this.ctx});

  /// What the parser expected to find.
  final String expected;

  /// Context at the point of failure (not advanced).
  final Ctx ctx;

  /// Re-types this error for a different output type.
  /// The phantom [T] changes; the data is unchanged.
  Err<U> cast<U>() => Err(expected: expected, ctx: ctx);
}

/// Returns a human-readable description of [err].
String formatFailure(Err<Object?> err) {
  final prev = err.ctx.text.substring(0, err.ctx.index);
  return 'Parse error, expected ${err.expected} '
      'at char ${err.ctx.index}: $prev${err.expected}';
}
