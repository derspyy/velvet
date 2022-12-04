use std::fs::File;
use reqwest::blocking::Client;
use chrono::Utc;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use std::collections::HashMap;

const VANILLA_ARGS: &str = "-Xmx2G -XX:+UnlockExperimentalVMOptions -XX:+UseG1GC -XX:G1NewSizePercent=20 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M";

#[derive(Serialize, Deserialize)]
struct LauncherProfiles {
    profiles: HashMap<String, Value>,
    settings: Value,
    version: u8
}

pub fn write_version(mc: &String, velvet: &String, z: &File) {

    let url = format!("https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json", &mc, &velvet);
    let client = Client::new();
    let json: serde_json::Value = client
    .get(&url)
    .header("User-Agent", "Velvet")
    .send()
    .expect("Couldn't communicate with Quilt's meta server.")
    .json()
    .unwrap();
    serde_json::to_writer_pretty(z, &json).unwrap();
}

pub fn write_profile(mc: &String, velvet: &String, x: &File) -> String {
    let mut json: LauncherProfiles = serde_json::from_reader(x).unwrap();

    // This copies the "Latest Release" java arguments and version.
    let mut args = VANILLA_ARGS;
    let mut dir = "";
    for x in json.profiles.values() {
        if x["type"] == "latest-release" {
            println!("Copying Java directory and arguments from: {} profile.", "Latest Release".purple().italic());
            if !x["javaArgs"].is_null() {
                args = &x["javaArgs"].as_str().unwrap();
            }
            if !x["javaDir"].is_null() {
                dir = &x["javaDir"].as_str().unwrap();
            }
            break
        }
    }
    let time = Utc::now();

    let new_profile = json!({
        "created": &time,
        "icon": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgBAMAAACBVGfHAAAAGFBMVEUAAAB7GWmyF1rqR1a3msT/nJfp0+r///+virVnAAAAAXRSTlMAQObYZgAAAI9JREFUKM990cENA0EIQ9FpgRbcglugBVr4Lbj9HCaRNtnZcHwSYMFaa63Vvb7rB5JS8QeiAqCfIA0C6yO/0CFJDQkcgWqSsTIlTpCSSJJoCAdAHqtqY/oOncgz492pIyTInrEK7kDtrci21AeQx+9olxwXWP0e2GRf4AaLRPYOV32CbSVb1TzAtsDloR94AcQTfFFwBa/NAAAAAElFTkSuQmCC",
        "javaArgs": format!("-Dloader.modsDir=.velvet/mods/{0} -Dloader.configDir=.velvet/config/{0} {1}", &mc, &args),
        "javaDir": &dir,
        "lastUsed": &time,
        "lastVersionId": format!("quilt-loader-{}-{}", &velvet, &mc),
        "name": format!("Velvet {}", &mc),
        "type": "custom",
    });

    json.profiles.insert(format!("velvet-quilt-loader-{}", &mc), new_profile);
    serde_json::to_string_pretty(&json).unwrap()
}

