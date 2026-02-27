use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct InitProjectRequest {
    /// project name
    pub project_name: String,
    /// project directory
    pub dir: PathBuf,
    /// witch feature enable
    pub features: Vec<String>,
    /// witch package manager use
    pub package_manager: PackageManagerKind,
}

impl InitProjectRequest {
    pub fn project_dir(&self) -> PathBuf {
        self.dir.join(&self.project_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManagerKind {
    Npm,
}

impl PackageManagerKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PackageManagerKind::Npm => "npm",
        }
    }
}

impl std::str::FromStr for PackageManagerKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "npm" => Ok(PackageManagerKind::Npm),
            other => anyhow::bail!("unknown package manager: {}", other),
        }
    }
}
