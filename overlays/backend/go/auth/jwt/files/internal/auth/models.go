package auth

import (
	"context"
	"errors"
	"time"

	"github.com/google/uuid"
)

var (
	// ErrInvalidCredentials means the email or password is wrong.
	ErrInvalidCredentials = errors.New("auth: invalid credentials")
	// ErrEmailTaken means the email is already registered.
	ErrEmailTaken = errors.New("auth: email already registered")
	// ErrInvalidToken means a token is missing, expired, or already used.
	ErrInvalidToken = errors.New("auth: invalid token")
	// ErrEmailNotVerified means the account must verify its email first.
	ErrEmailNotVerified = errors.New("auth: email not verified")
	// ErrMFAUnavailable means no second factor is configured.
	ErrMFAUnavailable = errors.New("auth: second factor unavailable")
	// ErrInvalidCode means a TOTP or recovery code is wrong.
	ErrInvalidCode = errors.New("auth: invalid code")
	// ErrWeakPassword means the password does not meet the policy.
	ErrWeakPassword = errors.New("auth: password must be 12-256 characters")
	// ErrInvalidEmail means the email address cannot be parsed.
	ErrInvalidEmail = errors.New("auth: invalid email")
)

const (
	tokenKindRefresh       = "refresh"
	tokenKindEmailVerify   = "email_verify"
	tokenKindPasswordReset = "password_reset"
)

// Account is the public account representation.
type Account struct {
	ID            uuid.UUID
	Email         string
	Role          string
	EmailVerified bool
	CreatedAt     time.Time
}

// Identity is the authenticated principal attached to a request.
type Identity struct {
	UserID uuid.UUID
	Role   string
}

// TokenPair is an access token and its rotated refresh token.
type TokenPair struct {
	AccessToken  string
	RefreshToken string
	ExpiresIn    int64
	Account      Account
}

// LoginResult is either a token pair or a second-factor challenge.
type LoginResult struct {
	Tokens       *TokenPair
	MFAChallenge string
	MFAExpiresIn int64
}

// SecondFactor verifies a user's configured second factor.
type SecondFactor interface {
	Required(ctx context.Context, userID uuid.UUID) (bool, error)
	Verify(ctx context.Context, userID uuid.UUID, code string) error
}

// UserRecord is the internal account row returned by the store.
type UserRecord struct {
	ID              uuid.UUID
	Email           string
	PasswordHash    string
	Role            string
	EmailVerifiedAt *time.Time
	CreatedAt       time.Time
}

func (u UserRecord) account() Account {
	return Account{
		ID:            u.ID,
		Email:         u.Email,
		Role:          u.Role,
		EmailVerified: u.EmailVerifiedAt != nil,
		CreatedAt:     u.CreatedAt,
	}
}

// TokenRecord is the internal auth token row returned by the store.
type TokenRecord struct {
	ID        uuid.UUID
	UserID    uuid.UUID
	Kind      string
	ExpiresAt time.Time
	UsedAt    *time.Time
}

// CreateTokenParams is the storage input for an opaque token.
type CreateTokenParams struct {
	ID        uuid.UUID
	UserID    uuid.UUID
	Kind      string
	TokenHash []byte
	ExpiresAt time.Time
}
