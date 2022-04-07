use gc_gcm::GcmFile;
use std::io;

const ISO_PATH: &str = "ssbm.iso";

fn all_files(iso: &'_ GcmFile) -> impl Iterator<Item = &'_ gc_gcm::FsNode> {
    iso.filesystem
        .files
        .iter()
        .filter(|e| matches!(e, gc_gcm::FsNode::File { .. }))
}

fn main() -> io::Result<()> {
    let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");
    let files = all_files(&iso).collect::<Vec<_>>();

    println!("{files:#?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{all_files, ISO_PATH};
    use gc_gcm::GcmFile;
    use insta;

    #[test]
    fn load_iso() {
        GcmFile::open(ISO_PATH).expect("could not open ISO");
    }

    #[test]
    fn load_files() {
        let iso = GcmFile::open(ISO_PATH).expect("could not open ISO");
        let files = all_files(&iso).collect::<Vec<_>>();
        insta::assert_debug_snapshot!(files);
    }
}
