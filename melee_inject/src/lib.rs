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

    /// Supported files for Young Link.
    #[derive(Debug, Clone)]
    pub struct YoungLink;
    impl YoungLink {
        /// Shared textures.
        pub const PlCl: &'static str = "PlCl.dat";
        /// Black costume.
        pub const PlClBk: &'static str = "PlClBk.dat";
        /// Blue costume.
        pub const PlClBu: &'static str = "PlClBu.dat";
        /// Neutral costume.
        pub const PlClNr: &'static str = "PlClNr.dat";
        /// Red costume.
        pub const PlClRe: &'static str = "PlClRe.dat";
        /// White costume.
        pub const PlClWh: &'static str = "PlClWh.dat";
    }

    /// Supported files for Male Wireframe.
    #[derive(Debug, Clone)]
    pub struct MaleWireframe;
    impl MaleWireframe {
        /// Shared textures.
        pub const PlBo: &'static str = "PlBo.dat";
        /// Neutral costume.
        pub const PlBoNr: &'static str = "PlBoNr.dat";
    }

    /// Supported files for Falco.
    #[derive(Debug, Clone)]
    pub struct Falco;
    impl Falco {
        /// Shared textures.
        pub const PlFc: &'static str = "PlFc.dat";
        /// Blue costume.
        pub const PlFcBu: &'static str = "PlFcBu.dat";
        /// Green costume.
        pub const PlFcGr: &'static str = "PlFcGr.dat";
        /// Neutral costume.
        pub const PlFcNr: &'static str = "PlFcNr.dat";
        /// Red costume.
        pub const PlFcRe: &'static str = "PlFcRe.dat";
    }

    /// Supported files for Master Hand.
    #[derive(Debug, Clone)]
    pub struct MasterHand;
    impl MasterHand {
        /// Shared textures.
        pub const PlMh: &'static str = "PlMh.dat";
        /// Neutral costume.
        pub const PlMhNr: &'static str = "PlMhNr.dat";
    }

    /// Supported files for Peach.
    #[derive(Debug, Clone)]
    pub struct Peach;
    impl Peach {
        /// Shared textures.
        pub const PlPe: &'static str = "PlPe.dat";
        /// Blue costume.
        pub const PlPeBu: &'static str = "PlPeBu.dat";
        /// Green costume.
        pub const PlPeGr: &'static str = "PlPeGr.dat";
        /// Neutral costume.
        pub const PlPeNr: &'static str = "PlPeNr.dat";
        /// White costume.
        pub const PlPeWh: &'static str = "PlPeWh.dat";
        /// Yellow costume.
        pub const PlPeYe: &'static str = "PlPeYe.dat";
    }

    /// Supported files for Game 'n Watch.
    #[derive(Debug, Clone)]
    pub struct GameNWatch;
    impl GameNWatch {
        /// Shared textures.
        pub const PlGw: &'static str = "PlGw.dat";
        /// Neutral costume.
        pub const PlGwNr: &'static str = "PlGwNr.dat";
    }

    /// Supported files for Luigi.
    #[derive(Debug, Clone)]
    pub struct Luigi;

    impl Luigi {
        /// Shared textures.
        pub const PlLg: &'static str = "PlLg.dat";
        /// Aqua costume.
        pub const PlLgAq: &'static str = "PlLgAq.dat";
        /// Neutral costume.
        pub const PlLgNr: &'static str = "PlLgNr.dat";
        /// Pink costume.
        pub const PlLgPi: &'static str = "PlLgPi.dat";
        /// White costume.
        pub const PlLgWh: &'static str = "PlLgWh.dat";
    }

    /// Supported files for Pikachu.
    #[derive(Debug, Clone)]
    pub struct Pikachu;

    impl Pikachu {
        /// Shared textures.
        pub const PlPk: &'static str = "PlPk.dat";
        /// Blue costume.
        pub const PlPkBu: &'static str = "PlPkBu.dat";
        /// Green costume.
        pub const PlPkGr: &'static str = "PlPkGr.dat";
        /// Neutral costume.
        pub const PlPkNr: &'static str = "PlPkNr.dat";
        /// Red costume.
        pub const PlPkRe: &'static str = "PlPkRe.dat";
    }

    /// Supported files for Fox.
    #[derive(Debug, Clone)]
    pub struct Fox;
    impl Fox {
        /// Shared textures.
        pub const PlFx: &'static str = "PlFx.dat";
        /// Green costume.
        pub const PlFxGr: &'static str = "PlFxGr.dat";
        /// Lavender costume.
        pub const PlFxLa: &'static str = "PlFxLa.dat";
        /// Neutral costume.
        pub const PlFxNr: &'static str = "PlFxNr.dat";
        /// Orange costume.
        pub const PlFxOr: &'static str = "PlFxOr.dat";
    }

    /// Supported files for Jigglypuff.
    #[derive(Debug, Clone)]
    pub struct Jigglypuff;

    impl Jigglypuff {
        /// Shared textures.
        pub const PlPr: &'static str = "PlPr.dat";
        /// Blue costume.
        pub const PlPrBu: &'static str = "PlPrBu.dat";
        /// Green costume.
        pub const PlPrGr: &'static str = "PlPrGr.dat";
        /// Neutral costume.
        pub const PlPrNr: &'static str = "PlPrNr.dat";
        /// Red costume.
        pub const PlPrRe: &'static str = "PlPrRe.dat";
        /// Yellow costume.
        pub const PlPrYe: &'static str = "PlPrYe.dat";
    }

    /// Supported files for SandBag.
    #[derive(Debug, Clone)]
    pub struct SandBag;
    impl SandBag {
        /// Shared textures.
        pub const PlSb: &'static str = "PlSb.dat";
        /// Neutral costume.
        pub const PlSbNr: &'static str = "PlSbNr.dat";
    }

    /// Supported files for Marth.
    #[derive(Debug, Clone)]
    pub struct Marth;
    impl Marth {
        /// Shared textures.
        pub const PlMs: &'static str = "PlMs.dat";
        /// Black costume.
        pub const PlMsBk: &'static str = "PlMsBk.dat";
        /// Green costume.
        pub const PlMsGr: &'static str = "PlMsGr.dat";
        /// Neutral costume.
        pub const PlMsNr: &'static str = "PlMsNr.dat";
        /// Red costume.
        pub const PlMsRe: &'static str = "PlMsRe.dat";
        /// White costume.
        pub const PlMsWh: &'static str = "PlMsWh.dat";
    }

    /// Supported files for Samus.
    #[derive(Debug, Clone)]
    pub struct Samus;
    impl Samus {
        /// Shared textures.
        pub const PlSs: &'static str = "PlSs.dat";
        /// Black costume.
        pub const PlSsBk: &'static str = "PlSsBk.dat";
        /// Green costume.
        pub const PlSsGr: &'static str = "PlSsGr.dat";
        /// Lavender costume.
        pub const PlSsLa: &'static str = "PlSsLa.dat";
        /// Neutral costume.
        pub const PlSsNr: &'static str = "PlSsNr.dat";
        /// Pink costume.
        pub const PlSsPi: &'static str = "PlSsPi.dat";
    }

    /// Supported files for GigaBowser.
    #[derive(Debug, Clone)]
    pub struct GigaBowser;
    impl GigaBowser {
        /// Shared textures.
        pub const PlGk: &'static str = "PlGk.dat";
        /// Neutral costume.
        pub const PlGkNr: &'static str = "PlGkNr.dat";
    }

    /// Supported files for Ness.
    #[derive(Debug, Clone)]
    pub struct Ness;
    impl Ness {
        /// Shared textures.
        pub const PlNs: &'static str = "PlNs.dat";
        /// Blue costume.
        pub const PlNsBu: &'static str = "PlNsBu.dat";
        /// Green costume.
        pub const PlNsGr: &'static str = "PlNsGr.dat";
        /// Neutral costume.
        pub const PlNsNr: &'static str = "PlNsNr.dat";
        /// Yellow costume.
        pub const PlNsYe: &'static str = "PlNsYe.dat";
    }

    /// Supported files for Zelda.
    #[derive(Debug, Clone)]
    pub struct Zelda;
    impl Zelda {
        /// Shared textures.
        pub const PlZd: &'static str = "PlZd.dat";
        /// Blue costume.
        pub const PlZdBu: &'static str = "PlZdBu.dat";
        /// Green costume.
        pub const PlZdGr: &'static str = "PlZdGr.dat";
        /// Neutral costume.
        pub const PlZdNr: &'static str = "PlZdNr.dat";
        /// Red costume.
        pub const PlZdRe: &'static str = "PlZdRe.dat";
        /// White costume.
        pub const PlZdWh: &'static str = "PlZdWh.dat";
    }

    /// Supported files for [Nana] Ice Climbers.
    #[derive(Debug, Clone)]
    pub struct IceClimbersNana;
    impl IceClimbersNana {
        /// Shared textures.
        pub const PlNn: &'static str = "PlNn.dat";
        /// Aqua costume.
        pub const PlNnAq: &'static str = "PlNnAq.dat";
        /// Neutral costume.
        pub const PlNnNr: &'static str = "PlNnNr.dat";
        /// White costume.
        pub const PlNnWh: &'static str = "PlNnWh.dat";
        /// Yellow costume.
        pub const PlNnYe: &'static str = "PlNnYe.dat";
    }

    /// Supported files for Pichu.
    #[derive(Debug, Clone)]
    pub struct Pichu;
    impl Pichu {
        /// Shared textures.
        pub const PlPc: &'static str = "PlPc.dat";
        /// Blue costume.
        pub const PlPcBu: &'static str = "PlPcBu.dat";
        /// Green costume.
        pub const PlPcGr: &'static str = "PlPcGr.dat";
        /// Neutral costume.
        pub const PlPcNr: &'static str = "PlPcNr.dat";
        /// Red costume.
        pub const PlPcRe: &'static str = "PlPcRe.dat";
    }

    /// Supported files for Crazy Hand.
    #[derive(Debug, Clone)]
    pub struct CrazyHand;
    impl CrazyHand {
        /// Shared textures.
        pub const PlCh: &'static str = "PlCh.dat";
        /// Neutral costume.
        pub const PlChNr: &'static str = "PlChNr.dat";
    }

    /// Supported files for Sheik.
    #[derive(Debug, Clone)]
    pub struct Sheik;
    impl Sheik {
        /// Shared textures.
        pub const PlSk: &'static str = "PlSk.dat";
        /// Blue costume.
        pub const PlSkBu: &'static str = "PlSkBu.dat";
        /// Green costume.
        pub const PlSkGr: &'static str = "PlSkGr.dat";
        /// Neutral costume.
        pub const PlSkNr: &'static str = "PlSkNr.dat";
        /// Red costume.
        pub const PlSkRe: &'static str = "PlSkRe.dat";
        /// White costume.
        pub const PlSkWh: &'static str = "PlSkWh.dat";
    }

    /// Supported files for Female Wireframe.
    #[derive(Debug, Clone)]
    pub struct FemaleWireframe;
    impl FemaleWireframe {
        /// Shared textures.
        pub const PlGl: &'static str = "PlGl.dat";
        /// Neutral costume.
        pub const PlGlNr: &'static str = "PlGlNr.dat";
    }

    /// Supported files for Yoshi.
    #[derive(Debug, Clone)]
    pub struct Yoshi;
    impl Yoshi {
        /// Shared textures.
        pub const PlYs: &'static str = "PlYs.dat";
        /// Aqua costume.
        pub const PlYsAq: &'static str = "PlYsAq.dat";
        /// Blue costume.
        pub const PlYsBu: &'static str = "PlYsBu.dat";
        /// Neutral costume.
        pub const PlYsNr: &'static str = "PlYsNr.dat";
        /// Pink costume.
        pub const PlYsPi: &'static str = "PlYsPi.dat";
        /// Red costume.
        pub const PlYsRe: &'static str = "PlYsRe.dat";
        /// Yellow costume.
        pub const PlYsYe: &'static str = "PlYsYe.dat";
    }

    /// Supported files for Donkey Kong.
    #[derive(Debug, Clone)]
    pub struct DonkeyKong;
    impl DonkeyKong {
        /// Shared textures.
        pub const PlDk: &'static str = "PlDk.dat";
        /// Black costume.
        pub const PlDkBk: &'static str = "PlDkBk.dat";
        /// Blue costume.
        pub const PlDkBu: &'static str = "PlDkBu.dat";
        /// Green costume.
        pub const PlDkGr: &'static str = "PlDkGr.dat";
        /// Neutral costume.
        pub const PlDkNr: &'static str = "PlDkNr.dat";
        /// Red costume.
        pub const PlDkRe: &'static str = "PlDkRe.dat";
    }

    /// Supported files for Mario.
    #[derive(Debug, Clone)]
    pub struct Mario;
    impl Mario {
        /// Shared textures.
        pub const PlMr: &'static str = "PlMr.dat";
        /// Black costume.
        pub const PlMrBk: &'static str = "PlMrBk.dat";
        /// Blue costume.
        pub const PlMrBu: &'static str = "PlMrBu.dat";
        /// Green costume.
        pub const PlMrGr: &'static str = "PlMrGr.dat";
        /// Neutral costume.
        pub const PlMrNr: &'static str = "PlMrNr.dat";
        /// Yellow costume.
        pub const PlMrYe: &'static str = "PlMrYe.dat";
    }

    /// Supported files for Ganondorf.
    #[derive(Debug, Clone)]
    pub struct Ganondorf;
    impl Ganondorf {
        /// Shared textures.
        pub const PlGn: &'static str = "PlGn.dat";
        /// Blue costume.
        pub const PlGnBu: &'static str = "PlGnBu.dat";
        /// Green costume.
        pub const PlGnGr: &'static str = "PlGnGr.dat";
        /// Lavender costume.
        pub const PlGnLa: &'static str = "PlGnLa.dat";
        /// Neutral costume.
        pub const PlGnNr: &'static str = "PlGnNr.dat";
        /// Red costume.
        pub const PlGnRe: &'static str = "PlGnRe.dat";
    }

    /// Supported files for Dr. Mario.
    #[derive(Debug, Clone)]
    pub struct DrMario;
    impl DrMario {
        /// Shared textures.
        pub const PlDr: &'static str = "PlDr.dat";
        /// Black costume.
        pub const PlDrBk: &'static str = "PlDrBk.dat";
        /// Blue costume.
        pub const PlDrBu: &'static str = "PlDrBu.dat";
        /// Green costume.
        pub const PlDrGr: &'static str = "PlDrGr.dat";
        /// Neutral costume.
        pub const PlDrNr: &'static str = "PlDrNr.dat";
        /// Red costume.
        pub const PlDrRe: &'static str = "PlDrRe.dat";
    }

    /// Supported files for Kirby.
    #[derive(Debug, Clone)]
    pub struct Kirby;
    impl Kirby {
        /// Shared textures.
        pub const PlKb: &'static str = "PlKb.dat";
        /// Blue costume.
        pub const PlKbBu: &'static str = "PlKbBu.dat";
        /// Blue costume, Copy Power (Donkey Kong).
        pub const PlKbBuCpDk: &'static str = "PlKbBuCpDk.dat";
        /// Blue costume, Copy Power (Falco).
        pub const PlKbBuCpFc: &'static str = "PlKbBuCpFc.dat";
        /// Blue costume, Copy Power (Mewtwo).
        pub const PlKbBuCpMt: &'static str = "PlKbBuCpMt.dat";
        /// Blue costume, Copy Power (Jigglypuff).
        pub const PlKbBuCpPr: &'static str = "PlKbBuCpPr.dat";
        /// Copy Power (Captain Falcon).
        pub const PlKbCpCa: &'static str = "PlKbCpCa.dat";
        /// Copy Power (Young Link).
        pub const PlKbCpCl: &'static str = "PlKbCpCl.dat";
        /// Copy Power (Donkey Kong).
        pub const PlKbCpDk: &'static str = "PlKbCpDk.dat";
        /// Copy Power (Dr. Mario).
        pub const PlKbCpDr: &'static str = "PlKbCpDr.dat";
        /// Copy Power (Falco).
        pub const PlKbCpFc: &'static str = "PlKbCpFc.dat";
        /// Copy Power (Roy).
        pub const PlKbCpFe: &'static str = "PlKbCpFe.dat";
        /// Copy Power (Fox).
        pub const PlKbCpFx: &'static str = "PlKbCpFx.dat";
        /// Copy Power (Ganondorf).
        pub const PlKbCpGn: &'static str = "PlKbCpGn.dat";
        /// Copy Power (Game 'n Watch).
        pub const PlKbCpGw: &'static str = "PlKbCpGw.dat";
        /// Copy Power (Bowser).
        pub const PlKbCpKp: &'static str = "PlKbCpKp.dat";
        /// Copy Power (Luigi).
        pub const PlKbCpLg: &'static str = "PlKbCpLg.dat";
        /// Copy Power (Link).
        pub const PlKbCpLk: &'static str = "PlKbCpLk.dat";
        /// Copy Power (Mario).
        pub const PlKbCpMr: &'static str = "PlKbCpMr.dat";
        /// Copy Power (Marth).
        pub const PlKbCpMs: &'static str = "PlKbCpMs.dat";
        /// Copy Power (Mewtwo).
        pub const PlKbCpMt: &'static str = "PlKbCpMt.dat";
        /// Copy Power (Ness).
        pub const PlKbCpNs: &'static str = "PlKbCpNs.dat";
        /// Copy Power (Pichu).
        pub const PlKbCpPc: &'static str = "PlKbCpPc.dat";
        /// Copy Power (Peach).
        pub const PlKbCpPe: &'static str = "PlKbCpPe.dat";
        /// Copy Power (Pikachu).
        pub const PlKbCpPk: &'static str = "PlKbCpPk.dat";
        /// Copy Power ([Popo] Ice Climbers).
        pub const PlKbCpPp: &'static str = "PlKbCpPp.dat";
        /// Copy Power (Jigglypuff).
        pub const PlKbCpPr: &'static str = "PlKbCpPr.dat";
        /// Copy Power (Sheik).
        pub const PlKbCpSk: &'static str = "PlKbCpSk.dat";
        /// Copy Power (Samus).
        pub const PlKbCpSs: &'static str = "PlKbCpSs.dat";
        /// Copy Power (Yoshi).
        pub const PlKbCpYs: &'static str = "PlKbCpYs.dat";
        /// Copy Power (Zelda).
        pub const PlKbCpZd: &'static str = "PlKbCpZd.dat";
        /// Green costume.
        pub const PlKbGr: &'static str = "PlKbGr.dat";
        /// Green costume, Copy Power (Donkey Kong).
        pub const PlKbGrCpDk: &'static str = "PlKbGrCpDk.dat";
        /// Green costume, Copy Power (Falco).
        pub const PlKbGrCpFc: &'static str = "PlKbGrCpFc.dat";
        /// Green costume, Copy Power (Mewtwo).
        pub const PlKbGrCpMt: &'static str = "PlKbGrCpMt.dat";
        /// Green costume, Copy Power (Jigglypuff).
        pub const PlKbGrCpPr: &'static str = "PlKbGrCpPr.dat";
        /// Neutral costume.
        pub const PlKbNr: &'static str = "PlKbNr.dat";
        /// Neutral costume, Copy Power (Donkey Kong).
        pub const PlKbNrCpDk: &'static str = "PlKbNrCpDk.dat";
        /// Neutral costume, Copy Power (Falco).
        pub const PlKbNrCpFc: &'static str = "PlKbNrCpFc.dat";
        /// Neutral costume, Copy Power (Game 'n Watch).
        pub const PlKbNrCpGw: &'static str = "PlKbNrCpGw.dat";
        /// Neutral costume, Copy Power (Mewtwo).
        pub const PlKbNrCpMt: &'static str = "PlKbNrCpMt.dat";
        /// Neutral costume, Copy Power (Jigglypuff).
        pub const PlKbNrCpPr: &'static str = "PlKbNrCpPr.dat";
        /// Red costume.
        pub const PlKbRe: &'static str = "PlKbRe.dat";
        /// Red costume, Copy Power (Donkey Kong).
        pub const PlKbReCpDk: &'static str = "PlKbReCpDk.dat";
        /// Red costume, Copy Power (Falco).
        pub const PlKbReCpFc: &'static str = "PlKbReCpFc.dat";
        /// Red costume, Copy Power (Mewtwo).
        pub const PlKbReCpMt: &'static str = "PlKbReCpMt.dat";
        /// Red costume, Copy Power (Jigglypuff).
        pub const PlKbReCpPr: &'static str = "PlKbReCpPr.dat";
        /// White costume.
        pub const PlKbWh: &'static str = "PlKbWh.dat";
        /// White costume, Copy Power (Donkey Kong).
        pub const PlKbWhCpDk: &'static str = "PlKbWhCpDk.dat";
        /// White costume, Copy Power (Falco).
        pub const PlKbWhCpFc: &'static str = "PlKbWhCpFc.dat";
        /// White costume, Copy Power (Mewtwo).
        pub const PlKbWhCpMt: &'static str = "PlKbWhCpMt.dat";
        /// White costume, Copy Power (Jigglypuff).
        pub const PlKbWhCpPr: &'static str = "PlKbWhCpPr.dat";
        /// Yellow costume.
        pub const PlKbYe: &'static str = "PlKbYe.dat";
        /// Yellow costume, Copy Power (Donkey Kong).
        pub const PlKbYeCpDk: &'static str = "PlKbYeCpDk.dat";
        /// Yellow costume, Copy Power (Falco).
        pub const PlKbYeCpFc: &'static str = "PlKbYeCpFc.dat";
        /// Yellow costume, Copy Power (Mewtwo).
        pub const PlKbYeCpMt: &'static str = "PlKbYeCpMt.dat";
        /// Yellow costume, Copy Power (Jigglypuff).
        pub const PlKbYeCpPr: &'static str = "PlKbYeCpPr.dat";
    }

    /// Supported files for Roy.
    #[derive(Debug, Clone)]
    pub struct Roy;
    impl Roy {
        /// Shared textures.
        pub const PlFe: &'static str = "PlFe.dat";
        /// Blue costume.
        pub const PlFeBu: &'static str = "PlFeBu.dat";
        /// Green costume.
        pub const PlFeGr: &'static str = "PlFeGr.dat";
        /// Neutral costume.
        pub const PlFeNr: &'static str = "PlFeNr.dat";
        /// Red costume.
        pub const PlFeRe: &'static str = "PlFeRe.dat";
        /// Yellow costume.
        pub const PlFeYe: &'static str = "PlFeYe.dat";
    }

    /// Supported files for Bowser.
    #[derive(Debug, Clone)]
    pub struct Bowser;
    impl Bowser {
        /// Shared textures.
        pub const PlKp: &'static str = "PlKp.dat";
        /// Black costume.
        pub const PlKpBk: &'static str = "PlKpBk.dat";
        /// Blue costume.
        pub const PlKpBu: &'static str = "PlKpBu.dat";
        /// Neutral costume.
        pub const PlKpNr: &'static str = "PlKpNr.dat";
        /// Red costume.
        pub const PlKpRe: &'static str = "PlKpRe.dat";
    }

    /// Supported files for Mewtwo.
    #[derive(Debug, Clone)]
    pub struct Mewtwo;
    impl Mewtwo {
        /// Shared textures.
        pub const PlMt: &'static str = "PlMt.dat";
        /// Blue costume.
        pub const PlMtBu: &'static str = "PlMtBu.dat";
        /// Green costume.
        pub const PlMtGr: &'static str = "PlMtGr.dat";
        /// Neutral costume.
        pub const PlMtNr: &'static str = "PlMtNr.dat";
        /// Red costume.
        pub const PlMtRe: &'static str = "PlMtRe.dat";
    }

    /// Supported files for [Popo] Ice Climbers.
    #[derive(Debug, Clone)]
    pub struct IceClimbersPopo;
    impl IceClimbersPopo {
        /// Shared textures.
        pub const PlPp: &'static str = "PlPp.dat";
        /// Green costume.
        pub const PlPpGr: &'static str = "PlPpGr.dat";
        /// Neutral costume.
        pub const PlPpNr: &'static str = "PlPpNr.dat";
        /// Orange costume.
        pub const PlPpOr: &'static str = "PlPpOr.dat";
        /// Red costume.
        pub const PlPpRe: &'static str = "PlPpRe.dat";
    }

    /// Supported files for Common.
    #[derive(Debug, Clone)]
    pub struct Common;
    impl Common {
        /// Shared textures.
        pub const PlCo: &'static str = "PlCo.dat";
    }

    /// Supported files for Link.
    #[derive(Debug, Clone)]
    pub struct Link;
    impl Link {
        /// Shared textures.
        pub const PlLk: &'static str = "PlLk.dat";
        /// Black costume.
        pub const PlLkBk: &'static str = "PlLkBk.dat";
        /// Blue costume.
        pub const PlLkBu: &'static str = "PlLkBu.dat";
        /// Neutral costume.
        pub const PlLkNr: &'static str = "PlLkNr.dat";
        /// Red costume.
        pub const PlLkRe: &'static str = "PlLkRe.dat";
        /// White costume.
        pub const PlLkWh: &'static str = "PlLkWh.dat";
    }
}

pub mod parse {
    //! Parsing functions for filesystem table entries.
    //!
    //! ````text
    //! 13.4.1 Format of a File Entry
    //! =============================
    //! +-----------+---------+----------+------------------------------+
    //! |   start   |   end   |   size   |   Description                |
    //! +-----------+---------+----------+------------------------------+
    //! |   0x00    |         |   1      | flags; 0: file 1: directory  |
    //! +-----------+---------+----------+------------------------------+
    //! |   0x01    |         |   3      | filename, offset into string |
    //! |           |         |          | table                        |
    //! +-----------+---------+----------+------------------------------+
    //! |   0x04    |         |   4      | file_offset or parent_offset |
    //! |           |         |          | (dir)                        |
    //! +-----------+---------+----------+------------------------------+
    //! |   0x08    |         |   4      | file_length or num_entries   |
    //! |           |         |          | (root) or next_offset (dir)  |
    //! +-----------+---------+----------+------------------------------+
    //! ````
    //! <https://www.gc-forever.com/yagcd/chap13.html>

    use std::io::{self, Read, Seek, SeekFrom, Write};
    use std::path::Path;

    /// Read bytes 0x04 -> 0x08 as a u32 (filename offset).
    pub fn node_file_offset(node: [u8; 0x0c]) -> u32 {
        let (filename_offset_bytes, _) = &node[4..8].split_at(std::mem::size_of::<u32>());
        let bytes: [u8; 4] = (*filename_offset_bytes)
            .try_into()
            .expect("failed to parse root node num_entries");
        u32::from_be_bytes(bytes)
    }

    /// Read bytes 0x08 -> 0x0c as u32 (num_entries).
    pub fn root_node_num_entries(node: [u8; 0x0c]) -> u32 {
        let (num_entries_bytes, _) = &node[8..0xc].split_at(std::mem::size_of::<u32>());
        let bytes: [u8; 4] = (*num_entries_bytes)
            .try_into()
            .expect("failed to parse root node num_entries");
        u32::from_be_bytes(bytes)
    }

    /// Read bytes 0x00 -> 0x01 as u8 (directory flag).
    pub fn node_is_directory(node: [u8; 0x0c]) -> bool {
        let (file_or_directory, _) = &node[0..1].split_at(std::mem::size_of::<u8>());
        let bytes: [u8; 1] = (*file_or_directory)
            .try_into()
            .expect("failed to parse directory flag: {node}");
        let directory_flag = u8::from_be_bytes(bytes);

        directory_flag != 0
    }

    #[allow(unused)]
    /// Output full filesystem table within the ISO on io::stdout.
    ///
    /// Assumes v1.02 NTSC GALE01.
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
    //! Replace characters and stage assets within the game.
    //!
    //! This library only handles replacing DAT files currently.
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

    /// Look up and read a file entry within an ISO, returning a no-op UpdateFST action.
    ///
    /// TODO: this function opens the ISO for every node! it should only open the file once
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
    /// ```text
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
    /// ```
    ///  <https://www.gc-forever.com/yagcd/chap13.html>
    ///
    /// v1.02 NTSC GALE01 Root Entry
    /// ============================
    /// ```text
    /// 0001 0203 0405 0607 0809 0a0b
    /// ---- ---- ---- ---- ---- ----
    /// 0100 0000 0000 0000 0000 04bc
    /// ^ ^       ^         ^-------- num_entries (0x04bc) (1212 entries)
    /// | |       \------------------ filename or parent_offset (0x00)
    /// | \-------------------------- filename string table offset (0x00)
    /// \---------------------------- flag (directory)
    /// ```
    ///
    /// - there are 0x4bc entries, each 0x0c long
    /// - string table offset starts at (0x04bc * 0x0c) = 0x38d0
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

        // create cursor over filesystem table
        let mut cursor = Cursor::new(new_fst);

        // read the root node
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

    /// Rebuild an ISO, given an updated filesystem table.
    ///
    /// The filesystem table has already been replaced with new data,
    /// so this function just writes a new disc image.
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
