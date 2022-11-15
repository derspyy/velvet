use std::{fs::File, io::{Write, Seek}};
use reqwest::blocking::Client;
use serde_json::json;
use chrono::Utc;
const VANILLA_ARGS: &str = "-Xmx2G -XX:+UnlockExperimentalVMOptions -XX:+UseG1GC -XX:G1NewSizePercent=20 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M";
pub fn write_version(mc: &String, velvet: &String, z: &File) {

    let url = format!("https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json", &mc, &velvet);
    let client = Client::new();
    let mut json: serde_json::Value = client
            .get(&url)
            .header("User-Agent", "Velvet")
            .send()
            .expect("Couldn't communicate with Quilt's meta server.")
            .json()
            .unwrap();

    // Quilt Installer Hack
    json["libraries"][11] = json!({
    "name": (format!("net.fabricmc:intermediary:{}", &mc)),
    "url": "https://maven.fabricmc.net/"
    });
    // TODO: remove this asap

    serde_json::to_writer_pretty(z, &json).unwrap();
}

pub fn write_profile (mc: &String, velvet: &String, mut z: &File) {
    let mut json: serde_json::Value = serde_json::from_reader(z).unwrap();
    let time = Utc::now();
    json["profiles"][format!("velvet-quilt-loader-{}", &mc)] = json!({
        "created": &time,
        "icon": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgBAMAAACBVGfHAAAAGFBMVEUAAAB7GWmyF1rqR1a3msT/nJfp0+r///+virVnAAAAAXRSTlMAQObYZgAAAI9JREFUKM990cENA0EIQ9FpgRbcglugBVr4Lbj9HCaRNtnZcHwSYMFaa63Vvb7rB5JS8QeiAqCfIA0C6yO/0CFJDQkcgWqSsTIlTpCSSJJoCAdAHqtqY/oOncgz492pIyTInrEK7kDtrci21AeQx+9olxwXWP0e2GRf4AaLRPYOV32CbSVb1TzAtsDloR94AcQTfFFwBa/NAAAAAElFTkSuQmCC",
        "javaArgs": format!("-Dloader.modsDir=.velvet/mods/{0} -Dloader.configDir=.velvet/config/{0} {1}", &mc, VANILLA_ARGS),
        "lastUsed": &time,
        "lastVersionId": format!("quilt-loader-{}-{}", &velvet, &mc),
        "name": format!("Velvet {}", &mc),

        "type": "custom",

    });
    z.rewind().unwrap();
    serde_json::to_writer_pretty(z, &json).unwrap();
    z.flush().unwrap();
}