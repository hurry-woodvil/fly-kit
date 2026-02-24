use crate::core::project::packages;

pub struct PackageSpec;

impl PackageSpec {
    const NEXT_CONFIG: &str = r#"const nextConfig = {};
export default nextConfig;
"#;

    const NEXT_ENV: &str = r#"/// <reference types="next" />
/// <reference types="next/image-types/global" />
"#;

    pub fn new() -> anyhow::Result<packages::PackageSpec> {
        let package_name = packages::PackageName("nextjs".to_string());
        let version_spec = packages::VersionSpec(packages::Version::Latest);
        let config_files = Some(vec![
            ("next.config.mjs".to_string(), Self::NEXT_CONFIG.to_string()),
            ("next-env.d.ts".to_string(), Self::NEXT_ENV.to_string()),
        ]);
        Ok(packages::PackageSpec {
            package_name,
            version_spec,
            config_files,
        })
    }
}
