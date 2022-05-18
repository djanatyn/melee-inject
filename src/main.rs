#![feature(pattern)]

use binrw::{io::Cursor, BinRead};
use clap::Parser;
use gc_gcm::{FsNode, GcmFile};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::str::pattern::Pattern;

const ISO_PATH: &str = "ssbm.iso";

pub mod characters {
    //! Supported character files for replacement.

    /// DAT files for Captain Falcon, used as replacement targets.
    #[allow(dead_code)]
    #[derive(Debug)]
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

    #[derive(Debug)]
    pub enum Character {
        CaptainFalcon(CaptainFalconFile),
    }

    impl Character {
        pub fn filename(character: &Self) -> &'static str {
            match character {
                Character::CaptainFalcon(CaptainFalconFile::PlCa) => "PlCa.dat",
                Character::CaptainFalcon(CaptainFalconFile::PlCaNr) => "PlCaNr.dat",
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
#[derive(Debug)]
pub struct Replacement {
    /// Which file to replace?
    pub target: Character,
    /// Path to replacement data.
    pub replacement: PathBuf,
}

#[allow(dead_code)]
fn find_character<'a>(target: &Character, iso: &'a GcmFile) -> Option<&'a FsNode> {
    let mut found = iso
        .filesystem
        .files
        .iter()
        .filter(|e| match e {
            FsNode::File { name, .. } => Character::filename(target).is_suffix_of(name),
            _ => false,
        })
        .collect::<Vec<_>>();

    if found.len() != 1 {
        panic!("did not match character: {found:#?}");
    }

    found.pop()
}

/// Given a set of potential replacements, attempt to rebuild the FST.
///
/// This function should:
/// - update offsets for files after the replacement, and
/// - apply padding between files (4 bytes)
#[allow(dead_code)]
fn rebuild_fst(iso: &GcmFile, replacements: &Vec<Replacement>) -> Vec<u8> {
    let new_fst = iso.fst_bytes.clone();

    for replacement in replacements {
        let (offset, size) = match find_character(&replacement.target, iso) {
            Some(FsNode::File { offset, size, .. }) => (offset, size),
            _ => panic!("failed to find character: {replacement:#?}"),
        };
        // information from YAGCD: https://www.gc-forever.com/yagcd/chap13.html
        //
        // 13.4 Format of the FST
        // ======================
        // +-----------+---------+----------+---------------------------------+
        // |   start   |   end   |   size   |   Description                   |
        // +-----------+---------+----------+---------------------------------+
        // |  0x00     |  0x0c   |  0x0c    | Root Directory Entry            |
        // +-----------+---------+----------+---------------------------------+
        // |  0x0c     |  ...    |  0x0c    | more File- or Directory Entries |
        // +-----------+---------+----------+---------------------------------+
        // |  ...      |  ...    |  ...     | String table                    |
        // +-----------+---------+----------+---------------------------------+
        //
        // 13.4.1 Format of a File Entry
        // =============================
        // +-----------+---------+----------+------------------------------+
        // |   start   |   end   |   size   |   Description                |
        // +-----------+---------+----------+------------------------------+
        // |   0x00    |         |   1      | flags; 0: file 1: directory  |
        // +-----------+---------+----------+------------------------------+
        // |   0x01    |         |   3      | filename, offset into string |
        // |           |         |          | table                        |
        // +-----------+---------+----------+------------------------------+
        // |   0x04    |         |   4      | file_offset or parent_offset |
        // |           |         |          | (dir)                        |
        // +-----------+---------+----------+------------------------------+
        // |   0x08    |         |   4      | file_length or num_entries   |
        // |           |         |          | (root) or next_offset (dir)  |
        // +-----------+---------+----------+------------------------------+
        //
        // v1.02 NTSC GALE01 Root Directory Entry:
        // 0100 0000 0000 0000 0000 04bc 01
        //
        // TODO: calculate offset adjustment (w/padding) for subsequent files
        // TODO: update offsets in new_fst
        // TODO: replace data in new_fst
    }

    new_fst
}

/// Attempt to build a new ISO given a set of replacements.
#[allow(dead_code)]
fn build_iso(_path: PathBuf, _dest: PathBuf, _replacements: &Vec<Replacement>) {
    todo!()
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
    /// Show FST bytes.
    ShowFST,
    /// Search (and display) a specific DAT file.
    SearchDAT(SearchDAT),
}

#[derive(Parser, Debug)]
pub struct SearchDAT {
    #[clap(short, long)]
    pub dat_query: String,
}

fn dat_files(iso: &GcmFile) -> impl Iterator<Item = &FsNode> {
    iso.filesystem.files.iter().filter(|e| match e {
        FsNode::File { name, .. } => ".dat".is_suffix_of(name),
        _ => false,
    })
}

fn show_fst(iso: &PathBuf) -> io::Result<()> {
    let iso = GcmFile::open(iso).expect("could not open ISO");
    io::stdout().write(&iso.fst_bytes)?;

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
        SubCommand::ShowFST => show_fst(&args.melee_iso),
        SubCommand::SearchDAT(query) => search_dat(&args.melee_iso, &query),
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

    #[test]
    fn try_replace_dat() {
        let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");

        let replacements = vec![
            // replace common files
            Replacement {
                target: Character::CaptainFalcon(CaptainFalconFile::PlCa),
                replacement: PathBuf::from("n64-falcon/PlCa.dat"),
            },
            // replace neutral skin
            Replacement {
                target: Character::CaptainFalcon(CaptainFalconFile::PlCaNr),
                replacement: PathBuf::from("n64-falcon/PlCaNr.dat"),
            },
        ];

        // rebuild FST using replacements
        let new_fst = rebuild_fst(&iso, &replacements);

        // calculate hashes for each FST
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        iso.fst_bytes.hash(&mut hasher1);
        new_fst.hash(&mut hasher2);
        let original_hash = hasher1.finish();
        let new_hash = hasher2.finish();

        // fst should be modified, hashes shoule be different
        assert_ne!(original_hash, new_hash);

        // store hash, it should remain the same
        insta::assert_debug_snapshot!("modified_fst_hash", new_hash)
    }
}
