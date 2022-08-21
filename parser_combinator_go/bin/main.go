package main

import (
	"fmt"

	p "github.com/kranfix/parser_combinator/parser_combinator_go"
)

func parse(text string) {
	expr, err := p.Parse(text)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%v\n", expr.Format())
}

func main() {
	parse("true")
	parse("false")
	parse("1")
	parse("+1")
	parse("-1")
	parse("+12")
	parse("-12")
	parse("Foo()")
	parse("Foo(1)")
	parse("Foo(1,true)")
	parse("Foo(1,true,Bar())")
	parse("Foo(1,true,Bar(true))")
}
