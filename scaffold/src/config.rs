use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Settings {
    pub template: String,
    pub edition: String,
    pub authors: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Overlay {
    template: Option<String>,
    edition: Option<String>,
    authors: Option<Vec<String>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            template: "bin".into(),
            edition: "2021".into(),
            authors: Vec::new(),
        }
    }
}

impl Settings {
    pub fn merge_env(
        mut self,
        values: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> Result<Self, String> {
        for (key, value) in values {
            match key.as_ref() {
                "RUSTY_TEMPLATE" => self.template = value.as_ref().into(),
                "RUSTY_EDITION" => self.edition = value.as_ref().into(),
                "RUSTY_AUTHORS" => {
                    self.authors = value
                        .as_ref()
                        .split(';')
                        .map(|v| v.trim().to_owned())
                        .collect()
                }
                _ => {}
            }
        }
        self.validate()?;
        Ok(self)
    }

    pub fn parse(source: &str) -> Result<Self, String> {
        Self::default().merge_toml(source)
    }

    pub fn merge_toml(mut self, source: &str) -> Result<Self, String> {
        let overlay: Overlay =
            toml::from_str(source).map_err(|e| format!("invalid configuration: {e}"))?;
        if let Some(value) = overlay.template {
            self.template = value;
        }
        if let Some(value) = overlay.edition {
            self.edition = value;
        }
        if let Some(value) = overlay.authors {
            self.authors = value;
        }
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), String> {
        crate::template::lookup(&self.template)?;
        if !["2018", "2021"].contains(&self.edition.as_str()) {
            return Err(format!(
                "unsupported edition {:?}; choose 2018 or 2021",
                self.edition
            ));
        }
        if self
            .authors
            .iter()
            .any(|a| a.trim().is_empty() || a.contains(['\n', '\r']))
        {
            return Err("authors must be nonempty single-line values".into());
        }
        Ok(())
    }

    pub fn apply(&self, plan: &mut crate::plan::Plan) -> Result<(), String> {
        self.validate()?;
        let source = plan
            .files
            .get_mut(std::path::Path::new("Cargo.toml"))
            .ok_or("plan has no Cargo.toml")?;
        let mut manifest: toml::Value = source
            .parse()
            .map_err(|e| format!("invalid manifest: {e}"))?;
        manifest["package"]["edition"] = toml::Value::String(self.edition.clone());
        manifest["package"]
            .as_table_mut()
            .ok_or("manifest has no package table")?
            .insert(
                "authors".into(),
                toml::Value::Array(
                    self.authors
                        .iter()
                        .cloned()
                        .map(toml::Value::String)
                        .collect(),
                ),
            );
        *source = toml::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// User defaults followed by the nearest ancestor `.rusty.toml`.
pub fn discover(
    start: &std::path::Path,
    user_config: Option<&std::path::Path>,
) -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    if let Some(path) = user_config.filter(|p| p.exists()) {
        paths.push(path.to_path_buf());
    }
    if let Some(path) = start
        .ancestors()
        .map(|p| p.join(".rusty.toml"))
        .find(|p| p.exists())
    {
        if !paths.contains(&path) {
            paths.push(path);
        }
    }
    paths
}
