use crate::core::project::feature::Feature;
use crate::core::project::model::{
    DependencyKind, InstallPlan, InstallSpec, PackageName, PackageVersion, ProjectFile, Version,
};

pub struct TypescriptFeature;

impl Feature for TypescriptFeature {
    fn id(&self) -> &'static str {
        "typescript"
    }

    fn install_plans(&self) -> Vec<InstallPlan> {
        vec![InstallPlan {
            kind: DependencyKind::Dev,
            packages: vec![
                InstallSpec {
                    name: PackageName("typescript"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("@types/node"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("@types/jest"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("@types/react"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("@types/react-dom"),
                    version: PackageVersion(Version::Latest),
                },
            ],
        }]
    }

    fn files(&self) -> Vec<ProjectFile> {
        vec![ProjectFile {
            path: "tsconfig.json".into(),
            content: include_str!("../../templates/tsconfig.json"),
        }]
    }
}
