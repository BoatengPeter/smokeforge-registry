package searchhttp

import "encoding/json"

// SearchResponse is the public search result page.
type SearchResponse struct {
	Query              string            `json:"query"`
	Hits               []json.RawMessage `json:"hits"`
	EstimatedTotalHits int               `json:"estimated_total_hits"`
	Limit              int               `json:"limit"`
	Offset             int               `json:"offset"`
	ProcessingTimeMs   int               `json:"processing_time_ms"`
}
