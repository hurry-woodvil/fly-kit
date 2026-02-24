use crate::core::project::packages;

pub struct PackageSpec;

impl PackageSpec {
    pub fn new() -> anyhow::Result<packages::PackageSpec> {
        let package_name = packages::PackageName("eslint-config-prettier".to_string());
        let version_spec = packages::VersionSpec(packages::Version::Latest);
        let config_files = None;
        Ok(packages::PackageSpec {
            package_name,
            version_spec,
            config_files,
        })
    }
}
