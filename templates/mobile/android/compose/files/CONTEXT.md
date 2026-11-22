# Project context

## Architecture

The app is a single-activity Compose application with unidirectional data flow.

```text
app
  -> feature/*
  -> core:data (Hilt aggregation)
  -> core:designsystem

feature/*
  -> core:model
  -> core:designsystem
  -> core:common

core:data
  -> core:network
  -> core:datastore
  -> core:model
  -> core:common

core:network
  -> core:model
  -> core:common

core:datastore
  -> core:model
  -> core:common

core:model
  -> core:common
```

Features do not depend on each other. `:app` translates navigation callbacks
between features and the root `NavHost`.

## Boundaries

- Domain types are data classes with no Android imports.
- Repository interfaces live in `:core:model`; implementations live in
  `:core:data`.
- DTOs and problem details stay in `:core:network`.
- Token encryption and preferences stay in `:core:datastore`.
- ViewModels never receive `Context`, `Activity`, or `Resources`.
- UI state is immutable; events flow upward; effects are explicit.
- `SessionManager` is the only owner of in-memory access and refresh tokens.
- `TokenAuthenticator` refreshes once on `401`; a failed refresh clears the
  session and returns the app to signed-out state.

## Quality gate

```bash
./gradlew test
./gradlew lint
./gradlew assembleDebug
```

No feature is complete without a test for the happy path and at least one
failure path, plus fresh verification output.
