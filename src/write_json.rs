use std::{fs::File, io::{Write, Seek}};
use reqwest::blocking::Client;
use serde_json::json;
use chrono::Utc;

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
    json["id"] = json!(format!("velvet-quilt-loader-{}-{}", &velvet, &mc));
    json["libraries"][13] = json!({
        "name": (format!("net.piuvas:velvet-quilt-loader:{}", &velvet)),
        "url": "https://file.garden/YAY9sxgapBy70PkV/maven/"
    });
    serde_json::to_writer_pretty(z, &json).unwrap();
}

pub fn write_profile (mc: &String, velvet: &String, mut z: &File) {
    let mut json: serde_json::Value = serde_json::from_reader(z).unwrap();
    let time = Utc::now();
    json["profiles"][format!("velvet-quilt-loader-{}", &mc)] = json!({
        "lastUsed": &time,
        "lastVersionId": format!("velvet-quilt-loader-{}-{}", &velvet, &mc),
        "created": &time,
        "name": format!("Velvet {}", &mc),
        "icon": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgBAMAAACBVGfHAAAAGFBMVEUAAAB7GWmyF1rqR1a3msT/nJfp0+r///+virVnAAAAAXRSTlMAQObYZgAAAI9JREFUKM990cENA0EIQ9FpgRbcglugBVr4Lbj9HCaRNtnZcHwSYMFaa63Vvb7rB5JS8QeiAqCfIA0C6yO/0CFJDQkcgWqSsTIlTpCSSJJoCAdAHqtqY/oOncgz492pIyTInrEK7kDtrci21AeQx+9olxwXWP0e2GRf4AaLRPYOV32CbSVb1TzAtsDloR94AcQTfFFwBa/NAAAAAElFTkSuQmCC",
        "type": "custom"
    });
    z.rewind().unwrap();
    serde_json::to_writer_pretty(z, &json).unwrap();
    z.flush().unwrap();
}