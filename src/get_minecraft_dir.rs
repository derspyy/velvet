#[allow(unused_imports)]
use anyhow::{Result, anyhow};
use async_std::path::PathBuf;
use home::home_dir;

#[cfg(target_os = "windows")]
pub fn dir() -> Result<PathBuf> {
    let mut dir = home_dir().ok_or_else(|| anyhow!("Couldn't find home directory!"))?;
    dir.push("AppData");
    dir.push("Roaming");
    dir.push(".minecraft");
    Ok(dir.into())
}

#[cfg(target_os = "linux")]
pub fn dir() -> Result<PathBuf> {
    let mut dir = home_dir().ok_or_else(|| anyhow!("Couldn't find home directory!"))?;
    dir.push(".minecraft");
    Ok(dir.into())
}

#[cfg(target_os = "macos")]
pub fn dir() -> Result<PathBuf> {
    let mut dir = home_dir().ok_or_else(|| anyhow!("Couldn't find home directory!"))?;
    dir.push("Library");
    dir.push("Application Support");
    dir.push("minecraft");
    Ok(dir.into())
}
