use anyhow::{anyhow, Result};
use async_std::prelude::*;
use async_std::{fs::File, path::PathBuf};
use iced::futures::future::join_all;
use percent_encoding::percent_decode_str;
use reqwest::{Client, ClientBuilder};
use serde_json::Value;

const VANILLA: [&str; 7] = [
    "AANobbMI", // sodium
    "gvQqBUqZ", // lithium
    "hEOCdOgW", // phosphor
    "hvFnDODi", // lazydfu
    "uXXizFIs", // ferrite-core
    "fQEb0iXm", // krypton
    "5ZwdcRci", // immediatelyfast
];

const VISUAL: [&str; 10] = [
    // effective
    "yBW8D80W", // lambdynamiclights
    "MPCX6s5C", // not-enough-animations
    "WhbRG4iK", // fallingleaves
    "mfzaZK3Z", // ears
    "Orvt0mRa", // indium
    "2Uev7LdA", // lambdabettergrass
    "1IjD5062", // continuity
    "qvIfYCYJ", // qsl
    "9s6osm5g", // cloth-config
    "YL57xq9U", // iris
];

const OPTIFINE: [&str; 9] = [
    "3IuO68q1", // puzzle
    "PRN43VSY", // animatica
    "Orvt0mRa", // indium
    "GNxdLCoP", // cull-leaves
    "1IjD5062", // continuity
    "2Uev7LdA", // lambdabettergrass
    "otVJckYQ", // cit-resewn
    "BVzZfTc1", // entitytexturefeatures
    "qvIfYCYJ", // qsl
];

const MODRINTH_SERVER: &str = "https://api.modrinth.com/v2/project";

pub async fn run(
    mc_version: String,
    modlist: &(bool, bool, bool),
    base_path: PathBuf,
) -> Result<String> {
    let client = ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "+",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;
    let mut mods: Vec<&str> = Vec::new();
    if modlist.0 {
        for x in VANILLA {
            mods.push(x)
        }
    }
    if modlist.1 {
        for x in VISUAL {
            mods.push(x)
        }
    }
    if modlist.2 {
        for x in OPTIFINE {
            mods.push(x)
        }
    }
    mods.sort();
    mods.dedup();

    let mut tasks = Vec::new();
    for task in mods {
        tasks.push(download_mod(
            task,
            client.clone(),
            mc_version.clone(),
            base_path.clone(),
        ));
    }

    let mut mods_not_found = String::new();

    let results = join_all(tasks).await;
    for result in results {
        match result {
            Ok(Status::Found) => {}
            Ok(Status::NotFound(name)) => {
                if mods_not_found.is_empty() {
                    mods_not_found.push_str(&name);
                } else {
                    mods_not_found.push(',');
                    mods_not_found.push(' ');
                    mods_not_found.push_str(&name);
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(mods_not_found)
}

enum Status {
    Found,
    NotFound(String),
}

async fn download_mod(
    x: &str,
    client: Client,
    mc_version: String,
    base_path: PathBuf,
) -> Result<Status, anyhow::Error> {
    let mut modrinth_url = format!("{}/{}", MODRINTH_SERVER, x);

    let name_response: Value = client.get(&modrinth_url).send().await?.json().await?;
    let name = name_response["title"]
        .as_str()
        .ok_or_else(|| anyhow!("Couldn't get project name!"))?
        .to_owned();

    modrinth_url = format!(
        "{}/version?loaders=[\"fabric\", \"quilt\"]&game_versions=[{:?}]",
        modrinth_url, mc_version
    );

    let version_response: Value = client.get(&modrinth_url).send().await?.json().await?;
    let versions = match version_response[0]["files"].as_array() {
        Some(x) => x,
        None => return Ok(Status::NotFound(name)),
    };

    if !versions.is_empty() {
        let url = versions[0]["url"]
            .as_str()
            .ok_or_else(|| anyhow!("Couldn't parse versions!"))?;
        let mut file_name = url
            .split('/')
            .last()
            .ok_or_else(|| anyhow!("Couldn't parse file name!"))?
            .to_string();
        file_name = percent_decode_str(&file_name).decode_utf8()?.into_owned();
        let path = base_path.join(file_name).with_extension("jar");
        let download = client.get(url).send().await?.bytes().await?;
        let mut mod_file = File::create(path).await?;
        mod_file.write(&download).await?;
    }
    Ok(Status::Found)
}
