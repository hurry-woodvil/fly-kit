use super::plan::Plan;

pub trait Feature: Send + Sync {
    fn key(&self) -> &'static str;
    fn plan(&self) -> anyhow::Result<Plan>;
}
