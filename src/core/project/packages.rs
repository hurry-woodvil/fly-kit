use crate::core::utils;
use std::{fmt, path::Path};

pub mod eslint;
pub mod eslint_config_next;
pub mod eslint_config_prettier;
pub mod jest;
pub mod jest_enviroment_jsdom;
pub mod next;
pub mod nextjs;
pub mod prettier;
pub mod react;
pub mod react_dom;
pub mod testing_libarary_user_event;
pub mod testing_library_jest_dom;
pub mod testing_library_react;
pub mod types_jest;
pub mod types_node;
pub mod types_react;
pub mod types_react_dom;
pub mod typescript;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageName(pub String);

impl PackageName {
    pub fn as_str(&self) -> &str {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Version {
    Latest,
    Pin(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionSpec(pub Version);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSpec {
    pub package_name: PackageName,
    pub version_spec: VersionSpec,
    pub config_files: Option<Vec<(String, String)>>,
}

impl PackageSpec {
    pub fn new(
        package_name: PackageName,
        version_spec: VersionSpec,
        config_files: Option<Vec<(String, String)>>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            package_name,
            version_spec,
            config_files,
        })
    }

    pub fn to_npm_arg(&self) -> String {
        match &self.version_spec.0 {
            Version::Latest => self.package_name.to_string(),
            Version::Pin(v) => format!("{}@{}", self.package_name.to_string(), v.to_string()),
        }
    }

    async fn create_config_files(&self, project_dir: &Path) -> anyhow::Result<()> {
        if let Some(config_files) = &self.config_files {
            for (file_name, content) in config_files {
                utils::write(project_dir.join(file_name), content).await?;
            }
        }

        Ok(())
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
    pub packages: Vec<PackageSpec>,
}

impl InstallPlan {
    pub async fn npm_command(&self, project_dir: &Path) -> anyhow::Result<()> {
        let mut args = vec!["i"];

        if matches!(self.kind, DependencyKind::Dev) {
            args.push("-D");
        }

        let packages: Vec<&str> = self
            .packages
            .iter()
            .map(|p| p.package_name.as_str())
            .collect();
        args.extend(packages);

        // npm install or npm install -D
        utils::run_cmd(project_dir, "npm", args).await?;

        for p in &self.packages {
            p.create_config_files(project_dir).await?;
        }

        Ok(())
    }
}

pub struct PackageBuilder;

impl PackageBuilder {
    pub fn package_build() -> anyhow::Result<Vec<PackageSpec>> {
        let mut package_specs: Vec<PackageSpec> = Vec::new();

        // next spec build
        package_specs.push(next::PackageSpec::new()?);

        // nextjs spec build
        package_specs.push(nextjs::PackageSpec::new()?);

        // react spec build
        package_specs.push(react::PackageSpec::new()?);

        // react-dom spec build
        package_specs.push(react_dom::PackageSpec::new()?);

        Ok(package_specs)
    }

    pub fn dev_package_build() -> anyhow::Result<Vec<PackageSpec>> {
        let mut package_specs: Vec<PackageSpec> = Vec::new();

        // typescript spec build
        package_specs.push(typescript::PackageSpec::new()?);

        // @types/react spec build
        package_specs.push(types_react::PackageSpec::new()?);

        // @types/react-dom spec build
        package_specs.push(types_react_dom::PackageSpec::new()?);

        // @types/node spec build
        package_specs.push(types_node::PackageSpec::new()?);

        // eslint spec build
        package_specs.push(eslint::PackageSpec::new()?);

        // eslint-config-next spec build
        package_specs.push(eslint_config_next::PackageSpec::new()?);

        // prettier spec build
        package_specs.push(prettier::PackageSpec::new()?);

        // eslint-config-prettier spec build
        package_specs.push(eslint_config_prettier::PackageSpec::new()?);

        // jest spec build
        package_specs.push(jest::PackageSpec::new()?);

        // jest-environment-jsdom spec build
        package_specs.push(jest_enviroment_jsdom::PackageSpec::new()?);

        // @types/jest spec build
        package_specs.push(types_jest::PackageSpec::new()?);

        // @testing-library/react spec build
        package_specs.push(testing_library_react::PackageSpec::new()?);

        // @testing-library/jest-dom spec build
        package_specs.push(testing_library_jest_dom::PackageSpec::new()?);

        // @testing-library/user-event spec build
        package_specs.push(testing_libarary_user_event::PackageSpec::new()?);

        Ok(package_specs)
    }
}
