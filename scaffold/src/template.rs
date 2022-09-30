use crate::{name::validate_name, render::render};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Render files in memory; paths are relative to the requested destination.
pub fn project_files(name: &str, kind: &str) -> Result<BTreeMap<PathBuf, String>, String> {
    validate_name(name)?;
    lookup(kind)?;
    let variables = BTreeMap::from([
        ("name".into(), name.into()),
        ("crate_name".into(), name.replace('-', "_")),
        ("edition".into(), "2021".into()),
    ]);
    let mut files = BTreeMap::new();
    files.insert(
        PathBuf::from("Cargo.toml"),
        render(include_str!("../templates/Cargo.toml.tpl"), &variables)?,
    );
    files.insert(PathBuf::from(".gitignore"), "/target/\n".into());
    match kind {
        "bin" => {
            files.insert(
                PathBuf::from("src/main.rs"),
                render(include_str!("../templates/bin/main.rs.tpl"), &variables)?,
            );
        }
        "cli" => {
            files.insert(
                PathBuf::from("src/main.rs"),
                render(include_str!("../templates/cli/main.rs.tpl"), &variables)?,
            );
            files
                .get_mut(std::path::Path::new("Cargo.toml"))
                .expect("manifest inserted")
                .push_str("clap = { version = \"4\", features = [\"derive\"] }\n");
        }
        "lib" => {
            files.insert(
                PathBuf::from("src/lib.rs"),
                render(include_str!("../templates/lib/lib.rs.tpl"), &variables)?,
            );
        }
        _ => return Err(format!("template {kind:?} has no renderer yet")),
    }
    Ok(files)
}

#[derive(Debug, Clone, Copy)]
pub struct Template {
    pub name: &'static str,
    pub description: &'static str,
}

pub fn catalog() -> &'static [Template] {
    &[
        Template {
            name: "bin",
            description: "Dependency-free binary application",
        },
        Template {
            name: "cli",
            description: "Clap-based command-line application",
        },
        Template {
            name: "lib",
            description: "Library with a unit test",
        },
    ]
}

pub fn lookup(name: &str) -> Result<&'static Template, String> {
    catalog()
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| format!("unknown template {name:?}; see `rusty templates`"))
}
