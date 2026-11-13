# syntax=docker/dockerfile:1

FROM golang:1.26-alpine AS build

WORKDIR /src

COPY go.mod ./
RUN go mod download

COPY . .
RUN go mod tidy

ARG VERSION=dev
ARG COMMIT=none
ARG BUILD_DATE=unknown

RUN CGO_ENABLED=0 go build \
    -trimpath \
    -ldflags "-s -w -X $(go list -m)/internal/version.version=${VERSION} -X $(go list -m)/internal/version.commit=${COMMIT} -X $(go list -m)/internal/version.date=${BUILD_DATE}" \
    -o /out/api ./cmd/api

FROM alpine:3.22

RUN apk add --no-cache ca-certificates wget \
    && addgroup -S app \
    && adduser -S -G app app

COPY --from=build /out/api /usr/local/bin/api

USER app
EXPOSE 8080
ENV HTTP_PORT=8080

HEALTHCHECK --interval=10s --timeout=3s --start-period=10s --retries=3 \
    CMD wget -qO- "http://127.0.0.1:${HTTP_PORT}/healthz" >/dev/null || exit 1

ENTRYPOINT ["/usr/local/bin/api"]
