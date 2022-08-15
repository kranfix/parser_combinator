package parser_combinator

import (
	"testing"

	"github.com/kranfix/parser_combinator/parser_combinator_go/context"
)

func TestBoolean(t *testing.T) {
	ctx := context.New("true")
	c, value, err := Boolean(ctx)

	if err != nil || value != true || c.Index() != 4 {
		t.Errorf("Expected 'true' but got '%v'", value)
	}

	ctx = context.New("false")
	c, value, err = Boolean(ctx)

	if err != nil || value != false || c.Index() != 5 {
		t.Errorf("Expected 'false' but got '%v'", value)
	}
}

func TestNumber(t *testing.T) {
	ctx := context.New("1")
	c, value, err := Number(ctx)
	if err != nil || value != 1 || c.Index() != 1 {
		t.Errorf("Expected 'true' but got '%v'", value)
	}

	ctx = context.New("+1")
	c, value, err = Number(ctx)
	if err != nil || value != 1 || c.Index() != 2 {
		t.Errorf("Expected 'true' but got '%v'", value)
	}

	ctx = context.New("-1")
	c, value, err = Number(ctx)
	if err != nil || value != -1 || c.Index() != 2 {
		t.Errorf("Expected 'true' but got '%v'", value)
	}

	ctx = context.New("12")
	c, value, err = Number(ctx)
	if err != nil || value != 12 || c.Index() != 2 {
		t.Errorf("Expected 'true' but got '%v'", value)
	}

	ctx = context.New("-12")
	c, value, err = Number(ctx)
	if err != nil || value != -12 || c.Index() != 3 {
		t.Errorf("Expected 'true' but got '%v'", value)
	}
}
