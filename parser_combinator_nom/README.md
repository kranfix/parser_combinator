# Parser Combinator in Rust

An example of a parser combinator based on [nom](https://crates.io/crates/nom).

## Supported grammar

bool: `true`, `false`

num(i32): `1`, `+1`, `+12`, `-1`, `-12`, etc

calls: `SomeCall()`, `CallWithArgs(arg1,arg2)`

## Run

```sh
cargo run --example example
```

# Tests

```sh
cargo test
```
