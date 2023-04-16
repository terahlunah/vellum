use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookConfig {
    pub meta: MetaConfig,
    pub content: ContentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaConfig {
    pub title: String,
    pub subtitle: Option<String>,
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentConfig {
    pub root: PathBuf,
}
