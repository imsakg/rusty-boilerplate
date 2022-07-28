use crate::{
    cli::{Cli, Command},
    plan::Plan,
};
use std::path::Path;

pub fn run(cli: Cli) -> Result<String, String> {
    let config = cli.config;
    match cli.command {
        None => Ok("Rust project scaffolding; use --help for available commands.\n".into()),
        Some(Command::Init {
            path,
            name,
            options,
        }) => {
            let root = path
                .canonicalize()
                .map_err(|e| format!("opening {}: {e}", path.display()))?;
            let name = name
                .or_else(|| root.file_name().and_then(|n| n.to_str()).map(str::to_owned))
                .ok_or("cannot infer a project name; use --name")?;
            let settings = load_settings(config.as_deref(), &options)?;
            let mut plan = Plan::project(&name, &settings.template)?;
            settings.apply(&mut plan)?;
            if options.dry_run {
                plan.preflight(&root).map_err(|e| e.to_string())?;
                return Ok(plan
                    .files
                    .keys()
                    .map(|p| format!("Would create {}\n", root.join(p).display()))
                    .collect());
            }
            let files = plan.write(&root).map_err(|e| e.to_string())?;
            Ok(format!(
                "Initialized {} ({} files)\n",
                root.display(),
                files.len()
            ))
        }
        Some(Command::Templates { name }) => {
            let templates = if let Some(name) = name {
                vec![crate::template::lookup(&name)?]
            } else {
                crate::template::catalog().iter().collect()
            };
            Ok(templates
                .iter()
                .map(|t| format!("{}: {}\n", t.name, t.description))
                .collect())
        }
        Some(Command::New {
            name,
            path,
            options,
        }) => {
            let root = path.unwrap_or_else(|| Path::new(&name).to_path_buf());
            if root.symlink_metadata().is_ok() {
                return Err(format!("destination already exists: {}", root.display()));
            }
            let settings = load_settings(config.as_deref(), &options)?;
            let mut plan = Plan::project(&name, &settings.template)?;
            settings.apply(&mut plan)?;
            if options.dry_run {
                plan.preflight(&root).map_err(|e| e.to_string())?;
                return Ok(plan
                    .files
                    .keys()
                    .map(|p| format!("Would create {}\n", root.join(p).display()))
                    .collect());
            }
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

fn load_settings(
    path: Option<&Path>,
    options: &crate::cli::ProjectArgs,
) -> Result<crate::config::Settings, String> {
    let mut settings = if let Some(path) = path {
        crate::config::Settings::parse(
            &std::fs::read_to_string(path)
                .map_err(|e| format!("reading config {}: {e}", path.display()))?,
        )?
    } else {
        crate::config::Settings::default()
    };
    if let Some(template) = &options.template {
        settings.template = template.clone();
    }
    settings.validate()?;
    Ok(settings)
}
