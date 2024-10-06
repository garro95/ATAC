use crate::request::request::Request;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use strum::Display;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<Arc<RwLock<Request>>>,

    #[serde(skip)]
    pub path: PathBuf,

    #[serde(skip)]
    pub file_format: CollectionFileFormat,
}

#[derive(Debug, Default, Copy, Clone, Display, Serialize, Deserialize)]
pub enum CollectionFileFormat {
    #[default]
    #[serde(alias = "json", alias = "JSON")]
    #[strum(to_string = "json")]
    Json,
    #[serde(alias = "yaml", alias = "YAML")]
    #[strum(to_string = "yaml")]
    Yaml,
}
