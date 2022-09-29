use crate::{name::validate_name, plan::Plan};
use clap::ValueEnum;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Kind {
    Module,
    Example,
    Test,
}

pub fn file_plan(kind: Kind, name: &str) -> Result<Plan, String> {
    validate_name(name)?;
    if name.contains('-') {
        return Err(
            "file names must be Rust identifiers; use underscores instead of hyphens".into(),
        );
    }
    let (path, source) = match kind {
        Kind::Example => (format!("examples/{name}.rs"), format!("fn main() {{\n    println!(\"Example: {name}\");\n}}\n")),
        Kind::Test => (format!("tests/{name}.rs"), "#[test]\nfn manifest_exists() {\n    assert!(std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"Cargo.toml\").is_file());\n}\n".into()),
        Kind::Module => (
            format!("src/{name}.rs"),
            format!("//! {name} module.\n\npub fn run() {{}}\n"),
        ),
    };
    Plan::from_files([(PathBuf::from(path), source)])
}

/// Find the nearest Cargo package; virtual workspace roots are not packages.
pub fn find_project(start: &std::path::Path) -> Result<PathBuf, String> {
    let start = start
        .canonicalize()
        .map_err(|e| format!("opening {}: {e}", start.display()))?;
    for path in start.ancestors() {
        let manifest = path.join("Cargo.toml");
        if manifest.is_file() {
            let source = std::fs::read_to_string(&manifest).map_err(|e| e.to_string())?;
            let value: toml::Value = source
                .parse()
                .map_err(|e| format!("invalid {}: {e}", manifest.display()))?;
            if value.get("package").is_some() {
                return Ok(path.to_path_buf());
            }
        }
    }
    Err(format!("no Cargo package found from {}", start.display()))
}
