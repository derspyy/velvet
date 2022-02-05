use crate::get_minecraft_dir;
use crate::write_json;
use std::{fs, io};
use std::path::PathBuf;
#[allow(unused_must_use)]
pub fn run(version: String) {
    let mut mc_path = PathBuf::from(&get_minecraft_dir::dir());
    println!("Testing expected directory...");

    while mc_path.is_dir() == false {
        println!("Your Minecraft directory was not found, could you enter it's path?");
        let mut temp_path = String::new();
        io::stdin().read_line(&mut temp_path).expect("Couldn't read.");
        temp_path.pop();
        mc_path.push(&temp_path)
    }

    println!("Expected directory exists.");

    let mut velvet_path = PathBuf::from(&mc_path);
    velvet_path.push(".velvet");

    let mut path_versions = PathBuf::from(&velvet_path);
    path_versions.push("versions");
    path_versions.push(&version);
    fs::create_dir_all(&path_versions);

    let mut path_mods = PathBuf::from(&velvet_path);
    path_mods.push("mods");
    fs::create_dir(&path_mods);

    let mut path_loader = PathBuf::from(&velvet_path);
    path_loader.push("loader");
    fs::create_dir(&path_loader);

    println!("Installing Velvet Loader for version: {:?}", &version);

    let version_folder_name = String::from("velvet-fabric-loader-0.12.12-".to_string() + &version);
    let mut path_version = PathBuf::from(&mc_path);
    path_version.push("versions");
    path_version.push(&version_folder_name);
    fs::create_dir_all(&path_version);
    println!("{:?}", &path_version);
    // The above creates it's folder version.

    path_version.push(&version_folder_name);
    let mut jar_file = add_ext(path_version);
    jar_file.set_extension("jar");
    fs::File::create(&jar_file);
    println!("{:?}", &jar_file);
    // The above creates it's dummy jar.

    jar_file.set_extension("json");
    fs::File::create(&jar_file);
    write_json::version_json(&version);
    println!("{:?}", &jar_file);
    // The above creates it's json.

    println!("Directories created successfully.")
}

fn add_ext(x: PathBuf) -> PathBuf {
    let y = String::from(x.into_os_string().into_string().unwrap() + ".jar");
    println!("{}", &y);
    PathBuf::from(&y)
}