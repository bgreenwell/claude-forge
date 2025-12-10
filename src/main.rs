mod commands;
mod templates;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{add, init}; // Only import used commands

#[derive(Parser)]
#[command(name = "cforge")]
#[command(about = "Scaffold and manage Claude Code marketplaces", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Marketplace repository
    Init {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Create a new Plugin in the current marketplace
    NewPlugin {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Add a component (Skill, Command, etc) to a plugin
    Add {
        /// Target a specific plugin by name (optional)
        #[arg(short, long)]
        plugin: Option<String>,

        #[command(subcommand)]
        component: add::Component,
    },
    /// Validate a plugin's structure and syntax
    Validate {
        /// Path to plugin (defaults to current dir)
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Register a local plugin into the marketplace.json registry
    Register {
        /// Relative path to the plugin folder (e.g. ./plugins/my-tool)
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, description } => init::init_marketplace(name, description)?,
        Commands::NewPlugin { name, description } => init::init_plugin(name, description)?,
        Commands::Add { plugin, component } => add::run(plugin, component)?,
        Commands::Validate { path } => commands::validate::validate_plugin(path)?,
        Commands::Register { path } => commands::register::register_plugin(path)?,
    }

    Ok(())
}
