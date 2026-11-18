package library

import (
	"testing"
)

func TestRequestHashDeterministic(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name   string
		method string
		path   string
		body   []byte
	}{
		{name: "create", method: "POST", path: "/api/v1/library", body: []byte(`{"id":"a"}`)},
		{name: "update", method: "PATCH", path: "/api/v1/library/a", body: []byte(`{"title":"x"}`)},
		{name: "delete", method: "DELETE", path: "/api/v1/library/a", body: nil},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			first := RequestHash(tt.method, tt.path, tt.body)
			second := RequestHash(tt.method, tt.path, tt.body)
			if first != second {
				t.Fatalf("hash changed: %q != %q", first, second)
			}
			if len(first) != 64 {
				t.Fatalf("hash length = %d, want 64", len(first))
			}
		})
	}

	if RequestHash("POST", "/a", []byte(`{"id":"a"}`)) == RequestHash("POST", "/a", []byte(`{"id":"b"}`)) {
		t.Fatal("different bodies produced the same hash")
	}
}
