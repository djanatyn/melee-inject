extern crate melee_inject;

use melee_inject::characters::CaptainFalcon;
use melee_inject::replace::{build_iso, rebuild_fst, Replacement};
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let replacements = vec![
        // replace potemkin
        Replacement {
            target_file: CaptainFalcon::PlCaGr,
            replacement: PathBuf::from("falcon/POTEMKIN FALCON.dat"),
        },
    ];

    let updates = rebuild_fst("ssbm.iso", &replacements);
    std::fs::write("potemkin-fst.bin", &updates.new_fst).expect("failed to write file");

    let rebuilt_iso = build_iso("ssbm.iso", &updates);
    std::fs::write("potemkin-melee.iso", rebuilt_iso).expect("failed to write file");

    Ok(())
}
