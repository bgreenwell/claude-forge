use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn register_plugin(plugin_path_str: String) -> Result<()> {
    // 1. Locate Marketplace Manifest (must be in root)
    let market_manifest = Path::new(".claude-plugin/marketplace.json");
    if !market_manifest.exists() {
        return Err(anyhow!(
            "❌ No marketplace.json found. Run this from the marketplace root."
        ));
    }

    // 2. Load Plugin Data
    let plugin_path = Path::new(&plugin_path_str);
    let plugin_manifest = plugin_path.join(".claude-plugin/plugin.json");
    if !plugin_manifest.exists() {
        return Err(anyhow!(
            "❌ Target path is not a valid plugin (missing plugin.json)"
        ));
    }

    let p_content = fs::read_to_string(&plugin_manifest)?;
    let p_json: Value = serde_json::from_str(&p_content)?;
    let p_name = p_json["name"]
        .as_str()
        .context("Plugin missing 'name' field")?;
    let p_desc = p_json["description"].as_str().unwrap_or("");

    // 3. Update Marketplace Registry
    let m_content = fs::read_to_string(market_manifest)?;
    let mut m_json: Value = serde_json::from_str(&m_content)?;

    if let Some(plugins) = m_json["plugins"].as_array_mut() {
        // Check for duplicates
        if plugins.iter().any(|p| p["name"] == p_name) {
            println!("⚠️  Plugin '{p_name}' is already registered.");
            return Ok(());
        }

        // Add new entry
        plugins.push(serde_json::json!({
            "name": p_name,
            "description": p_desc,
            "source": plugin_path_str // Saves the relative path
        }));
    }

    // 4. Save
    let file = fs::File::create(market_manifest)?;
    serde_json::to_writer_pretty(file, &m_json)?;

    println!("✅ Registered '{p_name}' in marketplace registry.");
    Ok(())
}
