use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use ureq;

const VANILLA_ARGS: &str = "-Xmx2G -XX:+UnlockExperimentalVMOptions -XX:+UseG1GC -XX:G1NewSizePercent=20 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M";
const ICON: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgBAMAAACBVGfHAAAAGFBMVEUAAAB7GWmyF1rqR1a3msT/nJfp0+r///+virVnAAAAAXRSTlMAQObYZgAAAI9JREFUKM990cENA0EIQ9FpgRbcglugBVr4Lbj9HCaRNtnZcHwSYMFaa63Vvb7rB5JS8QeiAqCfIA0C6yO/0CFJDQkcgWqSsTIlTpCSSJJoCAdAHqtqY/oOncgz492pIyTInrEK7kDtrci21AeQx+9olxwXWP0e2GRf4AaLRPYOV32CbSVb1TzAtsDloR94AcQTfFFwBa/NAAAAAElFTkSuQmCC";

#[derive(Serialize, Deserialize)]
struct LauncherProfiles {
    profiles: HashMap<String, Profile>,
    settings: Value,
    version: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    name: String,
    last_used: String,
    last_version_id: String,
    created: String,
    icon: String,
    #[serde(rename = "type")]
    _type: String,
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

pub fn write_version(mc: &String, velvet: &String, z: &File) -> Result<()> {
    let url = format!("https://meta.quiltmc.org/v3/versions/loader/{mc}/{velvet}/profile/json");
    let json: serde_json::Value = ureq::get(&url).call()?.into_json()?;
    serde_json::to_writer_pretty(z, &json)?;
    Ok(())
}

pub fn write_profile(mc: &String, velvet: &String, x: &File) -> Result<String> {
    let mut json: LauncherProfiles = serde_json::from_reader(x)?;

    // This copies the "Latest Release" java arguments and version.
    let mut args = String::new();
    let mut dir: Option<String> = None;
    for x in json.profiles.values() {
        if x._type == "latest-release" {
            args = x
                .extra
                .get("javaArgs")
                .map_or(VANILLA_ARGS.into(), |x| x.to_owned());
            dir = x.extra.get("javaDir").map(|x| x.to_owned());
            break;
        }
    }
    let mut extramap = HashMap::new();
    extramap.insert(
        "javaArgs".to_string(),
        format!("-Dloader.modsDir=.velvet/mods/{mc} -Dloader.configDir=.velvet/config/{mc} {args}"),
    );
    if let Some(x) = dir {
        extramap.insert("javaDir".to_string(), x);
    }
    let time = Utc::now().to_string();
    let new_profile = Profile {
        name: format!("Velvet {mc}"),
        last_used: time.clone(),
        last_version_id: format!("quilt-loader-{velvet}-{mc}"),
        created: time,
        icon: ICON.into(),
        _type: "custom".into(),
        extra: extramap,
    };
    json.profiles
        .insert(format!("velvet-quilt-loader-{mc}"), new_profile);
    Ok(serde_json::to_string_pretty(&json)?)
}
