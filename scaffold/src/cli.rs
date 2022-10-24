use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Rust project scaffolding
#[derive(Debug, Parser)]
#[command(name = "rusty", version, about = "Rust project scaffolding")]
pub struct Cli {
    /// Increase diagnostic detail
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
    /// Read defaults from this TOML file
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Default, Args)]
pub struct ProjectArgs {
    /// Built-in project template
    #[arg(short, long)]
    pub template: Option<String>,
    /// Print planned files without writing anything
    #[arg(long)]
    pub dry_run: bool,
    /// Override the Rust edition
    #[arg(long, value_parser = ["2018", "2021"])]
    pub edition: Option<String>,
    /// Set an author (repeat for multiple authors)
    #[arg(long = "author")]
    pub authors: Vec<String>,
    /// SPDX license choice for generated projects
    #[arg(long, value_parser = ["MIT", "Apache-2.0", "MIT OR Apache-2.0"])]
    pub license: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate a source file inside a Cargo project
    Generate {
        #[arg(value_enum)]
        kind: crate::generate::Kind,
        name: String,
        #[arg(long, default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    /// Inspect or initialize configuration
    Config {
        #[command(subcommand)]
        action: ConfigCommand,
    },
    /// Add a project to an existing directory without overwriting files
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        name: Option<String>,
        #[command(flatten)]
        options: ProjectArgs,
    },
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

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Write default settings to a new file
    Init {
        #[arg(default_value = ".rusty.toml")]
        path: PathBuf,
    },
    /// Print effective settings after configuration and environment layering
    Show,
}
