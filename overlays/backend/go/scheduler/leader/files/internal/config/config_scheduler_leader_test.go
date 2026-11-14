package config

import (
	"testing"
	"time"
)

func schedulerLeaderTestConfig() Config {
	return Config{
		SchedulerLeaseTTL:   30 * time.Second,
		SchedulerLeaseRenew: 10 * time.Second,
		SchedulerLeaseRetry: 5 * time.Second,
	}
}

func TestValidateSchedulerLeader(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name      string
		mutate    func(*Config)
		wantError bool
	}{
		{name: "valid"},
		{name: "zero ttl", mutate: func(config *Config) { config.SchedulerLeaseTTL = 0 }, wantError: true},
		{name: "zero renew", mutate: func(config *Config) { config.SchedulerLeaseRenew = 0 }, wantError: true},
		{name: "renew exceeds ttl", mutate: func(config *Config) { config.SchedulerLeaseRenew = 30 * time.Second }, wantError: true},
		{name: "zero retry", mutate: func(config *Config) { config.SchedulerLeaseRetry = 0 }, wantError: true},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			config := schedulerLeaderTestConfig()
			if test.mutate != nil {
				test.mutate(&config)
			}

			err := config.validateSchedulerLeader()
			if test.wantError && err == nil {
				t.Fatal("expected error")
			}

			if !test.wantError && err != nil {
				t.Fatalf("validate scheduler leader: %v", err)
			}
		})
	}
}
