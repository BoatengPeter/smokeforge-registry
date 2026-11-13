package config

import (
	"testing"
	"time"
)

func validConfig() Config {
	return Config{
		AppEnv:               "development",
		ServiceName:          "test-service",
		APITitle:             "Test API",
		HTTPPort:             8080,
		PublicBaseURL:        "http://localhost:8080",
		RequestTimeout:       30 * time.Second,
		ShutdownTimeout:      10 * time.Second,
		ReadHeaderTimeout:    5 * time.Second,
		ReadTimeout:          15 * time.Second,
		WriteTimeout:         30 * time.Second,
		IdleTimeout:          60 * time.Second,
		MaxBodyBytes:         1024,
		RateLimitRPS:         20,
		RateLimitBurst:       40,
		APIDocsEnabled:       "auto",
		LogLevel:             "info",
		GinMode:              "auto",
		CORSAllowedOrigins:   nil,
		CORSAllowedMethods:   []string{"GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"},
		CORSAllowedHeaders:   []string{"Authorization", "Content-Type", "X-Requested-With", "X-Refresh-Token-Transport"},
		CORSExposedHeaders:   []string{"X-Request-ID"},
		CORSAllowCredentials: true,
		CORSMaxAge:           12 * time.Hour,
		TrustedProxies:       nil,
		// smokeforge:anchor config-test-fields
	}
}

func TestConfigValidate(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name    string
		mutate  func(*Config)
		wantErr bool
	}{
		{
			name:   "valid",
			mutate: func(*Config) {},
		},
		{
			name:   "production docs disabled by default",
			mutate: func(config *Config) { config.AppEnv = "production"; config.APIDocsEnabled = "auto" },
		},
		{
			name:    "invalid environment",
			mutate:  func(config *Config) { config.AppEnv = "qa" },
			wantErr: true,
		},
		{
			name:    "invalid port",
			mutate:  func(config *Config) { config.HTTPPort = 0 },
			wantErr: true,
		},
		{
			name:    "wildcard cors",
			mutate:  func(config *Config) { config.CORSAllowedOrigins = []string{"*"} },
			wantErr: true,
		},
		{
			name:    "cors with path",
			mutate:  func(config *Config) { config.CORSAllowedOrigins = []string{"https://app.example.com/path"} },
			wantErr: true,
		},
		{
			name:    "rate limit without burst",
			mutate:  func(config *Config) { config.RateLimitRPS = 10; config.RateLimitBurst = 0 },
			wantErr: true,
		},
		{
			name:    "invalid docs mode",
			mutate:  func(config *Config) { config.APIDocsEnabled = "sometimes" },
			wantErr: true,
		},
		{
			name:    "invalid gin mode",
			mutate:  func(config *Config) { config.GinMode = "sometimes" },
			wantErr: true,
		},
		{
			name:   "wildcard cors without credentials",
			mutate: func(config *Config) { config.CORSAllowedOrigins = []string{"*"}; config.CORSAllowCredentials = false },
		},
		{
			name:    "wildcard cors with credentials",
			mutate:  func(config *Config) { config.CORSAllowedOrigins = []string{"*"}; config.CORSAllowCredentials = true },
			wantErr: true,
		},
		{
			name:    "invalid cors method",
			mutate:  func(config *Config) { config.CORSAllowedMethods = []string{"FETCH"} },
			wantErr: true,
		},
		{
			name:    "empty cors header",
			mutate:  func(config *Config) { config.CORSAllowedHeaders = []string{""} },
			wantErr: true,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			config := validConfig()
			test.mutate(&config)

			err := config.Validate()
			if test.wantErr && err == nil {
				t.Fatal("expected error")
			}

			if !test.wantErr && err != nil {
				t.Fatalf("validate: %v", err)
			}
		})
	}
}

func TestConfigHTTPMode(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name     string
		env      string
		mode     string
		expected string
	}{
		{name: "auto development", env: "development", mode: "auto", expected: "debug"},
		{name: "auto staging", env: "staging", mode: "auto", expected: "release"},
		{name: "auto production", env: "production", mode: "auto", expected: "release"},
		{name: "explicit test", env: "production", mode: "test", expected: "test"},
		{name: "explicit debug", env: "production", mode: "debug", expected: "debug"},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			config := validConfig()
			config.AppEnv = test.env
			config.GinMode = test.mode

			if got := config.HTTPMode(); got != test.expected {
				t.Fatalf("HTTPMode() = %q, want %q", got, test.expected)
			}
		})
	}
}

func TestConfigDocsEnabled(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name     string
		env      string
		docs     string
		expected bool
	}{
		{name: "auto development", env: "development", docs: "auto", expected: true},
		{name: "auto production", env: "production", docs: "auto", expected: false},
		{name: "explicit true", env: "production", docs: "true", expected: true},
		{name: "explicit false", env: "development", docs: "false", expected: false},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			config := validConfig()
			config.AppEnv = test.env
			config.APIDocsEnabled = test.docs

			if got := config.DocsEnabled(); got != test.expected {
				t.Fatalf("DocsEnabled() = %v, want %v", got, test.expected)
			}
		})
	}
}
