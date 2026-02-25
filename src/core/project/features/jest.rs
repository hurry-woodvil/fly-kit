use crate::core::project::feature::Feature;
use crate::core::project::model::{
    DependencyKind, InstallPlan, InstallSpec, PackageName, PackageVersion, ProjectFile, Version,
};

pub struct JestFeature;

impl Feature for JestFeature {
    fn id(&self) -> &'static str {
        "jest"
    }

    fn install_plans(&self) -> Vec<InstallPlan> {
        vec![InstallPlan {
            kind: DependencyKind::Dev,
            packages: vec![
                InstallSpec {
                    name: PackageName("jest"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("@testing-library/react"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("@testing-library/jest-dom"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("@testing-library/user-event"),
                    version: PackageVersion(Version::Latest),
                },
                InstallSpec {
                    name: PackageName("jest-environment-jsdom"),
                    version: PackageVersion(Version::Latest),
                },
            ],
        }]
    }

    fn files(&self) -> Vec<ProjectFile> {
        vec![
            ProjectFile {
                path: "jest.config.ts".into(),
                content: include_str!("../../templates/jest.config.ts"),
            },
            ProjectFile {
                path: "jest.setup.ts".into(),
                content: include_str!("../../templates/jest.setup.ts"),
            },
        ]
    }
}
