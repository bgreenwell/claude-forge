# AGENTS.md

This document provides context and guidelines for AI agents working on the `claude-forge` codebase.

## Project Overview

`claude-forge` (binary name: `cforge`) is a Rust CLI tool for scaffolding and managing "Claude Code" marketplaces and plugins. It helps users create valid directory structures and configuration files for the Claude ecosystem.

## Architecture & Code Structure

The project is organized as a standard Rust binary:

-   `src/main.rs`: Entry point. Defines the `clap` CLI structure (`Cli` struct and `Commands` enum).
-   `src/commands/`: Contains the implementation logic for each subcommand.
    -   `init.rs`: Scaffolds a new marketplace.
    -   `add.rs`: Adds components (skills, commands) to plugins.
    -   `register.rs`: Registers plugins in `marketplace.json`.
    -   `validate.rs`: Validates plugin structure.
-   `src/templates/`: Contains string templates or logic for generating boilerplate files.
-   `src/utils/`: Shared utility functions (file I/O, string manipulation).

## Tech Stack

-   **Language:** Rust (2021 edition)
-   **CLI Framework:** `clap` (derive feature)
-   **Error Handling:** `anyhow`
-   **Serialization:** `serde`, `serde_json`

## Development Workflow

### Dependency Management
Dependencies are managed in `Cargo.toml`. No manual setup is required beyond having a working Rust toolchain.

### Build & Run
Always verify changes by building and running the binary.

-   **Build:** `cargo build`
-   **Run:** `cargo run -- <command> <args>`
    -   *Example:* `cargo run -- init --name my-marketplace`

### Testing Strategy
**Important:** This project currently lacks a formal test suite (`cargo test` will likely do nothing).
-   **Validation:** You must manually verify changes by running the CLI commands against a temporary directory or file.
-   **Sanity Check:** Ensure `cargo check` passes to catch type errors.

### Code Style
-   **Formatting:** Run `cargo fmt` on all modified files.
-   **Linting:** Run `cargo clippy` and fix warnings.

## Git Conventions
-   **Commit Messages:** Use Conventional Commits (e.g., `feat: ...`, `fix: ...`).
-   **Scope:** Keep changes focused. If adding a new command, create a new module in `src/commands/` and register it in `main.rs`.

## Resources

For more information on developing plugins and marketplaces, refer to the [HOWTO_PLUGINS.md](HOWTO_PLUGINS.md) file in the root of this repository.
