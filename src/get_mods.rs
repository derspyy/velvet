use std::{fs::File, io::Write, path::PathBuf, thread};

use anyhow::{anyhow, Result};
use serde_json::Value;
use percent_encoding::percent_decode_str;


const VANILLA: [&str; 8] = [
    "AANobbMI", // sodium
    "gvQqBUqZ", // lithium
    "hEOCdOgW", // phosphor
    "hvFnDODi", // lazydfu
    "uXXizFIs", // ferrite-core
    "fQEb0iXm", // krypton
    "FWumhS4T", // smoothboot-fabric
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

pub fn run(
    mc_version: String,
    modlist: &(bool, bool, bool),
    base_path: PathBuf,
) -> Result<Vec<String>> {
    let agent = ureq::AgentBuilder::new()
        .user_agent(concat!( env!("CARGO_PKG_NAME"), "+", env!("CARGO_PKG_VERSION") ))
        .build();
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
    let mut thread_vec = Vec::new();
    for x in mods {
        let agent = agent.clone();
        let mc_version = mc_version.clone();
        let base_path = base_path.clone();
        thread_vec.push(thread::spawn(move || {
            download_mod(x, agent, mc_version, base_path)
        }));
    }

    let mut error_vec = Vec::new();

    for thread in thread_vec {
        if let Status::NotFound(x) = thread.join().unwrap()? {
            error_vec.push(x);
        }
    }

    Ok(error_vec)
}

enum Status {
    Found,
    NotFound(String),
}

fn download_mod(x: &str, agent: ureq::Agent, mc_version: String, base_path: PathBuf) -> Result<Status, anyhow::Error> {
    let mut modrinth_url = format!("{}/{}", MODRINTH_SERVER, x);

    let name_response: Value = agent
            .get(&modrinth_url)
            .call()?
            .into_json()?;
    let name = name_response["slug"].as_str()
            .ok_or_else(|| anyhow!("Couldn't get project name!"))?
            .to_string();

    modrinth_url = format!("{}/version?loaders=[\"fabric\", \"quilt\"]&game_versions=[{:?}]", modrinth_url, mc_version);

    let version_response: Value = agent
            .get(&modrinth_url)
            .call()?
            .into_json()?;
    let versions = match version_response[0]["files"].as_array() {
        Some(x) => x,
        None => return Ok( Status::NotFound(name) ),
    };

    if !versions.is_empty() {
        let url = versions[0]["url"].as_str()
                .ok_or_else(|| anyhow!("Couldn't parse versions!"))?;
        let mut file_name = url
                .split('/')
                .last()
                .ok_or_else(|| anyhow!("Couldn't parse file name!"))?
                .to_string();
        file_name = percent_decode_str(&file_name).decode_utf8()?.into_owned();
        let path = base_path.join(file_name).with_extension("jar");
        let download = agent.get(url).call()?;
        let mut bytes = Vec::new();
        download.into_reader().read_to_end(&mut bytes)?;
        let mut mod_file = File::create(path)?;
        mod_file.write_all(&bytes)?;
    }
    Ok(Status::Found)
}