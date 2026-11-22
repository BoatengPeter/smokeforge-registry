# AGENTS.md — generated Android app

This project was scaffolded by smokeforge. Read [CONTEXT.md](CONTEXT.md) before
changing architecture and [DESIGN.md](DESIGN.md) before visual work.

## Stack

- Kotlin, Jetpack Compose, Material 3, Navigation Compose
- Hilt + KSP, Retrofit/OkHttp, kotlinx.serialization
- DataStore + Android Keystore token storage
- Gradle Kotlin DSL, version catalog, build-logic conventions

## Layout

```text
app/                  Application, activity, root navigation, session gate
build-logic/          Android application/library/Compose/Hilt conventions
core/common/          Result, errors, dispatcher contracts
core/model/           Pure domain models and repository interfaces
core/designsystem/    Theme, tokens, shared components
core/network/         API interfaces, DTOs, RFC 9457 mapping
core/datastore/       Preferences and encrypted tokens
core/data/            Repository implementations, session policy, Hilt modules
core/testing/         Fakes and test data
feature/auth/         Sign-in, registration, password reset
feature/home/         API status and account
feature/settings/     Appearance, environment, sign-out
```

## Rules

- Features depend on `core:model` and `core:designsystem`; they never depend on
  another feature.
- `core:model` is pure Kotlin. Do not put Android framework types or Hilt
  annotations in domain models.
- ViewModels expose one immutable `UiState` through `StateFlow`; collect with
  lifecycle awareness.
- Repository implementations coordinate network and storage. UI never calls
  Retrofit, DataStore, or Keystore directly.
- Inject `DispatcherProvider`; never hardcode `Dispatchers.IO` in production code.
- Keep tokens in `SessionManager` and encrypted storage. Never log tokens,
  passwords, or authorization headers.
- Use `Navigation Compose`; keep cross-feature navigation in `:app`.
- Use strings resources and accessible labels. Keep touch targets at least 48dp.
- Read and invoke the project-scoped skills before changing this area:
  `.agents/skills/android-clean-architecture/SKILL.md`,
  `.agents/skills/kotlin-patterns/SKILL.md`,
  `.agents/skills/kotlin-testing/SKILL.md`, and
  `.agents/skills/mobile-android-design/SKILL.md`.
- Run `./gradlew test lint assembleDebug` before claiming completion.
