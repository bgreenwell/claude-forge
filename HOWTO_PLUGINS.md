# HOWTO: Building Claude Code Plugins & Marketplaces

This guide covers the end-to-end process of creating a Claude Code plugin, packaging it into a marketplace, and distributing it to your team or the community.

## 1\. Plugin Architecture

A **Plugin** is a directory (often a git repository) containing a manifest and capabilities.
A **Marketplace** is a catalog (a JSON file) that tells Claude Code where to find these plugins.

### Standard Directory Structure

A typical plugin repository looks like this:

```text
my-plugin-repo/
├── .claude-plugin/
│   └── plugin.json        # Plugin manifest (Required)
├── commands/              # Custom slash commands
│   └── hello.md
├── skills/                # Agent Skills (tools for Claude)
│   └── data-fetcher.md
├── agents/                # Specialized Sub-agents
│   └── code-reviewer.md
├── hooks/                 # Lifecycle hooks (e.g., on session start)
│   └── setup.sh
└── README.md              # Documentation
```

## 2\. Creating a Plugin

### Step A: The Manifest

Create a hidden directory `.claude-plugin` in your root and add `plugin.json`. This defines your plugin's identity.

**File:** `.claude-plugin/plugin.json`

```json
{
  "name": "my-utility-plugin",
  "description": "A collection of utilities for daily dev work.",
  "version": "1.0.0",
  "author": {
    "name": "Jane Doe",
    "email": "jane@example.com"
  }
}
```

### Step B: Adding Capabilities

#### 1\. Slash Commands (`commands/`)

Commands are Markdown files that act as prompt templates. They are invoked by the user via `/command-name`.

**File:** `commands/standup.md`

```markdown
---
description: Generate a daily standup report based on git activity
---

# Daily Standup
Please analyze my git activity for the last 24 hours and generate a text standup report.
Focus on:
1. Commits made
2. PRs opened/reviewed
3. Files changed
```

*Usage: User types `/standup` in Claude Code.*

#### 2\. Agent Skills (`skills/`)

Skills are tools Claude can use autonomously to perform tasks (like browsing docs or querying a database).

**File:** `skills/fetch-logs.md`

```markdown
---
description: Fetch server logs for a specific environment
argument-hint: [environment] [lines]
---

# Fetch Logs
Use the local `kubectl` command to fetch the last $2 lines of logs from the $1 environment.
Return the output directly.
```

*Usage: Claude decides to use this when you ask "Why is staging failing?"*

#### 3\. Sub-Agents (`agents/`)

Agents are specialized personas with specific system prompts and toolsets.

**File:** `agents/qa-bot.md`

```markdown
---
description: QA specialist for writing test cases
capabilities: ["write-tests", "analyze-coverage"]
---

# QA Bot
You are an expert QA engineer. Your goal is to take a piece of code and write comprehensive unit tests using Pytest.
Always check for edge cases and null values.
```

*Usage: Claude delegates to "QA Bot" for testing tasks.*

## 3\. Creating a Marketplace

A marketplace is simply a way to bundle and point to one or more plugins. It can be hosted in a Git repository or a local folder.

### Step A: The Marketplace Manifest

Create a `marketplace.json` file inside a `.claude-plugin` folder in your marketplace repository.

**File:** `.claude-plugin/marketplace.json`

```json
{
  "name": "engineering-tools",
  "description": "Standard tooling for the Engineering Team",
  "owner": {
    "name": "Acme Corp"
  },
  "plugins": [
    {
      "name": "my-utility-plugin",
      "source": "https://github.com/acme-corp/my-utility-plugin.git",
      "description": "Daily dev utilities"
    },
    {
      "name": "security-scanner",
      "source": "./local-plugins/security-scanner",
      "description": "Pre-commit security checks"
    }
  ]
}
```

*Note: `source` can be a git URL or a relative local path.*

## 4\. Testing & Distribution

### Local Development Loop

1.  **Create a local marketplace** pointing to your plugin's folder.
2.  **Add the marketplace** to Claude Code:
    ```bash
    /plugin marketplace add ./path/to/my-marketplace-repo
    ```
3.  **Install the plugin**:
    ```bash
    /plugin install my-utility-plugin@engineering-tools
    ```
4.  **Test**: Run your command `/standup` or ask Claude a question that triggers your skill.

### Publishing

1.  Push your plugin repository to a public (or private) Git host.
2.  Push your marketplace repository (containing the `marketplace.json`) to Git.
3.  Users install your marketplace using the URL:
    ```bash
    /plugin marketplace add https://github.com/your-username/your-marketplace-repo.git
    ```

## 5\. Best Practices

  * **Granularity**: Keep plugins focused (e.g., `python-tools`, `aws-utils`) rather than one massive `everything-plugin`.
  * **Documentation**: Include a root `README.md` in your plugin repo explaining its commands and requirements (like needed API keys).
  * **Validation**: Use `claude plugin validate` (if available in your CLI version) or a JSON validator to ensure your manifests are correct before pushing.
