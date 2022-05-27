#![feature(pattern)]

// TODO: output logs with tracing
// TODO: update README.md with goals, direction

use clap::Parser;
use gc_gcm::{FsNode, GcmFile};
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::str::pattern::Pattern;

const ISO_PATH: &str = "ssbm.iso";

pub mod characters {
    //! Supported character files for replacement.

    /// DAT files for Captain Falcon, used as replacement targets.
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub enum CaptainFalconFile {
        /// NTSC data & shared textures.
        PlCa,
        /// Blue costume.
        PlCaBu,
        /// Green costume.
        PlCaGr,
        /// Gray costume.
        PlCaGy,
        /// Neutral costume.
        PlCaNr,
        /// Red costume.
        PlCaRe,
        /// White costume.
        PlCaWh,
    }

    #[derive(Debug, Clone)]
    pub enum Character {
        CaptainFalcon(CaptainFalconFile),
    }

    impl Character {
        pub fn filename(character: &Self) -> &'static str {
            match character {
                Character::CaptainFalcon(CaptainFalconFile::PlCa) => "PlCa.dat",
                Character::CaptainFalcon(CaptainFalconFile::PlCaNr) => "PlCaNr.dat",
                Character::CaptainFalcon(CaptainFalconFile::PlCaGr) => "PlCaGr.dat",
                _ => todo!(),
            }
        }
    }
}

use characters::Character;

/// A queued replacement to be executed later.
///
/// This potential replacement is guaranteed to match a file in the FST.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Replacement {
    /// Which file to replace?
    pub target: Character,
    /// Path to replacement data.
    pub replacement: PathBuf,
}

/// An update to execute against the GCM FST.
#[derive(Clone)]
struct UpdateFST {
    name: String,
    original_offset: u32,
    updated_offset: u32,
    original_size: u32,
    updated_size: u32,
    data: Vec<u8>,
}

impl fmt::Debug for UpdateFST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UpdateFST")
            .field("name", &self.name)
            .field("original_offset", &self.original_offset)
            .field("updated_offset", &self.updated_offset)
            .field("original_size", &self.original_size)
            .field("updated_size", &self.updated_size)
            .finish()
    }
}

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Jonathan Strickland <djanatyn@gmail.com>")]
pub struct Args {
    /// Path to Melee ISO.
    #[clap(parse(from_os_str), short, long)]
    pub melee_iso: PathBuf,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    /// Search (and display) a specific DAT file.
    SearchDAT(SearchDAT),
    ShowFST,
}

#[derive(Parser, Debug)]
pub struct SearchDAT {
    #[clap(short, long)]
    pub dat_query: String,
}

fn read_file(file: &FsNode) -> io::Result<UpdateFST> {
    match file {
        FsNode::File { size, offset, name } => {
            let mut file = std::fs::File::open(ISO_PATH)?;
            let mut data = Vec::with_capacity(*size as usize);
            file.seek(SeekFrom::Start(*offset as u64))?;
            Read::by_ref(&mut file)
                // file.by_ref()
                .take(*size as u64)
                .read_to_end(&mut data)?;

            Ok(UpdateFST {
                name: name.to_string(),
                updated_offset: *offset,
                original_offset: *offset,

                original_size: *size,
                updated_size: *size,
                data,
            })
        }
        _ => panic!("failure"),
    }
}

#[allow(dead_code)]
fn update_fst(_updates: &Vec<UpdateFST>, _fst: Vec<u8>) -> Vec<u8> {
    todo!();
}

/// Given a set of potential replacements, attempt to rebuild the FST.
///
/// This function should:
/// - update offsets for files after the replacement, and
/// - apply padding between files (4 bytes)
///
/// 13.4 Format of the FST
/// ======================
/// +-----------+---------+----------+---------------------------------+
/// |   start   |   end   |   size   |   Description                   |
/// +-----------+---------+----------+---------------------------------+
/// |  0x00     |  0x0c   |  0x0c    | Root Directory Entry            |
/// +-----------+---------+----------+---------------------------------+
/// |  0x0c     |  ...    |  0x0c    | more File- or Directory Entries |
/// +-----------+---------+----------+---------------------------------+
/// |  ...      |  ...    |  ...     | String table                    |
/// +-----------+---------+----------+---------------------------------+
///
/// 13.4.1 Format of a File Entry
/// =============================
/// +-----------+---------+----------+------------------------------+
/// |   start   |   end   |   size   |   Description                |
/// +-----------+---------+----------+------------------------------+
/// |   0x00    |         |   1      | flags; 0: file 1: directory  |
/// +-----------+---------+----------+------------------------------+
/// |   0x01    |         |   3      | filename, offset into string |
/// |           |         |          | table                        |
/// +-----------+---------+----------+------------------------------+
/// |   0x04    |         |   4      | file_offset or parent_offset |
/// |           |         |          | (dir)                        |
/// +-----------+---------+----------+------------------------------+
/// |   0x08    |         |   4      | file_length or num_entries   |
/// |           |         |          | (root) or next_offset (dir)  |
/// +-----------+---------+----------+------------------------------+
///
/// (information from YAGCD)
/// $ pandoc -f html -t haddock 'https://www.gc-forever.com/yagcd/chap13.html'
///
fn rebuild_fst(iso: &GcmFile, replacements: &Vec<Replacement>) -> Vec<u8> {
    let new_fst = iso.fst_bytes.clone();

    let mut replacement_map: HashMap<u32, UpdateFST> = HashMap::new();
    for file in &iso.filesystem.files {
        match file {
            // for each file, insert a mutable UpdateFST, indexed by offset
            file @ FsNode::File { offset, .. } => {
                replacement_map.insert(*offset, read_file(&file).expect("failed to read file"))
            }
            _ => continue,
        };
    }

    for replacement in replacements {
        eprintln!("{replacement:?}");
        // first, locate the FST entry (within the target ISO) for the replacement
        // we search through the replacement_map we built up earlier
        let search = replacement_map.clone();
        let mut found = search
            .values()
            .filter(|update: &&UpdateFST| update.name == Character::filename(&replacement.target))
            .collect::<Vec<_>>();

        // we should find exactly one entry for each replacement
        // if not, abort
        let num_found = found.len();
        if num_found != 1 {
            panic!(
                "did not match character {:?}: {num_found} found",
                Character::filename(&replacement.target)
            );
        }
        let matching: &UpdateFST = found.pop().expect("failed to match character");

        // once we have the entry, we need the length of the new data
        // we load this using the path in the replacement definition
        let new_data: Vec<u8> = std::fs::read(&replacement.replacement).expect("could not open");
        let new_data_length = new_data.len();

        // now we can see whether the new data is larger or smaller
        let length_delta = dbg!(matching.original_size - new_data_length as u32);

        // bump updated_offset by length_delta for FST entries following the original offset
        if length_delta != 0 {
            for file in replacement_map.values_mut() {
                // if we find two matching offsets, this is the replacement target
                if file.original_offset == matching.original_offset {
                    // bump the size to match the new data
                    file.updated_size = new_data_length as u32;
                }

                // for everything following this offset,
                if file.original_offset >= matching.original_offset {
                    // bump the offset to reflect the updated data length
                    file.updated_offset += length_delta;
                }
            }
        }
    }

    // TODO: now that we have the updated FST definition, let's rebuild it
    use std::io::Cursor;

    // v1.02 NTSC GALE01 Root Directory Entry
    // ======================================
    // 0001 0203 0405 0607 0809 0a0b
    // ---- ---- ---- ---- ---- ----
    // 0100 0000 0000 0000 0000 04bc
    // ^ ^       ^         ^-------- num_entries (0x04bc) (1212 entries)
    // | |       \------------------ filename or parent_offset (0x00)
    // | \-------------------------- filename string table offset (0x00)
    // \---------------------------- flag (directory)
    //
    // there are 0x4bc entries, each 0x0c long
    // string table offset starts at (0x04bc * 0x0c) = 0x38d0

    // create cursor over filesystem table
    let mut cursor = Cursor::new(new_fst);

    // read the root node:
    // 0100 0000 0000 0000 0000 04bc
    let mut root = [0; 0xc];
    cursor.read(&mut root).expect("failed to read root node");

    let num_entries = root_node_num_entries(root);

    // skip root entry
    for entry_index in 1..num_entries {
        // seek to the correct offset
        let seek: u64 = (entry_index * 0x0c) as u64;
        cursor
            .seek(SeekFrom::Start(seek))
            .expect("failed to seek to fst entry: {seek}");

        // read node
        let mut node = [0; 0xc];
        cursor.read(&mut node).expect("failed to read node: {seek}");

        // skip directories
        if node_is_directory(node) {
            continue;
        }

        let file_offset = node_file_offset(node);

        match replacement_map.get(&file_offset) {
            Some(UpdateFST {
                name,
                updated_offset,
                ..
            }) => {
                if file_offset == (*updated_offset as u32) {
                    continue;
                }

                eprintln!(
                    "fst entry {seek:#0x}: {file_offset:#0x} -> {updated_offset:#0x} [{name}]"
                );

                // seek to file offset
                cursor
                    .seek(SeekFrom::Start(seek + 4))
                    .expect("failed to seek to file offset");
                cursor
                    .write(&updated_offset.to_be_bytes())
                    .expect("failed to write offset");
            }
            None => panic!("no replacement map entry found for node: {seek:#0x}"),
        };
    }

    let updated_fst = cursor.get_ref();

    // we only want the first (0x04bc * 0x0c) u8s
    updated_fst[0..(0x04bc * 0x0c)].to_vec()
}

fn node_file_offset(node: [u8; 0x0c]) -> u32 {
    // read bytes 0x04 -> 0x08 as u32 (filename offset)
    let (filename_offset_bytes, _) = &node[4..8].split_at(std::mem::size_of::<u32>());
    let bytes: [u8; 4] = (*filename_offset_bytes)
        .try_into()
        .expect("failed to parse root node num_entries");
    u32::from_be_bytes(bytes)
}

fn root_node_num_entries(node: [u8; 0x0c]) -> u32 {
    // read bytes 0x08 -> 0x0c as u32 (num_entries)
    let (num_entries_bytes, _) = &node[8..0xc].split_at(std::mem::size_of::<u32>());
    let bytes: [u8; 4] = (*num_entries_bytes)
        .try_into()
        .expect("failed to parse root node num_entries");
    u32::from_be_bytes(bytes)
}

fn node_is_directory(node: [u8; 0x0c]) -> bool {
    // read bytes 0x00 -> 0x01 as u8 (directory flag)
    let (file_or_directory, _) = &node[0..1].split_at(std::mem::size_of::<u8>());
    let bytes: [u8; 1] = (*file_or_directory)
        .try_into()
        .expect("failed to parse directory flag: {node}");
    let directory_flag = u8::from_be_bytes(bytes);

    directory_flag != 0
}

/// Attempt to build a new ISO given a set of replacements.
#[allow(dead_code)]
fn build_iso(_path: PathBuf, _dest: PathBuf, _replacements: &Vec<Replacement>) {
    todo!()
}

fn dat_files(iso: &GcmFile) -> impl Iterator<Item = &FsNode> {
    iso.filesystem.files.iter().filter(|e| match e {
        FsNode::File { name, .. } => ".dat".is_suffix_of(name),
        _ => false,
    })
}

fn show_fst(iso: &PathBuf) -> io::Result<()> {
    let iso = GcmFile::open(iso).expect("could not open ISO");
    io::stdout().write(&iso.fst_bytes[0..(0x04bc * 0x0c)])?;

    Ok(())
}

fn search_dat(iso_path: &PathBuf, args: &SearchDAT) -> io::Result<()> {
    let iso = GcmFile::open(iso_path).expect("could not open ISO");
    let files = dat_files(&iso)
        .filter(|file| match file {
            FsNode::File { name, .. } => args.dat_query == name.to_string(),
            _ => false,
        })
        .collect::<Vec<_>>();

    let length = files.len();

    // check that we matched one file
    if length != 1 {
        panic!("query didn't match one file: {length}");
    }

    match files.first() {
        Some(FsNode::File { size, offset, .. }) => {
            let mut file = std::fs::File::open(ISO_PATH)?;
            let mut contents = Vec::with_capacity(*size as usize);
            file.seek(SeekFrom::Start(*offset as u64))?;
            Read::by_ref(&mut file)
                // file.by_ref()
                .take(*size as u64)
                .read_to_end(&mut contents)?;

            io::stdout().write(&contents)?;

            Ok(())
        }
        _ => panic!("unknown error"),
    }
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    match args.subcmd {
        SubCommand::SearchDAT(query) => search_dat(&args.melee_iso, &query),
        SubCommand::ShowFST => show_fst(&args.melee_iso),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        characters::{CaptainFalconFile, Character},
        dat_files, rebuild_fst, Replacement, ISO_PATH,
    };
    use gc_gcm::{FsNode, GcmFile};
    use insta;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::path::PathBuf;
    use std::str::pattern::Pattern;

    #[test]
    fn load_iso() {
        GcmFile::open(ISO_PATH).expect("could not open ISO");
    }

    #[test]
    fn find_dat_files() {
        let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");
        let files = dat_files(&iso).collect::<Vec<_>>();
        insta::assert_debug_snapshot!(files);
    }

    // #[test]
    // fn try_replace_dat_same_size() {
    //     let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");

    //     let replacements = vec![
    //         // replace common files
    //         Replacement {
    //             target: Character::CaptainFalcon(CaptainFalconFile::PlCa),
    //             replacement: PathBuf::from("n64-falcon/PlCa.dat"),
    //         },
    //         // replace neutral skin
    //         Replacement {
    //             target: Character::CaptainFalcon(CaptainFalconFile::PlCaNr),
    //             replacement: PathBuf::from("n64-falcon/PlCaNr.dat"),
    //         },
    //     ];

    //     // rebuild FST using replacements
    //     let updates = rebuild_fst(&iso, &replacements);
    // }

    #[test]
    fn try_replace_dat_diff_size() {
        let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");

        let replacements = vec![
            // replace potemkin
            Replacement {
                target: Character::CaptainFalcon(CaptainFalconFile::PlCaGr),
                replacement: PathBuf::from("falcon/POTEMKIN FALCON.dat"),
            },
        ];

        // rebuild FST using replacements
        let updates = rebuild_fst(&iso, &replacements);

        std::fs::write("potemkin-fst.bin", updates).expect("failed to write file");

        // calculate hashes for each FST
        // let mut hasher1 = DefaultHasher::new();
        // let mut hasher2 = DefaultHasher::new();
        // iso.fst_bytes.hash(&mut hasher1);
        // new_fst.hash(&mut hasher2);
        // let original_hash = hasher1.finish();
        // let new_hash = hasher2.finish();

        // // fst should be modified, hashes shoule be different
        // assert_ne!(original_hash, new_hash);

        // // store hash, it should remain the same
        // insta::assert_debug_snapshot!("modified_fst_hash", new_hash)
    }
}
