use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct JavaRuntimesManifest {
    #[serde(flatten)]
    pub platforms: HashMap<String, HashMap<String, Vec<_JavaRuntimesManifestItem>>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct _JavaRuntimesManifestItem {
    pub manifest: _JavaRuntimesManifestItemValue,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct _JavaRuntimesManifestItemValue {
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JavaRuntimeFiles {
    pub files: HashMap<String, _JavaRuntimeFilesItem>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct _JavaRuntimeFilesItem {
    #[serde(rename = "type")]
    pub item_type: String,

    #[serde(default)]
    pub target: String,

    #[serde(default)]
    pub downloads: Option<_JavaRuntimeFilesItemDownloads>,

    #[serde(default)]
    pub executable: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct _JavaRuntimeFilesItemDownloads {
    pub raw: _JavaRuntimesManifestItemValue,
}
