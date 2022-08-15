package parser_combinator

import (
	"fmt"
	"strconv"

	com "github.com/kranfix/parser_combinator/parser_combinator_go/combinator"
	"github.com/kranfix/parser_combinator/parser_combinator_go/context"
)

type Expression interface {
	Format() string
}

type NumberT int

func (n NumberT) Format() string {
	return fmt.Sprintf("Number(%v)", n)
}

type BooleanT bool

func (b BooleanT) Format() string {
	return fmt.Sprintf("Boolean(%v)", b)
}

type CallData struct {
	target string
	args   []Expression
}

func (c CallData) Format() string {
	return fmt.Sprintf("%s(%v)", c.target, c.args)
}

func Parse(text string) (Expression, error) {
	ctx := context.New(text)
	c, value, err := Expr(ctx)
	if err != nil {
		msg := fmt.Errorf("Expected %s at %d", *err, c.Index())
		return NumberT(0), msg
	}
	return value, nil
}

func Expr(ctx context.Context) (context.Context, Expression, *string) {
	parser := com.Any([]com.Parser[Expression]{booleanExpr, numberExpr, callExpr})
	return parser(ctx)
}

func Boolean(ctx context.Context) (context.Context, bool, *string) {
	c, _, err := ctx.ParseStr("true")
	if err == nil {
		return c, true, nil
	}

	c, _, err = ctx.ParseStr("false")
	if err == nil {
		return c, false, nil
	}

	return c, false, err
}

func booleanExpr(ctx context.Context) (context.Context, Expression, *string) {
	c, value, err := Boolean(ctx)
	return c, BooleanT(value), err
}

func Number(ctx context.Context) (context.Context, int, *string) {
	c, value, err := ctx.ParseRegex("[+\\-]?[0-9]+(\\.[0-9]*)?", "number")
	if err != nil {
		return c, 0, err
	}
	intVar, err1 := strconv.Atoi(value)
	if err1 != nil {
		return c, 0, err
	}
	return c, intVar, nil
}

func numberExpr(ctx context.Context) (context.Context, Expression, *string) {
	c, value, err := Number(ctx)
	return c, NumberT(value), err
}

func ident(ctx context.Context) (context.Context, string, *string) {
	return ctx.ParseRegex("[a-zA-Z_][a-zA-Z0-9_]*", "identifier")
}

func Call(ctx context.Context) (context.Context, CallData, *string) {
	c, target, err := ident(ctx)
	if err != nil {
		return c, CallData{}, err
	}

	argsParser := com.Separated(com.Str(","), Expr)
	c, args, err := com.Delimited(com.Str("("), argsParser, com.Str(")"))(c)
	if err != nil {
		return c, CallData{}, err
	}
	return c, CallData{target, args}, nil
}

func callExpr(ctx context.Context) (context.Context, Expression, *string) {
	c, value, err := Call(ctx)
	return c, value, err
}
