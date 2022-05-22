#![feature(pattern)]

// TODO: output logs with tracing
// TODO: update README.md with goals, direction

use binrw::{io::Cursor, BinRead};
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

#[derive(Clone)]
/// An update to execute against the GCM FST.
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

#[allow(dead_code)]
#[derive(Debug, BinRead)]
#[br(big)]
struct DatHeader {
    file_size: i32,
    data_block_size: i32,
    relocation_table_count: i32,
    root_count1: i32,
    #[br(pad_after = 3)]
    root_count2: i32,
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

fn update_fst(updates: &Vec<UpdateFST>, fst: Vec<u8>) -> Vec<u8> {
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
/// v1.02 NTSC GALE01 Root Directory Entry
/// ======================================
/// 0001 0203 0405 0607 0809 0a0b
/// ---- ---- ---- ---- ---- ----
/// 0100 0000 0000 0000 0000 04bc
/// ^ ^       ^         ^-------- num_entries (0x04bc) (1212 entries)
/// | |       \------------------ parent_offset (0x00)
/// | \-------------------------- filename string table offset (0x00)
/// \---------------------------- flag (directory)
///
/// there are 0x4bc entries, each 0x0c long
/// string table offset starts at (0x04bc * 0x0c) = 0x38d0
///
#[allow(dead_code)]
fn rebuild_fst(iso: &GcmFile, replacements: &Vec<Replacement>) -> Vec<UpdateFST> {
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
        if length_delta > 0 {
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
                    dbg!(file);
                }
            }
        }
    }

    // TODO: now that we have the updated FST definition, let's rebuild it
    // walk through each 0x0c byte
    // - for each targte_original_offset key in replacement_map,
    // - if the target_original_offset matches

    todo!();
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

#[allow(dead_code)]
fn read_node(file: &FsNode) -> io::Result<DatHeader> {
    match file {
        FsNode::File { size, offset, .. } => {
            let mut file = std::fs::File::open(ISO_PATH)?;
            let mut contents = Vec::with_capacity(*size as usize);
            file.seek(SeekFrom::Start(*offset as u64))?;
            Read::by_ref(&mut file)
                // file.by_ref()
                .take(*size as u64)
                .read_to_end(&mut contents)?;

            // header is first 32 bytes
            // https://smashboards.com/threads/melee-dat-format.292603/
            Ok(DatHeader::read(&mut Cursor::new(&contents[..32])).expect("could not read header"))
        }
        _ => panic!("failure"),
    }
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    match args.subcmd {
        SubCommand::SearchDAT(query) => search_dat(&args.melee_iso, &query),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        characters::{CaptainFalconFile, Character},
        dat_files, read_node, rebuild_fst, Replacement, ISO_PATH,
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
    fn check_fst() {
        let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");
        let bytes = format!("{:#02x?}", iso.fst_bytes);
        insta::assert_snapshot!("fst", bytes)
    }

    #[test]
    fn find_dat_files() {
        let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");
        let files = dat_files(&iso).collect::<Vec<_>>();
        insta::assert_debug_snapshot!(files);
    }

    #[test]
    fn find_yoshi_header() {
        let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");
        let files = dat_files(&iso)
            .filter(|file| match file {
                FsNode::File { name, .. } => "TyYoshi.dat".is_suffix_of(name),
                _ => false,
            })
            .collect::<Vec<_>>();

        // there is only one yoshi
        assert_eq!(files.len(), 1);

        match files.first() {
            Some(yoshi @ FsNode::File { size, .. }) => {
                let header = read_node(yoshi).expect("could not read header");

                assert_eq!(header.file_size, *size as i32);
                insta::assert_debug_snapshot!(header)
            }
            _ => panic!("failed to find yoshi"),
        };
    }

    #[test]
    fn check_dat_headers() {
        let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");
        let headers = dat_files(&iso)
            .filter_map(|dat| {
                println!("checking header for {dat:#?}");

                // get file size of FsNode entry
                let (name, size) = match dat {
                    FsNode::File { name, size, .. } => (name, size),
                    _ => panic!("returned directory"),
                };

                // skip animation files
                if "AJ.dat".is_suffix_of(name) {
                    return None;
                }

                // parse header
                let header = read_node(dat).expect("could not read node");
                println!("header: {header:#?}");

                // compare FsNode size with DatHeader size
                assert_eq!(header.file_size, *size as i32);
                Some(header)
            })
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(headers)
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
                replacement: PathBuf::from("plcagr-falcon/PlCaGr POTEMKIN FALCON.dat"),
            },
        ];

        // rebuild FST using replacements
        let updates = rebuild_fst(&iso, &replacements);

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
