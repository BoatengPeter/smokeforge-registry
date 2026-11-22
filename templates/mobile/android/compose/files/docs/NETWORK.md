# Network

## Client

- Retrofit 3 with the Kotlin serialization converter.
- OkHttp 5 with a bearer interceptor and a single-refresh authenticator.
- Base URL comes from `BuildConfig.API_BASE_URL`.
- Debug defaults to `http://10.0.2.2:8080` for the Android emulator.
- Staging and release use Gradle properties:
  `-PstagingApiBaseUrl=...` and `-PreleaseApiBaseUrl=...`.

## Error contract

The API returns RFC 9457 problem details. The network layer parses:

- `title`
- `detail`
- `status`
- `code`
- `errors[]` field messages

`ApiErrorMapper` maps transport and HTTP failures to `AppError`. ViewModels do
not parse HTTP responses.

## Rules

- Never log credentials, tokens, or authorization headers.
- Never call Retrofit from a composable.
- Keep DTOs out of feature and domain modules.
- Refresh with the dedicated no-auth client to avoid authenticator recursion.
- Do not retry forever. The authenticator refreshes once per request chain.
