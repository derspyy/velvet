use colored::Colorize;

use crate::{get_minecraft_dir, write_json};
use std::fs::{File, remove_file, rename};
use std::path::PathBuf;
use std::{fs, io};

#[allow(unused_must_use)]
pub fn run(mc_version: &String, quilt_version: &String) -> PathBuf {
    let mut mc_path = PathBuf::from(&get_minecraft_dir::dir());
    println!("Testing expected directory...");

    while mc_path.is_dir() == false {
        println!("Minecraft directory was not found. Enter it's path:");
        let mut temp_path = String::new();
        io::stdin()
            .read_line(&mut temp_path)
            .expect("Couldn't read.");
        mc_path = PathBuf::from(&temp_path);
    }

    println!("Directory exists.");

    let mut velvet_path = PathBuf::from(&mc_path);
    velvet_path.push(".velvet");

    let mut path_versions = PathBuf::from(&velvet_path);
    path_versions.push("versions");
    path_versions.push(&mc_version);
    fs::create_dir_all(&path_versions);

    let mut path_mods = PathBuf::from(&velvet_path);
    path_mods.push("mods");
    path_mods.push(&mc_version);
    fs::remove_dir_all(&path_mods);
    fs::create_dir_all(&path_mods);

    let mut path_loader = PathBuf::from(&velvet_path);
    path_loader.push("loader");
    fs::create_dir(&path_loader);

    println!("Installing Quilt Loader for version: {}", &mc_version.purple().italic());

    let version_folder_name = format!("quilt-loader-{}-{}", &quilt_version, &mc_version);
    let mut path_version = PathBuf::from(&mc_path);
    path_version.push("versions");
    path_version.push(&version_folder_name);
    fs::create_dir_all(&path_version);
    // The above creates it's folder version.

    path_version.push(&version_folder_name);
    let mut jar_file = add_extension(path_version);
    jar_file.set_extension("jar");
    fs::File::create(&jar_file);
    // The above creates it's dummy jar.

    jar_file.set_extension("json");
    let json_file = File::create(&jar_file).unwrap();
    write_json::write_version(&mc_version, &quilt_version, &json_file);
    // The above creates it's json.

    println!("{}", "Directories created successfully.".dimmed());

    mc_path.push("launcher_profiles");
    mc_path.set_extension("json");

    let mut temp_path = PathBuf::from(&mc_path);
    temp_path.set_extension("temp");

    let read_profile = File::open(&mc_path).expect("Couldn't read. Are you using the official minecraft launcher?");
    let write_profile = File::create(&temp_path).expect("Couldn't write your profile. Is your minecraft directory protected?");

    write_json::write_profile(&mc_version, &quilt_version, &read_profile, &write_profile);

    remove_file(&mc_path);
    rename(&temp_path, &mc_path);

    path_mods
}

fn add_extension(x: PathBuf) -> PathBuf {
    let y = String::from(x.into_os_string().into_string().unwrap() + ".jar");
    PathBuf::from(&y)
}
