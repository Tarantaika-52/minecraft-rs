use types::Launcher;

mod helpers;
mod install;
mod internal_types;
mod jvm;
mod natives;
mod runtime;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), types::Error> {
    // Инициализация логгера
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Инициализация лаунчера
    let mut launcher = Launcher::new();
    launcher.set_path("./.minecraft");
    launcher.init_path()?;

    // Получение списка всех версий
    let versions = utils::get_versions_list().await?;

    // Получение определенной версии
    if let Some(version) = versions.find_version("1.20.1") {
        // Получение
        let info = version.get_info(&launcher).await?;

        install::install_libraries(&launcher, &info).await?;
        install::install_assets(&launcher, &info).await?;
        install::install_client(&launcher, &info).await?;

        jvm::install_jvm_runtime(&launcher, &info).await?;
        // run version
        let command = runtime::get_command(&launcher, &info)?;
    }

    Ok(())
}
