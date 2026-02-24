use crate::core::project::packages;

pub struct PackageSpec;

impl PackageSpec {
    const PRETTIER: &str = r#"{
  "semi": true,
  "singleQuote": true,
  "printWidth": 100
}
"#;

    const PRETTIER_IGNORE: &str = ".next\nnode_modules\n";

    pub fn new() -> anyhow::Result<packages::PackageSpec> {
        let package_name = packages::PackageName("prettier".to_string());
        let version_spec = packages::VersionSpec(packages::Version::Latest);
        let config_files = Some(vec![
            (".prettierrc.json".to_string(), Self::PRETTIER.to_string()),
            (
                ".prettierignore".to_string(),
                Self::PRETTIER_IGNORE.to_string(),
            ),
        ]);
        Ok(packages::PackageSpec {
            package_name,
            version_spec,
            config_files,
        })
    }
}
