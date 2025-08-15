use std::path::Path;

use crate::{
    helpers,
    internal_types::shared::{JsonAssetIndexes, VersionJson, VersionJsonLibrary},
    types::{Error, Launcher, Version},
    utils,
};

const ASSETS_URL_ROOT: &str = "https://resources.download.minecraft.net/";

async fn get_version_info(version: &Version, minecraft_dir: &Path) -> Result<VersionJson, Error> {
    log::info!("Downloading version manifest...");
    let path = minecraft_dir.join("versions").join(&version.id);
    std::fs::create_dir_all(&path)?;

    let path = path.join(format!("{}.json", &version.id));
    let url = &version.url;

    let file = helpers::http::get(url, None).await?;
    std::fs::write(path, &file)?;
    let info = serde_json::from_slice::<VersionJson>(&file)?;

    Ok(info)
}

pub async fn download_file(url: &str, path: &Path, client: &reqwest::Client) -> Result<(), Error> {
    if path.exists() {
        return Ok(());
    }

    let bytes = helpers::http::get(url, Some(client)).await?;
    tokio::fs::write(path, bytes).await?;
    Ok(())
}

async fn install_lib(
    lib: &VersionJsonLibrary,
    game_dir: &Path,
    natives_dir: &Path,
    client: &reqwest::Client,
) -> Result<(), Error> {
    if !lib.rules.is_empty() && !lib.check_rule_allow() {
        return Ok(());
    }

    log::info!("Installing lib \"{}\"", lib.name);

    let [lib_path, _, _, natives] = lib.parse_lib_name();

    let current_path = Path::new(game_dir).join("libraries");

    let download_url = &lib.downloads.artifact.url;
    let filename = &current_path.join(&lib.downloads.artifact.path);

    tokio::fs::create_dir_all(&filename.parent().unwrap_or(&current_path)).await?;

    download_file(download_url, filename, client).await?;

    if !natives.is_empty() {
        utils::unzip(filename, natives_dir).await?;
    }
    Ok(())
}

pub async fn install_libraries(launcher: &Launcher, info: &VersionJson) -> Result<(), Error> {
    let total_libs_count = info.libraries.len();
    log::info!("Installing [{}] libraries", total_libs_count);
    let client = reqwest::Client::builder().build()?;
    let natives_dir = launcher
        .path
        .join("versions")
        .join(&info.id)
        .join("natives");
    std::fs::create_dir_all(&natives_dir)?;

    for lib in info.libraries.iter() {
        install_lib(lib, &launcher.path, &natives_dir, &client).await?;
    }
    log::info!("Libs installed");
    Ok(())
}

pub async fn install_assets(launcher: &Launcher, info: &VersionJson) -> Result<(), Error> {
    let client = reqwest::Client::builder().build()?;
    let path = &launcher.path.join("assets").join("indexes");
    tokio::fs::create_dir_all(path).await?;
    let path = &path.join(format!("{}.json", info.assets));
    download_file(&info.asset_index.url, path, &client).await?;

    let assets_indexes = serde_json::from_slice::<JsonAssetIndexes>(&tokio::fs::read(path).await?)?;

    for (name, filehash) in assets_indexes.objects {
        log::info!("Loading asset \"{name}\"...");
        let hash = &filehash.hash;
        let path = &launcher
            .path
            .join("assets")
            .join("objects")
            .join(&hash[..2]);
        tokio::fs::create_dir_all(&path).await?;
        let url = format!("{}/{}/{}", ASSETS_URL_ROOT, &hash[..2], &hash);
        download_file(&url, path, &client).await?;
    }

    Ok(())
}

pub async fn install_client(launcher: &Launcher, info: &VersionJson) -> Result<(), Error> {
    log::info!("Installing client...");
    let path = launcher.path.join("versions").join(&info.id);
    tokio::fs::create_dir_all(&path).await?;
    let path = path.join(format!("{}.jar", info.id));
    if path.exists() {
        log::info!("Client already installed");
        return Ok(());
    }

    let client = reqwest::Client::builder().build()?;
    download_file(&info.downloads.client.url, &path, &client).await?;
    log::info!("Client installed!");
    Ok(())
}

impl Version {
    pub async fn get_info(&self, launcher: &Launcher) -> Result<VersionJson, Error> {
        let info = get_version_info(self, &launcher.path).await?;
        Ok(info)
    }
}
