package queue

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/google/uuid"
)

func TestJobEncodeDecodeRoundTrip(t *testing.T) {
	t.Parallel()

	expected := Job{
		ID:        uuid.MustParse("01934f5a-7d0f-7a61-9e0b-7b2f4b4b6a1e"),
		Type:      "test.job",
		Payload:   []byte(`{"value":"ok"}`),
		Attempt:   2,
		CreatedAt: time.Date(2026, 11, 14, 12, 0, 0, 0, time.UTC),
	}

	encoded, err := expected.Encode()
	if err != nil {
		t.Fatalf("encode: %v", err)
	}

	actual, err := Decode(encoded)
	if err != nil {
		t.Fatalf("decode: %v", err)
	}

	if actual.ID != expected.ID || actual.Type != expected.Type || actual.Attempt != expected.Attempt {
		t.Fatalf("job = %+v, want %+v", actual, expected)
	}

	if !actual.CreatedAt.Equal(expected.CreatedAt) {
		t.Fatalf("created at = %s, want %s", actual.CreatedAt, expected.CreatedAt)
	}
}

func TestJobRejectsInvalidEnvelope(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name string
		job  Job
	}{
		{name: "missing id", job: Job{Type: "test.job"}},
		{name: "missing type", job: Job{ID: uuid.New()}},
		{name: "negative attempt", job: Job{ID: uuid.New(), Type: "test.job", Attempt: -1}},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			if err := test.job.Validate(); !errors.Is(err, ErrInvalidJob) {
				t.Fatalf("error = %v, want invalid job", err)
			}
		})
	}
}

func TestRegistry(t *testing.T) {
	t.Parallel()

	registry := NewRegistry()
	handler := HandlerFunc(func(context.Context, Job) error { return nil })

	if err := registry.Register("test.job", handler); err != nil {
		t.Fatalf("register: %v", err)
	}

	if _, ok := registry.Lookup("test.job"); !ok {
		t.Fatal("handler was not registered")
	}

	if err := registry.Register("test.job", handler); !errors.Is(err, ErrHandlerRegistered) {
		t.Fatalf("duplicate error = %v", err)
	}
}
