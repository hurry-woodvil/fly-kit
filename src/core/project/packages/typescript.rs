use crate::core::project::packages;

pub struct PackageSpec;

impl PackageSpec {
    const TSCONFIG: &str = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["dom", "dom.iterable", "esnext"],
    "strict": true,
    "noEmit": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "jsx": "preserve",
    "baseUrl": ".",
    "paths": { "@/*": ["src/*"] },
    "plugins": [{ "name": "next" }]
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx"],
  "exclude": ["node_modules"]
}
"#;

    pub fn new() -> anyhow::Result<packages::PackageSpec> {
        let package_name = packages::PackageName("typescript".to_string());
        let version_spec = packages::VersionSpec(packages::Version::Latest);
        let config_files = Some(vec![(
            "tsconfig.json".to_string(),
            Self::TSCONFIG.to_string(),
        )]);
        Ok(packages::PackageSpec {
            package_name,
            version_spec,
            config_files,
        })
    }
}
