#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct SfxInfo {
    pub tagname: *mut c_char,
    pub name: [c_char; 9],
    pub priority: c_int,
    pub link: *mut SfxInfo,
    pub pitch: c_int,
    pub volume: c_int,
    pub usefulness: c_int,
    pub lumpnum: c_int,
    pub numchannels: c_int,
    pub driver_data: *mut c_void,
}

#[repr(C)]
#[derive(Default)]
pub struct MusicInfo {
    pub name: *mut c_char,
    pub lumpnum: c_int,
    pub data: *mut c_void,
    pub handle: *mut c_void,
}

// sfxenum_t in sounds.h ends with NUMSFX = 109
pub const NUMSFX: usize = Sfx::Radio as usize + 1;
// musicenum_t in sounds.h ends with NUMMUSIC = 68
pub const NUMMUSIC: usize = Mus::Dm2int as usize + 1;

const _: () = assert!(
    std::mem::size_of::<Sfx>() == std::mem::size_of::<std::ffi::c_int>(),
    "Sfx must be the same size as c_int"
);
const _: () = assert!(
    std::mem::size_of::<Mus>() == std::mem::size_of::<std::ffi::c_int>(),
    "Mus must be the same size as c_int"
);

/// Sound effect IDs, matching the `sfxenum_t` C enum in `sounds.h`.
#[doc(alias = "sfxenum_t")]
#[repr(C)]
#[derive(Default, PartialEq, Clone, Copy)]
pub enum Sfx {
    #[default]
    #[doc(alias = "sfx_None")]
    None = 0,
    #[doc(alias = "sfx_pistol")]
    Pistol = 1,
    #[doc(alias = "sfx_shotgn")]
    Shotgn = 2,
    #[doc(alias = "sfx_sgcock")]
    Sgcock = 3,
    #[doc(alias = "sfx_dshtgn")]
    Dshtgn = 4,
    #[doc(alias = "sfx_dbopn")]
    Dbopn = 5,
    #[doc(alias = "sfx_dbcls")]
    Dbcls = 6,
    #[doc(alias = "sfx_dbload")]
    Dbload = 7,
    #[doc(alias = "sfx_plasma")]
    Plasma = 8,
    #[doc(alias = "sfx_bfg")]
    Bfg = 9,
    #[doc(alias = "sfx_sawup")]
    Sawup = 10,
    #[doc(alias = "sfx_sawidl")]
    Sawidl = 11,
    #[doc(alias = "sfx_sawful")]
    Sawful = 12,
    #[doc(alias = "sfx_sawhit")]
    Sawhit = 13,
    #[doc(alias = "sfx_rlaunc")]
    Rlaunc = 14,
    #[doc(alias = "sfx_rxplod")]
    Rxplod = 15,
    #[doc(alias = "sfx_firsht")]
    Firsht = 16,
    #[doc(alias = "sfx_firxpl")]
    Firxpl = 17,
    #[doc(alias = "sfx_pstart")]
    Pstart = 18,
    #[doc(alias = "sfx_pstop")]
    Pstop = 19,
    #[doc(alias = "sfx_doropn")]
    Doropn = 20,
    #[doc(alias = "sfx_dorcls")]
    Dorcls = 21,
    #[doc(alias = "sfx_stnmov")]
    Stnmov = 22,
    #[doc(alias = "sfx_swtchn")]
    Swtchn = 23,
    #[doc(alias = "sfx_swtchx")]
    Swtchx = 24,
    #[doc(alias = "sfx_plpain")]
    Plpain = 25,
    #[doc(alias = "sfx_dmpain")]
    Dmpain = 26,
    #[doc(alias = "sfx_popain")]
    Popain = 27,
    #[doc(alias = "sfx_vipain")]
    Vipain = 28,
    #[doc(alias = "sfx_mnpain")]
    Mnpain = 29,
    #[doc(alias = "sfx_pepain")]
    Pepain = 30,
    #[doc(alias = "sfx_slop")]
    Slop = 31,
    #[doc(alias = "sfx_itemup")]
    Itemup = 32,
    #[doc(alias = "sfx_wpnup")]
    Wpnup = 33,
    #[doc(alias = "sfx_oof")]
    Oof = 34,
    #[doc(alias = "sfx_telept")]
    Telept = 35,
    #[doc(alias = "sfx_posit1")]
    Posit1 = 36,
    #[doc(alias = "sfx_posit2")]
    Posit2 = 37,
    #[doc(alias = "sfx_posit3")]
    Posit3 = 38,
    #[doc(alias = "sfx_bgsit1")]
    Bgsit1 = 39,
    #[doc(alias = "sfx_bgsit2")]
    Bgsit2 = 40,
    #[doc(alias = "sfx_sgtsit")]
    Sgtsit = 41,
    #[doc(alias = "sfx_cacsit")]
    Cacsit = 42,
    #[doc(alias = "sfx_brssit")]
    Brssit = 43,
    #[doc(alias = "sfx_cybsit")]
    Cybsit = 44,
    #[doc(alias = "sfx_spisit")]
    Spisit = 45,
    #[doc(alias = "sfx_bspsit")]
    Bspsit = 46,
    #[doc(alias = "sfx_kntsit")]
    Kntsit = 47,
    #[doc(alias = "sfx_vilsit")]
    Vilsit = 48,
    #[doc(alias = "sfx_mansit")]
    Mansit = 49,
    #[doc(alias = "sfx_pesit")]
    Pesit = 50,
    #[doc(alias = "sfx_sklatk")]
    Sklatk = 51,
    #[doc(alias = "sfx_sgtatk")]
    Sgtatk = 52,
    #[doc(alias = "sfx_skepch")]
    Skepch = 53,
    #[doc(alias = "sfx_vilatk")]
    Vilatk = 54,
    #[doc(alias = "sfx_claw")]
    Claw = 55,
    #[doc(alias = "sfx_skeswg")]
    Skeswg = 56,
    #[doc(alias = "sfx_pldeth")]
    Pldeth = 57,
    #[doc(alias = "sfx_pdiehi")]
    Pdiehi = 58,
    #[doc(alias = "sfx_podth1")]
    Podth1 = 59,
    #[doc(alias = "sfx_podth2")]
    Podth2 = 60,
    #[doc(alias = "sfx_podth3")]
    Podth3 = 61,
    #[doc(alias = "sfx_bgdth1")]
    Bgdth1 = 62,
    #[doc(alias = "sfx_bgdth2")]
    Bgdth2 = 63,
    #[doc(alias = "sfx_sgtdth")]
    Sgtdth = 64,
    #[doc(alias = "sfx_cacdth")]
    Cacdth = 65,
    #[doc(alias = "sfx_skldth")]
    Skldth = 66,
    #[doc(alias = "sfx_brsdth")]
    Brsdth = 67,
    #[doc(alias = "sfx_cybdth")]
    Cybdth = 68,
    #[doc(alias = "sfx_spidth")]
    Spidth = 69,
    #[doc(alias = "sfx_bspdth")]
    Bspdth = 70,
    #[doc(alias = "sfx_vildth")]
    Vildth = 71,
    #[doc(alias = "sfx_kntdth")]
    Kntdth = 72,
    #[doc(alias = "sfx_pedth")]
    Pedth = 73,
    #[doc(alias = "sfx_skedth")]
    Skedth = 74,
    #[doc(alias = "sfx_posact")]
    Posact = 75,
    #[doc(alias = "sfx_bgact")]
    Bgact = 76,
    #[doc(alias = "sfx_dmact")]
    Dmact = 77,
    #[doc(alias = "sfx_bspact")]
    Bspact = 78,
    #[doc(alias = "sfx_bspwlk")]
    Bspwlk = 79,
    #[doc(alias = "sfx_vilact")]
    Vilact = 80,
    #[doc(alias = "sfx_noway")]
    Noway = 81,
    #[doc(alias = "sfx_barexp")]
    Barexp = 82,
    #[doc(alias = "sfx_punch")]
    Punch = 83,
    #[doc(alias = "sfx_hoof")]
    Hoof = 84,
    #[doc(alias = "sfx_metal")]
    Metal = 85,
    #[doc(alias = "sfx_chgun")]
    Chgun = 86,
    #[doc(alias = "sfx_tink")]
    Tink = 87,
    #[doc(alias = "sfx_bdopn")]
    Bdopn = 88,
    #[doc(alias = "sfx_bdcls")]
    Bdcls = 89,
    #[doc(alias = "sfx_itmbk")]
    Itmbk = 90,
    #[doc(alias = "sfx_flame")]
    Flame = 91,
    #[doc(alias = "sfx_flamst")]
    Flamst = 92,
    #[doc(alias = "sfx_getpow")]
    Getpow = 93,
    #[doc(alias = "sfx_bospit")]
    Bospit = 94,
    #[doc(alias = "sfx_boscub")]
    Boscub = 95,
    #[doc(alias = "sfx_bossit")]
    Bossit = 96,
    #[doc(alias = "sfx_bospn")]
    Bospn = 97,
    #[doc(alias = "sfx_bosdth")]
    Bosdth = 98,
    #[doc(alias = "sfx_manatk")]
    Manatk = 99,
    #[doc(alias = "sfx_mandth")]
    Mandth = 100,
    #[doc(alias = "sfx_sssit")]
    Sssit = 101,
    #[doc(alias = "sfx_ssdth")]
    Ssdth = 102,
    #[doc(alias = "sfx_keenpn")]
    Keenpn = 103,
    #[doc(alias = "sfx_keendt")]
    Keendt = 104,
    #[doc(alias = "sfx_skeact")]
    Skeact = 105,
    #[doc(alias = "sfx_skesit")]
    Skesit = 106,
    #[doc(alias = "sfx_skeatk")]
    Skeatk = 107,
    #[doc(alias = "sfx_radio")]
    Radio = 108,
}

/// Music track IDs, matching the `musicenum_t` C enum in `sounds.h`.
#[doc(alias = "musicenum_t")]
#[repr(C)]
#[derive(Default, PartialEq, Clone, Copy)]
pub enum Mus {
    #[default]
    #[doc(alias = "mus_None")]
    None = 0,
    #[doc(alias = "mus_e1m1")]
    E1m1 = 1,
    #[doc(alias = "mus_e1m2")]
    E1m2 = 2,
    #[doc(alias = "mus_e1m3")]
    E1m3 = 3,
    #[doc(alias = "mus_e1m4")]
    E1m4 = 4,
    #[doc(alias = "mus_e1m5")]
    E1m5 = 5,
    #[doc(alias = "mus_e1m6")]
    E1m6 = 6,
    #[doc(alias = "mus_e1m7")]
    E1m7 = 7,
    #[doc(alias = "mus_e1m8")]
    E1m8 = 8,
    #[doc(alias = "mus_e1m9")]
    E1m9 = 9,
    #[doc(alias = "mus_e2m1")]
    E2m1 = 10,
    #[doc(alias = "mus_e2m2")]
    E2m2 = 11,
    #[doc(alias = "mus_e2m3")]
    E2m3 = 12,
    #[doc(alias = "mus_e2m4")]
    E2m4 = 13,
    #[doc(alias = "mus_e2m5")]
    E2m5 = 14,
    #[doc(alias = "mus_e2m6")]
    E2m6 = 15,
    #[doc(alias = "mus_e2m7")]
    E2m7 = 16,
    #[doc(alias = "mus_e2m8")]
    E2m8 = 17,
    #[doc(alias = "mus_e2m9")]
    E2m9 = 18,
    #[doc(alias = "mus_e3m1")]
    E3m1 = 19,
    #[doc(alias = "mus_e3m2")]
    E3m2 = 20,
    #[doc(alias = "mus_e3m3")]
    E3m3 = 21,
    #[doc(alias = "mus_e3m4")]
    E3m4 = 22,
    #[doc(alias = "mus_e3m5")]
    E3m5 = 23,
    #[doc(alias = "mus_e3m6")]
    E3m6 = 24,
    #[doc(alias = "mus_e3m7")]
    E3m7 = 25,
    #[doc(alias = "mus_e3m8")]
    E3m8 = 26,
    #[doc(alias = "mus_e3m9")]
    E3m9 = 27,
    #[doc(alias = "mus_inter")]
    Inter = 28,
    #[doc(alias = "mus_intro")]
    Intro = 29,
    #[doc(alias = "mus_bunny")]
    Bunny = 30,
    #[doc(alias = "mus_victor")]
    Victor = 31,
    #[doc(alias = "mus_introa")]
    Introa = 32,
    #[doc(alias = "mus_runnin")]
    Runnin = 33,
    #[doc(alias = "mus_stalks")]
    Stalks = 34,
    #[doc(alias = "mus_countd")]
    Countd = 35,
    #[doc(alias = "mus_betwee")]
    Betwee = 36,
    #[doc(alias = "mus_doom")]
    Doom = 37,
    #[doc(alias = "mus_the_da")]
    TheDa = 38,
    #[doc(alias = "mus_shawn")]
    Shawn = 39,
    #[doc(alias = "mus_ddtblu")]
    Ddtblu = 40,
    #[doc(alias = "mus_in_cit")]
    InCit = 41,
    #[doc(alias = "mus_dead")]
    Dead = 42,
    #[doc(alias = "mus_stlks2")]
    Stlks2 = 43,
    #[doc(alias = "mus_theda2")]
    Theda2 = 44,
    #[doc(alias = "mus_doom2")]
    Doom2 = 45,
    #[doc(alias = "mus_ddtbl2")]
    Ddtbl2 = 46,
    #[doc(alias = "mus_runni2")]
    Runni2 = 47,
    #[doc(alias = "mus_dead2")]
    Dead2 = 48,
    #[doc(alias = "mus_stlks3")]
    Stlks3 = 49,
    #[doc(alias = "mus_romero")]
    Romero = 50,
    #[doc(alias = "mus_shawn2")]
    Shawn2 = 51,
    #[doc(alias = "mus_messag")]
    Messag = 52,
    #[doc(alias = "mus_count2")]
    Count2 = 53,
    #[doc(alias = "mus_ddtbl3")]
    Ddtbl3 = 54,
    #[doc(alias = "mus_ampie")]
    Ampie = 55,
    #[doc(alias = "mus_theda3")]
    Theda3 = 56,
    #[doc(alias = "mus_adrian")]
    Adrian = 57,
    #[doc(alias = "mus_messg2")]
    Messg2 = 58,
    #[doc(alias = "mus_romer2")]
    Romer2 = 59,
    #[doc(alias = "mus_tense")]
    Tense = 60,
    #[doc(alias = "mus_shawn3")]
    Shawn3 = 61,
    #[doc(alias = "mus_openin")]
    Openin = 62,
    #[doc(alias = "mus_evil")]
    Evil = 63,
    #[doc(alias = "mus_ultima")]
    Ultima = 64,
    #[doc(alias = "mus_read_m")]
    ReadM = 65,
    #[doc(alias = "mus_dm2ttl")]
    Dm2ttl = 66,
    #[doc(alias = "mus_dm2int")]
    Dm2int = 67,
}

unsafe impl Sync for SfxInfo {}
unsafe impl Sync for MusicInfo {}

impl SfxInfo {
    const fn new(name: [c_char; 9], priority: c_int) -> Self {
        SfxInfo {
            tagname: std::ptr::null_mut(),
            name,
            priority,
            link: std::ptr::null_mut(),
            pitch: -1,
            volume: -1,
            usefulness: 0,
            lumpnum: 0,
            numchannels: -1,
            driver_data: std::ptr::null_mut(),
        }
    }

    const fn with_pitch(mut self, pitch: c_int) -> Self {
        self.pitch = pitch;
        self
    }

    const fn with_volume(mut self, volume: c_int) -> Self {
        self.volume = volume;
        self
    }
}

impl MusicInfo {
    const fn none() -> Self {
        MusicInfo {
            name: std::ptr::null_mut(),
            lumpnum: 0,
            data: std::ptr::null_mut(),
            handle: std::ptr::null_mut(),
        }
    }

    const fn new<const N: usize>(name: &'static [c_char; N]) -> Self {
        MusicInfo {
            name: name.as_ptr() as *mut c_char,
            lumpnum: 0,
            data: std::ptr::null_mut(),
            handle: std::ptr::null_mut(),
        }
    }
}

const fn name<const N: usize>(s: &str) -> [c_char; N] {
    let b = s.as_bytes();
    assert!(b.len() < N, "String is too long for the array");
    let mut a = [0i8; N];
    let mut i = 0;
    while i < b.len() {
        a[i] = b[i] as c_char;
        i += 1;
    }
    a
}

const N_none: [c_char; 9] = name("none");
const N_pistol: [c_char; 9] = name("pistol");
const N_shotgn: [c_char; 9] = name("shotgn");
const N_sgcock: [c_char; 9] = name("sgcock");
const N_dshtgn: [c_char; 9] = name("dshtgn");
const N_dbopn: [c_char; 9] = name("dbopn");
const N_dbcls: [c_char; 9] = name("dbcls");
const N_dbload: [c_char; 9] = name("dbload");
const N_plasma: [c_char; 9] = name("plasma");
const N_bfg: [c_char; 9] = name("bfg");
const N_sawup: [c_char; 9] = name("sawup");
const N_sawidl: [c_char; 9] = name("sawidl");
const N_sawful: [c_char; 9] = name("sawful");
const N_sawhit: [c_char; 9] = name("sawhit");
const N_rlaunc: [c_char; 9] = name("rlaunc");
const N_rxplod: [c_char; 9] = name("rxplod");
const N_firsht: [c_char; 9] = name("firsht");
const N_firxpl: [c_char; 9] = name("firxpl");
const N_pstart: [c_char; 9] = name("pstart");
const N_pstop: [c_char; 9] = name("pstop");
const N_doropn: [c_char; 9] = name("doropn");
const N_dorcls: [c_char; 9] = name("dorcls");
const N_stnmov: [c_char; 9] = name("stnmov");
const N_swtchn: [c_char; 9] = name("swtchn");
const N_swtchx: [c_char; 9] = name("swtchx");
const N_plpain: [c_char; 9] = name("plpain");
const N_dmpain: [c_char; 9] = name("dmpain");
const N_popain: [c_char; 9] = name("popain");
const N_vipain: [c_char; 9] = name("vipain");
const N_mnpain: [c_char; 9] = name("mnpain");
const N_pepain: [c_char; 9] = name("pepain");
const N_slop: [c_char; 9] = name("slop");
const N_itemup: [c_char; 9] = name("itemup");
const N_wpnup: [c_char; 9] = name("wpnup");
const N_oof: [c_char; 9] = name("oof");
const N_telept: [c_char; 9] = name("telept");
const N_posit1: [c_char; 9] = name("posit1");
const N_posit2: [c_char; 9] = name("posit2");
const N_posit3: [c_char; 9] = name("posit3");
const N_bgsit1: [c_char; 9] = name("bgsit1");
const N_bgsit2: [c_char; 9] = name("bgsit2");
const N_sgtsit: [c_char; 9] = name("sgtsit");
const N_cacsit: [c_char; 9] = name("cacsit");
const N_brssit: [c_char; 9] = name("brssit");
const N_cybsit: [c_char; 9] = name("cybsit");
const N_spisit: [c_char; 9] = name("spisit");
const N_bspit: [c_char; 9] = name("bspsit");
const N_kntsit: [c_char; 9] = name("kntsit");
const N_vilsit: [c_char; 9] = name("vilsit");
const N_mansit: [c_char; 9] = name("mansit");
const N_pesit: [c_char; 9] = name("pesit");
const N_sklatk: [c_char; 9] = name("sklatk");
const N_sgtatk: [c_char; 9] = name("sgtatk");
const N_skepch: [c_char; 9] = name("skepch");
const N_vilatk: [c_char; 9] = name("vilatk");
const N_claw: [c_char; 9] = name("claw");
const N_skeswg: [c_char; 9] = name("skeswg");
const N_pldeth: [c_char; 9] = name("pldeth");
const N_pdiehi: [c_char; 9] = name("pdiehi");
const N_podth1: [c_char; 9] = name("podth1");
const N_podth2: [c_char; 9] = name("podth2");
const N_podth3: [c_char; 9] = name("podth3");
const N_bgdth1: [c_char; 9] = name("bgdth1");
const N_bgdth2: [c_char; 9] = name("bgdth2");
const N_sgtdth: [c_char; 9] = name("sgtdth");
const N_cacdth: [c_char; 9] = name("cacdth");
const N_skldth: [c_char; 9] = name("skldth");
const N_brsdth: [c_char; 9] = name("brsdth");
const N_cybdth: [c_char; 9] = name("cybdth");
const N_spidth: [c_char; 9] = name("spidth");
const N_bspdth: [c_char; 9] = name("bspdth");
const N_vildth: [c_char; 9] = name("vildth");
const N_kntdth: [c_char; 9] = name("kntdth");
const N_pedth: [c_char; 9] = name("pedth");
const N_skedth: [c_char; 9] = name("skedth");
const N_posact: [c_char; 9] = name("posact");
const N_bgact: [c_char; 9] = name("bgact");
const N_dmact: [c_char; 9] = name("dmact");
const N_bspact: [c_char; 9] = name("bspact");
const N_bspwlk: [c_char; 9] = name("bspwlk");
const N_vilact: [c_char; 9] = name("vilact");
const N_noway: [c_char; 9] = name("noway");
const N_barexp: [c_char; 9] = name("barexp");
const N_punch: [c_char; 9] = name("punch");
const N_hoof: [c_char; 9] = name("hoof");
const N_metal: [c_char; 9] = name("metal");
const N_chgun: [c_char; 9] = name("chgun");
const N_tink: [c_char; 9] = name("tink");
const N_bdopn: [c_char; 9] = name("bdopn");
const N_bdcls: [c_char; 9] = name("bdcls");
const N_itmbk: [c_char; 9] = name("itmbk");
const N_flame: [c_char; 9] = name("flame");
const N_flamst: [c_char; 9] = name("flamst");
const N_getpow: [c_char; 9] = name("getpow");
const N_bospit: [c_char; 9] = name("bospit");
const N_boscub: [c_char; 9] = name("boscub");
const N_bossit: [c_char; 9] = name("bossit");
const N_bospn: [c_char; 9] = name("bospn");
const N_bosdth: [c_char; 9] = name("bosdth");
const N_manatk: [c_char; 9] = name("manatk");
const N_mandth: [c_char; 9] = name("mandth");
const N_sssit: [c_char; 9] = name("sssit");
const N_ssdth: [c_char; 9] = name("ssdth");
const N_keenpn: [c_char; 9] = name("keenpn");
const N_keendt: [c_char; 9] = name("keendt");
const N_skeact: [c_char; 9] = name("skeact");
const N_skesit: [c_char; 9] = name("skesit");
const N_skeatk: [c_char; 9] = name("skeatk");
const N_radio: [c_char; 9] = name("radio");

const MUS_e1m1: [c_char; 5] = name("e1m1");
const MUS_e1m2: [c_char; 5] = name("e1m2");
const MUS_e1m3: [c_char; 5] = name("e1m3");
const MUS_e1m4: [c_char; 5] = name("e1m4");
const MUS_e1m5: [c_char; 5] = name("e1m5");
const MUS_e1m6: [c_char; 5] = name("e1m6");
const MUS_e1m7: [c_char; 5] = name("e1m7");
const MUS_e1m8: [c_char; 5] = name("e1m8");
const MUS_e1m9: [c_char; 5] = name("e1m9");
const MUS_e2m1: [c_char; 5] = name("e2m1");
const MUS_e2m2: [c_char; 5] = name("e2m2");
const MUS_e2m3: [c_char; 5] = name("e2m3");
const MUS_e2m4: [c_char; 5] = name("e2m4");
const MUS_e2m5: [c_char; 5] = name("e2m5");
const MUS_e2m6: [c_char; 5] = name("e2m6");
const MUS_e2m7: [c_char; 5] = name("e2m7");
const MUS_e2m8: [c_char; 5] = name("e2m8");
const MUS_e2m9: [c_char; 5] = name("e2m9");
const MUS_e3m1: [c_char; 5] = name("e3m1");
const MUS_e3m2: [c_char; 5] = name("e3m2");
const MUS_e3m3: [c_char; 5] = name("e3m3");
const MUS_e3m4: [c_char; 5] = name("e3m4");
const MUS_e3m5: [c_char; 5] = name("e3m5");
const MUS_e3m6: [c_char; 5] = name("e3m6");
const MUS_e3m7: [c_char; 5] = name("e3m7");
const MUS_e3m8: [c_char; 5] = name("e3m8");
const MUS_e3m9: [c_char; 5] = name("e3m9");
const MUS_inter: [c_char; 6] = name("inter");
const MUS_intro: [c_char; 6] = name("intro");
const MUS_bunny: [c_char; 6] = name("bunny");
const MUS_victor: [c_char; 7] = name("victor");
const MUS_introa: [c_char; 7] = name("introa");
const MUS_runnin: [c_char; 7] = name("runnin");
const MUS_stalks: [c_char; 7] = name("stalks");
const MUS_countd: [c_char; 7] = name("countd");
const MUS_betwee: [c_char; 7] = name("betwee");
const MUS_doom: [c_char; 5] = name("doom");
const MUS_the_da: [c_char; 7] = name("the_da");
const MUS_shawn: [c_char; 6] = name("shawn");
const MUS_ddtblu: [c_char; 7] = name("ddtblu");
const MUS_in_cit: [c_char; 7] = name("in_cit");
const MUS_dead: [c_char; 5] = name("dead");
const MUS_stlks2: [c_char; 7] = name("stlks2");
const MUS_theda2: [c_char; 7] = name("theda2");
const MUS_doom2: [c_char; 6] = name("doom2");
const MUS_ddtbl2: [c_char; 7] = name("ddtbl2");
const MUS_runni2: [c_char; 7] = name("runni2");
const MUS_dead2: [c_char; 6] = name("dead2");
const MUS_stlks3: [c_char; 7] = name("stlks3");
const MUS_romero: [c_char; 7] = name("romero");
const MUS_shawn2: [c_char; 7] = name("shawn2");
const MUS_messag: [c_char; 7] = name("messag");
const MUS_count2: [c_char; 7] = name("count2");
const MUS_ddtbl3: [c_char; 7] = name("ddtbl3");
const MUS_ampie: [c_char; 6] = name("ampie");
const MUS_theda3: [c_char; 7] = name("theda3");
const MUS_adrian: [c_char; 7] = name("adrian");
const MUS_messg2: [c_char; 7] = name("messg2");
const MUS_romer2: [c_char; 7] = name("romer2");
const MUS_tense: [c_char; 6] = name("tense");
const MUS_shawn3: [c_char; 7] = name("shawn3");
const MUS_openin: [c_char; 7] = name("openin");
const MUS_evil: [c_char; 5] = name("evil");
const MUS_ultima: [c_char; 7] = name("ultima");
const MUS_read_m: [c_char; 7] = name("read_m");
const MUS_dm2ttl: [c_char; 7] = name("dm2ttl");
const MUS_dm2int: [c_char; 7] = name("dm2int");

#[no_mangle]
pub static mut S_sfx: [SfxInfo; NUMSFX] = [
    SfxInfo::new(N_none, 0),
    SfxInfo::new(N_pistol, 64),
    SfxInfo::new(N_shotgn, 64),
    SfxInfo::new(N_sgcock, 64),
    SfxInfo::new(N_dshtgn, 64),
    SfxInfo::new(N_dbopn, 64),
    SfxInfo::new(N_dbcls, 64),
    SfxInfo::new(N_dbload, 64),
    SfxInfo::new(N_plasma, 64),
    SfxInfo::new(N_bfg, 64),
    SfxInfo::new(N_sawup, 64),
    SfxInfo::new(N_sawidl, 118),
    SfxInfo::new(N_sawful, 64),
    SfxInfo::new(N_sawhit, 64),
    SfxInfo::new(N_rlaunc, 64),
    SfxInfo::new(N_rxplod, 70),
    SfxInfo::new(N_firsht, 70),
    SfxInfo::new(N_firxpl, 70),
    SfxInfo::new(N_pstart, 100),
    SfxInfo::new(N_pstop, 100),
    SfxInfo::new(N_doropn, 100),
    SfxInfo::new(N_dorcls, 100),
    SfxInfo::new(N_stnmov, 119),
    SfxInfo::new(N_swtchn, 78),
    SfxInfo::new(N_swtchx, 78),
    SfxInfo::new(N_plpain, 96),
    SfxInfo::new(N_dmpain, 96),
    SfxInfo::new(N_popain, 96),
    SfxInfo::new(N_vipain, 96),
    SfxInfo::new(N_mnpain, 96),
    SfxInfo::new(N_pepain, 96),
    SfxInfo::new(N_slop, 78),
    SfxInfo::new(N_itemup, 78),
    SfxInfo::new(N_wpnup, 78),
    SfxInfo::new(N_oof, 96),
    SfxInfo::new(N_telept, 32),
    SfxInfo::new(N_posit1, 98),
    SfxInfo::new(N_posit2, 98),
    SfxInfo::new(N_posit3, 98),
    SfxInfo::new(N_bgsit1, 98),
    SfxInfo::new(N_bgsit2, 98),
    SfxInfo::new(N_sgtsit, 98),
    SfxInfo::new(N_cacsit, 98),
    SfxInfo::new(N_brssit, 94),
    SfxInfo::new(N_cybsit, 92),
    SfxInfo::new(N_spisit, 90),
    SfxInfo::new(N_bspit, 90),
    SfxInfo::new(N_kntsit, 90),
    SfxInfo::new(N_vilsit, 90),
    SfxInfo::new(N_mansit, 90),
    SfxInfo::new(N_pesit, 90),
    SfxInfo::new(N_sklatk, 70),
    SfxInfo::new(N_sgtatk, 70),
    SfxInfo::new(N_skepch, 70),
    SfxInfo::new(N_vilatk, 70),
    SfxInfo::new(N_claw, 70),
    SfxInfo::new(N_skeswg, 70),
    SfxInfo::new(N_pldeth, 32),
    SfxInfo::new(N_pdiehi, 32),
    SfxInfo::new(N_podth1, 70),
    SfxInfo::new(N_podth2, 70),
    SfxInfo::new(N_podth3, 70),
    SfxInfo::new(N_bgdth1, 70),
    SfxInfo::new(N_bgdth2, 70),
    SfxInfo::new(N_sgtdth, 70),
    SfxInfo::new(N_cacdth, 70),
    SfxInfo::new(N_skldth, 70),
    SfxInfo::new(N_brsdth, 32),
    SfxInfo::new(N_cybdth, 32),
    SfxInfo::new(N_spidth, 32),
    SfxInfo::new(N_bspdth, 32),
    SfxInfo::new(N_vildth, 32),
    SfxInfo::new(N_kntdth, 32),
    SfxInfo::new(N_pedth, 32),
    SfxInfo::new(N_skedth, 32),
    SfxInfo::new(N_posact, 120),
    SfxInfo::new(N_bgact, 120),
    SfxInfo::new(N_dmact, 120),
    SfxInfo::new(N_bspact, 100),
    SfxInfo::new(N_bspwlk, 100),
    SfxInfo::new(N_vilact, 100),
    SfxInfo::new(N_noway, 78),
    SfxInfo::new(N_barexp, 60),
    SfxInfo::new(N_punch, 64),
    SfxInfo::new(N_hoof, 70),
    SfxInfo::new(N_metal, 70),
    SfxInfo::new(N_chgun, 64).with_volume(0).with_pitch(150),
    SfxInfo::new(N_tink, 60),
    SfxInfo::new(N_bdopn, 100),
    SfxInfo::new(N_bdcls, 100),
    SfxInfo::new(N_itmbk, 100),
    SfxInfo::new(N_flame, 32),
    SfxInfo::new(N_flamst, 32),
    SfxInfo::new(N_getpow, 60),
    SfxInfo::new(N_bospit, 70),
    SfxInfo::new(N_boscub, 70),
    SfxInfo::new(N_bossit, 70),
    SfxInfo::new(N_bospn, 70),
    SfxInfo::new(N_bosdth, 70),
    SfxInfo::new(N_manatk, 70),
    SfxInfo::new(N_mandth, 70),
    SfxInfo::new(N_sssit, 70),
    SfxInfo::new(N_ssdth, 70),
    SfxInfo::new(N_keenpn, 70),
    SfxInfo::new(N_keendt, 70),
    SfxInfo::new(N_skeact, 70),
    SfxInfo::new(N_skesit, 70),
    SfxInfo::new(N_skeatk, 70),
    SfxInfo::new(N_radio, 60),
];

#[no_mangle]
pub static mut S_music: [MusicInfo; NUMMUSIC] = [
    MusicInfo::none(),
    MusicInfo::new(&MUS_e1m1),
    MusicInfo::new(&MUS_e1m2),
    MusicInfo::new(&MUS_e1m3),
    MusicInfo::new(&MUS_e1m4),
    MusicInfo::new(&MUS_e1m5),
    MusicInfo::new(&MUS_e1m6),
    MusicInfo::new(&MUS_e1m7),
    MusicInfo::new(&MUS_e1m8),
    MusicInfo::new(&MUS_e1m9),
    MusicInfo::new(&MUS_e2m1),
    MusicInfo::new(&MUS_e2m2),
    MusicInfo::new(&MUS_e2m3),
    MusicInfo::new(&MUS_e2m4),
    MusicInfo::new(&MUS_e2m5),
    MusicInfo::new(&MUS_e2m6),
    MusicInfo::new(&MUS_e2m7),
    MusicInfo::new(&MUS_e2m8),
    MusicInfo::new(&MUS_e2m9),
    MusicInfo::new(&MUS_e3m1),
    MusicInfo::new(&MUS_e3m2),
    MusicInfo::new(&MUS_e3m3),
    MusicInfo::new(&MUS_e3m4),
    MusicInfo::new(&MUS_e3m5),
    MusicInfo::new(&MUS_e3m6),
    MusicInfo::new(&MUS_e3m7),
    MusicInfo::new(&MUS_e3m8),
    MusicInfo::new(&MUS_e3m9),
    MusicInfo::new(&MUS_inter),
    MusicInfo::new(&MUS_intro),
    MusicInfo::new(&MUS_bunny),
    MusicInfo::new(&MUS_victor),
    MusicInfo::new(&MUS_introa),
    MusicInfo::new(&MUS_runnin),
    MusicInfo::new(&MUS_stalks),
    MusicInfo::new(&MUS_countd),
    MusicInfo::new(&MUS_betwee),
    MusicInfo::new(&MUS_doom),
    MusicInfo::new(&MUS_the_da),
    MusicInfo::new(&MUS_shawn),
    MusicInfo::new(&MUS_ddtblu),
    MusicInfo::new(&MUS_in_cit),
    MusicInfo::new(&MUS_dead),
    MusicInfo::new(&MUS_stlks2),
    MusicInfo::new(&MUS_theda2),
    MusicInfo::new(&MUS_doom2),
    MusicInfo::new(&MUS_ddtbl2),
    MusicInfo::new(&MUS_runni2),
    MusicInfo::new(&MUS_dead2),
    MusicInfo::new(&MUS_stlks3),
    MusicInfo::new(&MUS_romero),
    MusicInfo::new(&MUS_shawn2),
    MusicInfo::new(&MUS_messag),
    MusicInfo::new(&MUS_count2),
    MusicInfo::new(&MUS_ddtbl3),
    MusicInfo::new(&MUS_ampie),
    MusicInfo::new(&MUS_theda3),
    MusicInfo::new(&MUS_adrian),
    MusicInfo::new(&MUS_messg2),
    MusicInfo::new(&MUS_romer2),
    MusicInfo::new(&MUS_tense),
    MusicInfo::new(&MUS_shawn3),
    MusicInfo::new(&MUS_openin),
    MusicInfo::new(&MUS_evil),
    MusicInfo::new(&MUS_ultima),
    MusicInfo::new(&MUS_read_m),
    MusicInfo::new(&MUS_dm2ttl),
    MusicInfo::new(&MUS_dm2int),
];

#[no_mangle]
pub extern "C" fn S_InitSfxLinks() {
    unsafe {
        S_sfx[Sfx::Chgun as usize].link = &mut S_sfx[Sfx::Pistol as usize] as *mut SfxInfo;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numsfx_matches_c_source() {
        assert_eq!(NUMSFX, 109);
    }

    #[test]
    fn nummusic_matches_c_source() {
        assert_eq!(NUMMUSIC, 68);
    }

    #[test]
    fn sfx_table_has_correct_length() {
        // The static array is sized by NUMSFX; this confirms no off-by-one.
        unsafe {
            assert_eq!(S_sfx.len(), NUMSFX);
        }
    }

    #[test]
    fn music_table_has_correct_length() {
        unsafe {
            assert_eq!(S_music.len(), NUMMUSIC);
        }
    }

    /// Entry 0 is the "none" sentinel – priority and numchannels must be 0 / -1.
    #[test]
    fn sfx_entry_zero_is_none_sentinel() {
        unsafe {
            assert_eq!(S_sfx[0].priority, 0);
            assert_eq!(S_sfx[0].numchannels, -1);
            assert_eq!(S_sfx[0].lumpnum, 0);
            // tagname and link are null initially
            assert!(S_sfx[0].tagname.is_null());
            assert!(S_sfx[0].link.is_null());
        }
    }

    /// sfx_pistol is entry 1 with priority 64.
    #[test]
    fn sfx_pistol_is_entry_1_with_priority_64() {
        unsafe {
            let entry = &S_sfx[Sfx::Pistol as usize];
            assert_eq!(entry.priority, 64);
            // name must start with "pistol"
            let name_bytes: Vec<u8> = entry.name.iter().map(|&c| c as u8).collect();
            assert!(
                name_bytes.starts_with(b"pistol"),
                "name should start with 'pistol'"
            );
        }
    }

    /// sfx_chgun (entry 86) must link to sfx_pistol after S_InitSfxLinks.
    #[test]
    fn sfx_chgun_links_to_pistol_after_init() {
        unsafe {
            S_InitSfxLinks();
            let pistol_ptr = &S_sfx[Sfx::Pistol as usize] as *const SfxInfo;
            let chgun_link = S_sfx[Sfx::Chgun as usize].link as *const SfxInfo;
            assert_eq!(
                chgun_link, pistol_ptr,
                "sfx_chgun.link must point to sfx_pistol"
            );
        }
    }

    #[test]
    fn sfx_info_size_matches_c() {
        // sizeof(sfxinfo_t) in C on 64-bit: tagname(8) + name[9](9) + pad(3)
        // + priority(4) + link(8) + pitch(4) + volume(4) + usefulness(4)
        // + lumpnum(4) + numchannels(4) + pad(4) + driver_data(8) = 64 bytes
        assert_eq!(std::mem::size_of::<SfxInfo>(), 64);
    }

    #[test]
    fn music_info_size_matches_c() {
        // sizeof(musicinfo_t) is 32 on 64-bit (four pointer-sized fields).
        assert_eq!(std::mem::size_of::<MusicInfo>(), 32);
    }
}
