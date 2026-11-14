package config

import (
	"testing"
	"time"
)

func TestValidateDedupe(t *testing.T) {
	t.Parallel()

	if err := (Config{DedupeTTL: time.Hour}).validateDedupe(); err != nil {
		t.Fatalf("valid config: %v", err)
	}

	if err := (Config{}).validateDedupe(); err == nil {
		t.Fatal("expected error for zero ttl")
	}
}
