# Crash handling

The desktop entry point installs a panic hook that writes a local report to the
temporary directory. Sentry is not initialized without an explicit DSN and
opt-in release configuration.

Diagnostics shown in About must redact tokens, passwords, authorization headers,
and other secret fields. Never attach SQLite files, keyring values, or refresh
tokens to a report.
