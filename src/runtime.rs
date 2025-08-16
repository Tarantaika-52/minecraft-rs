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
        ".minecraft/versions/1.21.8/1.21.8.jar",
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
        ".minecraft/runtime/java-runtime-delta/windows-x64/java-runtime-delta/bin/java.exe",
    );

    // dbg!(&libs);

    let command = binding
        .arg("-Xmx1G")
    		// .arg("-XstartOnFirstThread")
        .arg("-Djava.library.path=.minecraft/versions/1.21.8/natives")
        .arg("-cp")
        .arg(libs?)
        .arg("net.minecraft.client.main.Main")
        .args([
            "--username",
            "sigma_svinka",
            "--version",
            "1.21.8",
            "--gameDir",
            ".minecraft",
            "--assetsDir",
            ".minecraft/assets",
            "--assetIndex",
            "26",
            "--uuid",
            "6a9b09bb-2299-44a3-884d-e6668c6c5031",
            "--accessToken",
            "123456789123456789123456789", 
            "--userType",
            "legacy",
            "--versionType",
            "release",
        ]).envs(std::env::vars()).env(
        "DYLD_LIBRARY_PATH",
        ".minecraft/versions/1.21.8/natives",
    );

    // dbg!(&command);

    command.status()?;
    Ok(())
}
