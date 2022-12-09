use clap::Parser;
use rusty_boilerplate::{app, cli::Cli};
use std::{
    io::{self, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let json = cli.json;
    if cli.verbose > 0 {
        eprintln!(
            "rusty: running in {}",
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "<unavailable>".into())
        );
    }
    let (message, code) = match app::run(cli) {
        Ok(message) => {
            let text = if json {
                format!("{}\n", serde_json::json!({"ok": true, "message": message}))
            } else {
                message
            };
            (text, ExitCode::SUCCESS)
        }
        Err(error) => {
            if !json {
                eprintln!("error: {error}");
                return ExitCode::FAILURE;
            }
            (
                format!("{}\n", serde_json::json!({"ok": false, "error": error})),
                ExitCode::FAILURE,
            )
        }
    };
    match io::stdout().lock().write_all(message.as_bytes()) {
        Ok(()) => code,
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: writing output: {error}");
            ExitCode::FAILURE
        }
    }
}
