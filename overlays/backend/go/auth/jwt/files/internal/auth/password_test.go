package auth

import (
	"strings"
	"testing"
)

func TestHashPasswordRoundTrip(t *testing.T) {
	t.Parallel()

	hash, err := HashPassword("correct horse battery staple")
	if err != nil {
		t.Fatalf("hash: %v", err)
	}

	if !strings.HasPrefix(hash, "$argon2id$") {
		t.Fatalf("hash = %q, want argon2id", hash)
	}

	valid, err := VerifyPassword("correct horse battery staple", hash)
	if err != nil {
		t.Fatalf("verify: %v", err)
	}

	if !valid {
		t.Fatal("password did not verify")
	}
}

func TestVerifyPasswordRejectsWrongPassword(t *testing.T) {
	t.Parallel()

	hash, err := HashPassword("correct horse battery staple")
	if err != nil {
		t.Fatalf("hash: %v", err)
	}

	valid, err := VerifyPassword("wrong password value", hash)
	if err != nil {
		t.Fatalf("verify: %v", err)
	}

	if valid {
		t.Fatal("wrong password verified")
	}
}

func TestVerifyPasswordRejectsMalformedHash(t *testing.T) {
	t.Parallel()

	if _, err := VerifyPassword("password", "not-a-hash"); err == nil {
		t.Fatal("expected malformed hash error")
	}
}

func TestHashPasswordRejectsWeakPassword(t *testing.T) {
	t.Parallel()

	if _, err := HashPassword("short"); err == nil {
		t.Fatal("expected weak password error")
	}
}
