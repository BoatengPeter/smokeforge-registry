# Project context

## Architecture

The app is a single SwiftUI app target with local Swift packages.

```text
App -> Features, Data, DesignSystem
Features -> Core, DesignSystem
Data -> Networking, Storage, Core
Networking -> Core
Storage -> Core
Core -> (Foundation/Observation only)
```

Features never depend on each other. The app target owns the root coordinator
and session gate.

## Rules

- ViewModels are `@MainActor @Observable` classes.
- View-owned state uses `@State`; injected observables use `@Bindable`.
- Core, Networking, Storage, and Data do not import SwiftUI.
- Tokens are Keychain-only.
- `project.yml` is the source of truth; `.xcodeproj` is generated.
- Debug/Staging/Release are xcconfig-driven.
- No live network or service dependency in previews or tests.
- Use fakes from `TestSupport` for package tests.

## Quality gate

```bash
make project
make test
make build
```

Run these on macOS before finishing. Linux cannot verify SwiftUI or Xcode
project generation.
