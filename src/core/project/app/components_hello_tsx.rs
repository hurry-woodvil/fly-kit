use crate::core::project::app;
use std::path::PathBuf;

pub struct Template;

impl Template {
    const HELLO: &str = r#"export function Hello({ name }: { name: string }) {
  return <p>Hello {name}</p>;
}
"#;

    pub fn new(project_dir: &PathBuf) -> anyhow::Result<app::TemplateFile> {
        let dir = project_dir.join("components");
        let file_name = "Hello.tsx".to_string();
        let content = Self::HELLO.to_string();
        Ok(app::TemplateFile {
            dir,
            file_name,
            content,
        })
    }
}
