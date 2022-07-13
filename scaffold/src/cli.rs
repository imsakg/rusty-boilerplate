use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Rust project scaffolding
#[derive(Debug, Parser)]
#[command(name = "rusty", version, about = "Rust project scaffolding")]
pub struct Cli {
    /// Increase diagnostic detail
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Args)]
pub struct ProjectArgs {
    /// Built-in project template
    #[arg(short, long)]
    pub template: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List templates, or describe one by name
    Templates { name: Option<String> },
    /// Create a new project in a new directory
    New {
        name: String,
        #[arg(long)]
        path: Option<PathBuf>,
        #[command(flatten)]
        options: ProjectArgs,
    },
}
