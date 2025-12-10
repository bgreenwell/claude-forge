# claude-forge

`claude-forge` is a command-line interface tool for scaffolding and managing Claude Code marketplaces and plugins. It enforces a standardized structure, ensuring that marketplaces and their components are valid and consistent.

## Features

-   Scaffold new marketplaces and plugins with a single command.
-   Inject components like skills, commands, agents, and hooks into plugins.
-   Validate the integrity of plugin manifests and skill frontmatter.
-   Register plugins with a marketplace, preventing duplicates and ensuring structural correctness.
-   Interactive prompts for a guided experience, with command-line flags for automation.

## Quick start

### Installation

Install the tool locally using cargo:

```bash
cargo install --path .
```

### Usage

1.  **Initialize a new marketplace:**
    ```bash
    cforge init --name my-awesome-marketplace -d "a marketplace for my awesome plugins"
    ```
    This creates the following directory structure:
    ```
    my-awesome-marketplace/
    ├── .claude-plugin/
    │   └── marketplace.json
    ├── CLAUDE.md
    ├── plugins/
    └── README.md
    ```

2.  **Navigate into the new marketplace:**
    ```bash
    cd my-awesome-marketplace
    ```

3.  **Create a new plugin:**
    ```bash
    cforge new-plugin --name my-first-plugin -d "a plugin that does awesome things"
    ```
    This creates a new plugin inside the `plugins` directory:
    ```
    plugins/
    └── my-first-plugin/
        ├── .claude-plugin/
        │   └── plugin.json
        ├── agents/
        ├── commands/
        ├── hooks/
        ├── skills/
        └── README.md
    ```

4.  **Add a skill to the plugin:**
    ```bash
    cforge add --plugin my-first-plugin skill --name my-skill -d "a skill that performs a specific action"
    ```
    This adds a new skill file to your plugin's `skills` directory:
    ```
    plugins/
    └── my-first-plugin/
        └── skills/
            └── my-skill.md
    ```

5.  **Validate the plugin:**
    ```bash
    cforge validate --path plugins/my-first-plugin
    ```

6.  **Register the plugin:**
    ```bash
    cforge register plugins/my-first-plugin
    ```

## Development

To build or contribute to the project, clone the repository and build it with cargo.

```bash
cargo build
```

Run tests to ensure all components are working correctly.

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
