package config

import "errors"

func (c Config) validateDedupe() error {
	if c.DedupeTTL <= 0 {
		return errors.New("dedupe ttl must be positive")
	}

	return nil
}
