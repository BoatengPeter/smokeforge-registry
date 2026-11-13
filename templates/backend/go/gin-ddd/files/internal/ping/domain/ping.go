// Package domain contains the ping bounded context.
package domain

// Ping is a small domain value.
type Ping struct {
	message string
}

// NewPing constructs the canonical ping value.
func NewPing() Ping {
	return Ping{message: "pong"}
}

// Message returns the response text.
func (p Ping) Message() string {
	return p.message
}
