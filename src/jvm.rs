use crate::{
    helpers,
    install::download_file,
    internal_types::{
        shared::VersionJson,
        shared_jvm::{self, JavaRuntimesManifest},
    },
    types::{Error, Launcher, Platform},
    utils,
};

const JVM_MANIFEST_URL: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";

fn get_jvm_platform<'a>() -> &'a str {
    let platform = utils::get_platform();
    let arch = utils::get_arch();
    dbg!(&arch);

    match platform {
        Platform::Windows => {
            if arch == "x86" {
                "windows-x86"
            } else {
                "windows-x64"
            }
        }
        Platform::Linux => {
            if arch == "x86" {
                "linux-i386"
            } else {
                "linux"
            }
        }
        Platform::Darwin => {
            if arch == "aarch64" {
                "mac-os-arm64"
            } else {
                "mac-os"
            }
        }
    }
}

async fn get_jvm_runtimes(manifest_data: &JavaRuntimesManifest) -> Result<Vec<String>, Error> {
    let mut jvm_list: Vec<String> = Vec::new();

    let platform_jvms = manifest_data.platforms.get(get_jvm_platform());
    if let Some(list) = platform_jvms {
        for key in list.keys() {
            jvm_list.push(key.to_string());
        }
    }

    Ok(jvm_list)
}

pub async fn install_jvm_runtime(launcher: &Launcher, info: &VersionJson) -> Result<(), Error> {
    let platform_str = get_jvm_platform();
    log::info!("Getting jvm runtimes for {}", &platform_str);

    let client = reqwest::Client::builder().build()?;

    let raw_manifest_data = helpers::http::get(JVM_MANIFEST_URL, Some(&client)).await?;
    let manifest_data = serde_json::from_slice::<JavaRuntimesManifest>(&raw_manifest_data)?;

    let runtimes = get_jvm_runtimes(&manifest_data).await?;

    let version = &info.java_version.component;

    if !runtimes.contains(version) {
        panic!("Jvm version not found");
    }

    let url = &manifest_data
        .platforms
        .get(platform_str)
        .unwrap()
        .get(version)
        .unwrap()[0]
        .manifest
        .url;

    let platform_manifest = helpers::http::get(url, Some(&client)).await?;
    let platform_manifest =
        serde_json::from_slice::<shared_jvm::JavaRuntimeFiles>(&platform_manifest)?;

    let base_path = &launcher
        .path
        .join("runtime")
        .join(version)
        .join(platform_str)
        .join(version);
    tokio::fs::create_dir_all(&base_path).await?;

    let mut link_tasks = Vec::new();

    for (key, value) in platform_manifest.files {
        let current_path = base_path.join(&key);

        match value.item_type.as_str() {
            "file" => {
                if current_path.exists() {
                    continue;
                }
                log::info!("Installing file: {key}");
                let parent = current_path.parent().unwrap();
                tokio::fs::create_dir_all(parent).await?;

                download_file(&value.downloads.unwrap().raw.url, &current_path, &client).await?;

                utils::make_executable(&current_path).await?;
            }
            "directory" => {
                if current_path.exists() {
                    continue;
                }
                log::info!("Creating directory: {key}");

                tokio::fs::create_dir_all(current_path).await?;
            }
            "link" => {
                if current_path.exists() {
                    continue;
                }
                link_tasks.push((current_path, value));
            }
            _ => {}
        }
    }

    // Копирование файлов после загрузки всех основных файлов,
    // Чтобы избежать проблем с отсутсвием директорий и файлов для копирования
    for task in link_tasks {
        let current_path = &task.0;
        let value = &task.1;
        let parent = current_path.parent().unwrap();
        let copy_from = &parent.join(&value.target);
        log::info!("Creating link: {copy_from:?} -> {current_path:?}");

        tokio::fs::create_dir_all(parent).await?;
        tokio::fs::copy(copy_from, current_path).await?;
    }

    log::info!("Jvm installed!");

    Ok(())
}
