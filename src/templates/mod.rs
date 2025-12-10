pub fn get_claude_context() -> &'static str {
    r#"# Claude Code Marketplace Guide

## 🧠 Role & Context
You are managing a Claude Code Plugin Marketplace. Your goal is to maintain a strict, valid architecture while helping the user build useful tools.

## 🛠 Tool Usage Rules
**Creation**: ALWAYS use `claude-forge` to scaffold new items.
- New Plugin: `claude-forge new-plugin --name <name>`
- New Skill:  `claude-forge add skill --plugin <name> --name <skill-name>`

**Validation**: ALWAYS run validation after making edits.
- Command: `claude-forge validate`

## 📝 Editing Guidelines
1. **Frontmatter (YAML)**: Located between `---`. DO NOT DELETE. Only edit `description` or `argument-hint` if logic changes.
2. **Body**: You are free to edit the text below the second `---`.
3. **Manifests**: Ensure `plugin.json` remains valid JSON.
"#
}

pub fn get_plugin_manifest(name: &str, desc: &str) -> String {
    format!(
        r#"{{
  "name": "{name}",
  "description": "{desc}",
  "version": "0.1.0",
  "author": {{
    "name": "Your Name",
    "email": "you@example.com"
  }}
}}"#
    )
}

pub fn get_marketplace_manifest(name: &str, desc: &str) -> String {
    format!(
        r#"{{
  "name": "{name}",
  "description": "{desc}",
  "plugins": []
}}"#
    )
}

pub fn get_skill_template(name: &str, desc: &str) -> String {
    format!(
        r#"---
description: {desc}
argument-hint: [arg1]
---

# {name}

(Instructions for Claude: Describe how to use this tool, what inputs it expects, and the output format.)"#
    )
}

pub fn get_command_template(name: &str, desc: &str) -> String {
    format!(
        r#"---
description: {desc}
---

# {name}

(Instructions for Claude: This text is injected when the user types /{name}.)"#
    )
}

pub fn get_agent_template(name: &str, desc: &str, capabilities: &str) -> String {
    format!(
        r#"---
description: {desc}
capabilities: [{capabilities}]
---

# {name} Agent

You are a specialist agent. Your goal is...
"#
    )
}
