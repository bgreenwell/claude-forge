# cforge

`cforge` is a command-line interface tool for scaffolding and managing claude code marketplaces and plugins. it enforces a standardized structure, ensuring that marketplaces and their components are valid and consistent.

## features

-   scaffold new marketplaces and plugins with a single command.
-   inject components like skills, commands, agents, and hooks into plugins.
-   validate the integrity of plugin manifests and skill frontmatter.
-   register plugins with a marketplace, preventing duplicates and ensuring structural correctness.
-   interactive prompts for a guided experience, with command-line flags for automation.

## quick start

### installation

install the tool locally using cargo:

```bash
cargo install --path .
```

### usage

1.  **initialize a new marketplace:**
    ```bash
    cforge init --name my-awesome-marketplace -d "a marketplace for my awesome plugins"
    ```
    this creates the following directory structure:
    ```
    my-awesome-marketplace/
    ├── .claude-plugin/
    │   └── marketplace.json
    ├── CLAUDE.md
    ├── plugins/
    └── README.md
    ```

2.  **navigate into the new marketplace:**
    ```bash
    cd my-awesome-marketplace
    ```

3.  **create a new plugin:**
    ```bash
    cforge new-plugin --name my-first-plugin -d "a plugin that does awesome things"
    ```
    this creates a new plugin inside the `plugins` directory:
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

4.  **add a skill to the plugin:**
    ```bash
    cforge add --plugin my-first-plugin skill --name my-skill -d "a skill that performs a specific action"
    ```
    this adds a new skill file to your plugin's `skills` directory:
    ```
    plugins/
    └── my-first-plugin/
        └── skills/
            └── my-skill.md
    ```

5.  **validate the plugin:**
    ```bash
    cforge validate --path plugins/my-first-plugin
    ```

6.  **register the plugin:**
    ```bash
    cforge register plugins/my-first-plugin
    ```

## development

to build or contribute to the project, clone the repository and build it with cargo.

```bash
cargo build
```

run tests to ensure all components are working correctly.

```bash
cargo test
```
