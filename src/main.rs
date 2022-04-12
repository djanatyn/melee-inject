#![feature(pattern)]

use gc_gcm::{FsNode, GcmFile};
use std::{io, str::pattern::Pattern};

const ISO_PATH: &str = "ssbm.iso";

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

#[cfg(test)]
mod tests {
    use super::{dat_files, ISO_PATH};
    use gc_gcm::{FsNode, GcmFile};
    use insta;
    use std::io::{Read, Seek, SeekFrom};
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
            Some(yoshi @ FsNode::File { size, offset, .. }) => {
                let mut file = std::fs::File::open(ISO_PATH).expect("could not open ISO");
                let mut contents = Vec::with_capacity(*size as usize);
                file.seek(SeekFrom::Start(*offset as u64))
                    .expect("failed to seek to yoshi");
                file.by_ref()
                    .take(*size as u64)
                    .read_to_end(&mut contents)
                    .expect("failed to read yoshi");

                // header is first 32 bytes
                // https://smashboards.com/threads/melee-dat-format.292603/
                let header = &contents[..32];

                assert_eq!(contents.len(), *size as usize);
                insta::assert_debug_snapshot!(header)
            }
            _ => panic!("failed to find yoshi"),
        };
    }
}
