use std::{
    fs,
    path::{self, Path, PathBuf},
    process::Command,
};

use crate::{
    internal_types::shared::VersionJson,
    types::{Error, Launcher},
};

fn get_join_char<'a>() -> &'a str {
    #[cfg(unix)]
    {
        ":"
    }
    #[cfg(windows)]
    {
        ";"
    }
}

fn get_libs(info: &VersionJson, minecraft_dir: &Path) -> Result<String, Error> {
    let mut libs = Vec::new();

    for lib in info.libraries.iter() {
        if !lib.rules.is_empty() && !lib.check_rule_allow() {
            continue;
        }

        let [_, _, _, natives] = lib.parse_lib_name();
        if !natives.is_empty() {
            continue;
        }

        let path = &lib.downloads.artifact.path;
        let path = path::absolute(minecraft_dir.join("libraries").join(path))?;

        libs.push(path);
    }

    libs.push(PathBuf::from(
        "/Users/user/Desktop/projects/minecraft-rs/.minecraft/versions/1.20.1/1.20.1.jar",
    ));

    let join_char = get_join_char();
    let mut libstr = Vec::new();

    // VERY BAD CODE! WARNING!
    for i in libs.iter() {
        libstr.push(i.to_str().unwrap());
    }
    Ok(libstr.join(join_char))
}

pub fn get_command(launcher: &Launcher, info: &VersionJson) -> Result<(), Error> {
    let libs = get_libs(info, &launcher.path);

    let mut binding = Command::new(
        ".minecraft/runtime/java-runtime-gamma/mac-os-arm64/java-runtime-gamma/jre.bundle/Contents/Home/bin/java",
    );

    // dbg!(&libs);

    let command = binding
        .arg("-Xmx1G")
    		.arg("-XstartOnFirstThread")
        .arg("-Djava.library.path=/Users/user/Desktop/projects/minecraft-rs/.minecraft/versions/1.20.1/natives")
        .arg("-cp")
        .arg(libs?)
        .arg("net.minecraft.client.main.Main")
        .args([
            "--username",
            "USER",
            "--version",
            "1.20.1",
            "--gameDir",
            "/Users/user/Desktop/projects/minecraft-rs/.minecraft",
            "--assetsDir",
            "/Users/user/Desktop/projects/minecraft-rs/.minecraft/assets",
            "--assetIndex",
            "1.20",
            "--uuid",
            "00000000-0000-0000-0000-000000000000",
            "--accessToken",
            "123456789123456789123456789",
            "--userType",
            "legacy",
            "--versionType",
            "release",
        ]).envs(std::env::vars()).env(
        "DYLD_LIBRARY_PATH",
        "/Users/user/Desktop/projects/minecraft-rs/.minecraft/versions/1.20.1/natives",
    );

    // dbg!(&command);

    command.status()?;
    Ok(())
}
