use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use tokio::{fs::File, io::BufReader, process::Command};

use crate::{
    helpers,
    types::{self, Error, Platform, VersionsList},
};

const VERSION_MANIFEST_URL: &str =
    "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

/// Get current platform
///
/// # Return:
/// - [`Platform::Linux`]
/// - [`Platform::Windows`]
/// - [`Platform::Darwin`]
pub fn get_platform() -> Platform {
    env::consts::OS.into()
}

pub fn get_arch<'a>() -> &'a str {
    if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "arm") {
        "arm64"
    } else {
        "unknown"
    }
}

pub async fn make_executable(path: &Path) -> Result<(), Error> {
    #[cfg(unix)]
    {
        log::info!("Making {path:?} executable...");
        Command::new("chmod").arg("+x").arg(path).status().await?;
    }

    Ok(())
}

pub async fn unzip(path: &Path, dest: &Path) -> Result<(), Error> {
    let file = std::fs::File::open(path)?;

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        if file.is_dir() {
            let path = dest.join(outpath);
            fs::create_dir_all(path)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    let path = dest.join(p);
                    fs::create_dir_all(path)?;
                }
            }
            let path = dest.join(outpath);
            let mut outfile = fs::File::create(&path)?;
            io::copy(&mut file, &mut outfile)?;

            make_executable(&path).await?;
        }
    }

    Ok(())
}

pub fn get_minecraft_dir() -> PathBuf {
    let platform = get_platform();
    match platform {
        Platform::Windows => {
            let appdata = env::var("APP_DATA").expect("Failed to get \"APP_DATA\" directory");
            Path::new(&appdata).join(".minecraft")
        }
        Platform::Darwin => {
            let home = env::var("HOME").expect("Failed to get \"HOME\" directory");
            Path::new(&home)
                .join("Library")
                .join("Application Support")
                .join("minecraft")
        }
        _ => {
            let home = env::var("HOME").expect("Failed to get \"HOME\" directory");
            Path::new(&home).join(".minecraft")
        }
    }
}

pub async fn get_versions_list() -> Result<VersionsList, Error> {
    log::info!("Getting versions list...");
    let versions_raw = helpers::http::get(VERSION_MANIFEST_URL, None).await?;
    let list = serde_json::from_slice::<types::VersionsList>(&versions_raw)?;
    log::info!("Finded {} versions", list.versions.len());
    Ok(list)
}
