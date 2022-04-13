#![feature(pattern)]

use binrw::{io::Cursor, BinRead};
use gc_gcm::{FsNode, GcmFile};
use std::io::{self, Read, Seek, SeekFrom};
use std::str::pattern::Pattern;

const ISO_PATH: &str = "ssbm.iso";

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

fn dat_files(iso: &GcmFile) -> impl Iterator<Item = &FsNode> {
    iso.filesystem.files.iter().filter(|e| match e {
        FsNode::File { name, .. } => ".dat".is_suffix_of(name),
        _ => false,
    })
}

fn main() -> io::Result<()> {
    let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");
    let files = dat_files(&iso).collect::<Vec<_>>();

    println!("{files:#?}");

    Ok(())
}

#[allow(dead_code)]
fn read_node(file: &FsNode) -> io::Result<DatHeader> {
    match file {
        FsNode::File { size, offset, .. } => {
            let mut file = std::fs::File::open(ISO_PATH)?;
            let mut contents = Vec::with_capacity(*size as usize);
            file.seek(SeekFrom::Start(*offset as u64))?;
            file.by_ref()
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
            // skip animation files
            .filter(|dat| match dat {
                FsNode::File { name, .. } => !"AJ.dat".is_suffix_of(name),
                _ => panic!("returned directory"),
            })
            .map(|dat| {
                println!("checking header for {dat:#?}");

                // get file size of FsNode entry
                let size = match dat {
                    FsNode::File { name, size, .. } => size,
                    _ => panic!("returned directory"),
                };

                // parse header
                let header = read_node(dat).expect("could not read node");
                println!("header: {header:#?}");

                // compare FsNode size with DatHeader size
                assert_eq!(header.file_size, *size as i32);
                header
            })
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(headers)
    }
}
