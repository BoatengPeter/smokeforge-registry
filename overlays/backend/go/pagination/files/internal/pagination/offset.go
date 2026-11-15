package pagination

import (
	"fmt"
	"net/url"
	"strconv"
)

// ParseOffset parses page and page_size with defaults, caps, and a deep-offset
// guard. Invalid values fail before a query runs.
func ParseOffset(values url.Values, limits Limits) (Offset, error) {
	if err := limits.validate(); err != nil {
		return Offset{}, fmt.Errorf("%w: invalid limits", ErrInvalidInput)
	}

	page, err := positiveQueryInt(values, "page", 1)
	if err != nil {
		return Offset{}, err
	}

	pageSize, err := positiveQueryInt(values, "page_size", limits.DefaultLimit)
	if err != nil {
		return Offset{}, err
	}

	if pageSize > limits.MaxLimit {
		return Offset{}, fmt.Errorf("%w: page_size must be at most %d", ErrInvalidInput, limits.MaxLimit)
	}

	if page-1 > limits.MaxOffset/pageSize {
		return Offset{}, fmt.Errorf("%w: page exceeds the maximum offset", ErrInvalidInput)
	}

	return Offset{Page: page, PageSize: pageSize}, nil
}

func positiveQueryInt(values url.Values, name string, fallback int) (int, error) {
	raw := values.Get(name)
	if raw == "" {
		return fallback, nil
	}

	value, err := strconv.Atoi(raw)
	if err != nil || value < 1 {
		return 0, fmt.Errorf("%w: %s must be a positive integer", ErrInvalidInput, name)
	}

	return value, nil
}
