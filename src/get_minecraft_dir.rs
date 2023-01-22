#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use home::home_dir;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub fn dir() -> Result<PathBuf> {
    let mut dir = home_dir().ok_or_else(|| anyhow!("Couldn't find home directory!"))?;
    dir.push("AppData");
    dir.push("Roaming");
    dir.push(".minecraft");
    Ok(dir)
}

#[cfg(target_os = "linux")]
pub fn dir() -> Result<PathBuf> {
    let mut dir = home_dir().ok_or_else(|| anyhow!("Couldn't find home directory!"))?;
    dir.push(".minecraft");
    Ok(dir)
}

#[cfg(target_os = "macos")]
pub fn dir() -> Result<PathBuf> {
    let mut dir = home_dir().ok_or_else(|| anyhow!("Couldn't find home directory!"))?;
    dir.push("Library");
    dir.push("Application Support");
    dir.push("minecraft");
    Ok(dir)
}
