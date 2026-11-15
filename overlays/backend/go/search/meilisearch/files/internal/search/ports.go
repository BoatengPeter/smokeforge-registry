package search

import "context"

// Indexer writes documents to the configured index.
type Indexer interface {
	Upsert(ctx context.Context, documents []Document) error
	Delete(ctx context.Context, ids []string) error
}

// Searcher queries the configured index.
type Searcher interface {
	Search(ctx context.Context, query string, options Options) (Result, error)
}

// Engine combines indexing and querying.
type Engine interface {
	Indexer
	Searcher
}
