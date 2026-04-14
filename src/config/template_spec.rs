use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TemplateSpec {
    #[serde(rename = "api_service")]
    pub service_config: HashMap<String, TemplateConfig>,

    #[serde(rename = "global")]
    pub global_config: HashMap<String, TemplateConfig>
}

#[derive(Debug, Deserialize)]
pub struct TemplateConfig {
    paths: TemplatePath,

    #[serde(default)]
    files: HashMap<String, String>,

    #[serde(default)]
    dependencies: HashMap<String, TemplateDependencies>, 
}

#[derive(Debug, Deserialize)]
struct TemplatePath {
    fetch_path: String,
    build_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TemplateDependencies {
    Version(String),
    Detailed {
        version: String,
        #[serde(default)]
        features: Vec<String>,
    }
}