# Testing

## Quality gate

```bash
./gradlew test
./gradlew lint
./gradlew assembleDebug
```

## Coverage in the base

- `AuthViewModelTest`: validation, success, and server failure paths.
- `ApiErrorMapperTest`: transport, auth, conflict, and validation mapping.
- Fakes live in `:core:testing`; prefer fakes over mocks.
- UI screens are stateless and receive state plus callbacks, which keeps them
  previewable and testable.

## Rules

- Test behavior, not implementation details.
- Keep tests deterministic and offline.
- Never assert on tokens or secrets.
- Add a failure-path test with every new screen or repository operation.
- Run the full quality gate before claiming a task complete.
