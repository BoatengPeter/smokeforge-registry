package scheduler

import "context"

// LeaderElector runs the scheduler only while this process is the leader.
type LeaderElector interface {
	Run(ctx context.Context, leader func(context.Context) error) error
}

// AlwaysLeader is the single-instance scheduler implementation.
type AlwaysLeader struct{}

// Run executes the scheduler immediately.
func (AlwaysLeader) Run(ctx context.Context, leader func(context.Context) error) error {
	return leader(ctx)
}
