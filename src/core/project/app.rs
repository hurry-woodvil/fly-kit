use crate::core::utils;
use anyhow::Result;
use std::path::{Path, PathBuf};

mod about_page_tsx;
mod components_hello_tsx;
mod globals_css;
mod layout_tsx;
mod page_tsx;
mod tests_hello_test_tsx;

pub struct TemplateFile {
    dir: PathBuf,
    file_name: String,
    content: String,
}

impl TemplateFile {
    pub async fn create(&self) -> Result<()> {
        utils::write(self.dir.join(&self.file_name), &self.content).await?;
        Ok(())
    }
}

pub async fn create_template_files(project_dir: &Path) -> Result<()> {
    let src_dir = project_dir.join("src");

    let mut template_files: Vec<TemplateFile> = Vec::new();

    let layout_tsx = layout_tsx::Template::new(&src_dir)?;
    template_files.push(layout_tsx);

    let page_tsx = page_tsx::Template::new(&src_dir)?;
    template_files.push(page_tsx);

    let globals_css = globals_css::Template::new(&src_dir)?;
    template_files.push(globals_css);

    let about_page_tsx = about_page_tsx::Template::new(&src_dir)?;
    template_files.push(about_page_tsx);

    let components_hello_tsx = components_hello_tsx::Template::new(&src_dir)?;
    template_files.push(components_hello_tsx);

    let test_hello_test_tsx = tests_hello_test_tsx::Template::new(&src_dir)?;
    template_files.push(test_hello_test_tsx);

    for tf in template_files {
        tf.create().await?;
    }

    Ok(())
}
