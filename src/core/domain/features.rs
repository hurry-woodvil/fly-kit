use crate::domain::feature::Feature;

mod eslint;
mod jest;
mod next;
mod prettier;
mod react;
mod typescript;

pub const DEFAULT_FUTURES: &[&str] = &["eslint", "jest", "next", "prettier", "react", "typescript"];

pub fn default_futures_keys() -> Vec<String> {
    DEFAULT_FUTURES.iter().map(|s| s.to_string()).collect()
}

pub fn resolve(keys: &[String]) -> anyhow::Result<Vec<Box<dyn Feature>>> {
    let mut out: Vec<Box<dyn Feature>> = vec![];

    for k in keys {
        match k.as_str() {
            "eslint" => out.push(Box::new(eslint::Eslint::default())),
            "jest" => out.push(Box::new(jest::Jest::default())),
            "next" => out.push(Box::new(next::Next::default())),
            "prettier" => out.push(Box::new(prettier::Prettier::default())),
            "react" => out.push(Box::new(react::React::default())),
            "typescript" | "ts" => out.push(Box::new(typescript::Typescript::default())),
            other => anyhow::bail!("unknown feature: {}", other),
        }
    }

    Ok(out)
}
