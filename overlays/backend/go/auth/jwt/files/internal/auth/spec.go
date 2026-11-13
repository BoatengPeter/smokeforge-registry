package auth

import _ "embed"

//go:embed openapi.yaml
var openAPISpec []byte

// OpenAPISpec returns the auth contract for the merged Swagger document.
func (h *Handler) OpenAPISpec() []byte {
	return openAPISpec
}
