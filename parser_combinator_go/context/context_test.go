package context

import (
	"fmt"
	"strings"
	"testing"
)

func TestMatchString(t *testing.T) {
	c := New("abc")
	c, value, err := c.ParseStr("xy")
	msg := CheckFailure(c, value, err, 0)
	if msg != nil {
		t.Errorf(*msg)
		return
	}

	c = New("abcxyz")
	c, value, err = c.ParseStr("abc")
	msg = CheckSuccess(c, value, err, "abc", 3)
	if msg != nil {
		t.Errorf(*msg)
		return
	}

	c, value, err = c.ParseStr("mnp")
	msg = CheckFailure(c, value, err, 3)
	if msg != nil {
		t.Errorf(*msg)
		return
	}

	c, value, err = c.ParseStr("xyz")
	msg = CheckSuccess(c, value, err, "xyz", 6)
	if msg != nil {
		t.Errorf(*msg)
		return
	}
}

func TestMatchRegex(t *testing.T) {
	re := "[A-Za-z]+"
	c := New("5 Hello, world!")
	c, value, err := c.ParseRegex(re, "word")
	msg := CheckFailure(c, value, err, 0)
	if msg != nil {
		t.Errorf(*msg)
		return
	}

	c = c.skip(2)
	c, value, err = c.ParseRegex(re, "word")
	msg = CheckSuccess(c, value, err, "Hello", 7)
	if msg != nil {
		t.Errorf(*msg)
		return
	}
}

func CheckSuccess(ctx Context, value string, err *string, match string, index int) *string {
	if err != nil {
		msg := fmt.Sprintf("Error: %s\n", *err)
		return &msg
	} else if strings.Compare(value, match) != 0 {
		msg := fmt.Sprintf("Error: found value \"%s\" != \"%s\"", value, match)
		return &msg
	} else if ctx.index != index {
		msg := fmt.Sprintf("Error: index %d != %d", ctx.index, index)
		return &msg
	}
	return nil
}

func CheckFailure(ctx Context, value string, err *string, index int) *string {
	if err == nil {
		msg := fmt.Sprintf("Error: response must be a failure")
		return &msg
	} else if len(value) != 0 {
		msg := fmt.Sprintf("Error: when a ParseStr fails, the value must be an empty string: %s", value)
		return &msg
	} else if ctx.index != index {
		msg := fmt.Sprintf("Error: index %d != %d", ctx.index, index)
		return &msg
	}
	return nil
}
