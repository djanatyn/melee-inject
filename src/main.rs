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
    use gc_gcm::GcmFile;
    use insta;

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
}
