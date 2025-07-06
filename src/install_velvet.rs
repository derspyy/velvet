use crate::{get_minecraft_dir, write_json};
use anyhow::{Result, anyhow};
use async_std::fs;
use async_std::fs::File;
use async_std::path::PathBuf;
use async_std::prelude::*;
use rfd::AsyncFileDialog;

pub async fn run(mc_version: &String, quilt_version: &String) -> Result<PathBuf> {
    let mut mc_path: PathBuf = get_minecraft_dir::dir()?;
    while !mc_path.is_dir().await {
        mc_path = std::path::PathBuf::from(
            AsyncFileDialog::new()
                .set_title("Select .minecraft folder:")
                .pick_folder()
                .await
                .ok_or_else(|| anyhow!("Select a folder!"))?,
        )
        .into();
    }

    let velvet_path = PathBuf::from(&mc_path).join(".velvet");
    let path_mods = PathBuf::from(&velvet_path).join("mods").join(mc_version);

    fs::create_dir_all(&path_mods).await?;

    let version_folder_name = format!("quilt-loader-{}-{}", &quilt_version, &mc_version);
    let mut path_version = PathBuf::from(&mc_path)
        .join("versions")
        .join(&version_folder_name);
    fs::create_dir_all(&path_version).await?;
    path_version.push(format!("{version_folder_name}.jar"));
    fs::File::create(&path_version).await?; // dummy jar required by the launcher.

    path_version.set_extension("json");
    let json_file = File::create(&path_version).await?;
    write_json::write_version(mc_version, quilt_version, &json_file).await?;

    mc_path.push("launcher_profiles");
    mc_path.set_extension("json");

    let mut launcher_file = File::open(&mc_path).await?;
    let profile = write_json::write_profile(mc_version, quilt_version, &launcher_file).await?;

    launcher_file = File::create(&mc_path).await?;
    launcher_file.write_all(profile.as_bytes()).await?;

    Ok(path_mods)
}
