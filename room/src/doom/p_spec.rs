//! Rust port of vendor/doomgeneric/p_spec.c.
//!
//! Special sector/line action dispatcher: texture animation, height and
//! lighting changes, line tag handling, sector triggers, and the donut
//! effect.  Also contains the utility functions (`getSide`, `getSector`,
//! `twoSided`, `P_Find*Surrounding`) used by the floor/ceiling/platform
//! code.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_short, c_void};
use std::ptr;

use crate::doom::c_ffi::{
    line_t, mobj_t, sector_t, side_t, FLOORSPEED, FRACUNIT, ML_TWOSIDED, TICRATE,
};
use crate::doom::d_player::{PlayerT, CF_GODMODE};
use crate::doom::i_system::I_ErrorV;
use crate::doom::info::{MT_BFG, MT_BRUISERSHOT, MT_HEADSHOT, MT_PLASMA, MT_ROCKET, MT_TROOPSHOT};
use crate::doom::m_argv::{myargv, M_CheckParmWithArgs};
use crate::doom::m_misc::M_StrToInt;
use crate::doom::m_random::P_Random;
use crate::doom::p_floor::floormove_t;
use crate::doom::p_setup::{lines, numlines, numsectors, sectors, sides};
use crate::doom::p_switch::{buttonlist, MAXBUTTONS};
use crate::doom::p_tick::{leveltime, P_AddThinker};
use crate::doom::r_data::{
    flattranslation, numflats, texturetranslation, R_CheckTextureNumForName, R_FlatNumForName,
    R_TextureNumForName,
};
use crate::doom::s_sound::S_StartSound;

// ---------------------------------------------------------------------------
// Macros
// ---------------------------------------------------------------------------

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const MAXANIMS: usize = 32;
const MAXLINEANIMS: usize = 64;
const MAX_ADJOINING_SECTORS: usize = 20;
const PU_LEVSPEC: c_int = 5;

// vldoor_e
const vld_normal: c_int = 0;
const vld_close30ThenOpen: c_int = 1;
const vld_close: c_int = 2;
const vld_open: c_int = 3;
const vld_blazeRaise: c_int = 5;
const vld_blazeOpen: c_int = 6;
const vld_blazeClose: c_int = 7;

// floor_e
const lowerFloor: c_int = 0;
const lowerFloorToLowest: c_int = 1;
const turboLower: c_int = 2;
const raiseFloor: c_int = 3;
const raiseFloorToNearest: c_int = 4;
const raiseToTexture: c_int = 5;
const lowerAndChange: c_int = 6;
const raiseFloor24: c_int = 7;
const raiseFloor24AndChange: c_int = 8;
const raiseFloorCrush: c_int = 9;
const raiseFloorTurbo: c_int = 10;
const donutRaise: c_int = 11;

// ceiling_e
const raiseToHighest: c_int = 1;
const lowerAndCrush: c_int = 2;
const crushAndRaise: c_int = 3;
const fastCrushAndRaise: c_int = 4;
const silentCrushAndRaise: c_int = 5;

// plattype_e
const downWaitUpStay: c_int = 1;
const raiseToNearestAndChange: c_int = 3;
const blazeDWUS: c_int = 4;
const perpetualRaise: c_int = 0;

// stair_e
const build8: c_int = 0;
const turbo16: c_int = 1;

// sfx
const sfx_swtchn: c_int = 23;

// powers
const pw_ironfeet: usize = 3;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct anim_t {
    pub istexture: c_int,
    pub picnum: c_int,
    pub basepic: c_int,
    pub numpics: c_int,
    pub speed: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct animdef_t {
    pub istexture: c_int,
    pub endname: [c_char; 9],
    pub startname: [c_char; 9],
    pub speed: c_int,
}

#[cfg(target_pointer_width = "64")]
mod layout_checks {
    use super::*;
    const _: () = assert!(std::mem::size_of::<anim_t>() == 20);
    const _: () = assert!(std::mem::size_of::<animdef_t>() == 28);
}

// ---------------------------------------------------------------------------
// Animation definitions
// ---------------------------------------------------------------------------

const ANIMDEFS: &[(c_int, *mut c_char, *mut c_char, c_int)] = &[
    (0, cstr!("NUKAGE3"), cstr!("NUKAGE1"), 8),
    (0, cstr!("FWATER4"), cstr!("FWATER1"), 8),
    (0, cstr!("SWATER4"), cstr!("SWATER1"), 8),
    (0, cstr!("LAVA4"), cstr!("LAVA1"), 8),
    (0, cstr!("BLOOD3"), cstr!("BLOOD1"), 8),
    (0, cstr!("RROCK08"), cstr!("RROCK05"), 8),
    (0, cstr!("SLIME04"), cstr!("SLIME01"), 8),
    (0, cstr!("SLIME08"), cstr!("SLIME05"), 8),
    (0, cstr!("SLIME12"), cstr!("SLIME09"), 8),
    (1, cstr!("BLODGR4"), cstr!("BLODGR1"), 8),
    (1, cstr!("SLADRIP3"), cstr!("SLADRIP1"), 8),
    (1, cstr!("BLODRIP4"), cstr!("BLODRIP1"), 8),
    (1, cstr!("FIREWALL"), cstr!("FIREWALA"), 8),
    (1, cstr!("GSTFONT3"), cstr!("GSTFONT1"), 8),
    (1, cstr!("FIRELAVA"), cstr!("FIRELAV3"), 8),
    (1, cstr!("FIREMAG3"), cstr!("FIREMAG1"), 8),
    (1, cstr!("FIREBLU2"), cstr!("FIREBLU1"), 8),
    (1, cstr!("ROCKRED3"), cstr!("ROCKRED1"), 8),
    (1, cstr!("BFALL4"), cstr!("BFALL1"), 8),
    (1, cstr!("SFALL4"), cstr!("SFALL1"), 8),
    (1, cstr!("WFALL4"), cstr!("WFALL1"), 8),
    (1, cstr!("DBRAIN4"), cstr!("DBRAIN1"), 8),
    (-1, cstr!(""), cstr!(""), 0),
];

// ---------------------------------------------------------------------------
// Globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut anims: [anim_t; MAXANIMS] = [anim_t {
    istexture: 0,
    picnum: 0,
    basepic: 0,
    numpics: 0,
    speed: 0,
}; MAXANIMS];

#[no_mangle]
pub static mut lastanim: *mut anim_t = ptr::null_mut();

#[no_mangle]
pub static mut numlinespecials: c_short = 0;

#[no_mangle]
pub static mut linespeciallist: [*mut line_t; MAXLINEANIMS] = [ptr::null_mut(); MAXLINEANIMS];

#[no_mangle]
pub static mut levelTimer: c_int = 0; // boolean

#[no_mangle]
pub static mut levelTimeCount: c_int = 0;

// ---------------------------------------------------------------------------
// Externs from other modules / remaining C code
// ---------------------------------------------------------------------------

extern "C" {
    // Other Rust modules (declared here with c_ffi types for ABI compatibility)
    fn EV_DoDoor(line: *mut line_t, r#type: c_int) -> c_int;
    fn EV_DoFloor(line: *mut line_t, floortype: c_int) -> c_int;
    fn EV_DoCeiling(line: *mut line_t, r#type: c_int) -> c_int;
    fn EV_DoPlat(line: *mut line_t, r#type: c_int, amount: c_int) -> c_int;
    fn EV_BuildStairs(line: *mut line_t, r#type: c_int) -> c_int;
    fn EV_Teleport(line: *mut line_t, side: c_int, thing: *mut mobj_t) -> c_int;
    fn EV_LightTurnOn(line: *mut line_t, bright: c_int);
    fn EV_StartLightStrobing(line: *mut line_t);
    fn EV_TurnTagLightsOff(line: *mut line_t);
    fn EV_CeilingCrushStop(line: *mut line_t) -> c_int;
    fn EV_StopPlat(line: *mut line_t);
    fn P_SpawnLightFlash(sector: *mut sector_t);
    fn P_SpawnStrobeFlash(sector: *mut sector_t, fastOrSlow: c_int, inSync: c_int);
    fn P_SpawnGlowingLight(sector: *mut sector_t);
    fn P_SpawnFireFlicker(sector: *mut sector_t);
    fn P_SpawnDoorCloseIn30(sec: *mut sector_t);
    fn P_SpawnDoorRaiseIn5Mins(sec: *mut sector_t, secnum: c_int);
    fn P_ChangeSwitchTexture(line: *mut line_t, useAgain: c_int);
    fn P_DamageMobj(
        target: *mut mobj_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damage: c_int,
    );
    fn T_MoveFloor(floor: *mut floormove_t);

    // Remaining C modules
    fn G_ExitLevel();
    fn G_SecretExitLevel();
    fn W_CheckNumForName(name: *mut c_char) -> c_int;
    fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void;

    // C globals
    static mut timelimit: c_int;
    static mut deathmatch: c_int;
    static mut totalsecret: c_int;
}

// ---------------------------------------------------------------------------
// DEH_String shim — identity when dehacked is disabled.
// ---------------------------------------------------------------------------

#[inline(always)]
unsafe fn DEH_String(s: *mut c_char) -> *mut c_char {
    s
}

// ---------------------------------------------------------------------------
// P_InitPicAnims
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_InitPicAnims() {
    lastanim = anims.as_mut_ptr();
    for &(istexture, endname, startname, speed) in ANIMDEFS {
        if istexture == -1 {
            break;
        }
        let startname = DEH_String(startname);
        let endname = DEH_String(endname);

        if istexture != 0 {
            if R_CheckTextureNumForName(startname) == -1 {
                continue;
            }
            (*lastanim).picnum = R_TextureNumForName(endname);
            (*lastanim).basepic = R_TextureNumForName(startname);
        } else {
            if W_CheckNumForName(startname) == -1 {
                continue;
            }
            (*lastanim).picnum = R_FlatNumForName(endname);
            (*lastanim).basepic = R_FlatNumForName(startname);
        }

        (*lastanim).istexture = istexture;
        (*lastanim).numpics = (*lastanim).picnum - (*lastanim).basepic + 1;

        if (*lastanim).numpics < 2 {
            let s = std::ffi::CStr::from_ptr(startname).to_string_lossy();
            let e = std::ffi::CStr::from_ptr(endname).to_string_lossy();
            let msg =
                std::ffi::CString::new(format!("P_InitPicAnims: bad cycle from {} to {}", s, e))
                    .unwrap();
            I_ErrorV(msg.as_ptr());
        }

        (*lastanim).speed = speed;
        lastanim = lastanim.offset(1);
    }
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn getSide(currentSector: c_int, line: c_int, side: c_int) -> *mut side_t {
    let line_ptr = *(*sectors.offset(currentSector as isize))
        .lines
        .offset(line as isize) as *mut line_t;
    let side_idx = (*line_ptr).sidenum[side as usize];
    sides.offset(side_idx as isize)
}

#[no_mangle]
pub unsafe extern "C" fn getSector(
    currentSector: c_int,
    line: c_int,
    side: c_int,
) -> *mut sector_t {
    let line_ptr = *(*sectors.offset(currentSector as isize))
        .lines
        .offset(line as isize) as *mut line_t;
    let side_idx = (*line_ptr).sidenum[side as usize];
    (*sides.offset(side_idx as isize)).sector
}

#[no_mangle]
pub unsafe extern "C" fn twoSided(sector: c_int, line: c_int) -> c_int {
    let line_ptr = *(*sectors.offset(sector as isize))
        .lines
        .offset(line as isize) as *mut line_t;
    ((*line_ptr).flags as c_int) & (ML_TWOSIDED as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn getNextSector(line: *mut line_t, sec: *mut sector_t) -> *mut sector_t {
    if ((*line).flags as c_int) & (ML_TWOSIDED as c_int) == 0 {
        return ptr::null_mut();
    }
    if (*line).frontsector == sec as *mut c_void {
        return (*line).backsector as *mut sector_t;
    }
    return (*line).frontsector as *mut sector_t;
}

#[no_mangle]
pub unsafe extern "C" fn P_FindLowestFloorSurrounding(sec: *mut sector_t) -> c_int {
    let mut floor = (*sec).floorheight;
    for i in 0..(*sec).linecount {
        let check = *(*sec).lines.offset(i as isize) as *mut line_t;
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).floorheight < floor {
            floor = (*other).floorheight;
        }
    }
    floor
}

#[no_mangle]
pub unsafe extern "C" fn P_FindHighestFloorSurrounding(sec: *mut sector_t) -> c_int {
    let mut floor = -500 * FRACUNIT;
    for i in 0..(*sec).linecount {
        let check = *(*sec).lines.offset(i as isize) as *mut line_t;
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).floorheight > floor {
            floor = (*other).floorheight;
        }
    }
    floor
}

#[no_mangle]
pub unsafe extern "C" fn P_FindNextHighestFloor(sec: *mut sector_t, currentheight: c_int) -> c_int {
    let mut height = currentheight;
    let mut heightlist: [c_int; MAX_ADJOINING_SECTORS + 2] = [0; MAX_ADJOINING_SECTORS + 2];
    let mut h = 0;

    for i in 0..(*sec).linecount {
        let check = *(*sec).lines.offset(i as isize) as *mut line_t;
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).floorheight > height {
            if h == MAX_ADJOINING_SECTORS + 1 {
                height = (*other).floorheight;
            } else if h == MAX_ADJOINING_SECTORS + 2 {
                I_ErrorV(
                    b"Sector with more than 22 adjoining sectors. Vanilla will crash here\0"
                        .as_ptr() as *const c_char,
                );
            }
            heightlist[h] = (*other).floorheight;
            h += 1;
        }
    }

    if h == 0 {
        return currentheight;
    }

    let mut min = heightlist[0];
    for i in 1..h {
        if heightlist[i] < min {
            min = heightlist[i];
        }
    }
    min
}

#[no_mangle]
pub unsafe extern "C" fn P_FindLowestCeilingSurrounding(sec: *mut sector_t) -> c_int {
    let mut height = c_int::MAX;
    for i in 0..(*sec).linecount {
        let check = *(*sec).lines.offset(i as isize) as *mut line_t;
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).ceilingheight < height {
            height = (*other).ceilingheight;
        }
    }
    height
}

#[no_mangle]
pub unsafe extern "C" fn P_FindHighestCeilingSurrounding(sec: *mut sector_t) -> c_int {
    let mut height = 0;
    for i in 0..(*sec).linecount {
        let check = *(*sec).lines.offset(i as isize) as *mut line_t;
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).ceilingheight > height {
            height = (*other).ceilingheight;
        }
    }
    height
}

#[no_mangle]
pub unsafe extern "C" fn P_FindSectorFromLineTag(line: *mut line_t, start: c_int) -> c_int {
    for i in (start + 1)..numsectors {
        if (*sectors.offset(i as isize)).tag == (*line).tag {
            return i;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn P_FindMinSurroundingLight(sector: *mut sector_t, max: c_int) -> c_int {
    let mut min = max;
    for i in 0..(*sector).linecount {
        let line = *(*sector).lines.offset(i as isize) as *mut line_t;
        let check = getNextSector(line, sector);
        if check.is_null() {
            continue;
        }
        if ((*check).lightlevel as c_int) < min {
            min = (*check).lightlevel as c_int;
        }
    }
    min
}

// ---------------------------------------------------------------------------
// P_CrossSpecialLine
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_CrossSpecialLine(linenum: c_int, side: c_int, thing: *mut mobj_t) {
    let line = lines.offset(linenum as isize);

    if (*thing).player.is_null() {
        match (*thing).type_ {
            MT_ROCKET | MT_PLASMA | MT_BFG | MT_TROOPSHOT | MT_HEADSHOT | MT_BRUISERSHOT => {
                return;
            }
            _ => {}
        }

        let mut ok = 0;
        match (*line).special as c_int {
            39 | 97 | 125 | 126 | 4 | 10 | 88 => ok = 1,
            _ => {}
        }
        if ok == 0 {
            return;
        }
    }

    match (*line).special as c_int {
        2 => {
            EV_DoDoor(line, vld_open);
            (*line).special = 0;
        }
        3 => {
            EV_DoDoor(line, vld_close);
            (*line).special = 0;
        }
        4 => {
            EV_DoDoor(line, vld_normal);
            (*line).special = 0;
        }
        5 => {
            EV_DoFloor(line, raiseFloor);
            (*line).special = 0;
        }
        6 => {
            EV_DoCeiling(line, fastCrushAndRaise);
            (*line).special = 0;
        }
        8 => {
            EV_BuildStairs(line, build8);
            (*line).special = 0;
        }
        10 => {
            EV_DoPlat(line, downWaitUpStay, 0);
            (*line).special = 0;
        }
        12 => {
            EV_LightTurnOn(line, 0);
            (*line).special = 0;
        }
        13 => {
            EV_LightTurnOn(line, 255);
            (*line).special = 0;
        }
        16 => {
            EV_DoDoor(line, vld_close30ThenOpen);
            (*line).special = 0;
        }
        17 => {
            EV_StartLightStrobing(line);
            (*line).special = 0;
        }
        19 => {
            EV_DoFloor(line, lowerFloor);
            (*line).special = 0;
        }
        22 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0);
            (*line).special = 0;
        }
        25 => {
            EV_DoCeiling(line, crushAndRaise);
            (*line).special = 0;
        }
        30 => {
            EV_DoFloor(line, raiseToTexture);
            (*line).special = 0;
        }
        35 => {
            EV_LightTurnOn(line, 35);
            (*line).special = 0;
        }
        36 => {
            EV_DoFloor(line, turboLower);
            (*line).special = 0;
        }
        37 => {
            EV_DoFloor(line, lowerAndChange);
            (*line).special = 0;
        }
        38 => {
            EV_DoFloor(line, lowerFloorToLowest);
            (*line).special = 0;
        }
        39 => {
            EV_Teleport(line, side, thing);
            (*line).special = 0;
        }
        40 => {
            EV_DoCeiling(line, raiseToHighest);
            EV_DoFloor(line, lowerFloorToLowest);
            (*line).special = 0;
        }
        44 => {
            EV_DoCeiling(line, lowerAndCrush);
            (*line).special = 0;
        }
        52 => {
            G_ExitLevel();
        }
        53 => {
            EV_DoPlat(line, perpetualRaise, 0);
            (*line).special = 0;
        }
        54 => {
            EV_StopPlat(line);
            (*line).special = 0;
        }
        56 => {
            EV_DoFloor(line, raiseFloorCrush);
            (*line).special = 0;
        }
        57 => {
            EV_CeilingCrushStop(line);
            (*line).special = 0;
        }
        58 => {
            EV_DoFloor(line, raiseFloor24);
            (*line).special = 0;
        }
        59 => {
            EV_DoFloor(line, raiseFloor24AndChange);
            (*line).special = 0;
        }
        104 => {
            EV_TurnTagLightsOff(line);
            (*line).special = 0;
        }
        108 => {
            EV_DoDoor(line, vld_blazeRaise);
            (*line).special = 0;
        }
        109 => {
            EV_DoDoor(line, vld_blazeOpen);
            (*line).special = 0;
        }
        100 => {
            EV_BuildStairs(line, turbo16);
            (*line).special = 0;
        }
        110 => {
            EV_DoDoor(line, vld_blazeClose);
            (*line).special = 0;
        }
        119 => {
            EV_DoFloor(line, raiseFloorToNearest);
            (*line).special = 0;
        }
        121 => {
            EV_DoPlat(line, blazeDWUS, 0);
            (*line).special = 0;
        }
        124 => {
            G_SecretExitLevel();
        }
        125 => {
            if (*thing).player.is_null() {
                EV_Teleport(line, side, thing);
                (*line).special = 0;
            }
        }
        130 => {
            EV_DoFloor(line, raiseFloorTurbo);
            (*line).special = 0;
        }
        141 => {
            EV_DoCeiling(line, silentCrushAndRaise);
            (*line).special = 0;
        }
        // RETRIGGERS
        72 => {
            EV_DoCeiling(line, lowerAndCrush);
        }
        73 => {
            EV_DoCeiling(line, crushAndRaise);
        }
        74 => {
            EV_CeilingCrushStop(line);
        }
        75 => {
            EV_DoDoor(line, vld_close);
        }
        76 => {
            EV_DoDoor(line, vld_close30ThenOpen);
        }
        77 => {
            EV_DoCeiling(line, fastCrushAndRaise);
        }
        79 => {
            EV_LightTurnOn(line, 35);
        }
        80 => {
            EV_LightTurnOn(line, 0);
        }
        81 => {
            EV_LightTurnOn(line, 255);
        }
        82 => {
            EV_DoFloor(line, lowerFloorToLowest);
        }
        83 => {
            EV_DoFloor(line, lowerFloor);
        }
        84 => {
            EV_DoFloor(line, lowerAndChange);
        }
        86 => {
            EV_DoDoor(line, vld_open);
        }
        87 => {
            EV_DoPlat(line, perpetualRaise, 0);
        }
        88 => {
            EV_DoPlat(line, downWaitUpStay, 0);
        }
        89 => {
            EV_StopPlat(line);
        }
        90 => {
            EV_DoDoor(line, vld_normal);
        }
        91 => {
            EV_DoFloor(line, raiseFloor);
        }
        92 => {
            EV_DoFloor(line, raiseFloor24);
        }
        93 => {
            EV_DoFloor(line, raiseFloor24AndChange);
        }
        94 => {
            EV_DoFloor(line, raiseFloorCrush);
        }
        95 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0);
        }
        96 => {
            EV_DoFloor(line, raiseToTexture);
        }
        97 => {
            EV_Teleport(line, side, thing);
        }
        98 => {
            EV_DoFloor(line, turboLower);
        }
        105 => {
            EV_DoDoor(line, vld_blazeRaise);
        }
        106 => {
            EV_DoDoor(line, vld_blazeOpen);
        }
        107 => {
            EV_DoDoor(line, vld_blazeClose);
        }
        120 => {
            EV_DoPlat(line, blazeDWUS, 0);
        }
        126 => {
            if (*thing).player.is_null() {
                EV_Teleport(line, side, thing);
            }
        }
        128 => {
            EV_DoFloor(line, raiseFloorToNearest);
        }
        129 => {
            EV_DoFloor(line, raiseFloorTurbo);
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// P_ShootSpecialLine
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_ShootSpecialLine(thing: *mut mobj_t, line: *mut line_t) {
    if (*thing).player.is_null() {
        let mut ok = 0;
        match (*line).special as c_int {
            46 => ok = 1,
            _ => {}
        }
        if ok == 0 {
            return;
        }
    }

    match (*line).special as c_int {
        24 => {
            EV_DoFloor(line, raiseFloor);
            P_ChangeSwitchTexture(line, 0);
        }
        46 => {
            EV_DoDoor(line, vld_open);
            P_ChangeSwitchTexture(line, 1);
        }
        47 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0);
            P_ChangeSwitchTexture(line, 0);
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// P_PlayerInSpecialSector
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_PlayerInSpecialSector(player: *mut PlayerT) {
    let mo = (*player).mo as *mut mobj_t;
    let sub = (*mo).subsector as *mut crate::doom::c_ffi::subsector_t;
    let sector = (*sub).sector as *mut sector_t;

    if (*mo).z != (*sector).floorheight {
        return;
    }

    match (*sector).special as c_int {
        5 => {
            if (*player).powers[pw_ironfeet] == 0 {
                if leveltime & 0x1f == 0 {
                    P_DamageMobj(
                        (*player).mo as *mut mobj_t,
                        ptr::null_mut(),
                        ptr::null_mut(),
                        10,
                    );
                }
            }
        }
        7 => {
            if (*player).powers[pw_ironfeet] == 0 {
                if leveltime & 0x1f == 0 {
                    P_DamageMobj(
                        (*player).mo as *mut mobj_t,
                        ptr::null_mut(),
                        ptr::null_mut(),
                        5,
                    );
                }
            }
        }
        16 | 4 => {
            if (*player).powers[pw_ironfeet] == 0 || P_Random() < 5 {
                if leveltime & 0x1f == 0 {
                    P_DamageMobj(
                        (*player).mo as *mut mobj_t,
                        ptr::null_mut(),
                        ptr::null_mut(),
                        20,
                    );
                }
            }
        }
        9 => {
            (*player).secretcount += 1;
            (*sector).special = 0;
        }
        11 => {
            (*player).cheats &= !CF_GODMODE;
            if leveltime & 0x1f == 0 {
                P_DamageMobj(
                    (*player).mo as *mut mobj_t,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    20,
                );
            }
            if (*player).health <= 10 {
                G_ExitLevel();
            }
        }
        _ => {
            let msg = std::ffi::CString::new(format!(
                "P_PlayerInSpecialSector: unknown special {}",
                (*sector).special
            ))
            .unwrap();
            I_ErrorV(msg.as_ptr());
        }
    }
}

// ---------------------------------------------------------------------------
// P_UpdateSpecials
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_UpdateSpecials() {
    if levelTimer != 0 {
        levelTimeCount -= 1;
        if levelTimeCount == 0 {
            G_ExitLevel();
        }
    }

    let mut anim = anims.as_mut_ptr();
    while anim < lastanim {
        let base = (*anim).basepic;
        let numpics = (*anim).numpics;
        for i in base..base + numpics {
            let pic = base + ((leveltime / (*anim).speed + i) % numpics);
            if (*anim).istexture != 0 {
                *texturetranslation.offset(i as isize) = pic;
            } else {
                *flattranslation.offset(i as isize) = pic;
            }
        }
        anim = anim.offset(1);
    }

    for i in 0..numlinespecials as usize {
        let line = linespeciallist[i];
        match (*line).special as c_int {
            48 => {
                let sidenum = (*line).sidenum[0] as isize;
                (*sides.offset(sidenum)).textureoffset += FRACUNIT;
            }
            _ => {}
        }
    }

    for i in 0..MAXBUTTONS {
        if buttonlist[i].btimer != 0 {
            buttonlist[i].btimer -= 1;
            if buttonlist[i].btimer == 0 {
                let sidenum = (*buttonlist[i].line).sidenum[0] as isize;
                match buttonlist[i].where_ {
                    0 => {
                        (*sides.offset(sidenum)).toptexture = buttonlist[i].btexture as i16;
                    }
                    1 => {
                        (*sides.offset(sidenum)).midtexture = buttonlist[i].btexture as i16;
                    }
                    2 => {
                        (*sides.offset(sidenum)).bottomtexture = buttonlist[i].btexture as i16;
                    }
                    _ => {}
                }
                S_StartSound(
                    &mut buttonlist[i].soundorg as *mut _ as *mut c_void,
                    sfx_swtchn,
                );
                buttonlist[i] = std::mem::zeroed();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Donut overrun emulation
// ---------------------------------------------------------------------------

unsafe fn DonutOverrun(
    s3_floorheight: *mut c_int,
    s3_floorpic: *mut i16,
    _line: *mut line_t,
    _pillar_sector: *mut sector_t,
) {
    static mut first: c_int = 1;
    static mut tmp_s3_floorheight: c_int = 0;
    static mut tmp_s3_floorpic: c_int = 0;

    if first != 0 {
        first = 0;
        tmp_s3_floorheight = 0;
        tmp_s3_floorpic = 0x16;

        let p = M_CheckParmWithArgs(cstr!("-donut"), 2);
        if p > 0 {
            M_StrToInt(*myargv.offset((p + 1) as isize), &mut tmp_s3_floorheight);
            M_StrToInt(*myargv.offset((p + 2) as isize), &mut tmp_s3_floorpic);
            if tmp_s3_floorpic >= numflats {
                eprint!(
                    "DonutOverrun: The second parameter for \"-donut\" switch should be greater than 0 and less than number of flats ({}). Using default value ({}) instead. \n",
                    numflats, 0x16
                );
                tmp_s3_floorpic = 0x16;
            }
        }
    }

    *s3_floorheight = tmp_s3_floorheight;
    *s3_floorpic = tmp_s3_floorpic as i16;
}

// ---------------------------------------------------------------------------
// EV_DoDonut
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn EV_DoDonut(line: *mut line_t) -> c_int {
    let mut secnum = -1;
    let mut rtn = 0;

    while {
        secnum = P_FindSectorFromLineTag(line, secnum);
        secnum
    } >= 0
    {
        let s1 = sectors.offset(secnum as isize);
        if !(*s1).specialdata.is_null() {
            continue;
        }

        rtn = 1;
        let s2 = getNextSector(*(*s1).lines.offset(0) as *mut line_t, s1);

        if s2.is_null() {
            eprint!("EV_DoDonut: linedef had no second sidedef! Unexpected behavior may occur in Vanilla Doom. \n");
            break;
        }

        for i in 0..(*s2).linecount {
            let s3 =
                (*((*(*s2).lines.offset(i as isize)) as *mut line_t)).backsector as *mut sector_t;

            if s3 == s1 {
                continue;
            }

            let (s3_floorheight, s3_floorpic) = if s3.is_null() {
                eprint!("EV_DoDonut: WARNING: emulating buffer overrun due to NULL back sector. Unexpected behavior may occur in Vanilla Doom.\n");
                let mut fh = 0;
                let mut fp = 0i16;
                DonutOverrun(&mut fh, &mut fp, line, s1);
                (fh, fp)
            } else {
                ((*s3).floorheight, (*s3).floorpic)
            };

            // Spawn rising slime
            let floor = Z_Malloc(
                std::mem::size_of::<floormove_t>() as c_int,
                PU_LEVSPEC,
                ptr::null_mut(),
            ) as *mut floormove_t;
            P_AddThinker(&mut (*floor).thinker);
            (*s2).specialdata = floor as *mut c_void;
            (*floor).thinker.function.acp1 = Some(core::mem::transmute::<
                unsafe extern "C" fn(*mut floormove_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_MoveFloor));
            (*floor).r#type = donutRaise;
            (*floor).crush = 0;
            (*floor).direction = 1;
            (*floor).sector = s2 as *mut _;
            (*floor).speed = FLOORSPEED / 2;
            (*floor).texture = s3_floorpic;
            (*floor).newspecial = 0;
            (*floor).floordestheight = s3_floorheight;

            // Spawn lowering donut-hole
            let floor = Z_Malloc(
                std::mem::size_of::<floormove_t>() as c_int,
                PU_LEVSPEC,
                ptr::null_mut(),
            ) as *mut floormove_t;
            P_AddThinker(&mut (*floor).thinker);
            (*s1).specialdata = floor as *mut c_void;
            (*floor).thinker.function.acp1 = Some(core::mem::transmute::<
                unsafe extern "C" fn(*mut floormove_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_MoveFloor));
            (*floor).r#type = lowerFloor;
            (*floor).crush = 0;
            (*floor).direction = -1;
            (*floor).sector = s1 as *mut _;
            (*floor).speed = FLOORSPEED / 2;
            (*floor).floordestheight = s3_floorheight;

            break;
        }
    }

    rtn
}

// ---------------------------------------------------------------------------
// P_SpawnSpecials
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_SpawnSpecials() {
    if timelimit > 0 && deathmatch != 0 {
        levelTimer = 1;
        levelTimeCount = timelimit * 60 * TICRATE;
    } else {
        levelTimer = 0;
    }

    for i in 0..numsectors {
        let sector = sectors.offset(i as isize);
        if (*sector).special == 0 {
            continue;
        }
        match (*sector).special as c_int {
            1 => P_SpawnLightFlash(sector),
            2 => P_SpawnStrobeFlash(sector, crate::doom::c_ffi::FASTDARK, 0),
            3 => P_SpawnStrobeFlash(sector, crate::doom::c_ffi::SLOWDARK, 0),
            4 => {
                P_SpawnStrobeFlash(sector, crate::doom::c_ffi::FASTDARK, 0);
                (*sector).special = 4;
            }
            8 => P_SpawnGlowingLight(sector),
            9 => {
                totalsecret += 1;
            }
            10 => P_SpawnDoorCloseIn30(sector),
            12 => P_SpawnStrobeFlash(sector, crate::doom::c_ffi::SLOWDARK, 1),
            13 => P_SpawnStrobeFlash(sector, crate::doom::c_ffi::FASTDARK, 1),
            14 => P_SpawnDoorRaiseIn5Mins(sector, i),
            17 => P_SpawnFireFlicker(sector),
            _ => {}
        }
    }

    numlinespecials = 0;
    for i in 0..numlines {
        match (*lines.offset(i as isize)).special as c_int {
            48 => {
                if numlinespecials as c_int >= MAXLINEANIMS as c_int {
                    I_ErrorV(
                        b"Too many scrolling wall linedefs! (Vanilla limit is 64)\0".as_ptr()
                            as *const c_char,
                    );
                }
                linespeciallist[numlinespecials as usize] = lines.offset(i as isize);
                numlinespecials += 1;
            }
            _ => {}
        }
    }

    for i in 0..crate::doom::c_ffi::MAXCEILINGS as usize {
        crate::doom::p_ceilng::activeceilings[i] = ptr::null_mut();
    }
    for i in 0..crate::doom::c_ffi::MAXPLATS as usize {
        crate::doom::p_plats::activeplats[i] = ptr::null_mut();
    }
    for i in 0..MAXBUTTONS {
        buttonlist[i] = std::mem::zeroed();
    }
}

// ---------------------------------------------------------------------------
// Link anchor
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_Spec_Link_Anchor() {
    let _ = P_InitPicAnims as *const () as usize;
    let _ = P_SpawnSpecials as *const () as usize;
    let _ = P_UpdateSpecials as *const () as usize;
    let _ = P_CrossSpecialLine as *const () as usize;
    let _ = P_ShootSpecialLine as *const () as usize;
    let _ = P_PlayerInSpecialSector as *const () as usize;
    let _ = EV_DoDonut as *const () as usize;
    let _ = getSide as *const () as usize;
    let _ = getSector as *const () as usize;
    let _ = twoSided as *const () as usize;
    let _ = getNextSector as *const () as usize;
    let _ = P_FindLowestFloorSurrounding as *const () as usize;
    let _ = P_FindHighestFloorSurrounding as *const () as usize;
    let _ = P_FindNextHighestFloor as *const () as usize;
    let _ = P_FindLowestCeilingSurrounding as *const () as usize;
    let _ = P_FindHighestCeilingSurrounding as *const () as usize;
    let _ = P_FindSectorFromLineTag as *const () as usize;
    let _ = P_FindMinSurroundingLight as *const () as usize;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const ANIM_T_SIZEOF: usize = 20;
    const ANIMDEF_T_SIZEOF: usize = 28;

    #[test]
    fn anim_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<anim_t>(), ANIM_T_SIZEOF);
    }

    #[test]
    fn animdef_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<animdef_t>(), ANIMDEF_T_SIZEOF);
    }

    #[test]
    fn globals_are_zero_initialized() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(levelTimer, 0);
            assert_eq!(levelTimeCount, 0);
            assert_eq!(numlinespecials, 0);
            assert!(lastanim.is_null());
            for (i, &v) in linespeciallist.iter().enumerate() {
                assert!(v.is_null(), "linespeciallist[{i}] should be null");
            }
            for (i, a) in anims.iter().enumerate() {
                assert_eq!(a.istexture, 0, "anims[{i}].istexture should be 0");
                assert_eq!(a.picnum, 0, "anims[{i}].picnum should be 0");
            }
        }
    }

    #[test]
    fn constants_match_c() {
        assert_eq!(MAXANIMS, 32);
        assert_eq!(MAXLINEANIMS, 64);
        assert_eq!(MAX_ADJOINING_SECTORS, 20);
        assert_eq!(PU_LEVSPEC, 5);
        assert_eq!(sfx_swtchn, 23);
        assert_eq!(CF_GODMODE, 2);
        assert_eq!(pw_ironfeet, 3);
    }

    #[test]
    fn animation_defs_terminated() {
        let last = ANIMDEFS[ANIMDEFS.len() - 1];
        assert_eq!(last.0, -1);
    }
}
