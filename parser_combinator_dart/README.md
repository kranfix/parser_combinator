# parser_combinator_dart

[![style: very good analysis][very_good_analysis_badge]][very_good_analysis_link]
[![License: MIT][license_badge]][license_link]

A parser combinator library for Dart 3, inspired by the TypeScript and Go implementations in this repo.

Parsers are plain functions `(Ctx) → Result<T>`. Combinators are higher-order functions that compose them. Results are **sealed classes**, so every match is exhaustive at compile time.

## Core types

| Type        | Description                                                                                      |
| ----------- | ------------------------------------------------------------------------------------------------ |
| `Ctx`       | Immutable parse context — holds the input text and current position.                             |
| `Result<T>` | Sealed result: either `Ok<T>` (value + next `Ctx`) or `Err<T>` (expected token + failing `Ctx`). |
| `Parser<T>` | `typedef Parser<T> = Result<T> Function(Ctx ctx)`                                                |

## Combinators

| Combinator      | Signature                                          | Description                                                                    |
| --------------- | -------------------------------------------------- | ------------------------------------------------------------------------------ |
| `any`           | `Parser<T>(List<Parser<T>>)`                       | First match wins; returns the furthest failure on miss.                        |
| `many`          | `Parser<List<T>>(Parser<T>)`                       | Zero or more; stops on a non-consuming failure, propagates on a consuming one. |
| `sequence`      | `Parser<List<T>>(List<Parser<T>>)`                 | All must succeed in order (uniform output type).                               |
| `pair`          | `Parser<(A,B)>(Parser<A>, Parser<B>)`              | Two parsers → Dart 3 record.                                                   |
| `triple`        | `Parser<(A,B,C)>(Parser<A>, Parser<B>, Parser<C>)` | Three parsers → Dart 3 record.                                                 |
| `map`           | `Parser<B>(Parser<A>, B Function(A))`              | Transform a successful value.                                                  |
| `delimited`     | `Parser<T>(Parser<L>, Parser<T>, Parser<R>)`       | Parse left/middle/right; keep only the middle.                                 |
| `delimitedLeft` | `Parser<T>(Parser<L>, Parser<T>)`                  | Parse left then main; discard left.                                            |
| `separated`     | `Parser<List<T>>(Parser<S>, Parser<T>)`            | Zero or more occurrences of `T` separated by `S`.                              |

## Usage

```dart
import 'package:parser_combinator_dart/parser_combinator_dart.dart';

// A parser that matches a comma-separated list of integers.
final digits = (Ctx ctx) =>
    ctx.parseRegex(RegExp(r'[0-9]+'), 'integer');

final intList = separated(
  (ctx) => ctx.parseStr(','),
  map(digits, int.parse),
);

void main() {
  final res = intList(Ctx('1,2,3'));
  switch (res) {
    case Ok(:final value):
      print(value); // [1, 2, 3]
    case Err(:final expected, :final ctx):
      print('Expected $expected at ${ctx.index}');
  }
}
```

The library also ships a small example expression parser (`parser.dart`) that
demonstrates `any`, `separated`, `delimited`, and `map` together to parse
boolean literals, integer literals, and function calls like `add(1, mul(2, 3))`.

## Running tests

```sh
dart test
```

With coverage:

```sh
dart test --coverage=coverage
dart pub global run coverage:format_coverage --lcov --in=coverage --out=coverage/lcov.info --report-on=lib
genhtml coverage/lcov.info -o coverage/html
open coverage/html/index.html
```

[license_badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license_link]: https://opensource.org/licenses/MIT
[very_good_analysis_badge]: https://img.shields.io/badge/style-very_good_analysis-B22C89.svg
[very_good_analysis_link]: https://pub.dev/packages/very_good_analysis
