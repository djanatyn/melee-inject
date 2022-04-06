use gc_gcm::GcmFile;
use std::io;

fn main() -> io::Result<()> {
    let iso = GcmFile::open("ssbm.iso").expect("could not open ISO");

    let files = iso
        .filesystem
        .files
        .iter()
        .filter(|e| matches!(e, gc_gcm::FsNode::File { .. }));

    println!("{files:#?}");

    Ok(())
}
