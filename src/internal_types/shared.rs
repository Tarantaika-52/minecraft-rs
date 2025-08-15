use std::collections::HashMap;

use serde::Deserialize;

use crate::{types::Platform, utils};

#[derive(Debug, Deserialize)]
pub struct _VersionJsonLibraryDownloadsArtifact {
    pub path: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct _VersionJsonLibraryDownloads {
    pub artifact: _VersionJsonLibraryDownloadsArtifact,
}

#[derive(Debug, Deserialize)]
pub struct _VersionJsonLibraryRuleOs {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct _VersionJsonLibraryRule {
    pub action: String,
    pub os: _VersionJsonLibraryRuleOs,
}

impl _VersionJsonLibraryRule {
    fn parse_rule(&self) -> bool {
        let platform = utils::get_platform();
        let val = self.action != "allow";

        if self.os.name == "windows" && !matches!(platform, Platform::Windows) {
            return val;
        } else if self.os.name == "osx" && !matches!(platform, Platform::Darwin) {
            return val;
        } else if self.os.name == "linux" && !matches!(platform, Platform::Linux) {
            return val;
        }

        !val
    }
}

#[derive(Debug, Deserialize)]
pub struct VersionJsonLibrary {
    pub name: String,
    pub downloads: _VersionJsonLibraryDownloads,
    #[serde(default)]
    pub rules: Vec<_VersionJsonLibraryRule>,
}

impl VersionJsonLibrary {
    pub fn parse_lib_name(&self) -> [&str; 4] {
        let parts = self.name.split(":");
        let mut parsed: [&str; 4] = [""; 4];
        for (i, part) in parts.enumerate() {
            parsed[i] = part;
        }
        parsed
    }

    pub fn check_rule_allow(&self) -> bool {
        for i in self.rules.iter() {
            if !i.parse_rule() {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Deserialize)]
pub struct _JsonDownloadItem {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct _VersionJsonDownloads {
    pub client: _JsonDownloadItem,
}

#[derive(Debug, Deserialize)]
pub struct _JsonJavaVersion {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: u16,
}

#[derive(Debug, Deserialize)]
pub struct _JsonVersionArgs {
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct VersionJson {
    // pub arguments: _JsonVersionArgs,
    pub id: String,
    pub assets: String,
    #[serde(rename = "assetIndex")]
    pub asset_index: _JsonDownloadItem,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: u8,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    pub minimum_launcher_version: u8,
    #[serde(rename = "type")]
    pub version_type: String,
    pub downloads: _VersionJsonDownloads,

    #[serde(rename = "javaVersion")]
    pub java_version: _JsonJavaVersion,

    pub libraries: Vec<VersionJsonLibrary>,
}

#[derive(Debug, Deserialize)]
pub struct _JsonAssetIndexItem {
    pub hash: String,
}

#[derive(Debug, Deserialize)]
pub struct JsonAssetIndexes {
    pub objects: HashMap<String, _JsonAssetIndexItem>,
}
