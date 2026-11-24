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

check_root() {
  local project="$1"

  for file in AGENTS.md CLAUDE.md CONTEXT.md CONTEXT-MAP.md README.md Makefile skills-lock.json docs/CLIENTS.md docs/ENVIRONMENTS.md docs/INTEGRATION.md docs/SKILLS.md docs/TESTING.md; do
    test -f "$project/$file"
  done

  for skill in workspace-orchestration api-contract cross-client-testing; do
    test -f "$project/.agents/skills/$skill/SKILL.md"
    test -f "$project/.claude/skills/$skill/SKILL.md"
  done

  grep -q 'workspace-orchestration' "$project/AGENTS.md"
  grep -q 'api-contract' "$project/AGENTS.md"
  grep -q 'cross-client-testing' "$project/AGENTS.md"
  python3 - "$project/skills-lock.json" <<'PY'
import json
import sys
json.load(open(sys.argv[1], encoding="utf-8"))
PY
}

check_absent() {
  local project="$1"
  shift

  for app in "$@"; do
    test ! -d "$project/apps/$app"
  done
}

check_make_target() {
  local project="$1"
  shift

  (cd "$project" && make -n ci >/dev/null)
  for target in "$@"; do
    grep -q "^$target:" "$project/Makefile"
  done
}

echo "== workspace: API + web + desktop + Android + iOS"
all="$TMP/all"
"$CLI_BIN" new "$all" \
  --registry "$ROOT" \
  --kind workspace \
  --backend go/gin-layered \
  --frontend next \
  --desktop electron \
  --mobile android/compose \
  --mobile ios/swiftui \
  --name "Verify Workspace All" \
  --var module_path=example.com/workspace \
  --var package_manager=pnpm \
  --var application_id=com.example.workspace \
  --var package_name=com.example.workspace \
  --var api_base_url=http://10.0.2.2:8080 \
  --var bundle_id=com.example.workspace \
  --var ios_api_base_url=http://127.0.0.1:8080 \
  --var api_url=http://localhost:8080 \
  --no-input --no-git
check_root "$all"
for app in api web desktop android ios; do
  test -d "$all/apps/$app"
done
test -f "$all/.github/workflows/ci.yml"
grep -q 'apps/api/AGENTS.md' "$all/AGENTS.md"
grep -q 'apps/web/AGENTS.md' "$all/AGENTS.md"
grep -q 'apps/desktop/AGENTS.md' "$all/AGENTS.md"
check_make_target "$all" api-dev web-dev desktop-dev mobile-build

echo "== workspace: API + egui, no web"
desktop="$TMP/desktop"
"$CLI_BIN" new "$desktop" \
  --registry "$ROOT" \
  --kind workspace \
  --backend rust/axum \
  --desktop egui \
  --name "Verify Workspace Desktop" \
  --var crate_name=verify_workspace \
  --var http_port=8080 \
  --var package_manager=pnpm \
  --var bundle_id=com.example.workspace \
  --var api_url=http://localhost:8080 \
  --no-input --no-git
check_root "$desktop"
check_absent "$desktop" web android ios
grep -q 'rust-desktop:' "$desktop/.github/workflows/ci.yml"
! grep -q 'web:' "$desktop/.github/workflows/ci.yml"
grep -q 'desktop-dev:' "$desktop/Makefile"
! grep -q '^web-dev:' "$desktop/Makefile"
check_make_target "$desktop" api-dev desktop-dev

echo "== workspace: client only, web + iOS"
client="$TMP/client"
"$CLI_BIN" new "$client" \
  --registry "$ROOT" \
  --kind workspace \
  --frontend next \
  --mobile ios/swiftui \
  --feature ci:github \
  --name "Verify Workspace Client" \
  --var package_manager=pnpm \
  --var bundle_id=com.example.workspace \
  --var ios_api_base_url=http://127.0.0.1:8080 \
  --no-input --no-git
check_root "$client"
check_absent "$client" api desktop android
test -f "$client/.github/workflows/ci.yml"
! grep -q 'apps/api/AGENTS.md' "$client/AGENTS.md"
! grep -q 'api-dev:' "$client/Makefile"
grep -q 'web-dev:' "$client/Makefile"
grep -q 'mobile-build:' "$client/Makefile"
check_make_target "$client" web-dev mobile-build

echo "== workspace: API only"
api="$TMP/api"
"$CLI_BIN" new "$api" \
  --registry "$ROOT" \
  --kind workspace \
  --backend go/gin-layered \
  --name "Verify Workspace API" \
  --var module_path=example.com/workspace \
  --var package_manager=pnpm \
  --no-input --no-git
check_root "$api"
check_absent "$api" web desktop android ios
grep -q 'api-dev:' "$api/Makefile"
check_make_target "$api" api-dev

echo "Workspace template checks passed."
