// Package version exposes build metadata injected with -ldflags.
package version

import "fmt"

var (
	version = "dev"
	commit  = "none"
	date    = "unknown"
)

// String returns a human-readable build identifier.
func String() string {
	return fmt.Sprintf("%s (%s, %s)", version, commit, date)
}
