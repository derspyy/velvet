use crate::{get_minecraft_dir, write_json};
use std::io::Write;
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io};


#[allow(unused_must_use)]
pub fn run(mc_version: &String, quilt_version: &String) -> PathBuf {
    let mut mc_path = get_minecraft_dir::dir();
    while !mc_path.is_dir() {
        let mut temp_path = String::new();
        io::stdin()
            .read_line(&mut temp_path)
            .expect("Couldn't read.");
        mc_path = PathBuf::from(&temp_path);
    }

    let mut velvet_path = PathBuf::from(&mc_path);
    velvet_path.push(".velvet");

    let mut path_versions = PathBuf::from(&velvet_path);
    path_versions.push("versions");
    path_versions.push(mc_version);
    fs::create_dir_all(&path_versions);

    let mut path_mods = PathBuf::from(&velvet_path);
    path_mods.push("mods");
    path_mods.push(mc_version);
    fs::remove_dir_all(&path_mods);
    fs::create_dir_all(&path_mods);

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
    write_json::write_version(mc_version, quilt_version, &json_file);
    // The above creates it's json.

    mc_path.push("launcher_profiles");
    mc_path.set_extension("json");

    let mut launcher_file = File::open(&mc_path)
        .expect("Couldn't read. Are you using the official minecraft launcher?");
    let profile = write_json::write_profile(mc_version, quilt_version, &launcher_file);

    launcher_file = File::create(&mc_path)
        .expect("Couldn't write your profile. Is your minecraft directory protected?");
    write!(&mut launcher_file, "{}", &profile).unwrap();

    path_mods
}

fn add_extension(x: PathBuf) -> PathBuf {
    let y = x.into_os_string().into_string().unwrap() + ".ext";
    PathBuf::from(&y)
}
