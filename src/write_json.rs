use std::fs::File;
use reqwest::blocking::Client;
use serde_json::json;
use chrono::Utc;
use colored::Colorize;
const VANILLA_ARGS: &str = "-Xmx2G -XX:+UnlockExperimentalVMOptions -XX:+UseG1GC -XX:G1NewSizePercent=20 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M";
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

    /*
    Quilt Installer Hack
    json["libraries"][11] = json!({
    "name": (format!("net.fabricmc:intermediary:{}", &mc)),
    "url": "https://maven.fabricmc.net/"
    });
    */

    serde_json::to_writer_pretty(z, &json).unwrap();
}

pub fn write_profile(mc: &String, velvet: &String, x: &File) -> String {
    let mut json: serde_json::Value = serde_json::from_reader(x).unwrap();

    // This copies the "Latest Release" java arguments and version.
    let release_id = get_release(&json);
    let args = match get_args(&json, &release_id) {
        json!(null) => json!(VANILLA_ARGS),
        x => x.to_owned()
    };
    let java_dir = match get_version(&json, &release_id) {
        json!(null) => "",
        x => x.as_str().unwrap()
    };
    let time = Utc::now();

    json["profiles"][format!("velvet-quilt-loader-{}", &mc)] = json!({
        "created": &time,
        "icon": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgBAMAAACBVGfHAAAAGFBMVEUAAAB7GWmyF1rqR1a3msT/nJfp0+r///+virVnAAAAAXRSTlMAQObYZgAAAI9JREFUKM990cENA0EIQ9FpgRbcglugBVr4Lbj9HCaRNtnZcHwSYMFaa63Vvb7rB5JS8QeiAqCfIA0C6yO/0CFJDQkcgWqSsTIlTpCSSJJoCAdAHqtqY/oOncgz492pIyTInrEK7kDtrci21AeQx+9olxwXWP0e2GRf4AaLRPYOV32CbSVb1TzAtsDloR94AcQTfFFwBa/NAAAAAElFTkSuQmCC",
        "javaArgs": format!("-Dloader.modsDir=.velvet/mods/{0} -Dloader.configDir=.velvet/config/{0} {1}", &mc, &args.as_str().unwrap()),
        "javaDir": &java_dir,
        "lastUsed": &time,
        "lastVersionId": format!("quilt-loader-{}-{}", &velvet, &mc),
        "name": format!("Velvet {}", &mc),
        "type": "custom",

    });
    serde_json::to_string_pretty(&json).unwrap()
}
fn get_release(x: &serde_json::Value) -> Option<String> {
    for (name, value) in x["profiles"].as_object().unwrap() {
        if value["type"] == json!("latest-release") {
            println!("Copying Java directory from: {}.", "latest-release".purple().italic());
            return Some(name.to_owned())
        }
    }
    None
}
fn get_version<'a>(x: &'a serde_json::Value,y: &'a Option<String>) -> &'a serde_json::Value {
    match y {
        Some(z) => &x["profiles"][&z]["javaDir"],
        None => &json!(null)
    }
}
fn get_args<'a>(x: &'a serde_json::Value,y: &'a Option<String>) -> &'a serde_json::Value {
    match y {
        Some(z) => &x["profiles"][&z]["javaArgs"],
        None => &json!(null)
    }
}
