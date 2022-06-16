# melee-inject

## introduction

### Dispatches from RC: Parsing and Transforming Super Smash Bros. Melee
![slide-cover](/assets/slide-cover.png)

this is a presentation i gave at [recurse center](https://www.recurse.com/about)!
* [(YouTube) Dispatches from RC: Jonathan Strickland, "Parsing and Transforming Super Smash Bros. Melee"](https://www.youtube.com/watch?v=KejJrmT590g)
* [(slides)](https://docs.google.com/presentation/d/1sEnkbk3dOctiymV7YUATbzXb3zh2dj_D302XuHYNHi8/edit?usp=sharing)

## usage

``` rust
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
```

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
