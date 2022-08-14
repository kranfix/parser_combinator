package context

import (
	"regexp"
	"strings"
)

type Index = int

type Context struct {
	text  string
	index Index
}

func New(text string) Context {
	return Context{text: text, index: 0}
}

func (c Context) Index() int {
	return c.index
}

func (c Context) skip(n Index) Context {
	newIndex := c.index + n
	if newIndex > len(c.text) {
		newIndex = len(c.text)
	}
	return Context{text: c.text, index: newIndex}
}

func (c Context) ParseStr(match string) (Context, string, *string) {
	length := len(match)
	endIdx := c.index + length
	if endIdx > len(c.text) {
		return Failure[string](c, match)
	}

	text := c.text[c.index:endIdx]
	if strings.Compare(text, match) == 0 {
		return Success(c.skip(length), match)
	} else {
		return Failure[string](c, match)
	}
}

func (c Context) ParseRegex(re, expected string) (Context, string, *string) {
	r, err := regexp.Compile(re)
	if err != nil {
		msg := "Error in regexp.Compile: " + err.Error()
		return Failure[string](c, msg)
	}
	text := c.text[c.index:]
	bytes := []byte(text)
	loc := r.FindIndex(bytes)
	if loc[0] != 0 {
		return Failure[string](c, expected)
	}
	foundText := string(bytes[loc[0]:loc[1]])
	ctx := c.skip(len(foundText))
	return Success(ctx, foundText)
}

func Success[T any](ctx Context, value T) (Context, T, *string) {
	return ctx, value, nil
}

func Failure[T any](ctx Context, expected string) (Context, T, *string) {
	var value T
	return ctx, value, &expected
}
