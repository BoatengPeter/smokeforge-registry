// Package service contains application use cases.
package service

import "context"

// Ping is a small use case used to verify transport and service wiring.
type Ping struct{}

// NewPing constructs the ping use case.
func NewPing() *Ping {
	return &Ping{}
}

// Execute returns the configured response after checking request cancellation.
func (p *Ping) Execute(ctx context.Context) (string, error) {
	if err := ctx.Err(); err != nil {
		return "", err
	}

	return "pong", nil
}
