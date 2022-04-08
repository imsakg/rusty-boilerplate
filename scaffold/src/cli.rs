use clap::Parser;

/// Rust project scaffolding
#[derive(Debug, Parser)]
#[command(name = "rusty", version, about = "Rust project scaffolding")]
pub struct Cli {
    /// Increase diagnostic detail
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}
