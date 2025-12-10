# agents.md for cforge

this document provides development guidelines for ai agents working on the `cforge` codebase.

## project overview

`cforge` is a rust-based command-line tool for scaffolding and managing "claude code" marketplaces and plugins. it ensures a consistent and valid structure for all components, from the marketplace root down to individual skills and commands. the tool is designed to be used both interactively and through automated scripts.

the core logic is built around the `clap` crate for command-line parsing and `serde` for manifest serialization (json). the project is structured into modules for commands, templates, and utilities.

## development setup

the project is built with rust. ensure you have a recent version of the rust toolchain installed.

1.  clone the repository.
2.  navigate to the project root.
3.  all dependencies are managed by cargo and are listed in `cargo.toml`.

## build, run, and test

use the standard cargo commands for development workflows.

-   **build (debug):**
    ```bash
    cargo build
    ```

-   **build (release):**
    ```bash
    cargo build --release
    ```

-   **run:**
    to run the application, use `cargo run` followed by the desired command and arguments. for example:
    ```bash
    cargo run -- init --name test-project
    ```

-   **test:**
    this project does not yet have a formal test suite. manual testing by invoking the cli commands is the current method for validation.

## code style and conventions

this project follows standard rust conventions.

-   **formatting:**
    all code should be formatted using `cargo fmt`. before committing, run the following command from the project root:
    ```bash
    cargo fmt
    ```

-   **linting:**
    use `cargo clippy` to catch common mistakes and improve the code. run it before committing:
    ```bash
    cargo clippy -- -d warnings
    ```

## git and commit conventions

-   **commits:**
    write clear and concise commit messages. follow the conventional commits specification. each commit message should consist of a header, a body, and a footer.

    example:
    ```
    feat: add --description flag to init command

    this allows users to specify a marketplace description directly
    from the command line, bypassing the interactive prompt.
    ```

-   **branches:**
    create a new branch for each new feature or bug fix. branch names should be descriptive (e.g., `feat/add-description-flag`, `fix/validation-bug`).

-   **pull requests:**
    ensure all checks (format, lint) pass before submitting a pull request. provide a clear description of the changes in the pr.