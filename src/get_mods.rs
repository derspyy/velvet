use std::{path::PathBuf, io::copy, fs::{File, self}};

use ferinth::Ferinth;
use percent_encoding::percent_decode_str;

#[tokio::main]
pub async fn run(mc_version: &String, mut path: PathBuf) {
    let modrinth = Ferinth::default();
    let mods = [
        "AANobbMI", // sodium
        "gvQqBUqZ", // lithium
        "hEOCdOgW", // phosphor
        "hvFnDODi", // lazydfu
        "uXXizFIs", // ferrite-core
        "fQEb0iXm", // krypton
        "FWumhS4T", // smoothboot-fabric
        ];
    for x in mods {
        println!("Downloading: {}", modrinth.get_project(&x).await.unwrap().title);
        let versions = modrinth.list_versions_filtered(&x, Some(&["quilt", "fabric"]),  Some(&[&mc_version.as_str()]), None).await.unwrap();
        if versions.len() != 0 {
            let url = versions[0].files[0].url.to_owned();
            let mut file_name = url
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap()
                    .to_string();
            file_name = percent_decode_str(&file_name).decode_utf8().unwrap().into_owned();
            let download = reqwest::get(url).await.unwrap().text().await.unwrap();
            path.push(file_name);
            let mut mod_file = File::create(&path).unwrap();
            copy(&mut download.as_bytes(), &mut mod_file).unwrap();
            path.pop();
        } else {
            return
        }
    }
}
