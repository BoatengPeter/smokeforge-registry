// Package media owns image metadata and upload/download workflows.
package media

import (
	"errors"
	"time"

	"github.com/google/uuid"
)

// Status is the lifecycle state of an uploaded object.
type Status string

const (
	// StatusUnknown is the zero value and must never be persisted.
	StatusUnknown Status = ""
	// StatusPending means the client reserved an upload but has not completed it.
	StatusPending Status = "pending"
	// StatusReady means the object exists and passed validation.
	StatusReady Status = "ready"
)

// Transform describes an optional image resize.
type Transform struct {
	Width   int
	Height  int
	Fit     string
	Format  string
	Quality int
}

// IsZero reports whether no transformation was requested.
func (t Transform) IsZero() bool {
	return t.Width == 0 && t.Height == 0 && t.Fit == "" && t.Format == "" && t.Quality == 0
}

// Media is an image owned by one user.
type Media struct {
	ID          uuid.UUID `json:"id"`
	OwnerID     uuid.UUID `json:"-"`
	ObjectKey   string    `json:"-"`
	ContentType string    `json:"content_type"`
	ByteSize    int64     `json:"size_bytes"`
	Filename    string    `json:"filename"`
	Status      Status    `json:"status"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

// UploadInput reserves an upload for one authenticated user.
type UploadInput struct {
	Filename    string
	ContentType string
	ByteSize    int64
}

// Upload is the client-facing upload reservation.
type Upload struct {
	Media     Media
	URL       string
	Headers   map[string]string
	ExpiresIn int64
}

// URL is a timed image URL.
type URL struct {
	URL       string
	ExpiresIn int64
}

// ObjectInfo describes an object stored in the bucket.
type ObjectInfo struct {
	ByteSize    int64
	ContentType string
}

// UploadTarget is a signed direct-upload target.
type UploadTarget struct {
	URL       string
	Headers   map[string]string
	ExpiresIn int64
}

var (
	// ErrNotFound hides whether an object exists for another owner.
	ErrNotFound = errors.New("media: not found")
	// ErrInvalidInput reports invalid upload metadata or transformations.
	ErrInvalidInput = errors.New("media: invalid input")
	// ErrUnsupportedContentType reports a content type outside the allow-list.
	ErrUnsupportedContentType = errors.New("media: unsupported content type")
	// ErrUnsupportedTransform reports a transformation with no URL builder.
	ErrUnsupportedTransform = errors.New("media: transformations are not configured")
	// ErrObjectMissing reports that the client never uploaded the object.
	ErrObjectMissing = errors.New("media: uploaded object is missing")
	// ErrSizeMismatch reports an uploaded object with the wrong byte size.
	ErrSizeMismatch = errors.New("media: uploaded object size does not match")
	// ErrContentTypeMismatch reports an uploaded object with the wrong type.
	ErrContentTypeMismatch = errors.New("media: uploaded object content type does not match")
)
