package auth

import (
	"context"

	"github.com/google/uuid"
)

// Store is the persistence contract consumed by the auth service.
type Store interface {
	CreateUser(ctx context.Context, id uuid.UUID, email, passwordHash, role string) (UserRecord, error)
	GetUserByEmail(ctx context.Context, email string) (UserRecord, error)
	GetUserByID(ctx context.Context, id uuid.UUID) (UserRecord, error)
	MarkEmailVerified(ctx context.Context, id uuid.UUID) error
	UpdatePassword(ctx context.Context, id uuid.UUID, passwordHash string) error
	CreateToken(ctx context.Context, params CreateTokenParams) error
	GetTokenByHash(ctx context.Context, hash []byte) (TokenRecord, error)
	ConsumeToken(ctx context.Context, id uuid.UUID) (int64, error)
	RevokeToken(ctx context.Context, id uuid.UUID) error
	RevokeUserTokens(ctx context.Context, userID uuid.UUID, kind string) error
	GetOAuthAccount(ctx context.Context, provider, providerAccountID string) (uuid.UUID, error)
	CreateOAuthAccount(ctx context.Context, id, userID uuid.UUID, provider, providerAccountID, email string) error
}

// EmailSender delivers transactional authentication emails.
type EmailSender interface {
	Send(ctx context.Context, to, subject, body string) error
}

// SecondFactor verifies a user's configured second factor.
type SecondFactor interface {
	Required(ctx context.Context, userID uuid.UUID) (bool, error)
	Verify(ctx context.Context, userID uuid.UUID, code string) error
}

// TokenVerifier validates access tokens for authenticated routes.
type TokenVerifier interface {
	Authenticate(ctx context.Context, raw string) (Identity, error)
}
