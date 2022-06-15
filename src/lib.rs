// TODO: output logs with tracing

pub mod characters {
    #![allow(non_upper_case_globals)]
    //! Supported character files for replacement.

    /// DAT files for Captain Falcon, used as replacement targets.
    #[non_exhaustive]
    #[derive(Debug, Clone)]
    pub struct CaptainFalcon;
    impl CaptainFalcon {
        /// NTSC data & shared textures.
        pub const PlCa: &'static str = "PlCa.dat";
        /// Blue costume.
        pub const PlCaBu: &'static str = "PlCaBu.dat";
        /// Green costume.
        pub const PlCaGr: &'static str = "PlCaGr.dat";
        /// Gray costume.
        pub const PlCaGy: &'static str = "PlCaGu.dat";
        /// Neutral costume.
        pub const PlCaNr: &'static str = "PlCaNr.dat";
        /// Red costume.
        pub const PlCaRe: &'static str = "PlCaRe.dat";
        /// White costume.
        pub const PlCaWh: &'static str = "PlCaWh.dat";
    }
}

mod parse {
    //! Parsing functions for filesystem table entries.

    use std::io::{self, Read, Seek, SeekFrom, Write};
    use std::path::Path;

    pub fn node_file_offset(node: [u8; 0x0c]) -> u32 {
        // read bytes 0x04 -> 0x08 as u32 (filename offset)
        let (filename_offset_bytes, _) = &node[4..8].split_at(std::mem::size_of::<u32>());
        let bytes: [u8; 4] = (*filename_offset_bytes)
            .try_into()
            .expect("failed to parse root node num_entries");
        u32::from_be_bytes(bytes)
    }

    pub fn root_node_num_entries(node: [u8; 0x0c]) -> u32 {
        // read bytes 0x08 -> 0x0c as u32 (num_entries)
        let (num_entries_bytes, _) = &node[8..0xc].split_at(std::mem::size_of::<u32>());
        let bytes: [u8; 4] = (*num_entries_bytes)
            .try_into()
            .expect("failed to parse root node num_entries");
        u32::from_be_bytes(bytes)
    }

    pub fn node_is_directory(node: [u8; 0x0c]) -> bool {
        // read bytes 0x00 -> 0x01 as u8 (directory flag)
        let (file_or_directory, _) = &node[0..1].split_at(std::mem::size_of::<u8>());
        let bytes: [u8; 1] = (*file_or_directory)
            .try_into()
            .expect("failed to parse directory flag: {node}");
        let directory_flag = u8::from_be_bytes(bytes);

        directory_flag != 0
    }

    #[allow(unused)]
    pub fn show_fst<P: AsRef<Path>>(iso: P) -> io::Result<()> {
        let mut file = std::fs::File::open(&iso).expect("failed to open ISO");
        let mut fst = Vec::new();
        file.seek(SeekFrom::Start(0x456e00))
            .expect("failed to seek to fst");
        Read::by_ref(&mut file)
            .take(0x07529)
            .read_to_end(&mut fst)
            .expect("failed to read fst");

        io::stdout().write(&fst)?;

        Ok(())
    }
}

pub mod replace {
    use super::parse;
    use gc_gcm::{FsNode, GcmFile};
    use std::collections::HashMap;
    use std::fmt;
    use std::io::Cursor;
    use std::io::{self, Read, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};

    /// A queued replacement to be executed later.
    #[derive(Debug, Clone)]
    pub struct Replacement {
        /// Which file to replace?
        pub target_file: &'static str,
        /// Path to replacement data.
        pub replacement: PathBuf,
    }

    /// An update to execute against the GCM FST.
    #[derive(Clone)]
    pub struct UpdateFST {
        pub name: String,
        pub original_offset: u32,
        pub updated_offset: u32,
        pub original_size: u32,
        pub updated_size: u32,
        #[allow(dead_code)]
        pub data: Vec<u8>,
    }

    impl fmt::Debug for UpdateFST {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "UpdateFST [offset {:#0x} -> {:#0x}] [size {:#0x} -> {:#0x}] {}",
                self.original_offset,
                self.updated_offset,
                self.original_size,
                self.updated_size,
                self.name
            )
        }
    }

    /// Struct containing a rebuilt FST, with offsets adjusted.
    ///
    /// Can be used to create a bootable ISO.
    pub struct RebuiltFST {
        pub new_fst: Vec<u8>,
        pub replacements: HashMap<u32, UpdateFST>,
    }

    pub fn read_file<P: AsRef<Path>>(iso: P, file: &FsNode) -> io::Result<UpdateFST> {
        match file {
            FsNode::File { size, offset, name } => {
                let mut file = std::fs::File::open(iso)?;
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
    pub fn rebuild_fst<P: AsRef<Path>>(path: P, replacements: &Vec<Replacement>) -> RebuiltFST {
        let iso = GcmFile::open(&path).expect("could not open ISO");

        // read entire filesystem table (0x456e00 offset, 0x7529 length)
        // GcmFile#fst_bytes returns a truncated version
        let mut file = std::fs::File::open(&path).expect("failed to open ISO");
        let mut fst = Vec::new();
        file.seek(SeekFrom::Start(0x456e00))
            .expect("failed to seek to fst");
        Read::by_ref(&mut file)
            .take(0x07529)
            .read_to_end(&mut fst)
            .expect("failed to read fst");

        let new_fst = fst.clone();

        let mut replacement_map: HashMap<u32, UpdateFST> = HashMap::new();
        for file in &iso.filesystem.files {
            match file {
                // for each file, insert a mutable UpdateFST, indexed by offset
                file @ FsNode::File { offset, .. } => replacement_map.insert(
                    *offset,
                    read_file(&path, &file).expect("failed to read file"),
                ),
                _ => continue,
            };
        }

        for replacement in replacements {
            eprintln!("{replacement:?}");
            // first, locate the FST entry (within the target ISO) for the replacement
            // we search through the replacement_map we built up earlier
            let search = replacement_map.clone();
            let mut found = search
                .values()
                .filter(|update: &&UpdateFST| update.name == replacement.target_file)
                .collect::<Vec<_>>();

            // we should find exactly one entry for each replacement
            // if not, abort
            let num_found = found.len();
            if num_found != 1 {
                panic!(
                    "did not match character {:?}: {num_found} found",
                    replacement.target_file
                );
            }
            let matching: &UpdateFST = found.pop().expect("failed to match character");

            // once we have the entry, we need the length of the new data
            // we load this using the path in the replacement definition
            let new_data: Vec<u8> =
                std::fs::read(&replacement.replacement).expect("could not open");
            let new_data_length = new_data.len();

            // now we can see whether the new data is larger or smaller
            let length_delta: i32 =
                ((matching.original_size as i32) - (new_data_length as i32)).wrapping_abs();
            let offset_adjustment = dbg!(length_delta + length_delta.rem_euclid(4));

            // bump updated_offset by length_delta for FST entries following the original offset
            for file in replacement_map.values_mut() {
                // if we find two matching offsets, this is the replacement target
                if file.original_offset == matching.original_offset {
                    // bump the size to match the new data
                    file.updated_size = new_data_length as u32;
                    // update the data with the replacement
                    file.data = new_data.clone();
                }

                // for everything following this offset,
                if offset_adjustment != 0 && file.original_offset > matching.original_offset {
                    // bump the offset to reflect the updated data length
                    file.updated_offset -= offset_adjustment as u32;
                }
            }
        }

        // v1.02 NTSC GALE01 Root Directory Entry
        // ======================================
        // 0001 0203 0405 0607 0809 0a0b
        // ---- ---- ---- ---- ---- ----
        // 0100 0000 0000 0000 0000 04bc
        // ^ ^       ^         ^-------- num_entries (0x04bc) (1212 entries)
        // | |       \------------------ filename or parent_offset (0x00)
        // | \-------------------------- filename string table offset (0x00)
        // \---------------------------- flag (directory)
        //
        // there are 0x4bc entries, each 0x0c long
        // string table offset starts at (0x04bc * 0x0c) = 0x38d0

        // create cursor over filesystem table
        let mut cursor = Cursor::new(new_fst);

        // read the root node:
        // 0100 0000 0000 0000 0000 04bc
        let mut root = [0; 0xc];
        cursor.read(&mut root).expect("failed to read root node");

        let num_entries = parse::root_node_num_entries(root);

        // skip root entry
        for entry_index in 1..num_entries {
            // seek to the correct offset
            let seek: u64 = (entry_index * 0x0c) as u64;
            cursor
                .seek(SeekFrom::Start(seek))
                .expect("failed to seek to fst entry: {seek}");

            // read node
            let mut node = [0; 0xc];
            cursor.read(&mut node).expect("failed to read node: {seek}");

            // skip directories
            if parse::node_is_directory(node) {
                continue;
            }

            let file_offset = parse::node_file_offset(node);

            match replacement_map.get(&file_offset) {
                Some(UpdateFST {
                    name,
                    updated_offset,
                    updated_size,
                    ..
                }) => {
                    let fst_index = seek + 4;
                    eprintln!(
                        "fst entry {fst_index:#0x}: {file_offset:#0x} -> {updated_offset:#0x} [{name}]"
                    );

                    // seek to file offset
                    cursor
                        .seek(SeekFrom::Start(seek + 4))
                        .expect("failed to seek to file offset");
                    cursor
                        .write(&updated_offset.to_be_bytes())
                        .expect("failed to write offset");

                    // seek to size offset
                    cursor
                        .seek(SeekFrom::Start(seek + 8))
                        .expect("failed to seek to file offset");
                    cursor
                        .write(&updated_size.to_be_bytes())
                        .expect("failed to write size");
                }
                None => panic!("no replacement map entry found for node: {seek:#0x}"),
            };
        }

        RebuiltFST {
            new_fst: cursor.get_ref().to_vec(),
            replacements: replacement_map,
        }
    }

    pub fn build_iso<P: AsRef<Path>>(path: P, fst: &RebuiltFST) -> Vec<u8> {
        let mut new_iso = Vec::with_capacity(0x456e00);

        let mut melee = std::fs::File::open(&path).expect("failed to open ISO");
        Read::by_ref(&mut melee)
            .take(0x456e00)
            .read_to_end(&mut new_iso)
            .expect("failed to read melee up to FST");

        new_iso.extend(&fst.new_fst);

        let mut cursor = Cursor::new(new_iso);

        let mut updates = fst.replacements.values().collect::<Vec<_>>();
        updates.sort_by(|a, b| a.updated_offset.cmp(&b.updated_offset));

        for update in updates {
            println!("{update:?}");
            cursor
                .seek(SeekFrom::Start(update.updated_offset as u64))
                .expect("failed to seek");
            cursor
                .write(&update.data)
                .expect("failed to write {update:?}");
        }

        let end_position = cursor
            .seek(SeekFrom::End(0))
            .expect("failed to seek to end");

        let padding = std::iter::repeat(0).take(end_position.rem_euclid(0x20) as usize + 0x20);

        cursor
            .write(&padding.collect::<Vec<_>>())
            .expect("failed to write extra padding");
        cursor.get_mut().to_vec()
    }
}
