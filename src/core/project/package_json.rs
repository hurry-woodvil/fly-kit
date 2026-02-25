use anyhow::Context;
use std::{collections::BTreeMap, path::Path};

use serde::Serialize;
use tokio::fs;

#[derive(Debug, Serialize)]
pub struct PackageJson {
    name: String,
    version: String,
    private: bool,
    #[serde(rename = "type")]
    package_type: String,
    scripts: BTreeMap<String, String>,
}

impl PackageJson {
    pub fn new(name: String) -> Self {
        let mut scripts: BTreeMap<String, String> = BTreeMap::new();
        scripts.insert("dev".to_string(), "next dev".to_string());
        scripts.insert("build".to_string(), "next build".to_string());
        scripts.insert("start".to_string(), "next start".to_string());
        scripts.insert("lint".to_string(), "next lint".to_string());
        scripts.insert("format".to_string(), "prettier . --check".to_string());
        scripts.insert("format:write".to_string(), "prettier . --write".to_string());
        scripts.insert("test".to_string(), "jest".to_string());
        scripts.insert("test:watch".to_string(), "jest  --watch".to_string());
        Self {
            name,
            version: "0.1.0".to_string(),
            private: true,
            package_type: "module".to_string(),
            scripts,
        }
    }

    pub async fn write_package_json(&self, project_dir: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize package.")?;

        let path = project_dir.join("package.json");
        fs::write(&path, format!("{json}\n"))
            .await
            .with_context(|| format!("Failed to write file: {}", path.display()))?;

        Ok(())
    }
}
