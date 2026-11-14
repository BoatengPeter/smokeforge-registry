# Docker

The Docker overlay adds a multi-stage Go build, a non-root Alpine runtime, a
health check, and Compose configuration.

## Build and run

```bash
make install
make docker-up
```

The full stack listens on <http://localhost:8080>. Runtime configuration comes
from `.env`; `HTTP_PORT` inside the container is fixed at `8080`.

When PostgreSQL is included, Compose also runs a one-shot `migrate` service
before starting the API. The API waits for migrations to complete, so the full
stack starts with the schema ready.

## Run only PostgreSQL in Docker

```bash
make db-up
make db-migrate
make run
```

This keeps the API on your host while PostgreSQL runs in Docker. The generated
`DATABASE_URL` points at `localhost` and the published `POSTGRES_PORT`.

The runtime image includes the compiled API binary, CA certificates, `wget`
for the health check, and a non-root `app` user. When the database overlay is
selected, it also contains the `migrate` binary used by the one-shot service.
