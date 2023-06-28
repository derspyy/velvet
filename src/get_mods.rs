use anyhow::{anyhow, Result};
use async_std::fs::remove_file;
use async_std::{fs::File, path::PathBuf};
use async_std::{prelude::*, task};
use iced::futures::future::join_all;
use reqwest::{Client, ClientBuilder};
use serde_json::Value;
use std::collections::HashMap;

const VANILLA: [&str; 7] = [
    "AANobbMI", // sodium
    "gvQqBUqZ", // lithium
    "hEOCdOgW", // phosphor
    "hvFnDODi", // lazydfu
    "uXXizFIs", // ferrite-core
    "fQEb0iXm", // krypton
    "5ZwdcRci", // immediatelyfast
];

const VISUAL: [&str; 11] = [
    "pcPXJeZi", // effective
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
    "iG6ZHsUV", // cull-less-leaves
    "1IjD5062", // continuity
    "2Uev7LdA", // lambdabettergrass
    "otVJckYQ", // cit-resewn
    "BVzZfTc1", // entitytexturefeatures
    "qvIfYCYJ", // qsl
];

const MODRINTH_SERVER: &str = "https://api.modrinth.com/v2/project";

enum Status {
    Found(String, String, String),
    NotFound(String),
}

pub async fn run(
    mc_version: String,
    modlist: &(bool, bool, bool),
    (path_mods, version_file_path): (PathBuf, PathBuf),
) -> Result<Vec<String>> {
    let client = ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "+",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    let mut version_file = File::open(&version_file_path).await?;

    let mut bytes = Vec::new();
    version_file.read_to_end(&mut bytes).await?;
    let existing_mods: HashMap<String, String> = match serde_json::from_slice(&bytes) {
        Ok(x) => x,
        Err(_) => HashMap::new(),
    };

    let mut mods = Vec::new();

    if modlist.0 {
        VANILLA.into_iter().for_each(|x| mods.push(x));
    }
    if modlist.1 {
        VISUAL.into_iter().for_each(|x| mods.push(x));
    }
    if modlist.2 {
        OPTIFINE.into_iter().for_each(|x| mods.push(x));
    }

    mods.sort();
    mods.dedup();

    let mut get_versions = Vec::new();
    let mut download_mods = Vec::new();

    for x in mods {
        let task = task::spawn(check_latest(x, client.clone(), mc_version.clone()));
        get_versions.push(task);
    }

    let mut new_mods: HashMap<String, String> = HashMap::new();
    let mut mods_not_found = Vec::new();

    for result in join_all(get_versions).await {
        match result {
            Err(x) => return Err(x),
            Ok(Status::NotFound(x)) => mods_not_found.push(x),
            Ok(Status::Found(name, url, hash)) => {
                match existing_mods.get(&name) {
                    Some(x) if x == &hash => {
                        println!("Already found \x1b[35m{}\x1b[39m.", name)
                    }
                    _ => download_mods.push(task::spawn(download_mod(
                        url,
                        name.clone(),
                        path_mods.clone(),
                        client.clone(),
                    ))),
                }
                new_mods.insert(name, hash);
            }
        }
    }

    for (name, hash) in existing_mods {
        if new_mods.get(&name) != Some(&hash) {
            println!("Removing \x1b[35m{}\x1b[39m.", name);
            remove_file(path_mods.join(name).with_extension("jar")).await?
        }
    }

    join_all(download_mods).await;

    let mut version_file = File::create(version_file_path).await?;

    version_file
        .write_all(&serde_json::to_vec_pretty(&new_mods)?)
        .await?;

    Ok(mods_not_found)
}

async fn download_mod(url: String, file_name: String, path: PathBuf, client: Client) -> Result<()> {
    println!("Downloading \x1b[35m{}\x1b[39m.", file_name);
    let path = path.join(&file_name).with_extension("jar");
    let download = client.get(url).send().await?.bytes().await?;
    let mut mod_file = File::create(path).await?;
    mod_file.write(&download).await?;
    println!("Finished downloading \x1b[35m{}\x1b[39m.", file_name);
    Ok(())
}

async fn check_latest(x: &str, client: Client, mc_version: String) -> Result<Status> {
    let mut modrinth_url = format!("{}/{}", MODRINTH_SERVER, x);
    let name_response: Value = client.get(&modrinth_url).send().await?.json().await?;
    let name = name_response["slug"]
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
            .ok_or_else(|| anyhow!("Couldn't parse versions!"))?
            .into();
        let hash = versions[0]["hashes"]["sha1"]
            .as_str()
            .ok_or_else(|| anyhow!("Couldn't parse versions!"))?
            .into();
        Ok(Status::Found(name, url, hash))
    } else {
        Ok(Status::NotFound(name))
    }
}
