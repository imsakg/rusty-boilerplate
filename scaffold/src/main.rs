use clap::Parser;
use rusty_boilerplate::cli::Cli;

fn main() {
    let _cli = Cli::parse();
    println!("Rust project scaffolding; use --help for available commands.");
}
