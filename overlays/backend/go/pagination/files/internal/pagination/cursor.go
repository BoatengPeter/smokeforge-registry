package pagination

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net/url"
	"strings"
)

const (
	cursorVersion = 1
	maxCursorSize = 4096
	minSecretSize = 32
)

// Codec signs and verifies opaque cursor values.
type Codec struct {
	key []byte
}

// NewCodec builds an HMAC codec from a server-side secret.
func NewCodec(secret string) (*Codec, error) {
	if len([]byte(secret)) < minSecretSize {
		return nil, fmt.Errorf("%w: cursor secret must be at least %d bytes", ErrInvalidInput, minSecretSize)
	}

	return &Codec{key: []byte(secret)}, nil
}

// Encode serializes and signs cursor values.
func (c *Codec) Encode(values map[string]string) (string, error) {
	if len(values) == 0 {
		return "", fmt.Errorf("%w: cursor values must not be empty", ErrInvalidInput)
	}

	payload := cursorPayload{
		Version: cursorVersion,
		Values:  values,
	}

	encoded, err := json.Marshal(payload)
	if err != nil {
		return "", fmt.Errorf("%w: encode cursor: %w", ErrInvalidInput, err)
	}

	body := base64.RawURLEncoding.EncodeToString(encoded)

	return body + "." + c.sign(body), nil
}

// Decode verifies the signature and decodes cursor values.
func (c *Codec) Decode(raw string) (map[string]string, error) {
	if raw == "" || len(raw) > maxCursorSize {
		return nil, fmt.Errorf("%w: invalid cursor", ErrInvalidInput)
	}

	parts := strings.Split(raw, ".")
	if len(parts) != 2 {
		return nil, fmt.Errorf("%w: invalid cursor", ErrInvalidInput)
	}

	expected := c.sign(parts[0])
	if !hmac.Equal([]byte(parts[1]), []byte(expected)) {
		return nil, fmt.Errorf("%w: invalid cursor signature", ErrInvalidInput)
	}

	decoded, err := base64.RawURLEncoding.DecodeString(parts[0])
	if err != nil {
		return nil, fmt.Errorf("%w: decode cursor: %w", ErrInvalidInput, err)
	}

	var payload cursorPayload
	if err := json.Unmarshal(decoded, &payload); err != nil {
		return nil, fmt.Errorf("%w: decode cursor payload: %w", ErrInvalidInput, err)
	}

	if payload.Version != cursorVersion || len(payload.Values) == 0 {
		return nil, fmt.Errorf("%w: unsupported cursor", ErrInvalidInput)
	}

	for key := range payload.Values {
		if strings.TrimSpace(key) == "" {
			return nil, fmt.Errorf("%w: empty cursor field", ErrInvalidInput)
		}
	}

	return payload.Values, nil
}

// ParseCursor parses a cursor request limit and keeps the raw cursor opaque.
func ParseCursor(values url.Values, limits Limits) (Cursor, error) {
	if err := limits.validate(); err != nil {
		return Cursor{}, fmt.Errorf("%w: invalid limits", ErrInvalidInput)
	}

	limit, err := positiveQueryInt(values, "limit", limits.DefaultLimit)
	if err != nil {
		return Cursor{}, err
	}

	if limit > limits.MaxLimit {
		return Cursor{}, fmt.Errorf("%w: limit must be at most %d", ErrInvalidInput, limits.MaxLimit)
	}

	raw := strings.TrimSpace(values.Get("cursor"))
	if len(raw) > maxCursorSize {
		return Cursor{}, fmt.Errorf("%w: cursor is too long", ErrInvalidInput)
	}

	return Cursor{Limit: limit, Raw: raw}, nil
}

func (c *Codec) sign(body string) string {
	mac := hmac.New(sha256.New, c.key)
	mac.Write([]byte(body))

	return base64.RawURLEncoding.EncodeToString(mac.Sum(nil))
}

type cursorPayload struct {
	Version int               `json:"v"`
	Values  map[string]string `json:"k"`
}
