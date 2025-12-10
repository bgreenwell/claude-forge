use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

pub fn validate_plugin(path_opt: Option<String>) -> Result<()> {
    // 1. Resolve Path (Default to CWD)
    let current = std::env::current_dir()?;
    let root = match path_opt {
        Some(p) => Path::new(&p).to_path_buf(),
        None => current,
    };

    println!("🔍 Validating plugin at {root:?}...");
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
