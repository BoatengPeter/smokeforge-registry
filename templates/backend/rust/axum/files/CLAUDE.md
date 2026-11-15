# CLAUDE.md

Read [CONTEXT.md](CONTEXT.md) and the code layout in [README.md](README.md). Use
`.agents/skills` or `.claude/skills`; the Rust skills cover best practices,
async patterns, testing, and domain module design. HTTP responses live in
`src/response.rs`; typed errors live in `src/error.rs`. Run `make check` before
reporting completion.
