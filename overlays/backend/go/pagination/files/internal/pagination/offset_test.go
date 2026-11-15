package pagination

import (
	"errors"
	"net/url"
	"testing"
)

func testLimits() Limits {
	return Limits{
		DefaultLimit: 20,
		MaxLimit:     100,
		MaxOffset:    10000,
	}
}

func TestParseOffset(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name     string
		values   url.Values
		expected Offset
	}{
		{
			name:     "defaults",
			values:   url.Values{},
			expected: Offset{Page: 1, PageSize: 20},
		},
		{
			name:     "explicit page",
			values:   url.Values{"page": []string{"2"}, "page_size": []string{"10"}},
			expected: Offset{Page: 2, PageSize: 10},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			offset, err := ParseOffset(test.values, testLimits())
			if err != nil {
				t.Fatalf("parse: %v", err)
			}

			if offset != test.expected {
				t.Fatalf("offset = %#v, want %#v", offset, test.expected)
			}
		})
	}
}

func TestParseOffsetValidation(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name   string
		values url.Values
	}{
		{name: "zero page", values: url.Values{"page": []string{"0"}}},
		{name: "non numeric", values: url.Values{"page": []string{"one"}}},
		{name: "page size too high", values: url.Values{"page_size": []string{"101"}}},
		{name: "deep offset", values: url.Values{"page": []string{"1002"}, "page_size": []string{"10"}}},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			_, err := ParseOffset(test.values, testLimits())
			if !errors.Is(err, ErrInvalidInput) {
				t.Fatalf("error = %v, want invalid input", err)
			}
		})
	}
}

func TestParseOffsetLimits(t *testing.T) {
	t.Parallel()

	_, err := ParseOffset(url.Values{}, Limits{DefaultLimit: 20, MaxLimit: 10, MaxOffset: 100})
	if !errors.Is(err, ErrInvalidInput) {
		t.Fatalf("error = %v, want invalid input", err)
	}
}
