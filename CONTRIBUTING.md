# Contributing to claude-forge

Thank you for your interest in contributing to `claude-forge`! We welcome contributions from the community to help improve this tool.

## Getting started

1.  **Fork the repository** on GitHub.
2.  **Clone your fork** locally:
    ```bash
    git clone https://github.com/bgreenwell/claude-forge.git
    cd claude-forge
    ```
3.  **Install Rust**: Ensure you have the latest stable version of Rust installed. You can install it via [rustup](https://rustup.rs/).

## Development workflow

1.  **Create a new branch** for your feature or bug fix:
    ```bash
    git checkout -b feature/my-awesome-feature
    ```
2.  **Make your changes**.
3.  **Format your code**: We use `rustfmt` to maintain a consistent code style.
    ```bash
    cargo fmt
    ```
4.  **Lint your code**: We use `clippy` to catch common mistakes.
    ```bash
    cargo clippy -- -d warnings
    ```
5.  **Run tests**: Ensure that your changes don't break existing functionality.
    ```bash
    cargo test
    ```
    *Note: If you add new functionality, please add corresponding tests.*

## Submitting changes

1.  **Commit your changes** with clear and descriptive commit messages. We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.
    ```bash
    git commit -m "feat: add new command for validating skills"
    ```
2.  **Push to your fork**:
    ```bash
    git push origin feature/my-awesome-feature
    ```
3.  **Open a Pull Request** against the `main` branch of the original repository. Describe your changes in detail and link to any relevant issues.

## Code style

-   Follow standard Rust idioms.
-   Keep functions small and focused.
-   Document public APIs.

## License

By contributing, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
