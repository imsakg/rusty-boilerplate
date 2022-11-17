use clap::Parser;
use rusty_boilerplate::{app, cli::Cli};

fn main() {
    let cli = Cli::parse();
    let json = cli.json;
    match app::run(cli) {
        Ok(message) => {
            if json {
                println!("{}", serde_json::json!({"ok": true, "message": message}));
            } else {
                print!("{message}");
            }
        }
        Err(error) => {
            if json {
                println!("{}", serde_json::json!({"ok": false, "error": error}));
            } else {
                eprintln!("error: {error}");
            }
            std::process::exit(1);
        }
    }
}
