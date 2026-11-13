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
}

for stack in "${stacks[@]}"; do
  slug="${stack//\//-}"
  target="$TMP/$slug-minimal"
  echo "== $stack / api-minimal"
  "$CLI_BIN" new "$target" \
    --registry "$ROOT" \
    --kind backend \
    --stack "$stack" \
    --name "Verify $stack minimal" \
    --preset api-minimal \
    --var "module_path=example.com/verify/$slug/minimal" \
    --var http_port=8080 \
    --no-input --no-git
  run_generated_checks "$target"
done

for stack in "${stacks[@]}"; do
  slug="${stack//\//-}"
  target="$TMP/$slug-production"
  echo "== $stack / api-production"
  "$CLI_BIN" new "$target" \
    --registry "$ROOT" \
    --kind backend \
    --stack "$stack" \
    --name "Verify $stack production" \
    --preset api-production \
    --var "module_path=example.com/verify/$slug/production" \
    --var http_port=8080 \
    --no-input --no-git
  run_generated_checks "$target"
  if command -v docker >/dev/null 2>&1; then
    (cd "$target" && docker compose -f compose.yaml -f compose.override.yaml config >/dev/null)
  fi
done

echo "== database-only overlay"
target="$TMP/go-gin-layered-db"
"$CLI_BIN" new "$target" \
  --registry "$ROOT" \
  --kind backend \
  --stack go/gin-layered \
  --name "Verify database only" \
  --preset api-minimal \
  --feature database:postgres \
  --var module_path=example.com/verify/database \
  --var http_port=8080 \
  --no-input --no-git
run_generated_checks "$target"

echo "== generated code drift check"
target="$TMP/go-gin-layered-production"
find "$target/internal" "$target/cmd" -type f \( -name '*.go' -o -name '*.sql' -o -name '*.yaml' \) -print0 \
  | sort -z \
  | xargs -0 sha256sum >"$TMP/before.sha"
(cd "$target" && make generate >/dev/null)
find "$target/internal" "$target/cmd" -type f \( -name '*.go' -o -name '*.sql' -o -name '*.yaml' \) -print0 \
  | sort -z \
  | xargs -0 sha256sum >"$TMP/after.sha"
diff -u "$TMP/before.sha" "$TMP/after.sha"

echo "All Go template checks passed."
