# AGENTS.md

Guidelines for agentic coding agents working in the mbot repository.

## Project Overview

mbot is a Rust async bot/application using:
- **Tokio** - async runtime
- **Axum** - web framework with routing
- **tokio-cron-scheduler** - scheduled task execution (cron jobs)
- **pulldown-cmark** - Markdown parsing
- **chrono** - date/time handling
- **tracing** - structured logging

### Binaries

This project contains multiple binaries:
- **kcli** - Main scheduler CLI (`src/kcli/kcli.rs`)
- **kserverd** - TCP command server on port 6411 (`src/server/kserverd.rs`)

## Build/Lint/Test Commands

```bash
# Build the project
cargo build

# Build for release (optimized)
cargo build --release

# Run the main application
cargo run --bin kcli

# Run kserverd (TCP server on port 6411)
cargo run --bin kserverd

# Fast check without full compilation
cargo check

# Check a specific binary
cargo check --bin kserverd
cargo check --bin kcli

# Run all tests
cargo test

# Run a single test by name
cargo test test_name

# Run a single test with pattern match
cargo test --test pattern

# Run tests with output shown
cargo test -- --nocapture

# Format code (required before commits)
cargo fmt

# Lint with clippy (required before commits)
cargo clippy -- -D warnings

# Run clippy on all targets including tests
cargo clippy --all-targets -- -D warnings
```

## Code Style Guidelines

### Imports

Group imports in this order, separated by blank lines:
1. `std` imports
2. External crate imports (alphabetically)
3. Local crate/module imports

```rust
use std::fs;
use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Json, Router,
};
use chrono::Local;
use tracing::{info, Level};

use crate::modules::task;
```

- Use collapsible import groups `{}` for multiple items from same crate
- Import only what you need, avoid `.*` glob imports

### Formatting

- Run `cargo fmt` before committing - no exceptions
- Max line length: 100 characters (rustfmt default)
- Use 4 spaces for indentation (no tabs)
- Single blank line between functions
- No trailing whitespace

### Types and Naming

| Item | Convention | Example |
|------|------------|---------|
| Structs/Enums | PascalCase | `CreateTask`, `JobScheduler` |
| Functions | snake_case | `get_tasks`, `parse_markdown` |
| Variables | snake_case | `task_text`, `markdown_content` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_RETRIES` |
| Modules | snake_case | `mod task_handler;` |
| Type parameters | short uppercase | `T`, `K`, `V` |

- Prefer `String` for owned text, `&str` for borrowed
- Use `u32`, `u64` for IDs; avoid `i32` unless negative values needed
- Prefer `Result<T, E>` with specific error types over `Option<T>` when failures should be handled

### Error Handling

- Use `Result<(), Box<dyn std::error::Error>>` for main and top-level functions
- Use `?` operator for error propagation
- Use `.expect()` only for programming errors that should crash
- Use `.unwrap_or()` or `.unwrap_or_default()` for safe defaults

```rust
// Good: propagate errors
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;
    sched.start().await?;
    Ok(())
}

// Good: safe default
let task_text = line.split("] ").nth(1).unwrap_or("");

// Avoid in production code
let value = some_option.unwrap(); // Can panic
```

### Async Code

- Use `#[tokio::main]` for main function
- Mark async functions with `async fn`
- Use `.await` for async operations
- Prefer `tokio::sync` primitives over `std::sync` in async contexts

### Structs and Serialization

```rust
#[derive(serde::Deserialize)]
struct CreateTask {
    name: String,
}

#[derive(serde::Serialize)]
struct Task {
    id: u32,
    name: String,
}
```

- Use `serde` derive macros for API request/response types
- Keep DTOs (Data Transfer Objects) separate from domain types

### Functions

- Keep functions focused on a single responsibility
- Prefer pure functions when possible
- Document public functions with `///` doc comments
- Handler functions should be `async fn` returning axum types

```rust
/// Reads a markdown file and returns its contents.
pub fn read_markdown_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read markdown file")
}
```

### Comments

- Prefer self-documenting code over comments
- Use `///` for documentation comments on public items
- Use `//` for inline explanations of non-obvious logic
- Avoid commented-out code in commits

### Testing

- Place unit tests in the same file using `#[cfg(test)] mod tests`
- Place integration tests in `tests/` directory
- Test function names: `test_<function>_<scenario>`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tasks_extracts_task_text() {
        let content = "- [ ] Task 1\n- [ ] Task 2";
        let tasks = parse_tasks(content);
        assert_eq!(tasks, vec!["Task 1", "Task 2"]);
    }
}
```

## Project Structure

```
mbot/
├── Cargo.toml          # Dependencies and metadata
├── src/
│   ├── kcli/
│   │   └── kcli.rs     # Main scheduler CLI
│   └── server/
│       └── kserverd.rs # TCP command server (port 6411)
├── schedules/
│   └── schedule.md     # Task schedule data
└── tests/              # Integration tests (if any)
```

## Common Patterns

### Adding a new route

```rust
.route("/path", get(handler))
.route("/path", post(create_handler))
```

### Adding a cron job

```rust
let job = Job::new("0 * * * * *", |_uuid, _l| {
    // Task logic here
})?;
sched.add(job).await?;
```

### Adding a dependency

1. Add to `Cargo.toml` under `[dependencies]`
2. Run `cargo build` to verify
3. Import with `use crate_name::Item;`
