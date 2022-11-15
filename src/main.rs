use std::io;

use serde_json::{json, Value, to_string};
use colored::Colorize;

mod install_velvet;
mod get_minecraft_dir;
mod get_mods;
pub mod write_json;
mod front_end;

fn main() {
    println!("Welcome to {}!", "Velvet".purple().bold());
    println!("{}", "CLOSE THE MINECRAFT LAUNCHER!".red().bold().underline());
    let version_response: Value = reqwest::blocking::get("https://meta.quiltmc.org/v3/versions/loader")
        .expect("Failed to fetch latest Quilt version.")
        .json()
        .unwrap();
    let quilt_version = version_response[0]["version"].as_str().unwrap();
    println!("Latest Velvet Quilt version: {}.", &quilt_version.purple().italic());
    println!("Enter minecraft version:");
    let mut mc_version = String::new();
    io::stdin()
        .read_line(&mut mc_version)
        .expect("Couldn't read.");
    mc_version = rm_newline(mc_version);
    println!("Selected version: {}", &mc_version.purple().italic());
    let path_mods = install_velvet::run(&mc_version, &quilt_version.to_string());
    get_mods::run(&mc_version, path_mods);
    println!("Done. Enjoy!")
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