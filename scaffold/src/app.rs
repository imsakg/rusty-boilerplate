use crate::{
    cli::{Cli, Command},
    plan::Plan,
};
use std::path::Path;

pub fn run(cli: Cli) -> Result<String, String> {
    let config = cli.config;
    match cli.command {
        Some(Command::Config {
            action: crate::cli::ConfigCommand::Show,
        }) => {
            let settings = load_settings(config.as_deref(), &crate::cli::ProjectArgs::default())?;
            toml::to_string_pretty(&settings).map_err(|e| e.to_string())
        }
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
    let paths = if let Some(path) = path {
        vec![path.to_path_buf()]
    } else {
        let user = std::env::var_os("XDG_CONFIG_HOME")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|p| std::path::PathBuf::from(p).join(".config"))
            })
            .map(|p| p.join("rusty/config.toml"));
        crate::config::discover(
            &std::env::current_dir().map_err(|e| e.to_string())?,
            user.as_deref(),
        )
    };
    let mut settings = crate::config::Settings::default();
    for path in paths {
        settings = settings.merge_toml(
            &std::fs::read_to_string(&path)
                .map_err(|e| format!("reading config {}: {e}", path.display()))?,
        )?;
    }
    settings = settings.merge_env(std::env::vars())?;
    if let Some(template) = &options.template {
        settings.template = template.clone();
    }
    if let Some(edition) = &options.edition {
        settings.edition = edition.clone();
    }
    if !options.authors.is_empty() {
        settings.authors = options.authors.clone();
    }
    settings.validate()?;
    Ok(settings)
}
