package media

import (
	"context"
	"time"

	"github.com/google/uuid"
)

// Repository persists media metadata.
type Repository interface {
	Create(ctx context.Context, item Media) error
	Get(ctx context.Context, id, ownerID uuid.UUID) (Media, error)
	MarkReady(ctx context.Context, id, ownerID uuid.UUID) (Media, error)
	Delete(ctx context.Context, id, ownerID uuid.UUID) error
}

// ObjectStore signs and inspects private objects.
type ObjectStore interface {
	PresignPut(ctx context.Context, key, contentType string, ttl time.Duration) (UploadTarget, error)
	PresignGet(ctx context.Context, key string, ttl time.Duration) (string, error)
	Head(ctx context.Context, key string) (ObjectInfo, error)
	Delete(ctx context.Context, key string) error
}

// URLBuilder turns an object key into a transform URL.
type URLBuilder interface {
	BuildURL(objectKey string, transform Transform) (string, error)
}
