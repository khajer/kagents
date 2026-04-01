# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run binaries
cargo run --bin kserve      # Start HTTP server (port 6411)
cargo run --bin kcli        # Run CLI client

# Check / Lint / Format
cargo check
cargo check --bin kserve
cargo check --bin kcli
cargo fmt                             # Required before commits
cargo clippy -- -D warnings           # Required before commits
cargo clippy --all-targets -- -D warnings

# Test
cargo test
cargo test test_name                  # Run single test by name
cargo test -- --nocapture             # Show stdout during tests
```

## Architecture

This is a client-server agent management system with two binaries in a single crate (`mbot`):

**`kserve`** — HTTP server (port 6411, `src/server/`)
- `kserve.rs` — entry point: sets up SQLite pool, creates routes, starts axum server
- `db_func.rs` — creates the `agents` table on startup if it doesn't exist
- `handler.rs` — all axum route handlers + the `Agent` struct (used by DB queries)

**`kcli`** — CLI client (`src/kcli/kcli.rs`, single file)
- Uses `clap` derive macros for subcommands: `status`, `list`, `add`, `remove`
- Always pings `GET /ping` first to confirm server is running
- Defines its own local `Agent` struct (mirrors the server-side struct)
- Connects to hardcoded `http://127.0.0.1:6411`

**Data flow:**
```
kcli → HTTP (port 6411) → kserve → handler.rs → SQLite (agents.sqlite)
                                               ↘ workspace/{name}/ (filesystem)
```

**Side effects on agent create/delete:** The server creates/removes a `workspace/{agent_name}/` directory with a generated `readme.md` alongside each DB operation.

## Database

Single SQLite file `agents.sqlite` (auto-created by `kserve` on first run, gitignored):
```sql
CREATE TABLE IF NOT EXISTS agents (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL,
    token      TEXT NOT NULL,
    model      TEXT NOT NULL,
    created_at TEXT NOT NULL   -- RFC3339 string from chrono::Utc::now()
)
```
No migrations — schema is created idempotently on startup via `db_func::create_table_if_not_exists`.

## Code Style

Imports grouped in order: `std` → external crates → local (`crate::`), with blank lines between groups. Use collapsible `{}` for multiple items from the same crate. See `AGENTS.md` for the full style reference including naming conventions and error handling patterns.

Handler structs that appear in `pub` function signatures must be at least `pub(crate)`.
