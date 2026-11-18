package http

import _ "embed"

//go:embed openapi.yaml
var spec []byte

// OpenAPISpec contributes the Library sync contract to Swagger UI.
func (h *Handler) OpenAPISpec() []byte {
	return spec
}
