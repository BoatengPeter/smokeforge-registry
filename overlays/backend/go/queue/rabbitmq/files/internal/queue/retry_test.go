package queue

import (
	"errors"
	"testing"
	"time"
)

func TestRetryPolicyDelayUsesJitterBounds(t *testing.T) {
	t.Parallel()

	policy := RetryPolicy{
		Delays: []time.Duration{time.Second, 5 * time.Second},
		Jitter: 0.2,
	}

	tests := []struct {
		name    string
		attempt int
		random  float64
		min     time.Duration
		max     time.Duration
	}{
		{name: "first low", attempt: 0, random: 0, min: 800 * time.Millisecond, max: 800 * time.Millisecond},
		{name: "first high", attempt: 0, random: 0.999999, min: 1199 * time.Millisecond, max: 1200 * time.Millisecond},
		{name: "last delay reused", attempt: 9, random: 0.5, min: 5 * time.Second, max: 5 * time.Second},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			delay, err := policy.delay(test.attempt, func() float64 { return test.random })
			if err != nil {
				t.Fatalf("delay: %v", err)
			}

			if delay < test.min || delay > test.max {
				t.Fatalf("delay = %s, want between %s and %s", delay, test.min, test.max)
			}
		})
	}
}

func TestRetryPolicyRejectsInvalidInput(t *testing.T) {
	t.Parallel()

	if _, err := (RetryPolicy{}).Delay(0); !errors.Is(err, ErrInvalidRetryPolicy) {
		t.Fatalf("empty policy error = %v", err)
	}

	if _, err := (RetryPolicy{Delays: []time.Duration{time.Second}, Jitter: 1}).Delay(0); !errors.Is(err, ErrInvalidRetryPolicy) {
		t.Fatalf("jitter error = %v", err)
	}

	if _, err := (RetryPolicy{Delays: []time.Duration{time.Second}}).Delay(-1); !errors.Is(err, ErrInvalidRetryPolicy) {
		t.Fatalf("attempt error = %v", err)
	}
}
