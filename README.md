# mbot

A Rust-based AI agent management system with HTTP API and CLI interface. Manage your AI agents, track their configuration, and organize them in a structured workspace.

## Features

- **HTTP Server**: RESTful API for agent management (Axum + Tokio)
- **CLI Client**: Command-line interface for managing agents (kcli)
- **SQLite Database**: Persistent storage for agent configurations
- **File System Management**: Automatic workspace and documentation creation
- **Environment Configuration**: Flexible configuration via .env files
- **Modular Architecture**: Clean separation of concerns with multiple modules

## Prerequisites

- **Rust 1.70+** (install via [rustup](https://rustup.rs/))
- **Cargo** (comes with Rust)
- **Optional**: curl or any HTTP client for testing API

## Installation

1. **Clone or download the project**

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run the server:**
   ```bash
   cargo run --bin kserve
   ```

## Configuration

Create a `.env` file in the project root for configuration:

```env
# Server URL (default: http://127.0.0.1:6411)
SERVER_URL=http://127.0.0.1:6411

# Agent folder name (default: workspace)
# AGENTS_FOLDER=workspace
```

## Usage

### CLI Commands

#### Check Server Status
```bash
cargo run --bin kcli -- status
```

#### List All Agents
```bash
cargo run --bin kcli -- list
```

#### Add an Agent
```bash
cargo run --bin kcli -- add --name "AgentName" --token "your-token-here" --model "gpt-4"
```

#### Remove an Agent
```bash
cargo run --bin kcli -- remove --id 1
```

### API Endpoints

#### Health Check
```bash
GET /ping
```

#### List Agents
```bash
GET /list
```

#### Create Agent
```bash
POST /add
Content-Type: application/json

{
  "name": "AgentName",
  "token": "your-token-here",
  "model": "gpt-4"
}
```

#### Delete Agent
```bash
DELETE /remove
Content-Type: application/json

{
  "id": 1
}
```

### Example Workflow

```bash
# Terminal 1: Start the server
cargo run --bin kserve

# Terminal 2: Use the CLI
cargo run --bin kcli -- add --name "Agent1" --token "abc123" --model "gpt-4"
cargo run --bin kcli -- list
cargo run --bin kcli -- remove --id 1
```

## Project Structure

```
kagents/
├── Cargo.toml              # Project dependencies
├── .env                    # Environment configuration
├── agents.sqlite           # SQLite database (auto-created)
├── workspace/              # Agent folders (auto-created)
│   ├── Agent1/
│   │   └── readme.md
│   └── Agent2/
│       └── readme.md
└── src/
    ├── kcli/
    │   └── kcli.rs       # CLI client main file
    │   ├── command.rs     # CLI command definitions
    │   └── http_fn.rs     # HTTP client functions
    └── server/
        ├── kserve.rs      # HTTP server main file
        ├── db_func.rs     # Database operations
        └── handler.rs     # HTTP request handlers
```

## Development

### Build the Project
```bash
cargo build
```

### Run the Server
```bash
cargo run --bin kserve
```

### Run the CLI
```bash
cargo run --bin kcli --help
```

### Check Code
```bash
cargo check
```

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

### Run Tests
```bash
cargo test
```

## Agent Structure

Each agent is stored with the following information:
- **id**: Unique identifier (auto-generated)
- **name**: Agent name (used for folder creation)
- **token**: Authentication token for the agent
- **model**: AI model to use (e.g., gpt-4, gpt-3.5)
- **created_at**: Timestamp when the agent was created

Each agent gets its own folder under `workspace/` containing a `readme.md` file with basic agent information.
