use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{anyhow, Result};
use ferinth::Ferinth;
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

#[tokio::main]
pub async fn run(
    mc_version: String,
    modlist: &(bool, bool, bool),
    base_path: PathBuf,
) -> Result<()> {
    let modrinth = Ferinth::default();
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
        let versions = modrinth
            .list_versions_filtered(
                x,
                Some(&["quilt", "fabric"]),
                Some(&[mc_version.as_str()]),
                None,
            )
            .await?;

        if !versions.is_empty() {
            let url = versions[0].files[0].url.to_owned();
            let mut file_name = url
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .ok_or_else(|| anyhow!("Couldn't parse file name!"))?
                .to_string();
            file_name = percent_decode_str(&file_name).decode_utf8()?.into_owned();
            let path = base_path.join(file_name).with_extension("jar");
            let download = ureq::get(url.as_str()).call()?;
            let mut bytes = Vec::new();
            download.into_reader().read_to_end(&mut bytes)?;
            let mut mod_file = File::create(path)?;
            mod_file.write_all(&bytes)?;
        }
    }
    Ok(())
}
