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
  echo "  install/test/vet/build"
  if ! grep -qx 'APP_GIN_MODE=auto' "$project/.env"; then
    echo "  generated .env must set APP_GIN_MODE, not GIN_MODE" >&2
    return 1
  fi
  if grep -q '^GIN_MODE=' "$project/.env"; then
    echo "  generated .env must not export GIN_MODE to Gin" >&2
    return 1
  fi
  for doc in docs/README.md docs/ARCHITECTURE.md docs/API.md; do
    if [[ ! -f "$project/$doc" ]]; then
      echo "  generated docs are missing: $doc" >&2
      return 1
    fi
  done
  python3 - "$project" <<'PY'
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
missing = []
for document in [root / "README.md", *sorted((root / "docs").glob("*.md"))]:
    text = re.sub(r"```.*?```", "", document.read_text(), flags=re.S)
    text = re.sub(r"`[^`]*`", "", text)
    for target in re.findall(r"\]\(([^)]+)\)", text):
        if "://" in target or target.startswith("#") or " " in target:
            continue
        path = target.split("#", 1)[0]
        if path and not (document.parent / path).resolve().exists():
            missing.append(f"{document.relative_to(root)} -> {target}")
if missing:
    print("  broken documentation links:", file=sys.stderr)
    for item in missing:
        print(f"    {item}", file=sys.stderr)
    sys.exit(1)
PY
  (cd "$project" && GOWORK=off make install && GOWORK=off go test ./... && GOWORK=off go vet ./... && GOWORK=off go build ./...)
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
    --feature cache:redis \
    --feature queue:rabbitmq \
    --feature docker \
    --feature ci:github \
    --var "module_path=example.com/verify/$slug/full" \
    --var http_port=8080 \
    --no-input --no-git
  run_generated_checks "$target"
  if command -v docker >/dev/null 2>&1; then
    compose_files=(-f compose.yaml)
    if [[ -f "$target/compose.override.yaml" ]]; then
      compose_files+=(-f compose.override.yaml)
    fi
    for override in "$target"/compose.override.*.yaml; do
      if [[ -e "$override" ]]; then
        compose_files+=(-f "$(basename "$override")")
      fi
    done
    services="$(cd "$target" && docker compose "${compose_files[@]}" config --services)"
    for service in api db migrate redis rabbitmq worker scheduler; do
      if ! grep -qx "$service" <<<"$services"; then
        echo "  compose stack is missing service: $service" >&2
        exit 1
      fi
    done
    (cd "$target" && docker compose "${compose_files[@]}" config >/dev/null)
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

echo "== cache-only overlay"
target="$TMP/go-gin-layered-cache"
"$CLI_BIN" new "$target" \
  --registry "$ROOT" \
  --kind backend \
  --stack go/gin-layered \
  --name "Verify cache only" \
  --preset api-custom \
  --feature cache:redis \
  --var module_path=example.com/verify/cache \
  --var http_port=8080 \
  --no-input --no-git
run_generated_checks "$target"
if [[ ! -f "$target/docs/CACHE.md" ]]; then
  echo "  cache documentation is missing" >&2
  exit 1
fi
if command -v docker >/dev/null 2>&1; then
  (cd "$target" && docker compose -f compose.cache.yaml config >/dev/null)
fi

echo "== queue-only overlay"
target="$TMP/go-gin-layered-queue"
"$CLI_BIN" new "$target" \
  --registry "$ROOT" \
  --kind backend \
  --stack go/gin-layered \
  --name "Verify queue only" \
  --preset api-custom \
  --feature queue:rabbitmq \
  --var module_path=example.com/verify/queue \
  --var http_port=8080 \
  --no-input --no-git
run_generated_checks "$target"
if [[ ! -f "$target/docs/QUEUE.md" || ! -f "$target/cmd/worker/main.go" || ! -f "$target/cmd/scheduler/main.go" ]]; then
  echo "  queue worker/scheduler files are missing" >&2
  exit 1
fi
if command -v docker >/dev/null 2>&1; then
  (cd "$target" && docker compose -f compose.queue.yaml config >/dev/null)
fi

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
