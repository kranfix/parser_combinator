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
