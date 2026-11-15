#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CLI_BIN="${SMOKEFORGE_BIN:-}"
CARGO_BIN="${CARGO_BIN:-cargo}"
TMP="$(mktemp -d)"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$TMP/target}"
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-0}"
export CARGO_PROFILE_TEST_DEBUG="${CARGO_PROFILE_TEST_DEBUG:-0}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
trap 'rm -rf "$TMP"' EXIT

if [[ -z "$CLI_BIN" || ! -x "$CLI_BIN" ]]; then
  echo "SMOKEFORGE_BIN must point to an executable smokeforge binary" >&2
  exit 1
fi

if ! command -v "$CARGO_BIN" >/dev/null 2>&1; then
  echo "$CARGO_BIN is required" >&2
  exit 1
fi

generate() {
  local target="$1" preset="$2"
  shift 2
  "$CLI_BIN" new "$target" \
    --registry "$ROOT" \
    --kind backend \
    --stack rust/axum \
    --name "Verify Rust" \
    --preset "$preset" \
    --var crate_name=verify_app \
    --var http_port=8080 \
    --no-input --no-git "$@"
}

check() {
  local target="$1"
  echo "  fmt/clippy/test/build"
  (cd "$target" \
    && "$CARGO_BIN" fmt --check \
    && "$CARGO_BIN" clippy --all-targets --all-features -- -D warnings \
    && "$CARGO_BIN" test \
    && "$CARGO_BIN" build)
}

echo "== rust/axum base"
generate "$TMP/base" api-custom
check "$TMP/base"
for doc in docs/API.md docs/ARCHITECTURE.md; do
  test -f "$TMP/base/$doc"
done

grep -q '## Code layout' "$TMP/base/README.md"
grep -q 'response.rs' "$TMP/base/README.md"
grep -q 'src/response.rs' "$TMP/base/docs/ARCHITECTURE.md"
test -f "$TMP/base/src/response.rs"
test -f "$TMP/base/tests/response_test.rs"

echo "== rust/axum api preset with database/auth/docker/ci"
generate "$TMP/api" api --feature cache:redis --feature pagination --feature totp --feature search:meilisearch --feature search-sync:rabbitmq --feature media --feature media-provider:minio --feature queue:rabbitmq --feature scheduler:leader --feature dedupe:redis --feature compression:gzip --feature docker --feature ci:github
check "$TMP/api"
for file in migrations/0001_init.sql migrations/0002_auth.sql migrations/0003_totp.sql migrations/0004_media.sql Dockerfile compose.yaml compose.db.yaml compose.cache.yaml compose.queue.yaml .github/workflows/ci.yml docs/QUEUE.md docs/COMPRESSION.md docs/PAGINATION.md docs/CACHE.md docs/TOTP.md docs/SEARCH.md docs/SEARCH_SYNC.md docs/MEDIA.md docs/MEDIA_MINIO.md docs/DEDUPE.md docs/SCHEDULER.md; do
  test -f "$TMP/api/$file"
done
if command -v docker >/dev/null 2>&1; then
  (cd "$TMP/api" && docker compose -f compose.yaml -f compose.override.yaml config >/dev/null)
  (cd "$TMP/api" && docker compose -f compose.yaml -f compose.override.media.yaml -f compose.media.yaml config >/dev/null)
fi

echo "== rust skills"
for skill in rust-async-patterns rust-testing rust-patterns rust-best-practices-apollo rust-best-practices-beautifulrust; do
  test -f "$TMP/api/.agents/skills/$skill/SKILL.md"
  test -f "$TMP/api/.claude/skills/$skill/SKILL.md"
done

echo "All Rust template checks passed."
