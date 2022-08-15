package parser_combinator

import (
	"fmt"
	"strconv"

	com "github.com/kranfix/parser_combinator/parser_combinator_go/combinator"
	"github.com/kranfix/parser_combinator/parser_combinator_go/context"
)

type Expression = interface{}

func Parse(text string) (Expression, error) {
	ctx := context.New(text)
	c, value, err := Expr(ctx)
	if err != nil {
		msg := fmt.Errorf("Expected %s at %d", *err, c.Index())
		return 0, msg
	}
	return value, nil
}

type CallData struct {
	target string
	args   []Expression
}

func Expr(ctx context.Context) (context.Context, Expression, *string) {
	parser := com.Any([]com.Parser[Expression]{booleanExpr, numberExpr, callExpr})
	return parser(ctx)
}

func Boolean(ctx context.Context) (context.Context, bool, *string) {
	parser := com.Any([]com.Parser[string]{com.Str("true"), com.Str("false")})
	c, value, err := parser(ctx)
	if err != nil {
		return c, false, err
	}
	return c, len(value) == 4, nil
}

func booleanExpr(ctx context.Context) (context.Context, Expression, *string) {
	c, value, err := Boolean(ctx)
	return c, value, err
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
	return c, value, err
}

func ident(ctx context.Context) (context.Context, string, *string) {
	c, value, err := ctx.ParseRegex("[a-zA-Z_][a-zA-Z0-9_]*", "identifier")
	if err != nil {
		return c, "", err
	}
	return c, value, nil
}

func Call(ctx context.Context) (context.Context, CallData, *string) {
	c, target, err := ident(ctx)
	if err != nil {
		return c, CallData{}, err
	}
	c, args, err := com.Delimited(com.Str("("), args, com.Str(")"))(c)
	if err != nil {
		return c, CallData{}, err
	}
	return c, CallData{target, args}, nil
}

func callExpr(ctx context.Context) (context.Context, Expression, *string) {
	c, value, err := Call(ctx)
	return c, value, err
}

func args(ctx context.Context) (context.Context, []Expression, *string) {
	isFirstComma := true
	comma := func(ctx context.Context) (context.Context, string, *string) {
		if isFirstComma {
			isFirstComma = false
			return ctx, "", nil
		} else {
			return ctx.ParseStr(",")
		}
	}
	trailedArg := com.DelimitedLeft(comma, Expr)
	return com.Many(trailedArg)(ctx)
}
