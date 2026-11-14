// Package queue defines the job contract shared by publishers and workers.
package queue

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"sync"
	"time"

	"github.com/google/uuid"
)

var (
	// ErrInvalidJob means a job is missing required metadata.
	ErrInvalidJob = errors.New("queue: invalid job")
	// ErrUnknownJobType means no handler is registered for a job type.
	ErrUnknownJobType = errors.New("queue: unknown job type")
	// ErrHandlerRegistered means a handler is already registered for a job type.
	ErrHandlerRegistered = errors.New("queue: handler already registered")
)

// Job is the durable envelope passed through the queue.
type Job struct {
	ID        uuid.UUID       `json:"id"`
	Type      string          `json:"type"`
	Payload   json.RawMessage `json:"payload,omitempty"`
	Attempt   int             `json:"attempt"`
	CreatedAt time.Time       `json:"created_at"`
}

// Validate checks the job envelope before publishing or handling.
func (j Job) Validate() error {
	if j.ID == uuid.Nil {
		return fmt.Errorf("%w: id must not be empty", ErrInvalidJob)
	}

	if strings.TrimSpace(j.Type) == "" {
		return fmt.Errorf("%w: type must not be empty", ErrInvalidJob)
	}

	if j.Attempt < 0 {
		return fmt.Errorf("%w: attempt must not be negative", ErrInvalidJob)
	}

	return nil
}

// Encode validates and serializes a job.
func (j Job) Encode() ([]byte, error) {
	if err := j.Validate(); err != nil {
		return nil, err
	}

	encoded, err := json.Marshal(j)
	if err != nil {
		return nil, fmt.Errorf("queue: encode job: %w", err)
	}

	return encoded, nil
}

// Decode parses and validates a job envelope.
func Decode(data []byte) (Job, error) {
	var job Job
	if err := json.Unmarshal(data, &job); err != nil {
		return Job{}, fmt.Errorf("queue: decode job: %w", err)
	}

	if err := job.Validate(); err != nil {
		return Job{}, err
	}

	return job, nil
}

// Publisher writes one job to the queue.
type Publisher interface {
	Publish(ctx context.Context, job Job) error
}

// Deduplicator records successfully processed job IDs.
type Deduplicator interface {
	Seen(ctx context.Context, job Job) (bool, error)
	Mark(ctx context.Context, job Job) error
}

// Handler processes one job.
type Handler interface {
	Handle(ctx context.Context, job Job) error
}

// HandlerFunc adapts a function to Handler.
type HandlerFunc func(context.Context, Job) error

// Handle executes the function.
func (f HandlerFunc) Handle(ctx context.Context, job Job) error {
	return f(ctx, job)
}

// Registry maps job types to handlers.
type Registry struct {
	mu       sync.RWMutex
	handlers map[string]Handler
}

// NewRegistry constructs an empty handler registry.
func NewRegistry() *Registry {
	return &Registry{handlers: map[string]Handler{}}
}

// Register adds a handler for a job type.
func (r *Registry) Register(jobType string, handler Handler) error {
	if strings.TrimSpace(jobType) == "" {
		return fmt.Errorf("%w: handler type must not be empty", ErrInvalidJob)
	}

	if handler == nil {
		return fmt.Errorf("%w: handler must not be nil", ErrInvalidJob)
	}

	r.mu.Lock()
	defer r.mu.Unlock()

	if _, exists := r.handlers[jobType]; exists {
		return fmt.Errorf("%w: %s", ErrHandlerRegistered, jobType)
	}

	r.handlers[jobType] = handler

	return nil
}

// Lookup returns the handler for a job type.
func (r *Registry) Lookup(jobType string) (Handler, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	handler, ok := r.handlers[jobType]

	return handler, ok
}
