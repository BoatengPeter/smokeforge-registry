# AGENTS.md — generated iOS app

This project was scaffolded by smokeforge. Read [CONTEXT.md](CONTEXT.md) before
changing architecture and [DESIGN.md](DESIGN.md) before visual work.

## Stack

- Swift 6, SwiftUI, Observation, NavigationStack
- Local Swift packages: Core, Networking, Storage, Data, DesignSystem, TestSupport, AuthFeature, HomeFeature, SettingsFeature
- URLSession async/await and Keychain
- XcodeGen 2.48.0
- Swift Testing

## Rules

- Views render state and forward intent; business logic belongs in `@Observable`
  main-actor ViewModels.
- Features never depend on other features. Cross-feature navigation belongs in
  the app target.
- Core, Networking, Storage, and Data must not import SwiftUI or UIKit.
- Inject dependencies through initializers. Do not create live services inside
  views or ViewModels.
- Tokens live in Keychain and never in UserDefaults, logs, or UI state.
- Use immutable value types and typed state models by default.
- Avoid unstructured concurrency. Use `.task` for screen-scoped work and
  structured child tasks for parallel work.
- Use Dynamic Type, VoiceOver labels, and 44pt minimum touch targets.
- Use `@State` only for view-owned state; use `@Bindable` for injected
  observables.
- Keep `project.yml` as the source of truth; do not commit `.xcodeproj`.
- Read and invoke the project-scoped skills before changing relevant code:
  `.agents/skills/write-swift/SKILL.md`,
  `.agents/skills/swiftui-pro/SKILL.md`, and
  `.agents/skills/swiftui-expert-skill/SKILL.md`.
- Run `make ci` on macOS before claiming completion.
