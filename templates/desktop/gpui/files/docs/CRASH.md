# Crash handling

The desktop entry point installs a panic hook that writes a local report to
`<temp dir>/<crate_name>-panic.log` and then runs Rust's default hook, so the
panic message and backtrace hint still appear in the terminal during
`make run` and `make dev`. Sentry is not initialized without an explicit DSN and
opt-in release configuration.

Diagnostics shown in About must redact tokens, passwords, authorization headers,
and other secret fields. Never attach SQLite files, keyring values, or refresh
tokens to a report.
