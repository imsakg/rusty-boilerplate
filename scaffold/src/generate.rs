use crate::{name::validate_name, plan::Plan};
use clap::ValueEnum;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Kind {
    Module,
}

pub fn file_plan(kind: Kind, name: &str) -> Result<Plan, String> {
    validate_name(name)?;
    if name.contains('-') {
        return Err(
            "file names must be Rust identifiers; use underscores instead of hyphens".into(),
        );
    }
    let (path, source) = match kind {
        Kind::Module => (
            format!("src/{name}.rs"),
            format!("//! {name} module.\n\npub fn run() {{}}\n"),
        ),
    };
    Plan::from_files([(PathBuf::from(path), source)])
}
