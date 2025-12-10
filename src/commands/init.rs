use crate::templates;
use crate::utils;
use anyhow::{anyhow, Result};
use dialoguer::Input;
use std::path::Path;

pub fn init_marketplace(name_opt: Option<String>, desc_opt: Option<String>) -> Result<()> {
    // 1. Get Name & Description
    let name = match name_opt {
        Some(n) => n,
        None => Input::new()
            .with_prompt("Marketplace Name")
            .interact_text()?,
    };
    let desc = get_description(desc_opt)?;

    let root = Path::new(&name);
    if root.exists() {
        return Err(anyhow!("Directory '{name}' already exists"));
    }

    // 2. Scaffold Structure
    utils::create_dir(root)?;
    utils::create_dir(&root.join("plugins"))?;
    utils::create_dir(&root.join(".claude-plugin"))?;

    // 3. Write Files
    let manifest = templates::get_marketplace_manifest(&name, &desc);
    utils::create_file(&root.join(".claude-plugin/marketplace.json"), &manifest)?;

    let ctx = templates::get_claude_context();
    utils::create_file(&root.join("CLAUDE.md"), ctx)?;

    let readme = format!(
        "# {name}

{desc}"
    );
    utils::create_file(&root.join("README.md"), &readme)?;

    println!("\n✅ Marketplace initialized! To start:");
    println!("   cd {name}");
    println!("   claude-forge new-plugin --name my-first-tool");
    Ok(())
}

pub fn init_plugin(name_opt: Option<String>, desc_opt: Option<String>) -> Result<()> {
    // 1. Get Name & Description
    let name = match name_opt {
        Some(n) => n,
        None => Input::new()
            .with_prompt("Plugin Name (kebab-case)")
            .interact_text()?,
    };
    let desc = get_description(desc_opt)?;

    // Check context: Are we in a marketplace?
    // (Simple check: prefer creating in ./plugins/ if it exists)
    let cwd = std::env::current_dir()?;
    let target_dir = if cwd.join("plugins").exists() {
        cwd.join("plugins").join(&name)
    } else {
        cwd.join(&name)
    };

    if target_dir.exists() {
        return Err(anyhow!("Plugin directory already exists at {target_dir:?}"));
    }

    // 2. Structure
    utils::create_dir(&target_dir)?;
    utils::create_dir(&target_dir.join("commands"))?;
    utils::create_dir(&target_dir.join("skills"))?;
    utils::create_dir(&target_dir.join("agents"))?;
    utils::create_dir(&target_dir.join("hooks"))?;
    utils::create_dir(&target_dir.join(".claude-plugin"))?;

    // 3. Manifest
    let manifest = templates::get_plugin_manifest(&name, &desc);
    utils::create_file(&target_dir.join(".claude-plugin/plugin.json"), &manifest)?;

    utils::create_file(&target_dir.join("README.md"), &format!("# {name}"))?;

    println!("\n✅ Plugin created at {target_dir:?}");
    Ok(())
}

fn get_description(arg: Option<String>) -> Result<String> {
    match arg {
        Some(d) => Ok(d),
        None => Ok(Input::new().with_prompt("Description").interact_text()?),
    }
}
