package totp

import (
	"bytes"
	"testing"
)

func TestNewRecoveryCode(t *testing.T) {
	t.Parallel()

	first, err := newRecoveryCode()
	if err != nil {
		t.Fatalf("new recovery code: %v", err)
	}

	second, err := newRecoveryCode()
	if err != nil {
		t.Fatalf("new recovery code: %v", err)
	}

	if len(first) != recoveryCodeLen*2 {
		t.Fatalf("code length = %d", len(first))
	}

	if first == second {
		t.Fatal("recovery codes are not unique")
	}

	if bytes.Equal(hashRecoveryCode(first), []byte(first)) {
		t.Fatal("stored hash equals plaintext code")
	}
}
