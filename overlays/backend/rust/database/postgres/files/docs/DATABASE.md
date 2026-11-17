# Database

PostgreSQL uses SQLx with a bounded pool and embedded migrations.

```bash
make db-up
make db-migrate
make db-down
```

`make db-migrate` runs `cargo run --bin migrate`. The API does not migrate on
startup. `Database` implements the readiness check used by `/readyz`.

Queries use SQLx compile-time macros. The checked offline metadata lives in
`.sqlx/`, so `cargo build` never needs a live database. Regenerate it after
changing a query or migration:

```bash
cargo sqlx prepare
```

Database tests run when `TEST_DATABASE_URL` is set; otherwise they skip:

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/app make test
```
