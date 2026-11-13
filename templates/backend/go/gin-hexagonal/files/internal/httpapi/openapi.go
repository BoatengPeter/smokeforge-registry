package httpapi

import (
	"bytes"
	_ "embed"
	"encoding/json"
	"errors"
	"fmt"

	"gopkg.in/yaml.v3"
)

//go:embed openapi.yaml
var baseOpenAPISpec []byte

type document map[string]any

// mergeOpenAPISpecs combines the base contract with feature contracts into one
// document for Swagger UI. Paths and component names must not collide.
func mergeOpenAPISpecs(documents [][]byte) ([]byte, error) {
	if len(documents) == 0 {
		return nil, errors.New("openapi: at least one document is required")
	}

	merged, err := decodeDocument(documents[0])
	if err != nil {
		return nil, fmt.Errorf("openapi: decode base document: %w", err)
	}

	for index, data := range documents[1:] {
		document, decodeErr := decodeDocument(data)
		if decodeErr != nil {
			return nil, fmt.Errorf("openapi: decode document %d: %w", index+1, decodeErr)
		}

		if err := mergeDocument(merged, document); err != nil {
			return nil, fmt.Errorf("openapi: merge document %d: %w", index+1, err)
		}
	}

	output, err := json.MarshalIndent(merged, "", "  ")
	if err != nil {
		return nil, fmt.Errorf("openapi: marshal merged document: %w", err)
	}

	return append(output, '\n'), nil
}

func decodeDocument(data []byte) (document, error) {
	var decoded document

	decoder := yaml.NewDecoder(bytes.NewReader(data))
	if err := decoder.Decode(&decoded); err != nil {
		return nil, err
	}

	if len(decoded) == 0 {
		return nil, errors.New("empty document")
	}

	return decoded, nil
}

func mergeDocument(target, source document) error {
	for _, key := range []string{"paths", "components"} {
		if err := mergeSection(target, source, key); err != nil {
			return err
		}
	}

	mergeTags(target, source)

	return nil
}

func mergeSection(target, source document, key string) error {
	sourceSection, ok := source[key].(map[string]any)
	if !ok {
		return nil
	}

	targetSection, ok := target[key].(map[string]any)
	if !ok {
		targetSection = make(map[string]any, len(sourceSection))
		target[key] = targetSection
	}

	for name, value := range sourceSection {
		if _, exists := targetSection[name]; !exists {
			targetSection[name] = value

			continue
		}

		if key != "components" {
			return fmt.Errorf("%s.%s is declared more than once", key, name)
		}

		existing, existingOK := targetSection[name].(map[string]any)

		incoming, incomingOK := value.(map[string]any)
		if !existingOK || !incomingOK {
			return fmt.Errorf("%s.%s is declared more than once", key, name)
		}

		mergeMap(existing, incoming)
	}

	return nil
}

func mergeMap(target, source map[string]any) {
	for key, value := range source {
		if _, exists := target[key]; exists {
			continue
		}

		target[key] = value
	}
}

func mergeTags(target, source document) {
	incoming, ok := source["tags"].([]any)
	if !ok {
		return
	}

	existing, _ := target["tags"].([]any)
	seen := make(map[string]bool, len(existing))

	for _, tag := range existing {
		if item, ok := tag.(map[string]any); ok {
			if name, ok := item["name"].(string); ok {
				seen[name] = true
			}
		}
	}

	for _, tag := range incoming {
		item, ok := tag.(map[string]any)
		if !ok {
			continue
		}

		name, _ := item["name"].(string)
		if seen[name] {
			continue
		}

		existing = append(existing, tag)
		seen[name] = true
	}

	target["tags"] = existing
}
