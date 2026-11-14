package queue

import (
	"errors"
	"math/rand/v2"
	"time"
)

const defaultJitter = 0.2

// ErrInvalidRetryPolicy means retry delays or jitter are invalid.
var ErrInvalidRetryPolicy = errors.New("queue: invalid retry policy")

// RetryPolicy describes bounded retry delays with proportional jitter.
type RetryPolicy struct {
	Delays []time.Duration
	Jitter float64
}

// DefaultRetryPolicy retries after 1s, 5s, and 30s with ±20% jitter.
var DefaultRetryPolicy = RetryPolicy{
	Delays: []time.Duration{time.Second, 5 * time.Second, 30 * time.Second},
	Jitter: defaultJitter,
}

// Delay returns the retry delay for a zero-based retry attempt.
func (p RetryPolicy) Delay(attempt int) (time.Duration, error) {
	return p.delay(attempt, rand.Float64)
}

func (p RetryPolicy) delay(attempt int, random func() float64) (time.Duration, error) {
	if attempt < 0 || len(p.Delays) == 0 {
		return 0, ErrInvalidRetryPolicy
	}

	if p.Jitter < 0 || p.Jitter >= 1 {
		return 0, ErrInvalidRetryPolicy
	}

	index := attempt
	if index >= len(p.Delays) {
		index = len(p.Delays) - 1
	}

	base := p.Delays[index]
	if base <= 0 {
		return 0, ErrInvalidRetryPolicy
	}

	jitter := (random()*2 - 1) * p.Jitter

	delay := time.Duration(float64(base) * (1 + jitter))
	if delay <= 0 {
		return time.Millisecond, nil
	}

	return delay, nil
}
