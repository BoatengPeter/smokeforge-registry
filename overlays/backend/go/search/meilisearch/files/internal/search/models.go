// Package search defines provider-neutral document indexing and querying.
package search

import (
	"encoding/json"
	"errors"
)

// Document is one indexable record. ID must be stable and unique.
type Document struct {
	ID     string         `json:"id"`
	Fields map[string]any `json:"fields"`
}

// Options controls one page of results.
type Options struct {
	Limit  int
	Offset int
}

// Result is one page of provider results.
type Result struct {
	Query              string
	Hits               []json.RawMessage
	EstimatedTotalHits int
	Limit              int
	Offset             int
	ProcessingTimeMs   int
}

var (
	// ErrInvalidInput reports invalid query or pagination values.
	ErrInvalidInput = errors.New("search: invalid input")
	// ErrUnavailable reports that the search engine cannot serve the request.
	ErrUnavailable = errors.New("search: unavailable")
)
