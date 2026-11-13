package swagger

import (
	"strings"
	"testing"
)

func TestMergeOpenAPISpecs(t *testing.T) {
	t.Parallel()

	base := []byte(`openapi: 3.0.3
info:
  title: Base
  version: 0.1.0
paths:
  /healthz:
    get:
      responses:
        "200":
          description: ok
`)
	feature := []byte(`openapi: 3.0.3
info:
  title: Feature
  version: 0.1.0
paths:
  /api/v1/auth/login:
    post:
      responses:
        "204":
          description: ok
tags:
  - name: auth
`)

	merged, err := mergeOpenAPISpecs([][]byte{base, feature})
	if err != nil {
		t.Fatalf("merge: %v", err)
	}

	document := string(merged)
	for _, path := range []string{"/healthz", "/api/v1/auth/login", `"auth"`} {
		if !strings.Contains(document, path) {
			t.Fatalf("merged document missing %q:\n%s", path, document)
		}
	}
}
