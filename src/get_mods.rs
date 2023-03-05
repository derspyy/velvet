use std::{fs::File, io::Write, path::PathBuf};

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
) -> Result<()> {
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
    for x in mods {
        let modrinth_url = format!("{}/{}/version?loaders=[\"fabric\", \"quilt\"]?game_versions=[{:?}]", MODRINTH_SERVER, x, mc_version);
        let response: Value = agent
            .get(&modrinth_url)
            .call()?
            .into_json()?;
        let versions = response[0]["files"].as_array()
            .ok_or_else(|| anyhow!("Couldn't parse versions!"))?;

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
    }
    Ok(())
}