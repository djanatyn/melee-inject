#![feature(slice_group_by)]

use codegen::Scope;
use gc_gcm::{FsNode, GcmFile};
use std::collections::HashMap;

const SSBM_ISO: &str = "ssbm.iso";

use phf::phf_map;
use std::io;
use std::str::FromStr;

/// This information was foudn from DRGN's DTW:
/// <https://github.com/DRGN-DRC/DAT-Texture-Wizard/blob/75d5c5c2cfbedb8ecda2e8a3db8cdb083fb6b892/DAT%20Texture%20Wizard.py#L175>
const CHARACTER_PREFIXES: phf::Map<&'static str, &'static str> = phf_map! {
    "Bo" => "Male Wireframe",
    "Ca" => "Captain Falcon",
    "Ch" => "Crazy Hand",
    "Cl" => "Young Link",
    "Co" => "Common",
    "Dk" => "Donkey Kong",
    "Dr" => "Dr. Mario",
    "Fc" => "Falco",
    "Fe" => "Roy",
    "Fx" => "Fox",
    "Gk" => "GigaBowser",
    "Gl" => "Female Wireframe",
    "Gn" => "Ganondorf",
    "Gw" => "Game 'n Watch",
    "Ic" => "Ice Climbers",
    "Kb" => "Kirby",
    "Kp" => "Bowser",
    "Lg" => "Luigi",
    "Lk" => "Link",
    "Mh" => "Master Hand",
    "Mn" => "Menus Data",
    "Mr" => "Mario",
    "Ms" => "Marth",
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

const COLORS: phf::Map<&'static str, &'static str> = phf_map! {
    "Aq" => "aqua",
    "Bk" => "black",
    "Bu" => "blue",
    "Gr" => "green",
    "Gy" => "gray",
    "La" => "lavender",
    "Nr" => "neutral",
    "Or" => "orange",
    "Pi" => "pink",
    "Re" => "red",
    "Wh" => "white",
    "Ye" => "yellow",
};

#[derive(Debug)]
struct CharacterFile {
    filename: String,
    name: String,
    color: Option<String>,
}

impl FromStr for CharacterFile {
    type Err = io::Error;

    fn from_str(filename: &str) -> io::Result<Self> {
        println!("{filename}");

        // strip Pl prefix
        let prefix_stripped = filename
            .strip_prefix("Pl")
            .expect("failed to strip Pl prefix: {filename}");

        // take next two characters
        let char_code: String = prefix_stripped.chars().take(2).collect();

        // match against character prefixes
        let name = CHARACTER_PREFIXES
            .get(&char_code)
            .expect("failed to find prefix");

        // take remaining characters
        let remaining = prefix_stripped
            .strip_prefix(&char_code)
            .expect("failed to strip after character prefix")
            .strip_suffix(".dat")
            .expect("failed to strip .dat suffix");

        // return on animations and common files
        if remaining.len() == 0 || remaining == "AJ" || remaining == "DViWaitAJ" {
            return Ok(CharacterFile {
                filename: filename.to_string(),
                name: name.to_string(),
                color: None,
            });
        }

        // special handling for Kirby
        if char_code == "Kb" {
            // 3 types of Kirby file:
            // - PlKb.dat (covered previously)
            // - PlKb<COLOR>.dat
            // - PlKb<COLOR>Cp<CHAR_CODE>
            // - PlKbCp<CHAR_CODE>.dat

            return match remaining.len() {
                // <COLOR>
                2 => {
                    let color = COLORS.get(&remaining).expect("failed to match color");

                    Ok(CharacterFile {
                        filename: filename.to_string(),
                        name: name.to_string(),
                        color: Some(color.to_string()),
                    })
                }
                // Cp<CHAR_CODE>
                4 => {
                    let copied_char_code = remaining
                        .strip_prefix("Cp")
                        .expect("failed on char-specific kirby copy power file");

                    let copied_char = CHARACTER_PREFIXES
                        .get(&copied_char_code)
                        .expect("failed to find kirby copied char");

                    Ok(CharacterFile {
                        filename: filename.to_string(),
                        name: format!("Kirby Copy Power [{copied_char}]"),
                        color: None,
                    })
                }
                // <COLOR>Cp<CHAR_CODE>
                6 => {
                    let color_code: String = remaining.chars().take(2).collect();
                    let color = COLORS.get(&color_code).expect("failed to match color");

                    let copied_char_code = remaining
                        .strip_prefix(&color_code)
                        .expect("failed to strip color")
                        .strip_prefix("Cp")
                        .expect("failed to find kirby character code");

                    let copied_char = CHARACTER_PREFIXES
                        .get(&copied_char_code)
                        .expect("failed to find kirby copied char");

                    Ok(CharacterFile {
                        filename: filename.to_string(),
                        name: format!("Kirby Copy Power [{copied_char}]"),
                        color: Some(color.to_string()),
                    })
                }
                _ => panic!("unexpected kirby file"),
            };
        }

        let color = COLORS.get(&remaining).expect("failed to match color");

        Ok(CharacterFile {
            filename: filename.to_string(),
            name: name.to_string(),
            color: Some(color.to_string()),
        })
    }
}

#[derive(Debug)]
struct CharacterMatch {}

fn main() {
    let iso = GcmFile::open(SSBM_ISO).expect("could not open ISO");

    let mut scope = Scope::new();
    let mut characters: HashMap<String, Vec<CharacterFile>> = HashMap::new();

    for node in iso.filesystem.files {
        // find all file entries (skip directories)
        let name = match node {
            FsNode::File { name, .. } => name,
            _ => continue,
        };

        // skip files that dont end in .dat
        if !name.ends_with(".dat") {
            continue;
        }

        // character names start with "Pl-"
        if name.starts_with("Pl") {
            let file = name.parse::<CharacterFile>().expect("failed to parse");

            match characters.get_mut(&file.name) {
                Some(files) => files.push(file),
                None => {
                    let key = file.name.clone();
                    characters.insert(key, vec![file]);
                }
            };
        }

        // scope.new_struct(&name);
    }

    println!("{characters:#?}");

    // let output = scope.to_string();
}
