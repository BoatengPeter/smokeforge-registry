# CLAUDE.md

Read [CONTEXT.md](CONTEXT.md) and [README.md](README.md). Use the project-scoped
skills under `.agents/skills` or `.claude/skills`; `electron-best-practices` is
mandatory for Electron changes. Keep the renderer sandboxed, validate every IPC
payload, and run `make lint typecheck test build` before reporting completion.
