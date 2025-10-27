use std::path::PathBuf;
use std::collections::{HashMap, HashSet};

use anyhow::Result;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::task::spawn;
use iced::futures::future::try_join_all;
use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use sha1::{Digest, Sha1};

const VANILLA: [&str; 9] = [
    "AANobbMI", // sodium
    "gvQqBUqZ", // lithium
    "hEOCdOgW", // phosphor
    "hvFnDODi", // lazydfu
    "uXXizFIs", // ferrite-core
    "fQEb0iXm", // krypton
    "5ZwdcRci", // immediatelyfast
    "VSNURh3q", // c2me-fabric
    "KuNKN7d2", // noisium
];

const VISUAL: [&str; 11] = [
    "pcPXJeZi", // effective
    "yBW8D80W", // lambdynamiclights
    "MPCX6s5C", // not-enough-animations
    "mfzaZK3Z", // ears
    "Orvt0mRa", // indium
    "2Uev7LdA", // lambdabettergrass
    "1IjD5062", // continuity
    "YL57xq9U", // iris
    "uCdwusMi", // distanthorizons
    "P7dR8mSH", // fabric-api
    "9s6osm5g", // cloth-config
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
    // "qvIfYCYJ", qsl
    "P7dR8mSH", // fabric-api
];

const MODRINTH_SERVER: &str = "https://api.modrinth.com/v2/project";

enum Status {
    Found(&'static str, String, String),
    NotFound(String),
}

#[derive(Deserialize)]
struct Project {
    slug: String,
}

#[derive(Deserialize)]
struct Version {
    files: Vec<VersionFile>,
}

#[derive(Deserialize)]
struct VersionFile {
    url: String,
    hashes: Hashes,
}

#[derive(Deserialize)]
struct Hashes {
    sha1: String,
}

pub async fn run(
    mc_version: String,
    modlist: &(bool, bool, bool),
    path_mods: PathBuf,
) -> Result<Vec<String>> {
    let client = ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "+",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    let mut existing_mods = HashMap::new();
    let mut mod_folder_reader = fs::read_dir(&path_mods).await?;

    while let Some(file) = mod_folder_reader.next_entry().await? {
        let file_name = file.file_name().to_string_lossy().to_string();
        let mod_id = String::from(&file_name[0..8]);
        let file_bytes = fs::read(file.path()).await?;

        let result = Sha1::digest(file_bytes);
        let hash_hex = hex::encode(result);

        existing_mods.insert(mod_id, hash_hex);
    }

    let mut mods = HashSet::new();

    if modlist.0 {
        for x in VANILLA {
            mods.insert(x);
        }
    }
    if modlist.1 {
        for x in VISUAL {
            mods.insert(x);
        }
    }
    if modlist.2 {
        for x in OPTIFINE {
            mods.insert(x);
        }
    }

    let mut get_versions = Vec::new();
    let mut download_mods = Vec::new();

    for x in mods {
        let task = spawn(check_latest(x, client.clone(), mc_version.clone()));
        get_versions.push(task);
    }

    let mut new_mods = HashMap::new();
    let mut mods_not_found = Vec::new();

    for result in try_join_all(get_versions).await? {
        match result? {
            Status::NotFound(x) => mods_not_found.push(x),
            Status::Found(name, url, hash) => {
                match existing_mods.get(name) {
                    Some(x) if x == &hash => {
                        println!("Already found \x1b[35m{name}\x1b[39m.")
                    }
                    _ => download_mods.push(spawn(download_mod(
                        url,
                        name,
                        path_mods.clone(),
                        client.clone(),
                    ))),
                }
                new_mods.insert(name, hash);
            }
        }
    }

    for (name, hash) in existing_mods {
        if new_mods.get(name.as_str()) != Some(&hash) {
            println!("Removing \x1b[35m{name}\x1b[39m.");
            fs::remove_file(path_mods.join(name).with_extension("jar")).await?
        }
    }

    try_join_all(download_mods).await?;
    Ok(mods_not_found)
}

async fn download_mod(url: String, file_name: &str, path: PathBuf, client: Client) -> Result<()> {
    println!("Downloading \x1b[35m{file_name}\x1b[39m.");
    let path = path.join(file_name).with_extension("jar");
    let download = client.get(url).send().await?.bytes().await?;
    let mut mod_file = fs::File::create(path).await?;
    mod_file.write(&download).await?;
    println!("Finished downloading \x1b[35m{file_name}\x1b[39m.");
    Ok(())
}

async fn check_latest(x: &'static str, client: Client, mc_version: String) -> Result<Status> {
    let mut modrinth_url = format!("{MODRINTH_SERVER}/{x}");
    let project: Project = client.get(&modrinth_url).send().await?.json().await?;

    modrinth_url = format!(
        "{modrinth_url}/version?loaders=[\"fabric\", \"quilt\"]&game_versions=[{mc_version:?}]"
    );

    let version_response: Vec<Version> = client.get(&modrinth_url).send().await?.json().await?;
    if let Some(version) = version_response.first()
        && let Some(file) = version.files.first()
    {
        let url = file.url.to_owned();
        let hash = file.hashes.sha1.to_owned();
        Ok(Status::Found(x, url, hash))
    } else {
        Ok(Status::NotFound(project.slug))
    }
}
