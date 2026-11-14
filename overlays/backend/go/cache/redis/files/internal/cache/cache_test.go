package cache

import (
	"context"
	"errors"
	"testing"
	"time"
)

type memoryStore struct {
	values map[string][]byte
}

func newMemoryStore() *memoryStore {
	return &memoryStore{values: map[string][]byte{}}
}

func (s *memoryStore) Get(_ context.Context, key string) ([]byte, bool, error) {
	value, ok := s.values[key]

	return value, ok, nil
}

func (s *memoryStore) Set(_ context.Context, key string, value []byte, ttl time.Duration) error {
	if err := ValidateKey(key); err != nil {
		return err
	}

	if err := ValidateTTL(ttl); err != nil {
		return err
	}

	s.values[key] = value

	return nil
}

func (s *memoryStore) Delete(_ context.Context, key string) error {
	delete(s.values, key)

	return nil
}

func (s *memoryStore) Ping(context.Context) error {
	return nil
}

type cachedUser struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

func TestSetJSONGetJSONRoundTrip(t *testing.T) {
	t.Parallel()

	store := newMemoryStore()
	expected := cachedUser{ID: "user-1", Name: "Ada"}

	if err := SetJSON(context.Background(), store, "user:1", expected, time.Minute); err != nil {
		t.Fatalf("set json: %v", err)
	}

	actual, found, err := GetJSON[cachedUser](context.Background(), store, "user:1")
	if err != nil {
		t.Fatalf("get json: %v", err)
	}

	if !found {
		t.Fatal("cached value was not found")
	}

	if actual != expected {
		t.Fatalf("value = %+v, want %+v", actual, expected)
	}
}

func TestGetJSONMissing(t *testing.T) {
	t.Parallel()

	store := newMemoryStore()

	_, found, err := GetJSON[cachedUser](context.Background(), store, "missing")
	if err != nil {
		t.Fatalf("get json: %v", err)
	}

	if found {
		t.Fatal("missing key was reported as found")
	}
}

func TestGetJSONMalformed(t *testing.T) {
	t.Parallel()

	store := newMemoryStore()
	store.values["broken"] = []byte("{")

	_, found, err := GetJSON[cachedUser](context.Background(), store, "broken")
	if err == nil {
		t.Fatal("expected decode error")
	}

	if !found {
		t.Fatal("malformed value should be reported as found")
	}
}

func TestCacheRejectsInvalidInput(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name      string
		key       string
		ttl       time.Duration
		wantError error
	}{
		{name: "empty key", key: "", ttl: time.Minute, wantError: ErrInvalidKey},
		{name: "whitespace key", key: "   ", ttl: time.Minute, wantError: ErrInvalidKey},
		{name: "zero ttl", key: "key", ttl: 0, wantError: ErrInvalidTTL},
		{name: "negative ttl", key: "key", ttl: -time.Second, wantError: ErrInvalidTTL},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			err := SetJSON(context.Background(), newMemoryStore(), test.key, cachedUser{}, test.ttl)
			if !errors.Is(err, test.wantError) {
				t.Fatalf("error = %v, want %v", err, test.wantError)
			}
		})
	}
}
