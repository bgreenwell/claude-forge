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
    /// Validate a plugin or marketplace structure and syntax
    Validate {
        /// Path to plugin or marketplace (defaults to current dir)
        #[arg(short, long)]
        path: Option<String>,

        /// Stop validation at first error
        #[arg(long)]
        fail_fast: bool,
    },

    /// Register a local plugin into the marketplace.json registry
    Register {
        /// Relative path to the plugin folder (e.g. ./plugins/my-tool)
        path: String,
    },

    /// List plugins in marketplace or components in plugin
    List {
        /// Path to marketplace or plugin (defaults to current dir)
        #[arg(short, long)]
        path: Option<String>,

        /// Show verbose output (description, path, status)
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, description } => init::init_marketplace(name, description)?,
        Commands::NewPlugin { name, description } => init::init_plugin(name, description)?,
        Commands::Add { plugin, component } => add::run(plugin, component)?,
        Commands::Validate { path, fail_fast } => commands::validate::validate(path, fail_fast)?,
        Commands::Register { path } => commands::register::register_plugin(path)?,
        Commands::List { path, verbose } => commands::list::list(path, verbose)?,
    }

    Ok(())
}
