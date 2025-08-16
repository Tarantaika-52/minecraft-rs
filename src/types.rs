use std::path::PathBuf;

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::utils::get_minecraft_dir;

#[derive(Debug)]
pub enum Platform {
    Linux,
    Windows,
    Darwin,
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        match value {
            "linux" => Platform::Linux,
            "windows" => Platform::Windows,
            "macos" => Platform::Darwin,
            _ => {
                panic!("Unsupported platform")
            }
        }
    }
}

#[derive(Debug)]
pub struct Launcher {
    pub path: PathBuf,
}

impl Launcher {
    pub fn new() -> Self {
        let path = PathBuf::from("./");

        Launcher { path }
    }

    pub fn set_path(&mut self, path: &str) {
        let path = PathBuf::from(path);
        log::info!("Game path set to: {path:?}");
        self.path = path;
    }

    pub fn init_path(&self) -> Result<(), Error> {
        log::info!("Initializing game path...");
        std::fs::create_dir_all(&self.path)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionsList {
    pub versions: Vec<Version>,
}

impl VersionsList {
    pub fn find_version(&self, id: &str) -> Option<&Version> {
        self.versions.iter().find(|&i| i.id == id)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
}

#[derive(Debug, From)]
pub enum Error {
    #[from]
    io(std::io::Error),

    #[from]
    reqwest(reqwest::Error),

    #[from]
    json(serde_json::Error),

    #[from]
    zip(zip::result::ZipError),

    #[from]
    Infallible(std::convert::Infallible),
}
