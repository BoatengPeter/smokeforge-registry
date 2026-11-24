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

for command in xcodegen xcodebuild swift; do
  if ! command -v "$command" >/dev/null 2>&1; then
    echo "$command is required on macOS" >&2
    exit 1
  fi
done

check_generated() {
  local project="$1"
  test -f "$project/project.yml"
  test -f "$project/App/SmokeApp.swift"
  test -f "$project/App/Info.plist"
  test -f "$project/App/PrivacyInfo.xcprivacy"
  test -f "$project/App/Resources/en.lproj/Localizable.strings"
  test -f "$project/Packages/Core/Package.swift"
  test -f "$project/Packages/Networking/Package.swift"
  test -f "$project/Packages/Storage/Package.swift"
  test -f "$project/Packages/Data/Package.swift"
  test -f "$project/Packages/DesignSystem/Package.swift"
  test -f "$project/Packages/HomeFeature/Package.swift"
  test -f "$project/Packages/SettingsFeature/Package.swift"
  test -f "$project/Packages/TestSupport/Package.swift"
  test -f "$project/Packages/AuthFeature/Package.swift"
  test -f "$project/AGENTS.md"
  test -f "$project/CLAUDE.md"
  test -f "$project/CONTEXT.md"
  test -f "$project/skills-lock.json"
  test -f "$project/docs/ARCHITECTURE.md"
  grep -q 'write-swift' "$project/AGENTS.md"
  grep -q 'swiftui-pro' "$project/AGENTS.md"
  grep -q 'swiftui-expert-skill' "$project/AGENTS.md"
}

run_checks() {
  local project="$1"
  echo "  xcodegen/swift tests/simulator build"
  (cd "$project" && make project && make test && make build)
}

echo "== ios/swiftui standalone"
target="$TMP/ios"
"$CLI_BIN" new "$target" \
  --registry "$ROOT" \
  --kind mobile \
  --stack ios/swiftui \
  --name "Verify iOS" \
  --var bundle_id=com.example.verify.ios \
  --var ios_api_base_url=http://127.0.0.1:8080 \
  --no-input --no-git
check_generated "$target"
run_checks "$target"

echo "== fullstack + android + ios, no web"
fullstack="$TMP/fullstack"
"$CLI_BIN" new "$fullstack" \
  --registry "$ROOT" \
  --kind fullstack \
  --backend go/gin-layered \
  --mobile android/compose \
  --mobile ios/swiftui \
  --name "Verify Mobile Fullstack" \
  --preset mobile-api \
  --var module_path=example.com/mobilefullstack \
  --var package_manager=pnpm \
  --var application_id=com.example.android \
  --var package_name=com.example.android \
  --var api_base_url=http://10.0.2.2:8080 \
  --var bundle_id=com.example.ios \
  --var ios_api_base_url=http://127.0.0.1:8080 \
  --no-input --no-git
test -f "$fullstack/apps/api/go.mod"
test -f "$fullstack/apps/android/settings.gradle.kts"
test -f "$fullstack/apps/ios/project.yml"
test ! -d "$fullstack/apps/web"
grep -q 'mobile-build' "$fullstack/Makefile"
grep -q 'apps/ios' "$fullstack/CONTEXT-MAP.md"
check_generated "$fullstack/apps/ios"
run_checks "$fullstack/apps/ios"

echo "iOS template checks passed."
