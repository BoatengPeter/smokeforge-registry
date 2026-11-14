package config

import (
	"testing"
	"time"
)

func queueTestConfig() Config {
	return Config{
		RabbitMQURL:             "amqp://localhost:5672/",
		RabbitMQExchange:        "smokeforge.jobs",
		RabbitMQQueue:           "smokeforge.jobs",
		RabbitMQDLQ:             "smokeforge.jobs.dlq",
		RabbitMQPrefetch:        10,
		RabbitMQMaxRetries:      3,
		RabbitMQMaxMessageBytes: 262144,
		RabbitMQConnectTimeout:  time.Second,
		RabbitMQConnectRetries:  2,
		RabbitMQConnectBackoff:  time.Second,
		RabbitMQHeartbeat:       30 * time.Second,
		RabbitMQPublishTimeout:  5 * time.Second,
		WorkerConcurrency:       4,
		SchedulerHeartbeat:      "@every 1m",
	}
}

func TestValidateQueue(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name      string
		mutate    func(*Config)
		wantError bool
	}{
		{name: "valid"},
		{name: "bad scheme", mutate: func(config *Config) { config.RabbitMQURL = "http://localhost:5672" }, wantError: true},
		{name: "missing host", mutate: func(config *Config) { config.RabbitMQURL = "amqp:///vhost" }, wantError: true},
		{name: "empty exchange", mutate: func(config *Config) { config.RabbitMQExchange = "" }, wantError: true},
		{name: "empty queue", mutate: func(config *Config) { config.RabbitMQQueue = "" }, wantError: true},
		{name: "same queue and dlq", mutate: func(config *Config) { config.RabbitMQDLQ = config.RabbitMQQueue }, wantError: true},
		{name: "zero prefetch", mutate: func(config *Config) { config.RabbitMQPrefetch = 0 }, wantError: true},
		{name: "negative retries", mutate: func(config *Config) { config.RabbitMQMaxRetries = -1 }, wantError: true},
		{name: "zero max bytes", mutate: func(config *Config) { config.RabbitMQMaxMessageBytes = 0 }, wantError: true},
		{name: "zero connect timeout", mutate: func(config *Config) { config.RabbitMQConnectTimeout = 0 }, wantError: true},
		{name: "negative connect retries", mutate: func(config *Config) { config.RabbitMQConnectRetries = -1 }, wantError: true},
		{name: "zero backoff", mutate: func(config *Config) { config.RabbitMQConnectBackoff = 0 }, wantError: true},
		{name: "zero heartbeat", mutate: func(config *Config) { config.RabbitMQHeartbeat = 0 }, wantError: true},
		{name: "zero publish timeout", mutate: func(config *Config) { config.RabbitMQPublishTimeout = 0 }, wantError: true},
		{name: "zero concurrency", mutate: func(config *Config) { config.WorkerConcurrency = 0 }, wantError: true},
		{name: "empty schedule", mutate: func(config *Config) { config.SchedulerHeartbeat = "" }, wantError: true},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			config := queueTestConfig()
			if test.mutate != nil {
				test.mutate(&config)
			}

			err := config.validateQueue()
			if test.wantError && err == nil {
				t.Fatal("expected error")
			}

			if !test.wantError && err != nil {
				t.Fatalf("validate queue: %v", err)
			}
		})
	}
}
