package config

import (
	"testing"
	"time"
)

func cacheTestConfig() Config {
	return Config{
		RedisURL:          "redis://localhost:6379/0",
		RedisDialTimeout:  time.Second,
		RedisReadTimeout:  time.Second,
		RedisWriteTimeout: time.Second,
		RedisPoolSize:     4,
		RedisMinIdleConns: 1,
		RedisMaxRetries:   2,
	}
}

func TestValidateCache(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name      string
		mutate    func(*Config)
		wantError bool
	}{
		{name: "valid"},
		{name: "empty url", mutate: func(config *Config) { config.RedisURL = "" }, wantError: true},
		{name: "bad scheme", mutate: func(config *Config) { config.RedisURL = "http://localhost:6379" }, wantError: true},
		{name: "missing host", mutate: func(config *Config) { config.RedisURL = "redis:///0" }, wantError: true},
		{name: "zero dial timeout", mutate: func(config *Config) { config.RedisDialTimeout = 0 }, wantError: true},
		{name: "zero read timeout", mutate: func(config *Config) { config.RedisReadTimeout = 0 }, wantError: true},
		{name: "zero write timeout", mutate: func(config *Config) { config.RedisWriteTimeout = 0 }, wantError: true},
		{name: "empty pool", mutate: func(config *Config) { config.RedisPoolSize = 0 }, wantError: true},
		{name: "negative idle conns", mutate: func(config *Config) { config.RedisMinIdleConns = -1 }, wantError: true},
		{name: "idle conns exceed pool", mutate: func(config *Config) { config.RedisMinIdleConns = 5 }, wantError: true},
		{name: "negative retries", mutate: func(config *Config) { config.RedisMaxRetries = -1 }, wantError: true},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			config := cacheTestConfig()
			if test.mutate != nil {
				test.mutate(&config)
			}

			err := config.validateCache()
			if test.wantError && err == nil {
				t.Fatal("expected error")
			}

			if !test.wantError && err != nil {
				t.Fatalf("validate cache: %v", err)
			}
		})
	}
}
