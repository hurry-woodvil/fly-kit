use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PackageJson {
    name: String,
    version: String,
    private: bool,
    #[serde(rename = "type")]
    package_type: String,
}

impl PackageJson {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "0.1.0".to_string(),
            private: true,
            package_type: "module".to_string(),
        }
    }
}
