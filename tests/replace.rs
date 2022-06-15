use gc_gcm::GcmFile;
use melee_inject::characters::CaptainFalcon;
use melee_inject::replace::{build_iso, rebuild_fst, Replacement};
use std::path::PathBuf;

const ISO_PATH: &str = "ssbm.iso";

#[test]
fn load_iso() {
    GcmFile::open(ISO_PATH).expect("could not open ISO");
}

#[test]
fn try_replace_dat_same_size() {
    let replacements = vec![
        // replace common files
        Replacement {
            target_file: CaptainFalcon::PlCa,
            replacement: PathBuf::from("n64-falcon/PlCa.dat"),
        },
        // replace neutral skin
        Replacement {
            target_file: CaptainFalcon::PlCaNr,
            replacement: PathBuf::from("n64-falcon/PlCaNr.dat"),
        },
    ];

    // rebuild FST using replacements
    let updates = rebuild_fst(ISO_PATH, &replacements);
    std::fs::write("n64-falcon-fst.bin", &updates.new_fst).expect("failed to write file");

    let rebuilt_iso = build_iso(ISO_PATH, &updates);
    std::fs::write("n64-falcon-melee.iso", rebuilt_iso).expect("failed to write file");
}

#[test]
fn try_replace_dat_diff_size() {
    let replacements = vec![
        // replace potemkin
        Replacement {
            target_file: CaptainFalcon::PlCaGr,
            replacement: PathBuf::from("falcon/POTEMKIN FALCON.dat"),
        },
    ];

    let updates = rebuild_fst(ISO_PATH, &replacements);
    std::fs::write("potemkin-fst.bin", &updates.new_fst).expect("failed to write file");

    let rebuilt_iso = build_iso(ISO_PATH, &updates);
    std::fs::write("potemkin-melee.iso", rebuilt_iso).expect("failed to write file");
}
