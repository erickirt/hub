use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub general: Option<General>,
    pub providers: Vec<Provider>,
    pub models: Vec<ModelConfig>,
    pub pipelines: Vec<Pipeline>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct General {
    #[serde(default = "default_trace_content_enabled")]
    pub trace_content_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Provider {
    pub key: String,
    pub r#type: String,
    #[serde(default = "no_api_key")]
    pub api_key: String,
    #[serde(flatten)]
    pub params: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelConfig {
    pub key: String,
    pub r#type: String,
    pub provider: String,
    #[serde(flatten)]
    pub params: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PipelineType {
    Chat,
    Completion,
    Embeddings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pipeline {
    pub name: String,
    pub r#type: PipelineType,
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    pub plugins: Vec<PluginConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum PluginConfig {
    Logging {
        #[serde(default = "default_log_level")]
        level: String,
    },
    Tracing {
        endpoint: String,
        api_key: String,
    },
    ModelRouter {
        models: Vec<String>,
    },
}

fn default_trace_content_enabled() -> bool {
    true
}

fn default_log_level() -> String {
    "warning".to_string()
}

fn no_api_key() -> String {
    "".to_string()
}
