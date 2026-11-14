package config

import "errors"

func (c Config) validateSchedulerLeader() error {
	if c.SchedulerLeaseTTL <= 0 {
		return errors.New("scheduler lease ttl must be positive")
	}

	if c.SchedulerLeaseRenew <= 0 {
		return errors.New("scheduler lease renew must be positive")
	}

	if c.SchedulerLeaseRenew >= c.SchedulerLeaseTTL {
		return errors.New("scheduler lease renew must be shorter than the ttl")
	}

	if c.SchedulerLeaseRetry <= 0 {
		return errors.New("scheduler lease retry must be positive")
	}

	return nil
}
