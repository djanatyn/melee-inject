#![feature(slice_group_by)]

use codegen::Scope;
use gc_gcm::{FsNode, GcmFile};

const SSBM_ISO: &str = "ssbm.iso";

use phf::phf_map;
use std::io;
use std::str::FromStr;

/// This information was foudn from DRGN's DTW:
/// <https://github.com/DRGN-DRC/DAT-Texture-Wizard/blob/75d5c5c2cfbedb8ecda2e8a3db8cdb083fb6b892/DAT%20Texture%20Wizard.py#L175>
const CHARACTER_PREFIXES: phf::Map<&'static str, &'static str> = phf_map! {
    "Bo" => "[Boy] Male Wireframe",
    "Ca" => "Captain Falcon",
    "Ch" => "Crazy Hand",
    "Cl" => "Child/Young Link",
    "Co" => "Common to the cast",
    "Dk" => "Donkey Kong",
    "Dr" => "Dr. Mario",
    "Fc" => "Falco",
    "Fe" => "[Fire Emblem] Roy",
    "Fx" => "Fox",
    "Gk" => "[GigaKoopa] GigaBowser",
    "Gl" => "[Girl] Female Wireframe",
    "Gn" => "Ganondorf",
    "Gw" => "Game 'n Watch",
    "Ic" => "Ice Climbers",
    "Kb" => "Kirby",
    "Kp" => "[Koopa] Bowser",
    "Lg" => "Luigi",
    "Lk" => "Link",
    "Mh" => "Master Hand",
    "Mn" => "Menus Data",
    "Mr" => "Mario",
    "Ms" => "[Mars] Marth",
    "Mt" => "Mewtwo",
    "Nn" => "[Nana] Ice Climbers",
    "Ns" => "Ness",
    "Pc" => "Pichu",
    "Pe" => "Peach",
    "Pk" => "Pikachu",
    "Pn" => "[Popo/Nana] Ice Climbers",
    "Pp" => "[Popo] Ice Climbers",
    "Pr" => "Jigglypuff",
    "Sb" => "SandBag",
    "Sk" => "Sheik",
    "Ss" => "Samus",
    "Wf" => "Wolf",
    "Ys" => "Yoshi",
    "Zd" => "Zelda",
};

#[derive(Debug)]
struct CharacterFile {
    filename: String,
    name: String,
}

impl FromStr for CharacterFile {
    type Err = io::Error;

    fn from_str(filename: &str) -> io::Result<Self> {
        // strip Pl prefix
        let prefix_stripped = filename
            .strip_prefix("Pl")
            .expect("failed to strip Pl prefix: {filename}");

        // take next two characters
        let char_code: String = prefix_stripped.chars().take(2).collect();

        let name = CHARACTER_PREFIXES
            .get(&char_code)
            .expect("failed to find prefix: {char_code}");

        Ok(CharacterFile {
            filename: filename.to_string(),
            name: name.to_string(),
        })
    }
}

#[derive(Debug)]
struct CharacterMatch {}

fn main() {
    let iso = GcmFile::open(SSBM_ISO).expect("could not open ISO");

    let mut scope = Scope::new();
    let mut characters: Vec<CharacterFile> = Vec::new();

    for node in iso.filesystem.files {
        // find all file entries (skip directories)
        let name = match node {
            FsNode::File { name, .. } => name,
            _ => continue,
        };

        // skip files that don't end in .dat
        if !name.ends_with(".dat") {
            continue;
        }

        // character names start with "Pl-"
        if name.starts_with("Pl") {
            let file = name
                .parse::<CharacterFile>()
                .expect("failed to parse {name}");
            characters.push(file);
        }

        // scope.new_struct(&name);
    }

    let groupings = characters
        .group_by(|a, b| a.name == b.name)
        .collect::<Vec<_>>();

    println!("{groupings:#?}");

    // let output = scope.to_string();
}
