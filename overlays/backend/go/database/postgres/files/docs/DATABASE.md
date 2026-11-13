# Database

The PostgreSQL overlay adds a bounded pgx pool, embedded goose migrations, and a
migration command. Auth adds the `users` schema on top.

## Start and migrate

```bash
make db-up
make install
make db-migrate
make run
```

`make db-up` starts PostgreSQL from `compose.db.yaml`. `make db-migrate` runs
the embedded goose migrations through `cmd/migrate`. Migrations are compiled into
the binary from `internal/database/migrator/migrations` with `//go:embed`.

When auth is selected, `make db-seed` creates an opt-in verified demo user from
`SEED_USER_EMAIL` and `SEED_USER_PASSWORD`.

The default local connection string is:

```text
postgres://postgres:postgres@localhost:5432/app?sslmode=disable
```

Change `DATABASE_URL` and the compose credentials together when using another
database.

## Schema changes

1. Add a new `NNNNN_name.sql` file under
   `internal/database/migrator/migrations`.
2. Add or update queries under the owning feature package.
3. Run `make generate` to regenerate sqlc code from `sqlc.yaml`.
4. Run `make db-migrate`.
5. Run `make ci`.

`sqlc.yaml` is generated at the project root when auth is selected.

## Readiness

The database implements the HTTP readiness contract, so `/readyz` returns 503
while PostgreSQL is unreachable.

## Pool and retries

The pool reads `DATABASE_MAX_CONNS`, `DATABASE_MIN_CONNS`,
`DATABASE_MAX_CONN_LIFETIME`, `DATABASE_MAX_CONN_IDLE_TIME`,
`DATABASE_HEALTH_CHECK_PERIOD`, `DATABASE_CONNECT_TIMEOUT`,
`DATABASE_CONNECT_RETRIES`, and `DATABASE_CONNECT_BACKOFF`.
