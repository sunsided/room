//! Rust port of vendor/doomgeneric/st_stuff.c.
//!
//! Status bar logic: health, ammo, armor, keys, face widget, palette effects,
//! and cheat-code handling.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int};
use std::ptr;

use crate::c_write;
use crate::doom::am_map::automapactive;
use crate::doom::d_event::event_t;
use crate::doom::d_items::weaponinfo;
use crate::doom::d_mode;
use crate::doom::d_player::{PlayerT, MAXPLAYERS, NUMAMMO, NUMCARDS, NUMWEAPONS};
use crate::doom::doomstat::{gamemission, gamemode, gameversion};
use crate::doom::g_game::G_DeferedInitNew;
use crate::doom::g_game::{consoleplayer, deathmatch, gameskill, netgame, players};
use crate::doom::i_timer::TICRATE;
use crate::doom::i_video::I_SetPalette;
use crate::doom::m_cheat::{cheatseq_t, cht_CheckCheat, cht_GetParam};
use crate::doom::m_random::M_Random;
use crate::doom::p_inter::P_GivePower;
use crate::doom::p_telept::mobj_t;
use crate::doom::r_main::R_PointToAngle2;
use crate::doom::s_sound::S_ChangeMusic;
use crate::doom::st_lib::{
    st_binicon_t, st_multicon_t, st_number_t, st_percent_t, STlib_init, STlib_initBinIcon,
    STlib_initMultIcon, STlib_initNum, STlib_initPercent, STlib_updateBinIcon,
    STlib_updateMultIcon, STlib_updateNum, STlib_updatePercent,
};
use crate::doom::tables::{ANG180, ANG45};
use crate::doom::v_video::patch_t;
use crate::doom::v_video::{V_CopyRect, V_DrawPatch, V_RestoreBuffer, V_UseBuffer};
use crate::doom::w_wad::{W_CacheLumpName, W_CacheLumpNum, W_GetNumForName, W_ReleaseLumpName};
use crate::doom::z_zone::{Z_Malloc, PU_CACHE, PU_STATIC};
use crate::types::Boolean;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const ST_HEIGHT: c_int = 32;
const ST_WIDTH: c_int = 320;
const ST_X: c_int = 0;
const ST_Y: c_int = 200 - ST_HEIGHT;

const ST_FX: c_int = 143;
const ST_FY: c_int = 169;

const ST_NUMPAINFACES: c_int = 5;
const ST_NUMSTRAIGHTFACES: c_int = 3;
const ST_NUMTURNFACES: c_int = 2;
const ST_NUMSPECIALFACES: c_int = 3;

const ST_FACESTRIDE: c_int = ST_NUMSTRAIGHTFACES + ST_NUMTURNFACES + ST_NUMSPECIALFACES;

const ST_NUMEXTRAFACES: c_int = 2;
const ST_NUMFACES: c_int = ST_FACESTRIDE * ST_NUMPAINFACES + ST_NUMEXTRAFACES;

const ST_TURNOFFSET: c_int = ST_NUMSTRAIGHTFACES;
const ST_OUCHOFFSET: c_int = ST_TURNOFFSET + ST_NUMTURNFACES;
const ST_EVILGRINOFFSET: c_int = ST_OUCHOFFSET + 1;
const ST_RAMPAGEOFFSET: c_int = ST_EVILGRINOFFSET + 1;
const ST_GODFACE: c_int = ST_NUMPAINFACES * ST_FACESTRIDE;
const ST_DEADFACE: c_int = ST_GODFACE + 1;

const ST_FACESX: c_int = 143;
const ST_FACESY: c_int = 168;

const ST_EVILGRINCOUNT: c_int = 2 * TICRATE;
const ST_STRAIGHTFACECOUNT: c_int = TICRATE / 2;
const ST_TURNCOUNT: c_int = TICRATE;
const ST_OUCHCOUNT: c_int = TICRATE;
const ST_RAMPAGEDELAY: c_int = 2 * TICRATE;

const ST_MUCHPAIN: c_int = 20;

const ST_AMMOWIDTH: c_int = 3;
const ST_AMMOX: c_int = 44;
const ST_AMMOY: c_int = 171;

const ST_HEALTHWIDTH: c_int = 3;
const ST_HEALTHX: c_int = 90;
const ST_HEALTHY: c_int = 171;

const ST_ARMSX: c_int = 111;
const ST_ARMSY: c_int = 172;
const ST_ARMSBGX: c_int = 104;
const ST_ARMSBGY: c_int = 168;
const ST_ARMSXSPACE: c_int = 12;
const ST_ARMSYSPACE: c_int = 10;

const ST_FRAGSX: c_int = 138;
const ST_FRAGSY: c_int = 171;
const ST_FRAGSWIDTH: c_int = 2;

const ST_ARMORWIDTH: c_int = 3;
const ST_ARMORX: c_int = 221;
const ST_ARMORY: c_int = 171;

const ST_KEY0X: c_int = 239;
const ST_KEY0Y: c_int = 171;
const ST_KEY1X: c_int = 239;
const ST_KEY1Y: c_int = 181;
const ST_KEY2X: c_int = 239;
const ST_KEY2Y: c_int = 191;

const ST_AMMO0WIDTH: c_int = 3;
const ST_AMMO0X: c_int = 288;
const ST_AMMO0Y: c_int = 173;
const ST_AMMO1X: c_int = 288;
const ST_AMMO1Y: c_int = 179;
const ST_AMMO2X: c_int = 288;
const ST_AMMO2Y: c_int = 191;
const ST_AMMO3X: c_int = 288;
const ST_AMMO3Y: c_int = 185;

const ST_MAXAMMO0WIDTH: c_int = 3;
const ST_MAXAMMO0X: c_int = 314;
const ST_MAXAMMO0Y: c_int = 173;
const ST_MAXAMMO1X: c_int = 314;
const ST_MAXAMMO1Y: c_int = 179;
const ST_MAXAMMO2X: c_int = 314;
const ST_MAXAMMO2Y: c_int = 191;
const ST_MAXAMMO3X: c_int = 314;
const ST_MAXAMMO3Y: c_int = 185;

const AM_MSGHEADER: c_int = (('a' as c_int) << 24) + (('m' as c_int) << 16);
const AM_MSGENTERED: c_int = AM_MSGHEADER | (('e' as c_int) << 8);
const AM_MSGEXITED: c_int = AM_MSGHEADER | (('x' as c_int) << 8);

const STARTREDPALS: c_int = 1;
const STARTBONUSPALS: c_int = 9;
const NUMREDPALS: c_int = 8;
const NUMBONUSPALS: c_int = 4;
const RADIATIONPAL: c_int = 13;

// ---------------------------------------------------------------------------
// String literals (from d_englsh.h)
// ---------------------------------------------------------------------------

const STSTR_DQDON: *mut c_char = c"Degreelessness Mode On".as_ptr().cast_mut();
const STSTR_DQDOFF: *mut c_char = c"Degreelessness Mode Off".as_ptr().cast_mut();
const STSTR_FAADDED: *mut c_char = c"Ammo (no keys) Added".as_ptr().cast_mut();
const STSTR_KFAADDED: *mut c_char = c"Very Happy Ammo Added".as_ptr().cast_mut();
const STSTR_MUS: *mut c_char = c"Music Change".as_ptr().cast_mut();
const STSTR_NOMUS: *mut c_char = c"IMPOSSIBLE SELECTION".as_ptr().cast_mut();
const STSTR_NCON: *mut c_char = c"No Clipping Mode ON".as_ptr().cast_mut();
const STSTR_NCOFF: *mut c_char = c"No Clipping Mode OFF".as_ptr().cast_mut();
const STSTR_BEHOLD: *mut c_char = c"inVuln, Str, Inviso, Rad, Allmap, or Lite-amp"
    .as_ptr()
    .cast_mut();
const STSTR_BEHOLDX: *mut c_char = c"Power-up Toggled".as_ptr().cast_mut();
const STSTR_CHOPPERS: *mut c_char = c"... doesn't suck - GM".as_ptr().cast_mut();
const STSTR_CLEV: *mut c_char = c"Changing Level...".as_ptr().cast_mut();

// ---------------------------------------------------------------------------
// DEH_String shim — identity when dehacked is disabled.
// ---------------------------------------------------------------------------

#[inline(always)]
unsafe fn DEH_String(s: *mut c_char) -> *mut c_char {
    s
}

// ---------------------------------------------------------------------------
// Replicate the C `logical_gamemission` macro from `doomstat.h`.
// ---------------------------------------------------------------------------

unsafe fn logical_gamemission() -> c_int {
    if gamemission == d_mode::pack_chex {
        d_mode::doom
    } else if gamemission == d_mode::pack_hacx {
        d_mode::doom2
    } else {
        gamemission
    }
}

// ---------------------------------------------------------------------------
// Cheat helpers
// ---------------------------------------------------------------------------

const fn make_cheat_seq(seq: &[u8]) -> [c_char; 25] {
    let mut arr = [0i8; 25];
    let mut i = 0;
    while i < seq.len() {
        arr[i] = seq[i] as c_char;
        i += 1;
    }
    arr
}

const fn cheat(seq: &[u8], params: c_int) -> cheatseq_t {
    cheatseq_t {
        sequence: make_cheat_seq(seq),
        sequence_len: seq.len(),
        parameter_chars: params,
        chars_read: 0,
        param_chars_read: 0,
        parameter_buf: [0; 5],
    }
}

// ---------------------------------------------------------------------------
// Exported globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut st_backing_screen: *mut u8 = ptr::null_mut();

#[no_mangle]
pub static mut cheat_mus: cheatseq_t = cheat(b"idmus", 2);
#[no_mangle]
pub static mut cheat_god: cheatseq_t = cheat(b"iddqd", 0);
#[no_mangle]
pub static mut cheat_ammo: cheatseq_t = cheat(b"idkfa", 0);
#[no_mangle]
pub static mut cheat_ammonokey: cheatseq_t = cheat(b"idfa", 0);
#[no_mangle]
pub static mut cheat_noclip: cheatseq_t = cheat(b"idspispopd", 0);
#[no_mangle]
pub static mut cheat_commercial_noclip: cheatseq_t = cheat(b"idclip", 0);
#[no_mangle]
pub static mut cheat_powerup: [cheatseq_t; 7] = [
    cheat(b"idbeholdv", 0),
    cheat(b"idbeholds", 0),
    cheat(b"idbeholdi", 0),
    cheat(b"idbeholdr", 0),
    cheat(b"idbeholda", 0),
    cheat(b"idbeholdl", 0),
    cheat(b"idbehold", 0),
];
#[no_mangle]
pub static mut cheat_choppers: cheatseq_t = cheat(b"idchoppers", 0);
#[no_mangle]
pub static mut cheat_clev: cheatseq_t = cheat(b"idclev", 2);
#[no_mangle]
pub static mut cheat_mypos: cheatseq_t = cheat(b"idmypos", 0);

// ---------------------------------------------------------------------------
// Internal state
// ---------------------------------------------------------------------------

static mut plyr: *mut PlayerT = ptr::null_mut();
static mut st_firsttime: c_int = 0;
static mut lu_palette: c_int = 0;
static mut st_clock: u32 = 0;
static mut st_msgcounter: c_int = 0;
static mut st_chatstate: c_int = 0; // StartChatState = 0
static mut st_gamestate: c_int = 0; // AutomapState = 0
static mut st_statusbaron: c_int = 0;
static mut st_chat: c_int = 0;
static mut st_oldchat: c_int = 0;
static mut st_cursoron: c_int = 0;
static mut st_notdeathmatch: c_int = 0;
static mut st_armson: c_int = 0;
static mut st_fragson: c_int = 0;

static mut sbar: *mut patch_t = ptr::null_mut();
static mut tallnum: [*mut patch_t; 10] = [ptr::null_mut(); 10];
static mut tallpercent: *mut patch_t = ptr::null_mut();
static mut shortnum: [*mut patch_t; 10] = [ptr::null_mut(); 10];
static mut keys: [*mut patch_t; NUMCARDS] = [ptr::null_mut(); NUMCARDS];
static mut faces: [*mut patch_t; ST_NUMFACES as usize] = [ptr::null_mut(); ST_NUMFACES as usize];
static mut faceback: *mut patch_t = ptr::null_mut();
static mut armsbg: *mut patch_t = ptr::null_mut();
static mut arms: [[*mut patch_t; 2]; 6] = [[ptr::null_mut(); 2]; 6];

static mut w_ready: st_number_t = unsafe { std::mem::zeroed() };
static mut w_frags: st_number_t = unsafe { std::mem::zeroed() };
static mut w_health: st_percent_t = unsafe { std::mem::zeroed() };
static mut w_armsbg: st_binicon_t = unsafe { std::mem::zeroed() };
static mut w_arms: [st_multicon_t; 6] = unsafe { std::mem::zeroed() };
static mut w_faces: st_multicon_t = unsafe { std::mem::zeroed() };
static mut w_keyboxes: [st_multicon_t; 3] = unsafe { std::mem::zeroed() };
static mut w_armor: st_percent_t = unsafe { std::mem::zeroed() };
static mut w_ammo: [st_number_t; NUMAMMO] = unsafe { std::mem::zeroed() };
static mut w_maxammo: [st_number_t; NUMAMMO] = unsafe { std::mem::zeroed() };

static mut st_fragscount: c_int = 0;
static mut st_oldhealth: c_int = -1;
static mut oldweaponsowned: [c_int; NUMWEAPONS] = [0; NUMWEAPONS];
static mut st_facecount: c_int = 0;
static mut st_faceindex: c_int = 0;
static mut keyboxes: [c_int; 3] = [0; 3];
static mut st_randomnumber: c_int = 0;

static mut st_palette: c_int = 0;
static mut st_stopped: c_int = 1;

// ---------------------------------------------------------------------------
// Background refresh
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ST_refreshBackground() {
    if st_statusbaron != 0 {
        V_UseBuffer(st_backing_screen);
        V_DrawPatch(ST_X, 0, sbar);
        if netgame != 0 {
            V_DrawPatch(ST_FX, 0, faceback);
        }
        V_RestoreBuffer();
        V_CopyRect(ST_X, 0, st_backing_screen, ST_WIDTH, ST_HEIGHT, ST_X, ST_Y);
    }
}

// ---------------------------------------------------------------------------
// Cheat responder
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ST_Responder(ev: *mut event_t) -> c_int {
    let ev = &*ev;

    if ev.type_ == 1 && (ev.data1 as u32 & 0xffff0000) == AM_MSGHEADER as u32 {
        // ev_keyup + automap message
        match ev.data1 {
            AM_MSGENTERED => {
                st_gamestate = 0; // AutomapState
                st_firsttime = 1;
            }
            AM_MSGEXITED => {
                st_gamestate = 1; // FirstPersonState
            }
            _ => {}
        }
    } else if ev.type_ == 0 {
        // ev_keydown
        if netgame == 0 && gameskill != 4 {
            // sk_nightmare = 4
            if cht_CheckCheat(&raw mut cheat_god, ev.data2 as c_char) != 0 {
                (*plyr).cheats ^= 2; // CF_GODMODE
                if (*plyr).cheats & 2 != 0 {
                    if !(*plyr).mo.is_null() {
                        (*((*plyr).mo as *mut mobj_t)).health = 100;
                    }
                    (*plyr).health = 100; // deh_god_mode_health
                    (*plyr).message = DEH_String(STSTR_DQDON);
                } else {
                    (*plyr).message = DEH_String(STSTR_DQDOFF);
                }
            } else if cht_CheckCheat(&raw mut cheat_ammonokey, ev.data2 as c_char) != 0 {
                (*plyr).armorpoints = 200; // deh_idfa_armor
                (*plyr).armortype = 2; // deh_idfa_armor_class
                for i in 0..NUMWEAPONS {
                    (*plyr).weaponowned[i] = 1;
                }
                for i in 0..NUMAMMO {
                    (*plyr).ammo[i] = (*plyr).maxammo[i];
                }
                (*plyr).message = DEH_String(STSTR_FAADDED);
            } else if cht_CheckCheat(&raw mut cheat_ammo, ev.data2 as c_char) != 0 {
                (*plyr).armorpoints = 200; // deh_idkfa_armor
                (*plyr).armortype = 2; // deh_idkfa_armor_class
                for i in 0..NUMWEAPONS {
                    (*plyr).weaponowned[i] = 1;
                }
                for i in 0..NUMAMMO {
                    (*plyr).ammo[i] = (*plyr).maxammo[i];
                }
                for i in 0..NUMCARDS {
                    (*plyr).cards[i] = 1;
                }
                (*plyr).message = DEH_String(STSTR_KFAADDED);
            } else if cht_CheckCheat(&raw mut cheat_mus, ev.data2 as c_char) != 0 {
                let mut buf = [0i8; 3];
                let musnum: c_int;
                (*plyr).message = DEH_String(STSTR_MUS);
                cht_GetParam(&raw mut cheat_mus, buf.as_mut_ptr());

                if gamemode == d_mode::commercial || gameversion < d_mode::exe_ultimate {
                    musnum = 33
                        + (buf[0] as c_int - '0' as c_int) * 10
                        + (buf[1] as c_int - '0' as c_int)
                        - 1;
                    if ((buf[0] as c_int - '0' as c_int) * 10 + (buf[1] as c_int - '0' as c_int))
                        > 35
                    {
                        (*plyr).message = DEH_String(STSTR_NOMUS);
                    } else {
                        S_ChangeMusic(musnum, 1);
                    }
                } else {
                    musnum =
                        1 + (buf[0] as c_int - '1' as c_int) * 9 + (buf[1] as c_int - '1' as c_int);
                    if ((buf[0] as c_int - '1' as c_int) * 9 + (buf[1] as c_int - '1' as c_int))
                        > 31
                    {
                        (*plyr).message = DEH_String(STSTR_NOMUS);
                    } else {
                        S_ChangeMusic(musnum, 1);
                    }
                }
            } else if (logical_gamemission() == d_mode::doom
                && cht_CheckCheat(&raw mut cheat_noclip, ev.data2 as c_char) != 0)
                || (logical_gamemission() != d_mode::doom
                    && cht_CheckCheat(&raw mut cheat_commercial_noclip, ev.data2 as c_char) != 0)
            {
                (*plyr).cheats ^= 1; // CF_NOCLIP
                if (*plyr).cheats & 1 != 0 {
                    (*plyr).message = DEH_String(STSTR_NCON);
                } else {
                    (*plyr).message = DEH_String(STSTR_NCOFF);
                }
            }

            for i in 0..6 {
                if cht_CheckCheat(&mut cheat_powerup[i], ev.data2 as c_char) != 0 {
                    if (*plyr).powers[i] == 0 {
                        P_GivePower(plyr, i as c_int);
                    } else if i != 1 {
                        // pw_strength = 1
                        (*plyr).powers[i] = 1;
                    } else {
                        (*plyr).powers[i] = 0;
                    }
                    (*plyr).message = DEH_String(STSTR_BEHOLDX);
                }
            }

            if cht_CheckCheat(&mut cheat_powerup[6], ev.data2 as c_char) != 0 {
                (*plyr).message = DEH_String(STSTR_BEHOLD);
            } else if cht_CheckCheat(&raw mut cheat_choppers, ev.data2 as c_char) != 0 {
                (*plyr).weaponowned[7] = 1; // wp_chainsaw
                (*plyr).powers[0] = 1; // pw_invulnerability
                (*plyr).message = DEH_String(STSTR_CHOPPERS);
            } else if cht_CheckCheat(&raw mut cheat_mypos, ev.data2 as c_char) != 0 {
                static mut BUF: [c_char; 52] = [0; 52];
                let mo = players[consoleplayer as usize].mo as *mut mobj_t;
                c_write!(
                    BUF,
                    "ang=0x{:x};x,y=(0x{:x},0x{:x})",
                    (*mo).angle,
                    (*mo).x,
                    (*mo).y
                );
                (*plyr).message = std::ptr::addr_of_mut!(BUF[0]);
            }
        }

        if netgame == 0 && cht_CheckCheat(&raw mut cheat_clev, ev.data2 as c_char) != 0 {
            let mut buf = [0i8; 3];
            let mut epsd: c_int;
            let map: c_int;
            cht_GetParam(&raw mut cheat_clev, buf.as_mut_ptr());

            if gamemode == d_mode::commercial {
                epsd = 1;
                map = (buf[0] as c_int - '0' as c_int) * 10 + (buf[1] as c_int - '0' as c_int);
            } else {
                epsd = buf[0] as c_int - '0' as c_int;
                map = buf[1] as c_int - '0' as c_int;
            }

            if gameversion == d_mode::exe_chex {
                epsd = 1;
            }

            if epsd < 1 {
                return 0;
            }
            if map < 1 {
                return 0;
            }
            if gamemode == d_mode::retail && (epsd > 4 || map > 9) {
                return 0;
            }
            if gamemode == d_mode::registered && (epsd > 3 || map > 9) {
                return 0;
            }
            if gamemode == d_mode::shareware && (epsd > 1 || map > 9) {
                return 0;
            }
            if gamemode == d_mode::commercial && (epsd > 1 || map > 40) {
                return 0;
            }

            (*plyr).message = DEH_String(STSTR_CLEV);
            G_DeferedInitNew(gameskill, epsd, map);
        }
    }

    0
}

// ---------------------------------------------------------------------------
// Face widget
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ST_calcPainOffset() -> c_int {
    static mut lastcalc: c_int = 0;
    static mut oldhealth: c_int = -1;

    let health = if (*plyr).health > 100 {
        100
    } else {
        (*plyr).health
    };

    if health != oldhealth {
        lastcalc = ST_FACESTRIDE * (((100 - health) * ST_NUMPAINFACES) / 101);
        oldhealth = health;
    }
    lastcalc
}

#[no_mangle]
pub unsafe extern "C" fn ST_updateFaceWidget() {
    static mut lastattackdown: c_int = -1;
    static mut priority: c_int = 0;
    let diffang: u32;
    let i: c_int;

    if priority < 10 && (*plyr).health == 0 {
        priority = 9;
        st_faceindex = ST_DEADFACE;
        st_facecount = 1;
    }

    if priority < 9 && (*plyr).bonuscount != 0 {
        let mut doevilgrin = 0;
        for i in 0..NUMWEAPONS {
            if oldweaponsowned[i] != (*plyr).weaponowned[i] {
                doevilgrin = 1;
                oldweaponsowned[i] = (*plyr).weaponowned[i];
            }
        }
        if doevilgrin != 0 {
            priority = 8;
            st_facecount = ST_EVILGRINCOUNT;
            st_faceindex = ST_calcPainOffset() + ST_EVILGRINOFFSET;
        }
    }

    if priority < 8
        && (*plyr).damagecount != 0
        && !(*plyr).attacker.is_null()
        && (*plyr).attacker != (*plyr).mo
    {
        priority = 7;
        if (*plyr).health - st_oldhealth > ST_MUCHPAIN {
            st_facecount = ST_TURNCOUNT;
            st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET;
        } else {
            let badguyangle = R_PointToAngle2(
                (*((*plyr).mo as *mut mobj_t)).x,
                (*((*plyr).mo as *mut mobj_t)).y,
                (*((*plyr).attacker as *mut mobj_t)).x,
                (*((*plyr).attacker as *mut mobj_t)).y,
            );
            if badguyangle > (*((*plyr).mo as *mut mobj_t)).angle {
                diffang = badguyangle - (*((*plyr).mo as *mut mobj_t)).angle;
                i = if diffang > ANG180 { 1 } else { 0 };
            } else {
                diffang = (*((*plyr).mo as *mut mobj_t)).angle - badguyangle;
                i = if diffang <= ANG180 { 1 } else { 0 };
            }

            st_facecount = ST_TURNCOUNT;
            st_faceindex = ST_calcPainOffset();

            if diffang < ANG45 {
                st_faceindex += ST_RAMPAGEOFFSET;
            } else if i != 0 {
                st_faceindex += ST_TURNOFFSET;
            } else {
                st_faceindex += ST_TURNOFFSET + 1;
            }
        }
    }

    if priority < 7 && (*plyr).damagecount != 0 {
        if (*plyr).health - st_oldhealth > ST_MUCHPAIN {
            priority = 7;
            st_facecount = ST_TURNCOUNT;
            st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET;
        } else {
            priority = 6;
            st_facecount = ST_TURNCOUNT;
            st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET;
        }
    }

    if priority < 6 {
        if (*plyr).attackdown != 0 {
            if lastattackdown == -1 {
                lastattackdown = ST_RAMPAGEDELAY;
            } else {
                lastattackdown -= 1;
                if lastattackdown == 0 {
                    priority = 5;
                    st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET;
                    st_facecount = 1;
                    lastattackdown = 1;
                }
            }
        } else {
            lastattackdown = -1;
        }
    }

    if priority < 5 && (((*plyr).cheats & 2) != 0 || (*plyr).powers[0] != 0) {
        // CF_GODMODE || pw_invulnerability
        priority = 4;
        st_faceindex = ST_GODFACE;
        st_facecount = 1;
    }

    if st_facecount == 0 {
        st_faceindex = ST_calcPainOffset() + (st_randomnumber % 3);
        st_facecount = ST_STRAIGHTFACECOUNT;
        priority = 0;
    }

    st_facecount -= 1;
}

// ---------------------------------------------------------------------------
// Widget update
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ST_updateWidgets() {
    static mut largeammo: c_int = 1994;

    if weaponinfo[(*plyr).readyweapon as usize].ammo == 5 {
        // am_noammo
        w_ready.num = &raw mut largeammo as *mut c_int;
    } else {
        w_ready.num = (*plyr)
            .ammo
            .as_mut_ptr()
            .add(weaponinfo[(*plyr).readyweapon as usize].ammo as usize);
    }
    w_ready.data = (*plyr).readyweapon;

    for i in 0..3 {
        keyboxes[i] = if (*plyr).cards[i] != 0 {
            i as c_int
        } else {
            -1
        };
        if (*plyr).cards[i + 3] != 0 {
            keyboxes[i] = (i + 3) as c_int;
        }
    }

    ST_updateFaceWidget();

    st_notdeathmatch = if deathmatch == 0 { 1 } else { 0 };
    st_armson = if st_statusbaron != 0 && deathmatch == 0 {
        1
    } else {
        0
    };
    st_fragson = if deathmatch != 0 && st_statusbaron != 0 {
        1
    } else {
        0
    };
    st_fragscount = 0;

    for i in 0..MAXPLAYERS {
        if i != consoleplayer as usize {
            st_fragscount += (*plyr).frags[i];
        } else {
            st_fragscount -= (*plyr).frags[i];
        }
    }

    st_msgcounter -= 1;
    if st_msgcounter == 0 {
        st_chat = st_oldchat;
    }
}

// ---------------------------------------------------------------------------
// Ticker
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ST_Ticker() {
    st_clock = st_clock.wrapping_add(1);
    st_randomnumber = M_Random();
    ST_updateWidgets();
    st_oldhealth = (*plyr).health;
}

// ---------------------------------------------------------------------------
// Palette effects
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ST_doPaletteStuff() {
    let mut palette: c_int;
    let mut cnt = (*plyr).damagecount;

    if (*plyr).powers[1] != 0 {
        // pw_strength
        let bzc = 12 - ((*plyr).powers[1] >> 6);
        if bzc > cnt {
            cnt = bzc;
        }
    }

    if cnt != 0 {
        palette = (cnt + 7) >> 3;
        if palette >= NUMREDPALS {
            palette = NUMREDPALS - 1;
        }
        palette += STARTREDPALS;
    } else if (*plyr).bonuscount != 0 {
        palette = ((*plyr).bonuscount + 7) >> 3;
        if palette >= NUMBONUSPALS {
            palette = NUMBONUSPALS - 1;
        }
        palette += STARTBONUSPALS;
    } else if (*plyr).powers[3] > 4 * 32 || (*plyr).powers[3] & 8 != 0 {
        // pw_ironfeet
        palette = RADIATIONPAL;
    } else {
        palette = 0;
    }

    if gameversion == d_mode::exe_chex
        && (STARTREDPALS..STARTREDPALS + NUMREDPALS).contains(&palette)
    {
        palette = RADIATIONPAL;
    }

    if palette != st_palette {
        st_palette = palette;
        let pal = (W_CacheLumpNum(lu_palette, PU_CACHE) as *mut u8).add((palette * 768) as usize);
        I_SetPalette(pal);
    }
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ST_drawWidgets(refresh: c_int) {
    st_armson = if st_statusbaron != 0 && deathmatch == 0 {
        1
    } else {
        0
    };
    st_fragson = if deathmatch != 0 && st_statusbaron != 0 {
        1
    } else {
        0
    };

    STlib_updateNum(&raw mut w_ready, refresh);

    for i in 0..NUMAMMO {
        STlib_updateNum(std::ptr::addr_of_mut!(w_ammo[0]).add(i), refresh);
        STlib_updateNum(std::ptr::addr_of_mut!(w_maxammo[0]).add(i), refresh);
    }

    STlib_updatePercent(&raw mut w_health, refresh);
    STlib_updatePercent(&raw mut w_armor, refresh);
    STlib_updateBinIcon(&raw mut w_armsbg, refresh);

    for i in 0..6 {
        STlib_updateMultIcon(std::ptr::addr_of_mut!(w_arms[0]).add(i), refresh);
    }

    STlib_updateMultIcon(&raw mut w_faces, refresh);

    for i in 0..3 {
        STlib_updateMultIcon(std::ptr::addr_of_mut!(w_keyboxes[0]).add(i), refresh);
    }

    STlib_updateNum(&raw mut w_frags, refresh);
}

#[no_mangle]
pub unsafe extern "C" fn ST_doRefresh() {
    st_firsttime = 0;
    ST_refreshBackground();
    ST_drawWidgets(1);
}

#[no_mangle]
pub unsafe extern "C" fn ST_diffDraw() {
    ST_drawWidgets(0);
}

#[no_mangle]
pub unsafe extern "C" fn ST_Drawer(fullscreen: Boolean, refresh: Boolean) {
    st_statusbaron = if fullscreen.is_false() || automapactive != 0 {
        1
    } else {
        0
    };
    st_firsttime = if st_firsttime != 0 || refresh.is_truthy() {
        1
    } else {
        0
    };
    ST_doPaletteStuff();
    if st_firsttime != 0 {
        ST_doRefresh();
    } else {
        ST_diffDraw();
    }
}

// ---------------------------------------------------------------------------
// Graphics loading / unloading
// ---------------------------------------------------------------------------

type LoadCallback = unsafe extern "C" fn(*mut c_char, *mut *mut patch_t);

unsafe fn ST_loadUnloadGraphics(callback: LoadCallback) {
    let mut namebuf = [0i8; 9];

    for i in 0..10i32 {
        c_write!(namebuf, "STTNUM{}", i);
        callback(namebuf.as_mut_ptr(), &mut tallnum[i as usize]);
        c_write!(namebuf, "STYSNUM{}", i);
        callback(namebuf.as_mut_ptr(), &mut shortnum[i as usize]);
    }

    callback(c"STTPRCNT".as_ptr().cast_mut(), &raw mut tallpercent);

    for i in 0..NUMCARDS as c_int {
        c_write!(namebuf, "STKEYS{}", i);
        callback(namebuf.as_mut_ptr(), &mut keys[i as usize]);
    }

    callback(c"STARMS".as_ptr().cast_mut(), &raw mut armsbg);

    for i in 0..6i32 {
        c_write!(namebuf, "STGNUM{}", i + 2);
        callback(namebuf.as_mut_ptr(), &mut arms[i as usize][0]);
        arms[i as usize][1] = shortnum[(i + 2) as usize];
    }

    c_write!(namebuf, "STFB{}", consoleplayer as c_int);
    callback(namebuf.as_mut_ptr(), &raw mut faceback);

    callback(c"STBAR".as_ptr().cast_mut(), &raw mut sbar);

    let mut facenum: c_int = 0;
    for i in 0..ST_NUMPAINFACES {
        for j in 0..ST_NUMSTRAIGHTFACES {
            c_write!(namebuf, "STFST{}{}", i, j);
            callback(namebuf.as_mut_ptr(), &mut faces[facenum as usize]);
            facenum += 1;
        }
        c_write!(namebuf, "STFTR{}0", i);
        callback(namebuf.as_mut_ptr(), &mut faces[facenum as usize]);
        facenum += 1;
        c_write!(namebuf, "STFTL{}0", i);
        callback(namebuf.as_mut_ptr(), &mut faces[facenum as usize]);
        facenum += 1;
        c_write!(namebuf, "STFOUCH{}", i);
        callback(namebuf.as_mut_ptr(), &mut faces[facenum as usize]);
        facenum += 1;
        c_write!(namebuf, "STFEVL{}", i);
        callback(namebuf.as_mut_ptr(), &mut faces[facenum as usize]);
        facenum += 1;
        c_write!(namebuf, "STFKILL{}", i);
        callback(namebuf.as_mut_ptr(), &mut faces[facenum as usize]);
        facenum += 1;
    }

    callback(c"STFGOD0".as_ptr().cast_mut(), &mut faces[facenum as usize]);
    facenum += 1;
    callback(
        c"STFDEAD0".as_ptr().cast_mut(),
        &mut faces[facenum as usize],
    );
}

unsafe extern "C" fn ST_loadCallback(lumpname: *mut c_char, variable: *mut *mut patch_t) {
    *variable = W_CacheLumpName(lumpname, PU_STATIC) as *mut patch_t;
}

#[no_mangle]
pub unsafe extern "C" fn ST_loadGraphics() {
    ST_loadUnloadGraphics(ST_loadCallback);
}

#[no_mangle]
pub unsafe extern "C" fn ST_loadData() {
    lu_palette = W_GetNumForName(c"PLAYPAL".as_ptr());
    ST_loadGraphics();
}

unsafe extern "C" fn ST_unloadCallback(lumpname: *mut c_char, variable: *mut *mut patch_t) {
    W_ReleaseLumpName(lumpname);
    *variable = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn ST_unloadGraphics() {
    ST_loadUnloadGraphics(ST_unloadCallback);
}

#[no_mangle]
pub unsafe extern "C" fn ST_unloadData() {
    ST_unloadGraphics();
}

// ---------------------------------------------------------------------------
// Init / Start / Stop
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ST_initData() {
    st_firsttime = 1;
    plyr = std::ptr::addr_of_mut!(players[0]).add(consoleplayer as usize);
    st_clock = 0;
    st_chatstate = 0; // StartChatState
    st_gamestate = 1; // FirstPersonState
    st_statusbaron = 1;
    st_oldchat = 0;
    st_chat = 0;
    st_cursoron = 0;
    st_faceindex = 0;
    st_palette = -1;
    st_oldhealth = -1;

    for i in 0..NUMWEAPONS {
        oldweaponsowned[i] = (*plyr).weaponowned[i];
    }
    for i in 0..3 {
        keyboxes[i] = -1;
    }

    STlib_init();
}

#[no_mangle]
pub unsafe extern "C" fn ST_createWidgets() {
    STlib_initNum(
        &raw mut w_ready,
        ST_AMMOX,
        ST_AMMOY,
        std::ptr::addr_of_mut!(tallnum[0]),
        (*plyr)
            .ammo
            .as_mut_ptr()
            .add(weaponinfo[(*plyr).readyweapon as usize].ammo as usize),
        &raw mut st_statusbaron,
        ST_AMMOWIDTH,
    );
    w_ready.data = (*plyr).readyweapon;

    STlib_initPercent(
        &raw mut w_health,
        ST_HEALTHX,
        ST_HEALTHY,
        std::ptr::addr_of_mut!(tallnum[0]),
        &mut (*plyr).health,
        &raw mut st_statusbaron,
        tallpercent,
    );

    STlib_initBinIcon(
        &raw mut w_armsbg,
        ST_ARMSBGX,
        ST_ARMSBGY,
        armsbg,
        &raw mut st_notdeathmatch,
        &raw mut st_statusbaron,
    );

    for i in 0..6 {
        let i_i = i as c_int;
        STlib_initMultIcon(
            std::ptr::addr_of_mut!(w_arms[0]).add(i),
            ST_ARMSX + (i_i % 3) * ST_ARMSXSPACE,
            ST_ARMSY + (i_i / 3) * ST_ARMSYSPACE,
            arms[i].as_mut_ptr(),
            (*plyr).weaponowned.as_mut_ptr().add(i + 1),
            &raw mut st_armson,
        );
    }

    STlib_initNum(
        &raw mut w_frags,
        ST_FRAGSX,
        ST_FRAGSY,
        std::ptr::addr_of_mut!(tallnum[0]),
        &raw mut st_fragscount,
        &raw mut st_fragson,
        ST_FRAGSWIDTH,
    );

    STlib_initMultIcon(
        &raw mut w_faces,
        ST_FACESX,
        ST_FACESY,
        std::ptr::addr_of_mut!(faces[0]),
        &raw mut st_faceindex,
        &raw mut st_statusbaron,
    );

    STlib_initPercent(
        &raw mut w_armor,
        ST_ARMORX,
        ST_ARMORY,
        std::ptr::addr_of_mut!(tallnum[0]),
        &mut (*plyr).armorpoints,
        &raw mut st_statusbaron,
        tallpercent,
    );

    STlib_initMultIcon(
        std::ptr::addr_of_mut!(w_keyboxes[0]).add(0),
        ST_KEY0X,
        ST_KEY0Y,
        std::ptr::addr_of_mut!(keys[0]),
        &mut keyboxes[0],
        &raw mut st_statusbaron,
    );
    STlib_initMultIcon(
        std::ptr::addr_of_mut!(w_keyboxes[0]).add(1),
        ST_KEY1X,
        ST_KEY1Y,
        std::ptr::addr_of_mut!(keys[0]),
        &mut keyboxes[1],
        &raw mut st_statusbaron,
    );
    STlib_initMultIcon(
        std::ptr::addr_of_mut!(w_keyboxes[0]).add(2),
        ST_KEY2X,
        ST_KEY2Y,
        std::ptr::addr_of_mut!(keys[0]),
        &mut keyboxes[2],
        &raw mut st_statusbaron,
    );

    for i in 0..NUMAMMO {
        STlib_initNum(
            std::ptr::addr_of_mut!(w_ammo[0]).add(i),
            match i {
                0 => ST_AMMO0X,
                1 => ST_AMMO1X,
                2 => ST_AMMO2X,
                3 => ST_AMMO3X,
                _ => unreachable!(),
            },
            match i {
                0 => ST_AMMO0Y,
                1 => ST_AMMO1Y,
                2 => ST_AMMO2Y,
                3 => ST_AMMO3Y,
                _ => unreachable!(),
            },
            std::ptr::addr_of_mut!(shortnum[0]),
            (*plyr).ammo.as_mut_ptr().add(i),
            &raw mut st_statusbaron,
            ST_AMMO0WIDTH,
        );
    }

    for i in 0..NUMAMMO {
        STlib_initNum(
            std::ptr::addr_of_mut!(w_maxammo[0]).add(i),
            match i {
                0 => ST_MAXAMMO0X,
                1 => ST_MAXAMMO1X,
                2 => ST_MAXAMMO2X,
                3 => ST_MAXAMMO3X,
                _ => unreachable!(),
            },
            match i {
                0 => ST_MAXAMMO0Y,
                1 => ST_MAXAMMO1Y,
                2 => ST_MAXAMMO2Y,
                3 => ST_MAXAMMO3Y,
                _ => unreachable!(),
            },
            std::ptr::addr_of_mut!(shortnum[0]),
            (*plyr).maxammo.as_mut_ptr().add(i),
            &raw mut st_statusbaron,
            ST_MAXAMMO0WIDTH,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn ST_Start() {
    if st_stopped == 0 {
        ST_Stop();
    }
    ST_initData();
    ST_createWidgets();
    st_stopped = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ST_Stop() {
    if st_stopped != 0 {
        return;
    }
    I_SetPalette(W_CacheLumpNum(lu_palette, PU_CACHE) as *mut u8);
    st_stopped = 1;
}

#[no_mangle]
pub unsafe extern "C" fn ST_Init() {
    ST_loadData();
    st_backing_screen = Z_Malloc(ST_WIDTH * ST_HEIGHT, PU_STATIC, ptr::null_mut()) as *mut u8;
}
