package auth

import (
	"crypto/rand"
	"crypto/subtle"
	"encoding/base64"
	"errors"
	"fmt"
	"strconv"
	"strings"

	"golang.org/x/crypto/argon2"
)

const (
	argonMemory      = 64 * 1024
	argonIterations  = 3
	argonParallelism = 2
	argonSaltLength  = 16
	argonKeyLength   = 32
)

// HashPassword hashes a password with Argon2id and a random salt.
func HashPassword(password string) (string, error) {
	if err := validatePassword(password); err != nil {
		return "", err
	}

	salt := make([]byte, argonSaltLength)
	if _, err := rand.Read(salt); err != nil {
		return "", fmt.Errorf("generate password salt: %w", err)
	}

	hash := argon2.IDKey(
		[]byte(password),
		salt,
		argonIterations,
		argonMemory,
		argonParallelism,
		argonKeyLength,
	)

	return fmt.Sprintf(
		"$argon2id$v=19$m=%d,t=%d,p=%d$%s$%s",
		argonMemory,
		argonIterations,
		argonParallelism,
		base64.RawStdEncoding.EncodeToString(salt),
		base64.RawStdEncoding.EncodeToString(hash),
	), nil
}

// VerifyPassword compares a password with an encoded Argon2id hash.
func VerifyPassword(password, encoded string) (bool, error) {
	parts := strings.Split(encoded, "$")
	if len(parts) != 6 || parts[1] != "argon2id" || parts[2] != "v=19" {
		return false, errors.New("auth: invalid password hash")
	}

	params, err := parseArgonParams(parts[3])
	if err != nil {
		return false, err
	}

	salt, err := base64.RawStdEncoding.DecodeString(parts[4])
	if err != nil {
		return false, fmt.Errorf("decode password salt: %w", err)
	}

	expected, err := base64.RawStdEncoding.DecodeString(parts[5])
	if err != nil {
		return false, fmt.Errorf("decode password hash: %w", err)
	}

	if len(expected) == 0 || len(expected) > 1024 {
		return false, errors.New("auth: invalid password hash length")
	}

	actual := argon2.IDKey(
		[]byte(password),
		salt,
		params.iterations,
		params.memory,
		params.parallelism,
		uint32(len(expected)), //nolint:gosec // expected length is bounded above.
	)

	return subtle.ConstantTimeCompare(actual, expected) == 1, nil
}

type argonParams struct {
	memory      uint32
	iterations  uint32
	parallelism uint8
}

func parseArgonParams(value string) (argonParams, error) {
	params := argonParams{}

	for part := range strings.SplitSeq(value, ",") {
		key, raw, found := strings.Cut(part, "=")
		if !found {
			return argonParams{}, errors.New("auth: invalid password params")
		}

		number, err := strconv.ParseUint(raw, 10, 32)
		if err != nil {
			return argonParams{}, fmt.Errorf("parse password params: %w", err)
		}

		switch key {
		case "m":
			params.memory = uint32(number)
		case "t":
			params.iterations = uint32(number)
		case "p":
			if number > 255 {
				return argonParams{}, errors.New("auth: invalid password parallelism")
			}

			params.parallelism = uint8(number)
		default:
			return argonParams{}, errors.New("auth: unknown password parameter")
		}
	}

	if params.memory == 0 || params.iterations == 0 || params.parallelism == 0 {
		return argonParams{}, errors.New("auth: incomplete password params")
	}

	return params, nil
}

func validatePassword(password string) error {
	if len(password) < 12 || len(password) > 256 {
		return ErrWeakPassword
	}

	return nil
}
