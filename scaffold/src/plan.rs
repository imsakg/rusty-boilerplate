use crate::template::project_files;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, PathBuf};

#[derive(Debug, Clone)]
pub struct Plan {
    pub files: BTreeMap<PathBuf, String>,
}

impl Plan {
    pub fn project(name: &str, kind: &str) -> Result<Self, String> {
        Self::from_files(project_files(name, kind)?)
    }

    pub fn from_files(files: impl IntoIterator<Item = (PathBuf, String)>) -> Result<Self, String> {
        let mut result = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for (path, content) in files {
            if path.as_os_str().is_empty()
                || path
                    .components()
                    .any(|part| !matches!(part, Component::Normal(_)))
                || path.to_string_lossy().contains('\\')
            {
                return Err(format!("unsafe generated path {}", path.display()));
            }
            if !seen.insert(path.to_string_lossy().to_ascii_lowercase()) {
                return Err(format!("duplicate generated path {}", path.display()));
            }
            result.insert(path, content);
        }
        Ok(Self { files: result })
    }
}
