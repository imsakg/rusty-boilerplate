use crate::{name::validate_name, plan::Plan, render::render};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub fn load(directory: &Path, name: &str, edition: &str) -> Result<Plan, String> {
    validate_name(name)?;
    let variables = BTreeMap::from([
        ("name".into(), name.into()),
        ("crate_name".into(), name.replace('-', "_")),
        ("edition".into(), edition.into()),
    ]);
    let mut files = Vec::new();
    visit(directory, directory, &variables, &mut files)?;
    Plan::from_files(files)
}

fn visit(
    root: &Path,
    directory: &Path,
    variables: &BTreeMap<String, String>,
    files: &mut Vec<(PathBuf, String)>,
) -> Result<(), String> {
    let metadata = directory
        .symlink_metadata()
        .map_err(|e| format!("reading {}: {e}", directory.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "template directory is not an ordinary directory: {}",
            directory.display()
        ));
    }
    let mut entries = std::fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        if kind.is_symlink() {
            return Err(format!(
                "template symlinks are not supported: {}",
                path.display()
            ));
        }
        if kind.is_dir() {
            visit(root, &path, variables, files)?;
        } else if kind.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_str()
                .ok_or("template paths must be UTF-8")?;
            let relative = relative.strip_suffix(".tpl").unwrap_or(relative);
            let target = PathBuf::from(render(relative, variables)?);
            if files.len() >= 1024
                || entry.metadata().map_err(|e| e.to_string())?.len() > 1024 * 1024
            {
                return Err("local templates are limited to 1024 files and 1 MiB per file".into());
            }
            let source = std::fs::read_to_string(&path)
                .map_err(|e| format!("reading template {}: {e}", path.display()))?;
            files.push((target, render(&source, variables)?));
        } else {
            return Err(format!("unsupported template entry: {}", path.display()));
        }
    }
    Ok(())
}
