use crate::core::project::app;
use std::path::PathBuf;

pub struct Template;

impl Template {
    const ABOUT: &str = r#"export default function AboutPage() {
  return <h1>About</h1>;
}
"#;

    pub fn new(project_dir: &PathBuf) -> anyhow::Result<app::TemplateFile> {
        let dir = project_dir.join("app").join("about");
        let file_name = "page.tsx".to_string();
        let content = Self::ABOUT.to_string();
        Ok(app::TemplateFile {
            dir,
            file_name,
            content,
        })
    }
}
