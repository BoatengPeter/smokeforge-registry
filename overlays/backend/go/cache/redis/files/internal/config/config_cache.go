package config

import (
	"errors"
	"fmt"
	"net/url"
	"strings"
)

func (c Config) validateCache() error {
	rawURL := strings.TrimSpace(c.RedisURL)
	if rawURL == "" {
		return errors.New("redis url must not be empty")
	}

	parsed, err := url.Parse(rawURL)
	if err != nil {
		return fmt.Errorf("parse redis url: %w", err)
	}

	if parsed.Scheme != "redis" && parsed.Scheme != "rediss" {
		return fmt.Errorf("redis url scheme %q must be redis or rediss", parsed.Scheme)
	}

	if parsed.Host == "" {
		return errors.New("redis url must include a host")
	}

	if c.RedisDialTimeout <= 0 {
		return errors.New("redis dial timeout must be positive")
	}

	if c.RedisReadTimeout <= 0 {
		return errors.New("redis read timeout must be positive")
	}

	if c.RedisWriteTimeout <= 0 {
		return errors.New("redis write timeout must be positive")
	}

	if c.RedisPoolSize < 1 {
		return errors.New("redis pool size must be at least 1")
	}

	if c.RedisMinIdleConns < 0 || c.RedisMinIdleConns > c.RedisPoolSize {
		return errors.New("redis min idle conns must be between 0 and the pool size")
	}

	if c.RedisMaxRetries < 0 {
		return errors.New("redis max retries must not be negative")
	}

	return nil
}
