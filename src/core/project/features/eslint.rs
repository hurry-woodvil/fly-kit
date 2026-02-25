use crate::core::project::feature::Feature;
use crate::core::project::model::{
    DependencyKind, InstallPlan, InstallSpec, PackageName, PackageVersion, ProjectFile, Version,
};

pub struct EslintFeature;

impl Feature for EslintFeature {
    fn id(&self) -> &'static str {
        "eslint"
    }

    fn install_plans(&self) -> Vec<InstallPlan> {
        vec![InstallPlan {
            kind: DependencyKind::Dev,
            packages: vec![
                InstallSpec {
                    name: PackageName("eslint"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("eslint-config-next"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("eslint-config-prettier"),
                    version: PackageVersion(Version::Latest),
                },
            ],
        }]
    }

    fn files(&self) -> Vec<ProjectFile> {
        vec![ProjectFile {
            path: ".eslintrc.json".into(),
            content: include_str!("../../templates/.eslintrc.json"),
        }]
    }
}
