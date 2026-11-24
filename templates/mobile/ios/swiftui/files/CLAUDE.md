# CLAUDE.md — generated iOS app

Read [AGENTS.md](AGENTS.md), [CONTEXT.md](CONTEXT.md), and
[DESIGN.md](DESIGN.md) before changing code.

## Commands

```bash
make project
make test
make build
make ci
```

## Architecture

- `App` owns `@main`, the container, session gate, coordinator, and tab shell.
- `Core` owns pure models, errors, protocols, and theme preference.
- `Networking` owns URLSession APIs, DTOs, and problem details.
- `Storage` owns Keychain tokens.
- `Data` owns repositories and session policy.
- `DesignSystem` owns Smoke theme and shared components.
- `*Feature` packages own views and ViewModels.

## Boundaries

- Never import SwiftUI or UIKit in Core/Networking/Storage/Data.
- Never let a feature depend on another feature.
- Never store tokens in UserDefaults or expose them to views.
- Never hardcode animation or navigation workarounds that fight SwiftUI identity.
- Use `NavigationStack` and `navigationDestination` for navigation.
- Use `@Observable`, `@MainActor`, and `@State`/`@Bindable` correctly.
- Keep UI strings in package Localizable.strings resources.
- Invoke `.agents/skills/write-swift`, `.agents/skills/swiftui-pro`, and
  `.agents/skills/swiftui-expert-skill` before relevant changes.
