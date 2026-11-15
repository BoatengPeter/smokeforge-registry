# Docker

`make docker-build` builds a multi-stage image with a Rust builder and a
minimal Debian runtime. `make docker-up` starts the Compose stack. The image
runs as a non-root user and exposes `/healthz` for its health check.
