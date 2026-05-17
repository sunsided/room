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
pub struct MusicInfo {
    pub name: *mut c_char,
    pub lumpnum: c_int,
    pub data: *mut c_void,
    pub handle: *mut c_void,
}

pub const NUMSFX: usize = 109;
pub const NUMMUSIC: usize = 68;

const _: () = assert!(
    std::mem::size_of::<Sfx>() == std::mem::size_of::<std::ffi::c_int>(),
    "Sfx must be the same size as c_int"
);

/// Sound effect IDs, matching the `sfxenum_t` C enum in `sounds.h`.
#[non_exhaustive]
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

unsafe impl Sync for SfxInfo {}
unsafe impl Sync for MusicInfo {}

const fn name<const N: usize>(s: &str) -> [c_char; N] {
    let b = s.as_bytes();
    debug_assert!(b.len() < N, "String is too long for the array");
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

static MUS_e1m1: [c_char; 5] = name("e1m1");
static MUS_e1m2: [c_char; 5] = name("e1m2");
static MUS_e1m3: [c_char; 5] = name("e1m3");
static MUS_e1m4: [c_char; 5] = name("e1m4");
static MUS_e1m5: [c_char; 5] = name("e1m5");
static MUS_e1m6: [c_char; 5] = name("e1m6");
static MUS_e1m7: [c_char; 5] = name("e1m7");
static MUS_e1m8: [c_char; 5] = name("e1m8");
static MUS_e1m9: [c_char; 5] = name("e1m9");
static MUS_e2m1: [c_char; 5] = name("e2m1");
static MUS_e2m2: [c_char; 5] = name("e2m2");
static MUS_e2m3: [c_char; 5] = name("e2m3");
static MUS_e2m4: [c_char; 5] = name("e2m4");
static MUS_e2m5: [c_char; 5] = name("e2m5");
static MUS_e2m6: [c_char; 5] = name("e2m6");
static MUS_e2m7: [c_char; 5] = name("e2m7");
static MUS_e2m8: [c_char; 5] = name("e2m8");
static MUS_e2m9: [c_char; 5] = name("e2m9");
static MUS_e3m1: [c_char; 5] = name("e3m1");
static MUS_e3m2: [c_char; 5] = name("e3m2");
static MUS_e3m3: [c_char; 5] = name("e3m3");
static MUS_e3m4: [c_char; 5] = name("e3m4");
static MUS_e3m5: [c_char; 5] = name("e3m5");
static MUS_e3m6: [c_char; 5] = name("e3m6");
static MUS_e3m7: [c_char; 5] = name("e3m7");
static MUS_e3m8: [c_char; 5] = name("e3m8");
static MUS_e3m9: [c_char; 5] = name("e3m9");
static MUS_inter: [c_char; 6] = name("inter");
static MUS_intro: [c_char; 6] = name("intro");
static MUS_bunny: [c_char; 6] = name("bunny");
static MUS_victor: [c_char; 7] = name("victor");
static MUS_introa: [c_char; 7] = name("introa");
static MUS_runnin: [c_char; 7] = name("runnin");
static MUS_stalks: [c_char; 7] = name("stalks");
static MUS_countd: [c_char; 7] = name("countd");
static MUS_betwee: [c_char; 7] = name("betwee");
static MUS_doom: [c_char; 5] = name("doom");
static MUS_the_da: [c_char; 7] = name("the_da");
static MUS_shawn: [c_char; 6] = name("shawn");
static MUS_ddtblu: [c_char; 7] = name("ddtblu");
static MUS_in_cit: [c_char; 7] = name("in_cit");
static MUS_dead: [c_char; 5] = name("dead");
static MUS_stlks2: [c_char; 7] = name("stlks2");
static MUS_theda2: [c_char; 7] = name("theda2");
static MUS_doom2: [c_char; 6] = name("doom2");
static MUS_ddtbl2: [c_char; 7] = name("ddtbl2");
static MUS_runni2: [c_char; 7] = name("runni2");
static MUS_dead2: [c_char; 6] = name("dead2");
static MUS_stlks3: [c_char; 7] = name("stlks3");
static MUS_romero: [c_char; 7] = name("romero");
static MUS_shawn2: [c_char; 7] = name("shawn2");
static MUS_messag: [c_char; 7] = name("messag");
static MUS_count2: [c_char; 7] = name("count2");
static MUS_ddtbl3: [c_char; 7] = name("ddtbl3");
static MUS_ampie: [c_char; 6] = name("ampie");
static MUS_theda3: [c_char; 7] = name("theda3");
static MUS_adrian: [c_char; 7] = name("adrian");
static MUS_messg2: [c_char; 7] = name("messg2");
static MUS_romer2: [c_char; 7] = name("romer2");
static MUS_tense: [c_char; 6] = name("tense");
static MUS_shawn3: [c_char; 7] = name("shawn3");
static MUS_openin: [c_char; 7] = name("openin");
static MUS_evil: [c_char; 5] = name("evil");
static MUS_ultima: [c_char; 7] = name("ultima");
static MUS_read_m: [c_char; 7] = name("read_m");
static MUS_dm2ttl: [c_char; 7] = name("dm2ttl");
static MUS_dm2int: [c_char; 7] = name("dm2int");

#[no_mangle]
pub static mut S_sfx: [SfxInfo; NUMSFX] = [
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_none,
        priority: 0,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_pistol,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_shotgn,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sgcock,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_dshtgn,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_dbopn,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_dbcls,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_dbload,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_plasma,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bfg,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sawup,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sawidl,
        priority: 118,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sawful,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sawhit,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_rlaunc,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_rxplod,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_firsht,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_firxpl,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_pstart,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_pstop,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_doropn,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_dorcls,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_stnmov,
        priority: 119,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_swtchn,
        priority: 78,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_swtchx,
        priority: 78,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_plpain,
        priority: 96,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_dmpain,
        priority: 96,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_popain,
        priority: 96,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_vipain,
        priority: 96,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_mnpain,
        priority: 96,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_pepain,
        priority: 96,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_slop,
        priority: 78,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_itemup,
        priority: 78,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_wpnup,
        priority: 78,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_oof,
        priority: 96,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_telept,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_posit1,
        priority: 98,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_posit2,
        priority: 98,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_posit3,
        priority: 98,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bgsit1,
        priority: 98,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bgsit2,
        priority: 98,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sgtsit,
        priority: 98,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_cacsit,
        priority: 98,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_brssit,
        priority: 94,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_cybsit,
        priority: 92,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_spisit,
        priority: 90,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bspit,
        priority: 90,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_kntsit,
        priority: 90,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_vilsit,
        priority: 90,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_mansit,
        priority: 90,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_pesit,
        priority: 90,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sklatk,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sgtatk,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_skepch,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_vilatk,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_claw,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_skeswg,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_pldeth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_pdiehi,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_podth1,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_podth2,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_podth3,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bgdth1,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bgdth2,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sgtdth,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_cacdth,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_skldth,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_brsdth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_cybdth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_spidth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bspdth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_vildth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_kntdth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_pedth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_skedth,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_posact,
        priority: 120,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bgact,
        priority: 120,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_dmact,
        priority: 120,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bspact,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bspwlk,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_vilact,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_noway,
        priority: 78,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_barexp,
        priority: 60,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_punch,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_hoof,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_metal,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_chgun,
        priority: 64,
        link: std::ptr::null_mut(),
        pitch: 150,
        volume: 0,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_tink,
        priority: 60,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bdopn,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bdcls,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_itmbk,
        priority: 100,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_flame,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_flamst,
        priority: 32,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_getpow,
        priority: 60,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bospit,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_boscub,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bossit,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bospn,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_bosdth,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_manatk,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_mandth,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_sssit,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_ssdth,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_keenpn,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_keendt,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_skeact,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_skesit,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_skeatk,
        priority: 70,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
    SfxInfo {
        tagname: std::ptr::null_mut(),
        name: N_radio,
        priority: 60,
        link: std::ptr::null_mut(),
        pitch: -1,
        volume: -1,
        usefulness: 0,
        lumpnum: 0,
        numchannels: -1,
        driver_data: std::ptr::null_mut(),
    },
];

const fn pe1m1() -> *mut c_char {
    MUS_e1m1.as_ptr() as *mut c_char
}
const fn pe1m2() -> *mut c_char {
    MUS_e1m2.as_ptr() as *mut c_char
}
const fn pe1m3() -> *mut c_char {
    MUS_e1m3.as_ptr() as *mut c_char
}
const fn pe1m4() -> *mut c_char {
    MUS_e1m4.as_ptr() as *mut c_char
}
const fn pe1m5() -> *mut c_char {
    MUS_e1m5.as_ptr() as *mut c_char
}
const fn pe1m6() -> *mut c_char {
    MUS_e1m6.as_ptr() as *mut c_char
}
const fn pe1m7() -> *mut c_char {
    MUS_e1m7.as_ptr() as *mut c_char
}
const fn pe1m8() -> *mut c_char {
    MUS_e1m8.as_ptr() as *mut c_char
}
const fn pe1m9() -> *mut c_char {
    MUS_e1m9.as_ptr() as *mut c_char
}
const fn pe2m1() -> *mut c_char {
    MUS_e2m1.as_ptr() as *mut c_char
}
const fn pe2m2() -> *mut c_char {
    MUS_e2m2.as_ptr() as *mut c_char
}
const fn pe2m3() -> *mut c_char {
    MUS_e2m3.as_ptr() as *mut c_char
}
const fn pe2m4() -> *mut c_char {
    MUS_e2m4.as_ptr() as *mut c_char
}
const fn pe2m5() -> *mut c_char {
    MUS_e2m5.as_ptr() as *mut c_char
}
const fn pe2m6() -> *mut c_char {
    MUS_e2m6.as_ptr() as *mut c_char
}
const fn pe2m7() -> *mut c_char {
    MUS_e2m7.as_ptr() as *mut c_char
}
const fn pe2m8() -> *mut c_char {
    MUS_e2m8.as_ptr() as *mut c_char
}
const fn pe2m9() -> *mut c_char {
    MUS_e2m9.as_ptr() as *mut c_char
}
const fn pe3m1() -> *mut c_char {
    MUS_e3m1.as_ptr() as *mut c_char
}
const fn pe3m2() -> *mut c_char {
    MUS_e3m2.as_ptr() as *mut c_char
}
const fn pe3m3() -> *mut c_char {
    MUS_e3m3.as_ptr() as *mut c_char
}
const fn pe3m4() -> *mut c_char {
    MUS_e3m4.as_ptr() as *mut c_char
}
const fn pe3m5() -> *mut c_char {
    MUS_e3m5.as_ptr() as *mut c_char
}
const fn pe3m6() -> *mut c_char {
    MUS_e3m6.as_ptr() as *mut c_char
}
const fn pe3m7() -> *mut c_char {
    MUS_e3m7.as_ptr() as *mut c_char
}
const fn pe3m8() -> *mut c_char {
    MUS_e3m8.as_ptr() as *mut c_char
}
const fn pe3m9() -> *mut c_char {
    MUS_e3m9.as_ptr() as *mut c_char
}
const fn pinter() -> *mut c_char {
    MUS_inter.as_ptr() as *mut c_char
}
const fn pintro() -> *mut c_char {
    MUS_intro.as_ptr() as *mut c_char
}
const fn pbunny() -> *mut c_char {
    MUS_bunny.as_ptr() as *mut c_char
}
const fn pvictor() -> *mut c_char {
    MUS_victor.as_ptr() as *mut c_char
}
const fn pintroa() -> *mut c_char {
    MUS_introa.as_ptr() as *mut c_char
}
const fn prunnin() -> *mut c_char {
    MUS_runnin.as_ptr() as *mut c_char
}
const fn pstalks() -> *mut c_char {
    MUS_stalks.as_ptr() as *mut c_char
}
const fn pcountd() -> *mut c_char {
    MUS_countd.as_ptr() as *mut c_char
}
const fn pbetwee() -> *mut c_char {
    MUS_betwee.as_ptr() as *mut c_char
}
const fn pdoom() -> *mut c_char {
    MUS_doom.as_ptr() as *mut c_char
}
const fn pthe_da() -> *mut c_char {
    MUS_the_da.as_ptr() as *mut c_char
}
const fn pshawn() -> *mut c_char {
    MUS_shawn.as_ptr() as *mut c_char
}
const fn pddtblu() -> *mut c_char {
    MUS_ddtblu.as_ptr() as *mut c_char
}
const fn pin_cit() -> *mut c_char {
    MUS_in_cit.as_ptr() as *mut c_char
}
const fn pdead() -> *mut c_char {
    MUS_dead.as_ptr() as *mut c_char
}
const fn pstlks2() -> *mut c_char {
    MUS_stlks2.as_ptr() as *mut c_char
}
const fn ptheda2() -> *mut c_char {
    MUS_theda2.as_ptr() as *mut c_char
}
const fn pdoom2() -> *mut c_char {
    MUS_doom2.as_ptr() as *mut c_char
}
const fn pddtbl2() -> *mut c_char {
    MUS_ddtbl2.as_ptr() as *mut c_char
}
const fn prunni2() -> *mut c_char {
    MUS_runni2.as_ptr() as *mut c_char
}
const fn pdead2() -> *mut c_char {
    MUS_dead2.as_ptr() as *mut c_char
}
const fn pstlks3() -> *mut c_char {
    MUS_stlks3.as_ptr() as *mut c_char
}
const fn promero() -> *mut c_char {
    MUS_romero.as_ptr() as *mut c_char
}
const fn pshawn2() -> *mut c_char {
    MUS_shawn2.as_ptr() as *mut c_char
}
const fn pmessag() -> *mut c_char {
    MUS_messag.as_ptr() as *mut c_char
}
const fn pcount2() -> *mut c_char {
    MUS_count2.as_ptr() as *mut c_char
}
const fn pddtbl3() -> *mut c_char {
    MUS_ddtbl3.as_ptr() as *mut c_char
}
const fn pampie() -> *mut c_char {
    MUS_ampie.as_ptr() as *mut c_char
}
const fn ptheda3() -> *mut c_char {
    MUS_theda3.as_ptr() as *mut c_char
}
const fn padrian() -> *mut c_char {
    MUS_adrian.as_ptr() as *mut c_char
}
const fn pmessg2() -> *mut c_char {
    MUS_messg2.as_ptr() as *mut c_char
}
const fn promer2() -> *mut c_char {
    MUS_romer2.as_ptr() as *mut c_char
}
const fn ptense() -> *mut c_char {
    MUS_tense.as_ptr() as *mut c_char
}
const fn pshawn3() -> *mut c_char {
    MUS_shawn3.as_ptr() as *mut c_char
}
const fn popenin() -> *mut c_char {
    MUS_openin.as_ptr() as *mut c_char
}
const fn pev() -> *mut c_char {
    MUS_evil.as_ptr() as *mut c_char
}
const fn pultima() -> *mut c_char {
    MUS_ultima.as_ptr() as *mut c_char
}
const fn pread_m() -> *mut c_char {
    MUS_read_m.as_ptr() as *mut c_char
}
const fn pdm2ttl() -> *mut c_char {
    MUS_dm2ttl.as_ptr() as *mut c_char
}
const fn pdm2int() -> *mut c_char {
    MUS_dm2int.as_ptr() as *mut c_char
}

#[no_mangle]
pub static mut S_music: [MusicInfo; NUMMUSIC] = [
    MusicInfo {
        name: std::ptr::null_mut(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m1(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m3(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m4(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m5(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m6(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m7(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m8(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe1m9(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m1(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m3(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m4(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m5(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m6(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m7(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m8(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe2m9(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m1(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m3(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m4(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m5(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m6(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m7(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m8(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pe3m9(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pinter(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pintro(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pbunny(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pvictor(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pintroa(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: prunnin(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pstalks(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pcountd(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pbetwee(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pdoom(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pthe_da(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pshawn(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pddtblu(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pin_cit(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pdead(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pstlks2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: ptheda2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pdoom2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pddtbl2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: prunni2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pdead2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pstlks3(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: promero(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pshawn2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pmessag(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pcount2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pddtbl3(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pampie(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: ptheda3(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: padrian(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pmessg2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: promer2(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: ptense(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pshawn3(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: popenin(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pev(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pultima(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pread_m(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pdm2ttl(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
    MusicInfo {
        name: pdm2int(),
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: std::ptr::null_mut(),
    },
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
    use std::ffi::CStr;

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
