use anyhow::{Context, Result};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn create_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).with_context(|| format!("Failed to create directory {path:?}"))?;
        println!("{} {:?}", "Created dir:".green(), path);
    }
    Ok(())
}

pub fn create_file(path: &Path, content: &str) -> Result<()> {
    if !path.exists() {
        fs::write(path, content).with_context(|| format!("Failed to write file {path:?}"))?;
        println!("{} {:?}", "Created file:".green(), path);
    } else {
        println!("{} {:?}", "Skipped (exists):".yellow(), path);
    }
    Ok(())
}

/// Locates the target plugin directory based on flags, CWD, or interactive selection.
pub fn resolve_plugin_root(explicit_name: &Option<String>) -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    // STRATEGY 1: Explicit Flag (--plugin my-tool)
    if let Some(name) = explicit_name {
        let candidates = vec![
            current_dir.join("plugins").join(name),
            current_dir.join(name),
        ];
        for path in candidates {
            if path.exists() && path.join(".claude-plugin/plugin.json").exists() {
                return Ok(path);
            }
        }
        return Err(anyhow::anyhow!(
            "Plugin '{name}' not found. Are you in the marketplace root?"
        ));
    }

    // STRATEGY 2: Implicit (We are inside a plugin folder)
    if let Some(root) = find_plugin_root_upwards(&current_dir) {
        return Ok(root);
    }

    // STRATEGY 3: Interactive (We are in marketplace root -> Show Menu)
    let plugins_dir = current_dir.join("plugins");
    if plugins_dir.exists() {
        let entries: Vec<PathBuf> = fs::read_dir(plugins_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir() && p.join(".claude-plugin/plugin.json").exists())
            .collect();

        if entries.is_empty() {
            return Err(anyhow::anyhow!(
                "No plugins found in ./plugins. Create one first!"
            ));
        }

        let selections: Vec<String> = entries
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        println!("Multiple plugins detected. Where should this component go?");
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections)
            .interact()?;

        return Ok(entries[selection].clone());
    }

    Err(anyhow::anyhow!(
        "❌ Could not detect plugin context. Run inside a plugin or use --plugin <name>"
    ))
}

fn find_plugin_root_upwards(start: &Path) -> Option<PathBuf> {
    let mut current = start;
    loop {
        if current.join(".claude-plugin/plugin.json").exists() {
            return Some(current.to_path_buf());
        }
        match current.parent() {
            Some(p) => current = p,
            None => break,
        }
    }
    None
}
