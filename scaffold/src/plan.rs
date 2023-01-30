use crate::template::project_files;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, PathBuf};

#[derive(Debug, Clone)]
pub struct Plan {
    pub files: BTreeMap<PathBuf, String>,
}

impl Plan {
    /// Create files using exclusive opens: existing files are never truncated.
    pub fn write(&self, root: &std::path::Path) -> std::io::Result<Vec<PathBuf>> {
        use std::io::Write;
        self.preflight(root)?;
        let mut written = Vec::new();
        for (path, contents) in &self.files {
            let target = root.join(path);
            std::fs::create_dir_all(target.parent().expect("relative file has a parent"))?;
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&target)?;
            file.write_all(contents.as_bytes())?;
            written.push(target);
        }
        Ok(written)
    }

    pub fn preflight(&self, root: &std::path::Path) -> std::io::Result<()> {
        // Public plan contents may have been edited after construction.
        Self::from_files(self.files.keys().cloned().map(|p| (p, String::new())))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
        for path in self.files.keys() {
            let target = root.join(path);
            if target.symlink_metadata().is_ok() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    format!("refusing to overwrite {}", target.display()),
                ));
            }
            for ancestor in target
                .parent()
                .into_iter()
                .flat_map(|p| p.ancestors())
                .take_while(|p| p.starts_with(root))
            {
                if let Ok(metadata) = ancestor.symlink_metadata() {
                    if metadata.file_type().is_symlink() || !metadata.is_dir() {
                        return Err(std::io::Error::other(format!(
                            "unsafe output ancestor {}",
                            ancestor.display()
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn project(name: &str, kind: &str) -> Result<Self, String> {
        Self::from_files(project_files(name, kind)?)
    }

    pub fn from_files(files: impl IntoIterator<Item = (PathBuf, String)>) -> Result<Self, String> {
        let mut result = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for (path, content) in files {
            if path.components().any(|part| {
                let part = part.as_os_str().to_string_lossy();
                let stem = part
                    .split('.')
                    .next()
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                let device = ["con", "prn", "aux", "nul"].contains(&stem.as_str())
                    || ((stem.starts_with("com") || stem.starts_with("lpt"))
                        && stem.len() == 4
                        && matches!(stem.as_bytes()[3], b'1'..=b'9'));
                device
                    || part.ends_with(['.', ' '])
                    || part.chars().any(char::is_control)
                    || part.eq_ignore_ascii_case(".git")
                    || part.contains([':', '<', '>', '|', '?', '*'])
            }) {
                return Err(format!(
                    "nonportable or repository-control path {}",
                    path.display()
                ));
            }
            if path.as_os_str().is_empty()
                || path
                    .components()
                    .any(|part| !matches!(part, Component::Normal(_)))
                || path.to_string_lossy().contains('\\')
            {
                return Err(format!("unsafe generated path {}", path.display()));
            }
            let path: PathBuf = path.components().collect();
            if !seen.insert(path.to_string_lossy().to_ascii_lowercase()) {
                return Err(format!("duplicate generated path {}", path.display()));
            }
            result.insert(path, content);
        }
        for path in result.keys() {
            if path
                .ancestors()
                .skip(1)
                .any(|ancestor| seen.contains(&ancestor.to_string_lossy().to_ascii_lowercase()))
            {
                return Err(format!(
                    "generated file is also a directory: {}",
                    path.display()
                ));
            }
        }
        Ok(Self { files: result })
    }
}
