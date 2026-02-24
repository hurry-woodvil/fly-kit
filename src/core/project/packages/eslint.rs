use crate::core::project::packages;

pub struct PackageSpec;

impl PackageSpec {
    const ESLINT: &str = r#"{
  "extends": ["next/core-web-vitals", "prettier"]
}
"#;

    pub fn new() -> anyhow::Result<packages::PackageSpec> {
        let package_name = packages::PackageName("eslint".to_string());
        let version_spec = packages::VersionSpec(packages::Version::Latest);
        let config_files = Some(vec![(
            ".eslintrc.json".to_string(),
            Self::ESLINT.to_string(),
        )]);
        Ok(packages::PackageSpec {
            package_name,
            version_spec,
            config_files,
        })
    }
}
