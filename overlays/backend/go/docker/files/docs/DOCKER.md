# Docker

The Docker overlay adds a multi-stage Go build, a non-root Alpine runtime, a
health check, and Compose configuration.

## Build and run

```bash
make install
make docker-build
make docker-up
```

The API listens on <http://localhost:8080>. Runtime configuration comes from
`.env`; `HTTP_PORT` inside the container is fixed at `8080`.

The runtime image includes only the compiled binary, CA certificates, `wget`
for the health check, and a non-root `app` user.
