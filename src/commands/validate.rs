use anyhow::Result;
use colored::*;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

// Marketplace schema
#[derive(Deserialize)]
struct Marketplace {
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

// Main entry point with auto-detection
pub fn validate(path_opt: Option<String>, fail_fast: bool) -> Result<()> {
    let root = resolve_path(path_opt)?;

    if is_marketplace(&root)? {
        validate_marketplace(&root, fail_fast)
    } else {
        validate_plugin(&root)
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

// Validate single plugin
fn validate_plugin(root: &Path) -> Result<()> {
    println!("Validating plugin at {:?}...", root);
    let mut errors: Vec<String> = Vec::new();

    // 2. Check Manifest
    let manifest_path = root.join(".claude-plugin/plugin.json");
    if !manifest_path.exists() {
        errors.push("❌ Missing .claude-plugin/plugin.json".to_string());
    } else {
        let content = fs::read_to_string(&manifest_path)?;
        if serde_json::from_str::<serde_json::Value>(&content).is_err() {
            errors.push("❌ plugin.json contains invalid JSON".to_string());
        }
    }

    // 3. Check Skills (YAML Frontmatter)
    let skills_dir = root.join("skills");
    if skills_dir.exists() {
        for entry in fs::read_dir(skills_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                let content = fs::read_to_string(&path)?;
                if !validate_frontmatter(&content) {
                    let filename = path.file_name().unwrap();
                    errors.push(format!(
                        "❌ Skill {filename:?} has invalid YAML frontmatter"
                    ));
                }
            }
        }
    }

    // 4. Report
    if errors.is_empty() {
        println!("{}", "✅ Plugin structure is VALID.".green().bold());
        Ok(())
    } else {
        for e in errors {
            println!("{}", e.red());
        }
        Err(anyhow::anyhow!("Validation failed"))
    }
}

// Validate entire marketplace
fn validate_marketplace(root: &Path, fail_fast: bool) -> Result<()> {
    println!("Validating marketplace at {:?}...", root);

    // Parse marketplace.json
    let marketplace_path = root.join(".claude-plugin/marketplace.json");
    let content = fs::read_to_string(&marketplace_path)?;
    let marketplace: Marketplace = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid marketplace.json: {}", e))?;

    let total = marketplace.plugins.len();
    println!("Found {} plugin(s) in marketplace\n", total);

    let mut passed = 0;
    let mut failed = 0;

    for (idx, plugin) in marketplace.plugins.iter().enumerate() {
        println!("[{}/{}] {} (v{})", idx + 1, total, plugin.name, plugin.version);

        // Resolve plugin path
        let plugin_path = root.join(&plugin.source);

        if !plugin_path.exists() {
            println!("  {} Plugin directory not found", "✗".red());
            failed += 1;
            if fail_fast {
                return Err(anyhow::anyhow!("Validation failed for {}", plugin.name));
            }
            continue;
        }

        // Validate the plugin
        match validate_plugin(&plugin_path) {
            Ok(_) => {
                passed += 1;
            }
            Err(_) => {
                failed += 1;
                if fail_fast {
                    return Err(anyhow::anyhow!("Validation failed for {}", plugin.name));
                }
            }
        }
        println!();
    }

    // Print summary
    println!("Marketplace validation: {}/{} plugins valid", passed, total);

    if failed > 0 {
        Err(anyhow::anyhow!("{} plugin(s) failed validation", failed))
    } else {
        println!("{}", "All plugins valid!".green().bold());
        Ok(())
    }
}

fn validate_frontmatter(content: &str) -> bool {
    if !content.starts_with("---") {
        return false;
    }
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return false;
    }
    // Attempt to parse the YAML block
    serde_yaml::from_str::<serde_json::Value>(parts[1]).is_ok()
}
