package totp

import _ "embed"

//go:embed openapi.yaml
var openAPISpec []byte

// OpenAPISpec returns the TOTP contract for the merged Swagger document.
func (h *Handler) OpenAPISpec() []byte {
	return openAPISpec
}
