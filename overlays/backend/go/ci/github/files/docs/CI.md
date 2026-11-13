# CI

The GitHub Actions overlay adds a workflow that runs:

1. `go mod download`
2. `gofmt` check
3. `go vet`
4. `golangci-lint`
5. Unit tests
6. Race tests
7. Build
8. `govulncheck`

No database service is required for the generated unit tests. Add a Postgres
service and `TEST_DATABASE_URL` when you add integration-tagged database tests.
