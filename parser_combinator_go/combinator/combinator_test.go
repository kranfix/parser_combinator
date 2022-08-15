package combinator

import (
	"testing"

	"github.com/kranfix/parser_combinator/parser_combinator_go/context"
)

func TestAny(t *testing.T) {
	parser := Any([]Parser[string]{Str("a"), Str("b"), Str("c")})

	c := context.New("bxy")
	c, value, err := parser(c)

	if err != nil || value != "b" || c.Index() != 1 {
		t.Errorf("Expected 'b' but got '%v'", value)
	}

	c = context.New("xyz")
	c, value, err = parser(c)

	if err != nil || len(value) != 0 || c.Index() != 0 {
		t.Errorf("Expected 'b' but got '%v'", value)
	}
}

func TestMany(t *testing.T) {
	parser := Many(Str("a"))

	c := context.New("xaxaaa")

	c, values, err := parser(c)
	if err != nil || len(values) != 0 || c.Index() != 0 {
		t.Errorf("Expected 'a' but got '%v'", values)
	}

	c, _, _ = c.ParseStr("x")

	c, values, err = parser(c)
	if err != nil || len(values) != 1 || c.Index() != 2 {
		t.Errorf("Expected 'a' but got '%v'", values)
	}

	c, _, _ = c.ParseStr("x")

	c, values, err = parser(c)
	if err != nil || len(values) != 3 || c.Index() != 6 {
		t.Errorf("Expected 'a' but got '%v'", values)
	}
	t.Logf("values: %v", values)
}

func TestDelimitedLeft(t *testing.T) {
	parser := DelimitedLeft(Str(","), Str("abc"))

	c := context.New(",abc")
	c, value, err := parser(c)

	if err != nil || value != "abc" || c.Index() != 4 {
		t.Errorf("Expected 'abc' but got '%v'", value)
	}
}

func TestDelimited(t *testing.T) {
	parser := Delimited(Str("("), Str("abc"), Str(")"))

	c := context.New("(abc)")
	c, value, err := parser(c)

	if err != nil || value != "abc" || c.Index() != 5 {
		t.Errorf("Expected 'abc' but got '%v'", value)
	}
}

func TestSeparated(t *testing.T) {
	parser1 := Separated(Str(","), Str("abc"))
	c := context.New("abc,abc,abc,xyz")
	c, value, err := parser1(c)
	if err != nil || len(value) != 3 || c.Index() != 11 {
		t.Errorf("Expected 'abc' but got '%v'", value)
	}

	parser2 := Separated(Str(","), Str("abc"))
	c = context.New("_abc,abc,xyz")
	c, value, err = parser2(c)
	if err != nil || c.Index() != 0 {
		t.Errorf("Expected '%s' at index %d", *err, c.Index())
	}

	parser3 := Separated(Str(","), Str("abc"))
	c = context.New(",abc,abc,xyz")
	c, value, err = parser3(c)
	if err != nil || c.Index() != 0 {
		t.Errorf("Expected 'abc' but got '%v'", value)
	}
}
