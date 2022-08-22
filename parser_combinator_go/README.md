# Parser Combinator in go

A parser combinator written in Golang.

## Supported grammar

bool: `true`, `false`

num(int32): `1`, `+1`, `+12`, `-1`, `-12`, etc

calls: `SomeCall()`, `CallWithArgs(arg1,arg2)`

## Run example

```
go run bin/main.go
```

## Run tests

```
go test .
go test ./context
go test ./combinator
```
