# Testing

## Quality gate

Run on macOS:

```bash
make project
make test
make build
```

`make test` runs every Swift package's tests. `make build` compiles the app for
the iOS Simulator without signing.

## Base coverage

- Core: theme persistence.
- Data: session state transitions.
- AuthFeature: validation, login failure, registration flow.
- TestSupport provides fakes; avoid network calls in unit tests.

## Rules

- Tests are deterministic and do not depend on live services.
- Prefer Swift Testing (`@Test`, `#expect`) for unit tests.
- Use XCUITest only when a flow cannot be tested below the UI.
- Snapshot tests, performance tests, and UI suites are follow-up overlays.
