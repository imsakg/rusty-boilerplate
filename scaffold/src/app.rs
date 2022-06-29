use crate::{
    cli::{Cli, Command},
    plan::Plan,
};
use std::path::Path;

pub fn run(cli: Cli) -> Result<String, String> {
    match cli.command {
        None => Ok("Rust project scaffolding; use --help for available commands.\n".into()),
        Some(Command::New {
            name,
            path,
            options,
        }) => {
            let root = path.unwrap_or_else(|| Path::new(&name).to_path_buf());
            if root.symlink_metadata().is_ok() {
                return Err(format!("destination already exists: {}", root.display()));
            }
            let plan = Plan::project(&name, options.template.as_deref().unwrap_or("bin"))?;
            let written = plan
                .write(&root)
                .map_err(|e| format!("creating {}: {e}", root.display()))?;
            Ok(format!(
                "Created {} ({} files)\n",
                root.display(),
                written.len()
            ))
        }
    }
}
