#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CLI_BIN="${SMOKEFORGE_BIN:-}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if [[ -z "$CLI_BIN" || ! -x "$CLI_BIN" ]]; then
  echo "SMOKEFORGE_BIN must point to an executable smokeforge binary" >&2
  exit 1
fi

stacks=(
  go/gin-flat
  go/gin-layered
  go/gin-hexagonal
  go/gin-ddd
)

run_generated_checks() {
  local project="$1"
  echo "  tidy/test/vet/build"
  (cd "$project" && GOWORK=off go mod tidy && GOWORK=off go test ./... && GOWORK=off go vet ./... && GOWORK=off go build ./...)
  assert_auth_boundaries "$project"
}

assert_auth_boundaries() {
  local project="$1"

  if [[ ! -d "$project/internal/auth" ]]; then
    return
  fi

  local imports
  imports="$(cd "$project" && GOWORK=off go list -f '{{join .Imports "\n"}}' ./internal/auth)"

  if printf '%s\n' "$imports" | grep -Eq '^(net/http|net/smtp|github.com/gin-gonic/gin|github.com/markbates/goth|github.com/gorilla/sessions|github.com/jackc/pgx)'; then
    echo "  auth core imports transport or infrastructure" >&2
    return 1
  fi
}

for stack in "${stacks[@]}"; do
  slug="${stack//\//-}"
  target="$TMP/$slug-minimal"
  echo "== $stack / api-custom"
  "$CLI_BIN" new "$target" \
    --registry "$ROOT" \
    --kind backend \
    --stack "$stack" \
    --name "Verify $stack base" \
    --preset api-custom \
    --var "module_path=example.com/verify/$slug/minimal" \
    --var http_port=8080 \
    --no-input --no-git
  run_generated_checks "$target"
done

for stack in "${stacks[@]}"; do
  slug="${stack//\//-}"
  target="$TMP/$slug-full"
  echo "== $stack / api + TOTP + Docker + CI"
  "$CLI_BIN" new "$target" \
    --registry "$ROOT" \
    --kind backend \
    --stack "$stack" \
    --name "Verify $stack full" \
    --preset api \
    --feature totp \
    --feature docker \
    --feature ci:github \
    --var "module_path=example.com/verify/$slug/full" \
    --var http_port=8080 \
    --no-input --no-git
  run_generated_checks "$target"
  if command -v docker >/dev/null 2>&1; then
    (cd "$target" && docker compose -f compose.yaml -f compose.override.yaml config >/dev/null)
  fi
done

echo "== default backend preset"
target="$TMP/go-gin-layered-default"
"$CLI_BIN" new "$target" \
  --registry "$ROOT" \
  --kind backend \
  --stack go/gin-layered \
  --name "Verify default backend preset" \
  --var module_path=example.com/verify/default \
  --var http_port=8080 \
  --no-input --no-git
if [[ ! -d "$target/internal/database" || ! -d "$target/internal/auth" ]]; then
  echo "  default backend preset must include database and auth" >&2
  exit 1
fi
run_generated_checks "$target"

echo "== database-only overlay"
target="$TMP/go-gin-layered-db"
"$CLI_BIN" new "$target" \
  --registry "$ROOT" \
  --kind backend \
  --stack go/gin-layered \
  --name "Verify database only" \
  --preset api-custom \
  --feature database:postgres \
  --var module_path=example.com/verify/database \
  --var http_port=8080 \
  --no-input --no-git
run_generated_checks "$target"

echo "== generated code drift check"
target="$TMP/go-gin-layered-full"
{
  find "$target" -maxdepth 1 -type f -name '*.go' -print0
  find "$target/internal" "$target/cmd" -type f \( -name '*.go' -o -name '*.sql' -o -name '*.yaml' \) -print0
} | sort -z | xargs -0 sha256sum >"$TMP/before.sha"
(cd "$target" && make generate >/dev/null)
{
  find "$target" -maxdepth 1 -type f -name '*.go' -print0
  find "$target/internal" "$target/cmd" -type f \( -name '*.go' -o -name '*.sql' -o -name '*.yaml' \) -print0
} | sort -z | xargs -0 sha256sum >"$TMP/after.sha"
diff -u "$TMP/before.sha" "$TMP/after.sha"

echo "All Go template checks passed."
