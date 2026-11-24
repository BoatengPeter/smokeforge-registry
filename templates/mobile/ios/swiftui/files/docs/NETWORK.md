# Networking

## Client

- `URLSession` with async/await.
- `APIClient` normalizes the base URL and owns request/response handling.
- `Core` repository protocols keep networking out of views.
- RFC 9457 problem details map to `AppError` in the Data layer.

## Configuration

- `Info.plist` receives `APIBaseURL`, `AppEnvironment`, and version values from
  build settings and xcconfigs.
- Debug: `http://127.0.0.1:8080`.
- Staging and Release: HTTPS placeholders that must be replaced for the client.
- ATS local networking is allowed for the simulator; release endpoints should be
  HTTPS.

## Rules

- Never call URLSession from a view.
- Never log tokens, passwords, or authorization headers.
- Decode unknown server fields leniently only at DTO boundaries.
- Map transport, HTTP, validation, and decoding errors explicitly.
