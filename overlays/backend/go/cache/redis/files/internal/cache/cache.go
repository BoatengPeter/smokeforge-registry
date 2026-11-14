// Package cache defines the cache boundary used by application code.
package cache

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"time"
)

var (
	// ErrInvalidKey means a cache key is empty.
	ErrInvalidKey = errors.New("cache: key must not be empty")
	// ErrInvalidTTL means a cache write has no positive expiration.
	ErrInvalidTTL = errors.New("cache: ttl must be positive")
)

// Store is the cache contract consumed by application code.
type Store interface {
	Get(ctx context.Context, key string) ([]byte, bool, error)
	Set(ctx context.Context, key string, value []byte, ttl time.Duration) error
	Delete(ctx context.Context, key string) error
	Ping(ctx context.Context) error
}

// GetJSON reads and decodes a cached JSON value.
func GetJSON[T any](ctx context.Context, store Store, key string) (T, bool, error) {
	var zero T

	if err := ValidateKey(key); err != nil {
		return zero, false, err
	}

	value, found, err := store.Get(ctx, key)
	if err != nil || !found {
		return zero, found, err
	}

	var decoded T
	if err := json.Unmarshal(value, &decoded); err != nil {
		return zero, true, fmt.Errorf("cache: decode value: %w", err)
	}

	return decoded, true, nil
}

// SetJSON encodes and writes a JSON cache value.
func SetJSON[T any](
	ctx context.Context,
	store Store,
	key string,
	value T,
	ttl time.Duration,
) error {
	if err := ValidateKey(key); err != nil {
		return err
	}

	if err := ValidateTTL(ttl); err != nil {
		return err
	}

	encoded, err := json.Marshal(value)
	if err != nil {
		return fmt.Errorf("cache: encode value: %w", err)
	}

	return store.Set(ctx, key, encoded, ttl)
}

// ValidateKey rejects keys that cannot be namespaced safely.
func ValidateKey(key string) error {
	if strings.TrimSpace(key) == "" {
		return ErrInvalidKey
	}

	return nil
}

// ValidateTTL requires an explicit positive expiration.
func ValidateTTL(ttl time.Duration) error {
	if ttl <= 0 {
		return ErrInvalidTTL
	}

	return nil
}
