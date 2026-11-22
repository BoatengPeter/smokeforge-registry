# CLAUDE.md — generated Android app

Read [AGENTS.md](AGENTS.md), [CONTEXT.md](CONTEXT.md), and
[DESIGN.md](DESIGN.md) before changing code.

## Commands

```bash
./gradlew test
./gradlew lint
./gradlew assembleDebug
```

## Architecture

- `:app` owns the session gate, root navigation, and bottom navigation.
- `:core:model` contains pure domain models and repository interfaces.
- `:core:data` implements repositories and session policy.
- `:core:network` owns Retrofit APIs, DTOs, and RFC 9457 mapping.
- `:core:datastore` owns DataStore and Keystore-backed token encryption.
- `:core:designsystem` owns Material 3 theme and shared UI.
- `:feature:*` owns screen state, ViewModels, and Compose UI.

## Boundaries

- Never use Android framework types in `:core:model`.
- Never let a feature depend on another feature.
- Never expose DTOs or storage entities to the UI.
- Never log credentials or tokens.
- Never add secrets to the APK.
- Keep debug-only cleartext limited to local development.
- Prefer fakes over mocks in tests.

## Skills

Invoke `.agents/skills/android-clean-architecture` before changing module
boundaries, data flow, repositories, or use cases. Invoke
`.agents/skills/kotlin-patterns` before Kotlin implementation or refactoring.
Invoke `.agents/skills/kotlin-testing` before writing or changing tests. Invoke
`.agents/skills/mobile-android-design` before changing Compose layouts, theme,
navigation, accessibility, or adaptive behaviour.
