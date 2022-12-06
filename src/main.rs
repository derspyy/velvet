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
        if !x.version.contains('-') {
            quilt_version = x.version;
            break
        }
    }
    println!("Latest Velvet Quilt version: {}.", &quilt_version.purple().italic());
    println!("Enter minecraft version:");
    let mc_version = input();
    println!("\nChoose your {}!\n", "modlists".purple());
    println!("{} Only vanilla performance-enhancing modlist.", "vanilla -".purple().bold());
    println!("{} Immersive and beautiful modlist.", "visual -".purple().bold());
    println!("{} Optifine parity modlist. {}", "optifine -".purple().bold(), "(select this if using optifine-based resource packs)".dimmed());
    let mut modlists: (bool, bool, bool) = (true, false, false);

    println!("\nEnter the {} to toggle: {}", "modlist's name".purple().bold(), "(Press enter to install)".dimmed());
    loop {
        println!("\n{}: {} {}: {} {}: {}",
        "vanilla".purple().bold(), modlists.0,
        "visual".purple().bold(), modlists.1,
        "optifine".purple().bold(), modlists.2);

        let modlist_string = input();
        match modlist_string.as_str() {
            "" => { break }
            "vanilla" => { modlists.0 = !modlists.0 },
            "visual" => { modlists.1 = !modlists.1 },
            "optifine" => { modlists.2 = !modlists.2 }
            _ => { println!("Invalid input!") }
        }
    }

    let path_mods = install_velvet::run(&mc_version, &quilt_version);
    get_mods::run(&mc_version, &modlists, path_mods);
    println!("Done. Enjoy! {}", "Don't forget to restart the minecraft launcher.".red().underline());

    // Wait for Return
    println!("{}", "Press enter to exit.".dimmed());
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn input() -> String {
    let mut x = String::new();
    io::stdin()
    .read_line(&mut x)
    .expect("Couldn't read.");
    while x.ends_with('\n') || x.ends_with('\r') {
        x.pop();
    }
    x
}