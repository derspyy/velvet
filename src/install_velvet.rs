use crate::{get_minecraft_dir, write_json};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::{fs, io};

#[allow(unused_must_use)]
pub fn run(mc_version: &String, velvet_version: &String) {
    let mut mc_path = PathBuf::from(&get_minecraft_dir::dir());
    println!("Testing expected directory...");

    while mc_path.is_dir() == false {
        println!("Your Minecraft directory was not found, could you enter it's path?");
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
    fs::create_dir(&path_mods);

    let mut path_loader = PathBuf::from(&velvet_path);
    path_loader.push("loader");
    fs::create_dir(&path_loader);

    println!("Installing Velvet Loader for version: {:?}", &mc_version);

    let version_folder_name = format!("velvet-quilt-loader-{}-{}", &velvet_version, &mc_version);
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
    write_json::write_version(&mc_version, &velvet_version, &json_file);
    // The above creates it's json.

    println!("Directories created successfully.");

    mc_path.push("launcher_profiles");
    mc_path.set_extension("json");

    let profile = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&mc_path)
        .expect("Couldn't add the Velvet profile. Is your minecraft directory protected?");

    write_json::write_profile(&mc_version, &velvet_version, &profile);
}

fn add_extension(x: PathBuf) -> PathBuf {
    let y = String::from(x.into_os_string().into_string().unwrap() + ".jar");
    PathBuf::from(&y)
}
