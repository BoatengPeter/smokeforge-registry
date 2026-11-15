// Package pagination provides signed cursor and offset pagination helpers.
package pagination

import "errors"

// ErrInvalidInput reports invalid pagination parameters or cursors.
var ErrInvalidInput = errors.New("pagination: invalid input")

// Limits bounds pagination input before it reaches a query.
type Limits struct {
	DefaultLimit int
	MaxLimit     int
	MaxOffset    int
}

// Offset describes a page in offset pagination.
type Offset struct {
	Page     int
	PageSize int
}

// OffsetValue returns the SQL OFFSET for this page.
func (o Offset) OffsetValue() int {
	return (o.Page - 1) * o.PageSize
}

// Limit returns the SQL LIMIT for this page.
func (o Offset) Limit() int {
	return o.PageSize
}

// Cursor describes one cursor-paginated request.
type Cursor struct {
	Limit int
	Raw   string
}

// Metadata is the pagination portion of a list response.
type Metadata struct {
	Mode       string `json:"mode"`
	Limit      int    `json:"limit"`
	Offset     int    `json:"offset,omitempty"`
	Page       int    `json:"page,omitempty"`
	PageSize   int    `json:"page_size,omitempty"`
	TotalItems *int64 `json:"total_items,omitempty"`
	TotalPages *int64 `json:"total_pages,omitempty"`
	NextCursor string `json:"next_cursor,omitempty"`
	HasMore    bool   `json:"has_more"`
}

// Page is one page of items with pagination metadata.
type Page[T any] struct {
	Items      []T      `json:"items"`
	Pagination Metadata `json:"pagination"`
}

// NewPage keeps empty pages as JSON arrays instead of null.
func NewPage[T any](items []T, metadata Metadata) Page[T] {
	if items == nil {
		items = []T{}
	}

	return Page[T]{Items: items, Pagination: metadata}
}

// OffsetMetadata builds metadata for an offset page.
func OffsetMetadata(offset Offset, totalItems int64) Metadata {
	totalPages := int64(0)
	if offset.PageSize > 0 {
		totalPages = (totalItems + int64(offset.PageSize) - 1) / int64(offset.PageSize)
	}

	return Metadata{
		Mode:       "offset",
		Limit:      offset.PageSize,
		Offset:     offset.OffsetValue(),
		Page:       offset.Page,
		PageSize:   offset.PageSize,
		TotalItems: &totalItems,
		TotalPages: &totalPages,
		HasMore:    int64(offset.OffsetValue()+offset.PageSize) < totalItems,
	}
}

// CursorMetadata builds metadata for a cursor page. An empty next cursor means
// the caller reached the end.
func CursorMetadata(limit int, nextCursor string) Metadata {
	return Metadata{
		Mode:       "cursor",
		Limit:      limit,
		NextCursor: nextCursor,
		HasMore:    nextCursor != "",
	}
}

func (l Limits) validate() error {
	if l.DefaultLimit < 1 || l.MaxLimit < 1 {
		return ErrInvalidInput
	}

	if l.DefaultLimit > l.MaxLimit {
		return ErrInvalidInput
	}

	if l.MaxOffset < 0 {
		return ErrInvalidInput
	}

	return nil
}
