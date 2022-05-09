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
    pub enum CaptainFalcon {
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

    pub enum Character {
        CaptainFalcon(CaptainFalcon),
    }
}

use characters::Character;

/// A queued replacement to be executed later.
///
/// This potential replacement is guaranteed to match a file in the FST.
#[allow(dead_code)]
pub struct Replacement {
    /// Which file to replace?
    pub target: Character,
    /// Path to replacement data.
    pub replacement: PathBuf,
}

/// Given a set of potential replacements, attempt to rebuild the FST.
///
/// This function should:
/// - update offsets for files after the replacement, and
/// - apply padding between files (4 bytes)
#[allow(dead_code)]
fn rebuild_fst(_fst: &Vec<u8>, _replacements: &Vec<Replacement>) -> Vec<u8> {
    todo!()
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
    use super::{dat_files, read_node, ISO_PATH};
    use gc_gcm::{FsNode, GcmFile};
    use insta;
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
}
