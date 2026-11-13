// Package app contains the flat application implementation.
package app

import "context"

// Pinger is the small application behavior exposed by the API.
type Pinger struct{}

// NewPinger constructs the application.
func NewPinger() *Pinger {
	return &Pinger{}
}

// Ping returns the configured response after checking request cancellation.
func (p *Pinger) Ping(ctx context.Context) (string, error) {
	if err := ctx.Err(); err != nil {
		return "", err
	}

	return "pong", nil
}
