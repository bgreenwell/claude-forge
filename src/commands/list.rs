use anyhow::Result;
use colored::*;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

// Marketplace schema (same as validate.rs)
#[derive(Deserialize)]
struct Marketplace {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    owner: Option<Owner>,
    plugins: Vec<PluginEntry>,
}

#[derive(Deserialize)]
struct Owner {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    email: Option<String>,
}

#[derive(Deserialize)]
struct PluginEntry {
    name: String,
    version: String,
    source: String,
    description: Option<String>,
}

// Plugin schema
#[derive(Deserialize)]
struct Plugin {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    version: String,
    skills: Option<Vec<String>>,
    commands: Option<Vec<String>>,
    agents: Option<Vec<String>>,
    hooks: Option<Vec<String>>,
}

struct ComponentInfo {
    name: String,
    component_type: String,
    file_path: String,
}

// Main entry point with auto-detection
pub fn list(path_opt: Option<String>, verbose: bool) -> Result<()> {
    let root = resolve_path(path_opt)?;

    if is_marketplace(&root)? {
        list_plugins(&root, verbose)
    } else if is_plugin(&root)? {
        list_components(&root, verbose)
    } else {
        Err(anyhow::anyhow!("Not in a marketplace or plugin directory"))
    }
}

// Helper to resolve path
fn resolve_path(path_opt: Option<String>) -> Result<PathBuf> {
    let current = std::env::current_dir()?;
    Ok(match path_opt {
        Some(p) => Path::new(&p).to_path_buf(),
        None => current,
    })
}

// Check if directory is a marketplace
fn is_marketplace(path: &Path) -> Result<bool> {
    Ok(path.join(".claude-plugin/marketplace.json").exists())
}

// Check if directory is a plugin
fn is_plugin(path: &Path) -> Result<bool> {
    Ok(path.join(".claude-plugin/plugin.json").exists())
}

// List plugins in marketplace
fn list_plugins(root: &Path, verbose: bool) -> Result<()> {
    let marketplace_path = root.join(".claude-plugin/marketplace.json");
    let content = fs::read_to_string(&marketplace_path)?;
    let marketplace: Marketplace = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid marketplace.json: {}", e))?;

    if marketplace.plugins.is_empty() {
        println!("No plugins found in marketplace");
        return Ok(());
    }

    if verbose {
        print_plugins_verbose(&marketplace.plugins, root)?;
    } else {
        print_plugins_simple(&marketplace.plugins);
    }

    Ok(())
}

// List components in plugin
fn list_components(root: &Path, verbose: bool) -> Result<()> {
    let mut components = Vec::new();

    // Find skills
    if let Ok(entries) = fs::read_dir(root.join("skills")) {
        for entry in entries.flatten() {
            if entry.path().extension().is_some_and(|ext| ext == "md") {
                let name = entry.file_name().to_string_lossy().replace(".md", "");
                components.push(ComponentInfo {
                    name,
                    component_type: "skill".to_string(),
                    file_path: format!("skills/{}", entry.file_name().to_string_lossy()),
                });
            }
        }
    }

    // Find commands
    if let Ok(entries) = fs::read_dir(root.join("commands")) {
        for entry in entries.flatten() {
            if entry.path().extension().is_some_and(|ext| ext == "md") {
                let name = entry.file_name().to_string_lossy().replace(".md", "");
                components.push(ComponentInfo {
                    name,
                    component_type: "command".to_string(),
                    file_path: format!("commands/{}", entry.file_name().to_string_lossy()),
                });
            }
        }
    }

    // Find agents
    if let Ok(entries) = fs::read_dir(root.join("agents")) {
        for entry in entries.flatten() {
            if entry.path().extension().is_some_and(|ext| ext == "md") {
                let name = entry.file_name().to_string_lossy().replace(".md", "");
                components.push(ComponentInfo {
                    name,
                    component_type: "agent".to_string(),
                    file_path: format!("agents/{}", entry.file_name().to_string_lossy()),
                });
            }
        }
    }

    // Find hooks
    if let Ok(entries) = fs::read_dir(root.join("hooks")) {
        for entry in entries.flatten() {
            if entry.path().extension().is_some_and(|ext| ext == "sh") {
                let name = entry.file_name().to_string_lossy().replace(".sh", "");
                components.push(ComponentInfo {
                    name,
                    component_type: "hook".to_string(),
                    file_path: format!("hooks/{}", entry.file_name().to_string_lossy()),
                });
            }
        }
    }

    if components.is_empty() {
        println!("No components found in plugin");
        return Ok(());
    }

    print_components_table(&components, verbose);

    Ok(())
}

// Simple plugin list
fn print_plugins_simple(plugins: &[PluginEntry]) {
    for plugin in plugins {
        println!("{:<25} v{}", plugin.name, plugin.version);
    }
}

// Verbose plugin list
fn print_plugins_verbose(plugins: &[PluginEntry], root: &Path) -> Result<()> {
    println!(
        "{:<25} {:<10} {:<8} {}",
        "NAME".bold(),
        "VERSION".bold(),
        "STATUS".bold(),
        "DESCRIPTION".bold()
    );
    println!("{}", "-".repeat(80));

    for plugin in plugins {
        let plugin_path = root.join(&plugin.source);
        let status = if plugin_path.join(".claude-plugin/plugin.json").exists() {
            "valid".green()
        } else {
            "missing".red()
        };

        let desc = plugin
            .description
            .as_deref()
            .unwrap_or("")
            .chars()
            .take(40)
            .collect::<String>();

        println!(
            "{:<25} {:<10} {:<8} {}",
            plugin.name,
            format!("v{}", plugin.version),
            status,
            desc
        );
    }

    Ok(())
}

// Component table
fn print_components_table(components: &[ComponentInfo], _verbose: bool) {
    println!(
        "{:<20} {:<10} {}",
        "COMPONENT".bold(),
        "TYPE".bold(),
        "FILE".bold()
    );
    println!("{}", "-".repeat(60));

    for component in components {
        println!(
            "{:<20} {:<10} {}",
            component.name, component.component_type, component.file_path
        );
    }
}
