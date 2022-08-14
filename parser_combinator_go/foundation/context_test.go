package Context

import "testing"

func TestMatchStringSuccess(t *testing.T) {
	c := New("abc")
	c, value, err := c.ParseStr("ab")
	if err != nil {
		t.Errorf("Error: %s", *err)
	} else if value != "ab" {
		t.Errorf("Error: %s", *err)
	} else if c.index != 2 {
		t.Errorf("Error: %s", *err)
	}
}

func TestMatchStringFailure(t *testing.T) {
	c := New("abc")
	c, value, err := c.ParseStr("xy")
	if err == nil {
		t.Errorf("Error: response must be a failure")
	} else if value != "" {
		t.Errorf("Error: when a ParseStr fails, the value must be an empty string")
	} else if c.index != 0 {
		t.Errorf("Error: %s", *err)
	}

	c, value, err = c.ParseStr("ax")
	if err == nil {
		t.Errorf("Error: %s", *err)
	} else if value != "" {
		t.Errorf("Error: %s", *err)
	} else if c.index != 0 {
		t.Errorf("Error: %s", *err)
	}
}
