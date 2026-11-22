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

export ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-}}"
if [[ -z "$ANDROID_HOME" ]]; then
  echo "ANDROID_HOME or ANDROID_SDK_ROOT must point to the Android SDK" >&2
  exit 1
fi

check_generated() {
  local project="$1"
  local package_path="$2"
  test -f "$project/settings.gradle.kts"
  test -f "$project/build-logic/convention/build.gradle.kts"
  test -f "$project/app/build.gradle.kts"
  test -f "$project/core/data/src/main/kotlin/$package_path/core/data/SessionManager.kt"
  test -f "$project/feature/auth/src/main/kotlin/$package_path/feature/auth/AuthViewModel.kt"
  test -f "$project/feature/auth/src/main/res/values/strings.xml"
  test -f "$project/AGENTS.md"
  test -f "$project/CLAUDE.md"
  test -f "$project/CONTEXT.md"
  test -f "$project/skills-lock.json"
  test -f "$project/docs/ARCHITECTURE.md"
  grep -q 'android-clean-architecture' "$project/AGENTS.md"
  grep -q 'mobile-android-design' "$project/AGENTS.md"
}

run_checks() {
  local project="$1"
  local gradle="${GRADLE_BIN:-./gradlew}"
  echo "  gradle test/lint/assemble"
  (cd "$project" && "$gradle" --no-daemon test lint assembleDebug)
}

echo "== android/compose standalone"
target="$TMP/android"
"$CLI_BIN" new "$target" \
  --registry "$ROOT" \
  --kind mobile \
  --stack android/compose \
  --name "Verify Android" \
  --var application_id=com.example.verify \
  --var package_name=com.example.verify \
  --var api_base_url=http://10.0.2.2:8080 \
  --no-input --no-git
check_generated "$target" com/example/verify
grep -q 'com.example.verify' "$target/app/build.gradle.kts"
run_checks "$target"

echo "== fullstack + android/compose"
fullstack="$TMP/fullstack"
"$CLI_BIN" new "$fullstack" \
  --registry "$ROOT" \
  --kind fullstack \
  --backend go/gin-layered \
  --mobile android/compose \
  --name "Verify Fullstack Android" \
  --preset fullstack-api \
  --var module_path=example.com/fullstack \
  --var package_manager=pnpm \
  --var application_id=com.example.fullstack \
  --var package_name=com.example.fullstack \
  --var api_base_url=http://10.0.2.2:8080 \
  --no-input --no-git
check_generated "$fullstack/apps/android" com/example/fullstack
test -f "$fullstack/.github/workflows/ci.yml"
grep -q 'mobile-build' "$fullstack/Makefile"
grep -q 'apps/android' "$fullstack/CONTEXT-MAP.md"
run_checks "$fullstack/apps/android"

echo "Android template checks passed."
