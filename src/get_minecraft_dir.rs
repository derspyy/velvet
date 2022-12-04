#[allow(unused_imports)]
use dirs::{config_dir, home_dir};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub fn dir() -> PathBuf {
    let mut dir = config_dir().expect("Couldn't read your config directory. Is it protected?");
    dir.push(".minecraft");
    dir
}

#[cfg(not(target_os = "windows"))]
pub fn dir() -> PathBuf {
    let mut dir = home_dir().expect("Couldn't read your home directory. Is it protected?");
    dir.push(".minecraft");
    dir
}
