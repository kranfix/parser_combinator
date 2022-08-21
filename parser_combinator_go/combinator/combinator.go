package combinator

import (
	"github.com/kranfix/parser_combinator/parser_combinator_go/context"
)

type Parser[T any] func(ctx context.Context) (context.Context, T, *string)

func Str(match string) Parser[string] {
	return func(ctx context.Context) (context.Context, string, *string) {
		return ctx.ParseStr(match)
	}
}

func Any[T any](parsers []Parser[T]) Parser[T] {
	return func(ctx context.Context) (context.Context, T, *string) {
		if len(parsers) == 0 {
			return context.Failure[T](ctx, "Any: no parsers")
		}

		var failure *string = nil
		lastCtx := ctx
		for _, parser := range parsers {
			ctx, value, err := parser(ctx)

			if err == nil {
				return ctx, value, nil
			}

			if failure != nil || lastCtx.Index() < ctx.Index() {
				lastCtx = ctx
				failure = err
			}
		}
		var value T
		return ctx, value, failure
	}
}

func Many[T any](parser Parser[T]) Parser[[]T] {
	return func(ctx context.Context) (context.Context, []T, *string) {
		values := make([]T, 0, 50)
		lastCtx := ctx
		for {
			ctx, value, err := parser(lastCtx)
			if err != nil {
				return lastCtx, values, nil
			}
			lastCtx = ctx
			values = append(values, value)
		}

	}
}

func DelimitedLeft[T any, L any](left Parser[L], main Parser[T]) Parser[T] {
	return func(ctx context.Context) (context.Context, T, *string) {
		ctx, _, err := left(ctx)
		if err != nil {
			return context.Failure[T](ctx, "DelimitedLeft: "+*err)
		}
		ctx, value, err := main(ctx)
		if err != nil {
			return context.Failure[T](ctx, "DelimitedLeft: "+*err)
		}
		return ctx, value, nil
	}
}

//func DelimitedRight[T any, R any](main Parser[T], right Parser[R]) Parser[T] {
//	return func(ctx context.Context) (context.Context, T, *string) {
//		ctx, value, err := main(ctx)
//		if err != nil {
//			return context.Failure[T](ctx, "DelimitedRight: "+*err)
//		}
//		ctx, _, err = right(ctx)
//		if err != nil {
//			return context.Failure[T](ctx, "DelimitedRight: "+*err)
//		}
//		return ctx, value, nil
//	}
//}

func Delimited[T any, L any, R any](left Parser[L], main Parser[T], right Parser[R]) Parser[T] {
	return func(ctx context.Context) (context.Context, T, *string) {
		ctx, _, err := left(ctx)
		if err != nil {
			return context.Failure[T](ctx, "Delimited: "+*err)
		}
		ctx, value, err := main(ctx)
		if err != nil {
			return context.Failure[T](ctx, "Delimited: "+*err)
		}
		ctx, _, err = right(ctx)
		if err != nil {
			return context.Failure[T](ctx, "Delimited: "+*err)
		}
		return ctx, value, nil
	}
}

func Separated[T, S any](separator Parser[S], main Parser[T]) Parser[[]T] {
	skipSeparator := SkipFirst(separator)
	return Many(DelimitedLeft(skipSeparator, main))
}

func SkipFirst[T any](parser Parser[T]) Parser[T] {
	isFirst := true
	return func(ctx context.Context) (context.Context, T, *string) {
		if isFirst {
			isFirst = false
			var val T
			return ctx, val, nil
		}
		return parser(ctx)
	}
}
