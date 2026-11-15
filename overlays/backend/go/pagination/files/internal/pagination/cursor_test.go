package pagination

import (
	"errors"
	"net/url"
	"testing"
)

func TestCursorCodecRoundTrip(t *testing.T) {
	t.Parallel()

	codec, err := NewCodec(paginationFixtureSecret)
	if err != nil {
		t.Fatalf("new codec: %v", err)
	}

	encoded, err := codec.Encode(map[string]string{
		"created_at": "2026-11-15T00:00:00Z",
		"id":         "doc-1",
	})
	if err != nil {
		t.Fatalf("encode: %v", err)
	}

	decoded, err := codec.Decode(encoded)
	if err != nil {
		t.Fatalf("decode: %v", err)
	}

	if decoded["id"] != "doc-1" || decoded["created_at"] != "2026-11-15T00:00:00Z" {
		t.Fatalf("decoded = %#v", decoded)
	}
}

func TestCursorCodecRejectsTampering(t *testing.T) {
	t.Parallel()

	codec, err := NewCodec(paginationFixtureSecret)
	if err != nil {
		t.Fatalf("new codec: %v", err)
	}

	encoded, err := codec.Encode(map[string]string{"id": "doc-1"})
	if err != nil {
		t.Fatalf("encode: %v", err)
	}

	if _, err := codec.Decode(encoded + "x"); !errors.Is(err, ErrInvalidInput) {
		t.Fatalf("tamper error = %v, want invalid input", err)
	}

	if _, err := codec.Decode("not-a-cursor"); !errors.Is(err, ErrInvalidInput) {
		t.Fatalf("format error = %v, want invalid input", err)
	}
}

func TestNewCodecRejectsShortSecret(t *testing.T) {
	t.Parallel()

	if _, err := NewCodec("short"); !errors.Is(err, ErrInvalidInput) {
		t.Fatalf("error = %v, want invalid input", err)
	}
}

func TestParseCursor(t *testing.T) {
	t.Parallel()

	cursor, err := ParseCursor(url.Values{
		"limit":  []string{"10"},
		"cursor": []string{"opaque"},
	}, testLimits())
	if err != nil {
		t.Fatalf("parse: %v", err)
	}

	if cursor.Limit != 10 || cursor.Raw != "opaque" {
		t.Fatalf("cursor = %#v", cursor)
	}
}

func TestParseCursorValidation(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name   string
		values url.Values
	}{
		{name: "zero limit", values: url.Values{"limit": []string{"0"}}},
		{name: "limit too high", values: url.Values{"limit": []string{"101"}}},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			_, err := ParseCursor(test.values, testLimits())
			if !errors.Is(err, ErrInvalidInput) {
				t.Fatalf("error = %v, want invalid input", err)
			}
		})
	}
}
