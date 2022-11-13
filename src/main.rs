use std::io;

mod install_velvet;
mod get_minecraft_dir;
mod get_mods;
pub mod write_json;
mod front_end;

fn main() {
    println!("Starting...");
    let velvet_version = rm_newline(reqwest::blocking::get("https://file.garden/YAY9sxgapBy70PkV/maven/net/piuvas/velvet-quilt-loader/latest.txt")
        .expect("Failed to fetch latest Velvet version.")
        .text()
        .unwrap()
    );
    println!("Latest Velvet Quilt version: {}.", velvet_version);

    println!("Enter minecraft version.");
    let mut mc_version = String::new();
    io::stdin()
        .read_line(&mut mc_version)
        .expect("Couldn't read.");
    mc_version = rm_newline(mc_version);
    println!("Selected version: {}", &mc_version);
    install_velvet::run(&mc_version, &velvet_version);
}

fn rm_newline(mut x: String) -> String {
    if x.ends_with('\n') {
        x.pop();
        if x.ends_with('\r') {
            x.pop();
        }
    }
    x
}