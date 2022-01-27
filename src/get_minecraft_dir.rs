use dirs;
use std::env;
use std::io;

pub fn dir() -> String {
    println!("Searching for Minecraft directory...");
    fn home() -> String {
        return match dirs::home_dir() {
            Some(path) => {
                let user = path.display();
                user.to_string()

            }
            None => {
                println!("couldn't get home dir");
                let user = "";
                user.to_string()
            }
        };
    }
    fn os() -> &'static str {
        let x = match env::consts::OS {
            "windows" => "\\AppData\\Roaming\\.minecraft",
            "linux" => "/.minecraft",
            "macos" => "/Library/Application Support/minecraft",
            _ => "unknown",
        };
        return x;
    }
    let mut mc_dir = String::new();
    if os() != "unknown" {
        mc_dir.push_str(&*&home());
        mc_dir.push_str(&os());
        println!("Expected Minecraft directory: {}", mc_dir);
    } else {
        println!("Couldn't predict your directory! Enter your minecraft directory:");
        io::stdin()
            .read_line(&mut mc_dir)
            .expect("Operation failed.");
    }
    mc_dir
}
