use std::{path::PathBuf, io::Write, fs::File};

use colored::Colorize;
use ferinth::Ferinth;
use percent_encoding::percent_decode_str;

const VANILLA: [&str; 8] = [
    "AANobbMI", // sodium
    "YL57xq9U", // iris
    "gvQqBUqZ", // lithium
    "hEOCdOgW", // phosphor
    "hvFnDODi", // lazydfu
    "uXXizFIs", // ferrite-core
    "fQEb0iXm", // krypton
    "FWumhS4T", // smoothboot-fabric
];

const VISUAL: [&str; 9] = [
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
pub async fn run(mc_version: &String, modlist: &(bool, bool, bool), mut path: PathBuf) {
    path.push("mod.jar");
    let modrinth = Ferinth::default();
    let mut mods: Vec<&str> = Vec::new();
    if modlist.0 { for x in VANILLA { mods.push(x) } }
    if modlist.1 { for x in VISUAL { mods.push(x) } }
    if modlist.2 { for x in OPTIFINE { mods.push(x) } }
    mods.sort();
    mods.dedup();
    
    for x in mods {
        let versions = modrinth.list_versions_filtered(x, Some(&["quilt", "fabric"]),  Some(&[mc_version.as_str()]), None).await.unwrap();

        // Check if there's an available version
        match versions.len() {
            0 => {
                println!("{} {} {} {}","The mod".red(), modrinth.get_project(x).await.unwrap().title.purple(), "is not available for".red(), &mc_version.purple());
            },
            _ => {
                let url = versions[0].files[0].url.to_owned();
                let mut file_name = url
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap()
                    .to_string();
                file_name = percent_decode_str(&file_name).decode_utf8().unwrap().into_owned();
                path.set_file_name(&file_name);

                println!("{} {}", "Downloading:".dimmed(), modrinth.get_project(x).await.unwrap().title.purple());
                let download = reqwest::get(url).await.unwrap().bytes().await.unwrap();
                let mut mod_file = File::create(&path).unwrap();
                mod_file.write_all(&download).unwrap();
            }
        }
    }
}
