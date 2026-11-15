# Database

PostgreSQL uses SQLx with a bounded pool and embedded migrations.

```bash
make db-up
make db-migrate
make db-down
```

`make db-migrate` runs `cargo run --bin migrate`. The API does not migrate on
startup. `Database` implements the readiness check used by `/readyz`.
