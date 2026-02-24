use crate::core::project::packages;

pub struct PackageSpec;

impl PackageSpec {
    const JEST_CONFIG: &str = r#"import nextJest from 'next/jest';
const createJestConfig = nextJest({ dir: './' });

const config = {
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
  },
};

export default createJestConfig(config);
"#;

    const JEST_SETUP: &str = "import '@testing-library/jest-dom';";

    pub fn new() -> anyhow::Result<packages::PackageSpec> {
        let package_name = packages::PackageName("jest".to_string());
        let version_spec = packages::VersionSpec(packages::Version::Latest);
        let config_files = Some(vec![
            ("jest.config.ts".to_string(), Self::JEST_CONFIG.to_string()),
            ("jest.setup.ts".to_string(), Self::JEST_SETUP.to_string()),
        ]);
        Ok(packages::PackageSpec {
            package_name,
            version_spec,
            config_files,
        })
    }
}
