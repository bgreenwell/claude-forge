use crate::templates;
use crate::utils;
use anyhow::Result;
use clap::Subcommand;
use colored::*;
use convert_case::{Case, Casing};
use dialoguer::Input;

#[derive(Subcommand)]
pub enum Component {
    Skill {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    Command {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    Agent {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        capabilities: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    Hook {
        #[arg(short, long)]
        event: String,
    },
}

pub fn run(plugin_flag: Option<String>, component: Component) -> Result<()> {
    // 1. Resolve Target Plugin
    let root = utils::resolve_plugin_root(&plugin_flag)?;
    let plugin_name = root.file_name().unwrap().to_string_lossy();
    println!("🔧 Targeting plugin: {}", plugin_name.cyan());

    match component {
        Component::Skill { name, description } => {
            let n = get_name(name, "Skill Name")?;
            let desc = get_description(description)?;
            let filename = format!("{}.md", n.to_case(Case::Kebab));

            let content = templates::get_skill_template(&n, &desc);
            utils::create_file(&root.join("skills").join(filename), &content)?;
        }
        Component::Command { name, description } => {
            let n = get_name(name, "Command Name")?;
            let desc = get_description(description)?;
            let filename = format!("{}.md", n.to_case(Case::Kebab));

            let content = templates::get_command_template(&n, &desc);
            utils::create_file(&root.join("commands").join(filename), &content)?;
        }
        Component::Agent {
            name,
            capabilities,
            description,
        } => {
            let n = get_name(name, "Agent Name")?;
            let desc = get_description(description)?;
            let caps = capabilities.unwrap_or_else(|| "bash, search".to_string());
            let filename = format!("{}.md", n.to_case(Case::Kebab));

            let content = templates::get_agent_template(&n, &desc, &caps);
            utils::create_file(&root.join("agents").join(filename), &content)?;
        }
        Component::Hook { event } => {
            let filename = match event.as_str() {
                "start" => "on_session_start.sh",
                "message" => "on_user_message.sh",
                _ => return Err(anyhow::anyhow!("Unknown event. Use 'start' or 'message'")),
            };

            let path = root.join("hooks").join(filename);
            let content = "#!/bin/bash\necho 'Hook triggered'";
            utils::create_file(&path, content)?;

            // Make executable (Unix only)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&path)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&path, perms)?;
                println!("{} marked as executable (+x)", filename.green());
            }
        }
    }
    Ok(())
}

fn get_name(arg: Option<String>, prompt: &str) -> Result<String> {
    match arg {
        Some(n) => Ok(n),
        None => Ok(Input::new().with_prompt(prompt).interact_text()?),
    }
}

fn get_description(arg: Option<String>) -> Result<String> {
    match arg {
        Some(d) => Ok(d),
        None => Ok(Input::new().with_prompt("Description").interact_text()?),
    }
}
