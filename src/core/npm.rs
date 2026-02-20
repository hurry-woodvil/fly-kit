use crate::core::command;
use std::{fmt, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageName(String);

impl PackageName {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for PackageName {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.trim();
        if s.is_empty() {
            anyhow::bail!("Package name cannot be empty");
        }
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionSpec(String);

impl TryFrom<&str> for VersionSpec {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.trim();
        if s.is_empty() {
            anyhow::bail!("Version specification cannot be empty");
        }
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for VersionSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallSpec {
    pub name: PackageName,
    pub version: Option<VersionSpec>,
}

impl InstallSpec {
    pub fn new(name: impl TryInto<PackageName, Error = anyhow::Error>) -> anyhow::Result<Self> {
        Ok(Self {
            name: name.try_into()?,
            version: None,
        })
    }

    pub fn with_version(
        name: impl TryInto<PackageName, Error = anyhow::Error>,
        version: impl TryInto<VersionSpec, Error = anyhow::Error>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            name: name.try_into()?,
            version: Some(version.try_into()?),
        })
    }

    pub fn to_npm_arg(&self) -> String {
        match &self.version {
            Some(v) => format!("{}@{}", self.name, v),
            None => self.name.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DependencyKind {
    Prod,
    Dev,
}

#[derive(Debug, Clone)]
pub struct InstallPlan {
    pub kind: DependencyKind,
    pub packages: Vec<InstallSpec>,
}

impl InstallPlan {
    pub async fn npm_command(&self, project_dir: &Path) -> anyhow::Result<()> {
        let mut args: Vec<&str> = Vec::new();

        if matches!(self.kind, DependencyKind::Dev) {
            args.push("-D");
        }

        let packages: Vec<&str> = self.packages.iter().map(|p| p.name.as_str()).collect();
        args.extend(packages);

        command::run_cmd(project_dir, "npm", args).await?;

        Ok(())
    }
}

pub async fn install_dependencies(project_dir: &Path) -> anyhow::Result<()> {
    let nextjs_spec = InstallSpec::new("nextjs")?;
    let react_spec = InstallSpec::new("react")?;
    let react_dom_spec = InstallSpec::new("react-dom")?;

    let plan = InstallPlan {
        kind: DependencyKind::Prod,
        packages: vec![nextjs_spec, react_spec, react_dom_spec],
    };

    plan.npm_command(project_dir).await?;

    Ok(())
}

pub async fn install_dev_dependencies(project_dir: &Path) -> anyhow::Result<()> {
    let typescript_spec = InstallSpec::new("typescript")?;
    let types_react_spec = InstallSpec::new("@types/react")?;
    let types_react_dom_spec = InstallSpec::new("@types/react-dom")?;
    let types_node_spec = InstallSpec::new("@types/node")?;
    let eslint_spec = InstallSpec::new("eslint")?;
    let eslint_config_next_spec = InstallSpec::new("eslint-config-next")?;
    let prettier_spec = InstallSpec::new("prettier")?;
    let eslint_config_prettier_spec = InstallSpec::new("eslint-config-prettier")?;
    let jest_spec = InstallSpec::new("jest")?;
    let jest_enviroment_jsdom_spec = InstallSpec::new("jest-environment-jsdom")?;
    let types_jest_spec = InstallSpec::new("@types/jest")?;
    let testing_library_react_spec = InstallSpec::new("@types/react")?;
    let testing_library_jest_dom_spec = InstallSpec::new("@types/jest-dom")?;
    let testing_library_user_event_spec = InstallSpec::new("@types/user-event")?;

    let plan = InstallPlan {
        kind: DependencyKind::Dev,
        packages: vec![
            typescript_spec,
            types_react_spec,
            types_react_dom_spec,
            types_node_spec,
            eslint_spec,
            eslint_config_next_spec,
            prettier_spec,
            eslint_config_prettier_spec,
            jest_spec,
            jest_enviroment_jsdom_spec,
            types_jest_spec,
            testing_library_react_spec,
            testing_library_jest_dom_spec,
            testing_library_user_event_spec,
        ],
    };

    plan.npm_command(project_dir).await?;

    Ok(())
}
