mod eslint;
mod jest;
mod next;
mod prettier;
mod react;
mod typescript;

use crate::core::project::feature::Feature;

pub fn default_features() -> Vec<Box<dyn Feature>> {
    vec![
        Box::new(eslint::EslintFeature),
        Box::new(jest::JestFeature),
        Box::new(next::NextFeature),
        Box::new(prettier::PrettierFeature),
        Box::new(react::ReactFeature),
        Box::new(typescript::TypescriptFeature),
    ]
}

pub fn resolve_feature(kind: &str) -> anyhow::Result<Box<dyn Feature>> {
    match kind {
        "eslint" => Ok(Box::new(eslint::EslintFeature)),
        "jest" => Ok(Box::new(jest::JestFeature)),
        "next" => Ok(Box::new(next::NextFeature)),
        "prettier" => Ok(Box::new(prettier::PrettierFeature)),
        "react" => Ok(Box::new(react::ReactFeature)),
        "typescript" => Ok(Box::new(typescript::TypescriptFeature)),
        other => anyhow::bail!("unknown feature: {}", other),
    }
}

pub fn resolve_features(kinds: &[&str]) -> anyhow::Result<Vec<Box<dyn Feature>>> {
    kinds.iter().map(|k| resolve_feature(k)).collect()
}
