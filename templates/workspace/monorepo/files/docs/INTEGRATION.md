# Integration

## API contract

When an API is selected, it owns:

- authentication and authorization;
- persistence and migrations;
- error and problem-detail responses;
- server-side configuration and secrets.

Clients consume the same contract through their own networking layer. Shared
behavior belongs in `docs/` or the API contract, not in copied root code.

## Client wiring

- Web uses same-origin BFF routes where applicable.
- Desktop keeps API access and refresh tokens in the trusted main process.
- Android and iOS keep tokens in platform secure storage.
- API-less workspaces are standalone clients and do not claim a server contract.

## Cross-client changes

Update every affected app guide and the root `CONTEXT-MAP.md` when a change
crosses client boundaries. Keep platform-specific implementation details in the
app docs.
