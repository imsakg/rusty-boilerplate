use clap::Parser;
use rusty_boilerplate::{app, cli::Cli};

fn main() {
    match app::run(Cli::parse()) {
        Ok(message) => print!("{message}"),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
