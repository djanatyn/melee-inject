# melee-inject

replace DAT files within v1.02 NTSC GALE01 using rust

## introduction

### Dispatches from RC: Parsing and Transforming Super Smash Bros. Melee
![slide-cover](/assets/slide-cover.png)

this is a presentation i gave at [The Recurse Center](https://www.recurse.com/about)!
* [(YouTube) Dispatches from RC: Jonathan Strickland, "Parsing and Transforming Super Smash Bros. Melee"](https://www.youtube.com/watch?v=KejJrmT590g)
* [(slides)](https://docs.google.com/presentation/d/1sEnkbk3dOctiymV7YUATbzXb3zh2dj_D302XuHYNHi8/edit?usp=sharing)

## usage

this example is using `dang3r` potemkin inspired animelee falcon:
* https://ssbmtextures.com/characters/potemkin-inspired-animelee-falcon/

`Cargo.toml`:

``` toml
[dependencies]
melee_inject = { git = "https://github.com/djanatyn/melee-inject" }
```

`src/main.rs`:

``` rust
use melee_inject::characters::CaptainFalcon;
use melee_inject::replace::{build_iso, rebuild_fst, Replacement};
use std::io;
use std::path::PathBuf;

const SSBM_ISO: &str = "<path-to-ssbm.iso>";

fn main() -> io::Result<()> {
    let replacements = vec![
        // replace potemkin
        Replacement {
            target_file: CaptainFalcon::PlCaGr,
            replacement: PathBuf::from("<path-to-skin.dat>"),
        },
    ];

    let updates = rebuild_fst(SSBM_ISO, &replacements);
    std::fs::write("modified-fst.bin", &updates.new_fst).expect("failed to write file");

    let rebuilt_iso = build_iso(SSBM_ISO, &updates);
    std::fs::write("modified-melee.iso", rebuilt_iso).expect("failed to write file");

    Ok(())
}
```

`cargo run`:

```
[src/main.rs:226] matching.original_size - new_data_length as u32 = 123584
...
UpdateFST [offset 0x4f5b0000 -> 0x4f5b0000] [size 0x805cb -> 0x805cb] PlCaBu.dat
UpdateFST [offset 0x4f638000 -> 0x4f638000] [size 0x805ab -> 0x622eb] PlCaGr.dat
UpdateFST [offset 0x4f6c0000 -> 0x4f6a1d40] [size 0x8058b -> 0x8058b] PlCaGy.dat
...
```

![replacement plan](/assets/potemkin-replacement.png)

```
❯ radiff2 ssbm-segment.fst potemkin-segment.fst
0x00000021 0805ab => 0622eb 0x00000021
0x00000029 6c0000 => 6a1d40 0x00000029
```

![successful run](/assets/success.png)
