// Package domain contains pure application types.
package domain

// Response is the value returned by the ping use case.
type Response struct {
	Message string
}

// NewResponse constructs the canonical ping response.
func NewResponse() Response {
	return Response{Message: "pong"}
}
