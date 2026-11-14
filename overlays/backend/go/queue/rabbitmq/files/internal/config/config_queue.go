package config

import (
	"errors"
	"fmt"
	"net/url"
	"strings"
)

func (c Config) validateQueue() error {
	if err := c.validateQueueURL(); err != nil {
		return err
	}

	if err := c.validateQueueNames(); err != nil {
		return err
	}

	if err := c.validateQueueLimits(); err != nil {
		return err
	}

	return c.validateQueueTimeouts()
}

func (c Config) validateQueueURL() error {
	parsed, err := url.Parse(strings.TrimSpace(c.RabbitMQURL))
	if err != nil {
		return fmt.Errorf("parse rabbitmq url: %w", err)
	}

	if parsed.Scheme != "amqp" && parsed.Scheme != "amqps" {
		return fmt.Errorf("rabbitmq url scheme %q must be amqp or amqps", parsed.Scheme)
	}

	if parsed.Host == "" {
		return errors.New("rabbitmq url must include a host")
	}

	return nil
}

func (c Config) validateQueueNames() error {
	if strings.TrimSpace(c.RabbitMQExchange) == "" {
		return errors.New("rabbitmq exchange must not be empty")
	}

	if strings.TrimSpace(c.RabbitMQQueue) == "" {
		return errors.New("rabbitmq queue must not be empty")
	}

	if strings.TrimSpace(c.RabbitMQDLQ) == "" {
		return errors.New("rabbitmq dlq must not be empty")
	}

	if c.RabbitMQQueue == c.RabbitMQDLQ {
		return errors.New("rabbitmq queue and dlq must be different")
	}

	return nil
}

func (c Config) validateQueueLimits() error {
	if c.RabbitMQPrefetch < 1 {
		return errors.New("rabbitmq prefetch must be at least 1")
	}

	if c.RabbitMQMaxRetries < 0 {
		return errors.New("rabbitmq max retries must not be negative")
	}

	if c.RabbitMQMaxMessageBytes < 1 {
		return errors.New("rabbitmq max message bytes must be positive")
	}

	if c.WorkerConcurrency < 1 {
		return errors.New("worker concurrency must be at least 1")
	}

	return nil
}

func (c Config) validateQueueTimeouts() error {
	if c.RabbitMQConnectTimeout <= 0 {
		return errors.New("rabbitmq connect timeout must be positive")
	}

	if c.RabbitMQConnectRetries < 0 {
		return errors.New("rabbitmq connect retries must not be negative")
	}

	if c.RabbitMQConnectBackoff <= 0 {
		return errors.New("rabbitmq connect backoff must be positive")
	}

	if c.RabbitMQHeartbeat <= 0 {
		return errors.New("rabbitmq heartbeat must be positive")
	}

	if c.RabbitMQPublishTimeout <= 0 {
		return errors.New("rabbitmq publish timeout must be positive")
	}

	if strings.TrimSpace(c.SchedulerHeartbeat) == "" {
		return errors.New("scheduler heartbeat must not be empty")
	}

	return nil
}
