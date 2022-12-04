use std::io;

use serde::Deserialize;
use colored::{Colorize, control};

mod install_velvet;
mod get_minecraft_dir;
mod get_mods;
pub mod write_json;

#[derive(Deserialize)]
struct Versions {
    version: String
}

fn main() {

    #[cfg(target_os = "windows")]
    control::set_virtual_terminal(true).unwrap();

    println!("Welcome to {}!", "Velvet".purple().bold());
    let version_response: Vec<Versions> = reqwest::blocking::get("https://meta.quiltmc.org/v3/versions/loader")
        .expect("Failed to fetch latest Quilt version.")
        .json()
        .unwrap();
    let mut quilt_version = String::new();
    for x in version_response {
        if x.version.contains("-") == false {
            quilt_version = x.version;
            break
        }
    }
    println!("Latest Velvet Quilt version: {}.", &quilt_version.purple().italic());
    println!("Enter minecraft version:");
    let mut mc_version = String::new();
    io::stdin()
        .read_line(&mut mc_version)
        .expect("Couldn't read.");
    mc_version = rm_newline(mc_version);
    println!("Selected version: {}", &mc_version.purple().italic());
    let path_mods = install_velvet::run(&mc_version, &quilt_version);
    get_mods::run(&mc_version, path_mods);
    println!("Done. Enjoy! {}", "Don't forget to restart the minecraft launcher.".red().underline());

    // Wait for Return
    println!("{}", "Press enter to exit.".dimmed());
    let _exit = io::stdin().read_line(&mut mc_version).unwrap();
}

pub fn rm_newline(mut x: String) -> String {
    if x.ends_with('\n') {
        x.pop();
        if x.ends_with('\r') {
            x.pop();
        }
    }
    x
}