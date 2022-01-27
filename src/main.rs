use std::io;
mod install_velvet;
mod get_minecraft_dir;
mod get_mods;
mod write_json;
mod front_end;

fn main() {
    println!("Starting...");
    println!("Enter minecraft version.");
    let mut version = String::new();
    io::stdin()
        .read_line(&mut version)
        .expect("Couldn't read.");
    if version.ends_with('\n') {
        version.pop();
        if version.ends_with('\r') {
            version.pop();
        }
    }
    println!("Selected version: {}", &version);
    install_velvet::run(version);

}
