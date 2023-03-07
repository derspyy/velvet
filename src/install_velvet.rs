use crate::{get_minecraft_dir, write_json};
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::fs;
use rfd::FileDialog;

#[allow(unused_must_use)]
pub fn run(mc_version: &String, quilt_version: &String) -> Result<PathBuf> {
    let mut mc_path = get_minecraft_dir::dir()?;
    while !mc_path.is_dir() {
        mc_path = FileDialog::new()
            .set_title("Select .minecraft folder:")
            .pick_folder()
            .ok_or_else(|| anyhow!("Select a folder!"))?;
    }

    let velvet_path = PathBuf::from(&mc_path).join(".velvet");

    let path_versions = PathBuf::from(&velvet_path)
        .join("versions")
        .join(mc_version);
    fs::create_dir_all(path_versions);

    let path_mods = PathBuf::from(&velvet_path).join("mods").join(mc_version);
    fs::remove_dir_all(&path_mods);
    fs::create_dir_all(&path_mods);

    let version_folder_name = format!("quilt-loader-{}-{}", &quilt_version, &mc_version);
    let mut path_version = PathBuf::from(&mc_path)
        .join("versions")
        .join(&version_folder_name);
    fs::create_dir_all(&path_version);
    path_version.push(format!("{}.jar", version_folder_name));
    fs::File::create(&path_version); // Dummy jar required by the launcher

    path_version.set_extension("json");
    let json_file = File::create(&path_version)?;
    write_json::write_version(mc_version, quilt_version, &json_file);

    mc_path.push("launcher_profiles");
    mc_path.set_extension("json");

    let mut launcher_file = File::open(&mc_path)?;
    let profile = write_json::write_profile(mc_version, quilt_version, &launcher_file)?;

    launcher_file = File::create(&mc_path)?;
    write!(&mut launcher_file, "{}", &profile)?;

    Ok(path_mods)
}
