//! Rust port of vendor/doomgeneric/p_switch.c.
//!
//! Switch/button logic: texture swapping, timed buttons, and line-special
//! dispatch for usable switches (doors, lifts, lights, exits, etc.).

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_void};
use std::os::raw::c_short;

use crate::i_error;
use crate::doom::d_mode;
use crate::doom::p_ceilng::EV_DoCeiling;
use crate::doom::p_floor::{side_t, EV_BuildStairs, EV_DoFloor};
use crate::doom::p_lights::{line_t, EV_LightTurnOn};
use crate::doom::p_plats::EV_DoPlat;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const MAXSWITCHES: usize = 50;
pub const MAXBUTTONS: usize = 16;
pub const BUTTONTIME: c_int = 35;

// bwhere_e values
const top: c_int = 0;
const middle: c_int = 1;
const bottom: c_int = 2;

// sfx enum values
const sfx_swtchn: c_int = 23;
const sfx_swtchx: c_int = 24;

// line flags
const ML_SECRET: i16 = 32;

// vldoor_e values
const vld_normal: c_int = 0;
const vld_close: c_int = 2;
const vld_open: c_int = 3;
const vld_blazeRaise: c_int = 5;
const vld_blazeOpen: c_int = 6;
const vld_blazeClose: c_int = 7;

// floor_e values used here
const floor_lowerFloor: c_int = 0;
const floor_lowerFloorToLowest: c_int = 1;
const floor_turboLower: c_int = 2;
const floor_raiseFloor: c_int = 3;
const floor_raiseFloorToNearest: c_int = 4;
const floor_raiseFloorCrush: c_int = 9;
const floor_raiseFloorTurbo: c_int = 10;
const floor_raiseFloor512: c_int = 12;

// ceiling_e values used here
const ceiling_lowerToFloor: c_int = 0;
const ceiling_crushAndRaise: c_int = 3;

// plattype_e values used here
const plat_downWaitUpStay: c_int = 1;
const plat_raiseAndChange: c_int = 2;
const plat_raiseToNearestAndChange: c_int = 3;
const plat_blazeDWUS: c_int = 4;

// stair_e values used here
const stair_build8: c_int = 0;
const stair_turbo16: c_int = 1;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct button_t {
    pub line: *mut line_t,
    pub where_: c_int,
    pub btexture: c_int,
    pub btimer: c_int,
    _pad: [u8; 4],
    pub soundorg: *mut c_void,
}

#[cfg(target_pointer_width = "64")]
mod layout_checks {
    use super::*;
    const _: () = assert!(std::mem::size_of::<button_t>() == 32);
    const _: () = assert!(std::mem::offset_of!(button_t, line) == 0);
    const _: () = assert!(std::mem::offset_of!(button_t, where_) == 8);
    const _: () = assert!(std::mem::offset_of!(button_t, btexture) == 12);
    const _: () = assert!(std::mem::offset_of!(button_t, btimer) == 16);
    const _: () = assert!(std::mem::offset_of!(button_t, soundorg) == 24);
}

// ---------------------------------------------------------------------------
// Data tables
// ---------------------------------------------------------------------------

/// Alpha switch list: pairs of texture names and the game episodes they
/// appear in.
struct SwitchDef {
    name1: &'static [u8],
    name2: &'static [u8],
    episode: i16,
}

const ALPH_SWITCH_LIST: [SwitchDef; 41] = [
    // Doom shareware episode 1 switches
    SwitchDef {
        name1: b"SW1BRCOM\0",
        name2: b"SW2BRCOM\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1BRN1\0",
        name2: b"SW2BRN1\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1BRN2\0",
        name2: b"SW2BRN2\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1BRNGN\0",
        name2: b"SW2BRNGN\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1BROWN\0",
        name2: b"SW2BROWN\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1COMM\0",
        name2: b"SW2COMM\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1COMP\0",
        name2: b"SW2COMP\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1DIRT\0",
        name2: b"SW2DIRT\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1EXIT\0",
        name2: b"SW2EXIT\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1GRAY\0",
        name2: b"SW2GRAY\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1GRAY1\0",
        name2: b"SW2GRAY1\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1METAL\0",
        name2: b"SW2METAL\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1PIPE\0",
        name2: b"SW2PIPE\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1SLAD\0",
        name2: b"SW2SLAD\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1STARG\0",
        name2: b"SW2STARG\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1STON1\0",
        name2: b"SW2STON1\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1STON2\0",
        name2: b"SW2STON2\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1STONE\0",
        name2: b"SW2STONE\0",
        episode: 1,
    },
    SwitchDef {
        name1: b"SW1STRTN\0",
        name2: b"SW2STRTN\0",
        episode: 1,
    },
    // Doom registered episodes 2&3 switches
    SwitchDef {
        name1: b"SW1BLUE\0",
        name2: b"SW2BLUE\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1CMT\0",
        name2: b"SW2CMT\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1GARG\0",
        name2: b"SW2GARG\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1GSTON\0",
        name2: b"SW2GSTON\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1HOT\0",
        name2: b"SW2HOT\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1LION\0",
        name2: b"SW2LION\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1SATYR\0",
        name2: b"SW2SATYR\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1SKIN\0",
        name2: b"SW2SKIN\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1VINE\0",
        name2: b"SW2VINE\0",
        episode: 2,
    },
    SwitchDef {
        name1: b"SW1WOOD\0",
        name2: b"SW2WOOD\0",
        episode: 2,
    },
    // Doom II switches
    SwitchDef {
        name1: b"SW1PANEL\0",
        name2: b"SW2PANEL\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1ROCK\0",
        name2: b"SW2ROCK\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1MET2\0",
        name2: b"SW2MET2\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1WDMET\0",
        name2: b"SW2WDMET\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1BRIK\0",
        name2: b"SW2BRIK\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1MOD1\0",
        name2: b"SW2MOD1\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1ZIM\0",
        name2: b"SW2ZIM\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1STON6\0",
        name2: b"SW2STON6\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1TEK\0",
        name2: b"SW2TEK\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1MARB\0",
        name2: b"SW2MARB\0",
        episode: 3,
    },
    SwitchDef {
        name1: b"SW1SKULL\0",
        name2: b"SW2SKULL\0",
        episode: 3,
    },
    // terminator
    SwitchDef {
        name1: b"\0",
        name2: b"\0",
        episode: 0,
    },
];

// ---------------------------------------------------------------------------
// Globals
// ---------------------------------------------------------------------------

/// Flat array of (texture1, texture2) pairs; length = numswitches × 2.
#[no_mangle]
pub static mut switchlist: [c_int; MAXSWITCHES * 2] = [0; MAXSWITCHES * 2];

/// Number of valid switch pairs initialised by P_InitSwitchList.
#[no_mangle]
pub static mut numswitches: c_int = 0;

/// Active button timers.
#[no_mangle]
pub static mut buttonlist: [button_t; MAXBUTTONS] = [button_t {
    line: std::ptr::null_mut(),
    where_: 0,
    btexture: 0,
    btimer: 0,
    _pad: [0; 4],
    soundorg: std::ptr::null_mut(),
}; MAXBUTTONS];

// ---------------------------------------------------------------------------
// External declarations (unported C functions)
// ---------------------------------------------------------------------------

extern "C" {
    fn R_TextureNumForName(name: *mut c_char) -> c_int;
    fn S_StartSound(origin: *mut c_void, sfxid: c_int);
    fn EV_VerticalDoor(line: *mut line_t, thing: *mut c_void);
    fn EV_DoDoor(line: *mut line_t, dtype: c_int) -> c_int;
    fn EV_DoLockedDoor(line: *mut line_t, dtype: c_int, thing: *mut c_void) -> c_int;
    fn G_ExitLevel();
    fn G_SecretExitLevel();
    fn EV_DoDonut(line: *mut line_t) -> c_int;

    static mut gamemode: c_int;
    static mut sides: *mut side_t;
}

// ---------------------------------------------------------------------------
// P_InitSwitchList
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_InitSwitchList() {
    unsafe {
        let episode: i16 = match gamemode {
            m if m == d_mode::registered || m == d_mode::retail => 2,
            m if m == d_mode::commercial => 3,
            _ => 1,
        };

        let mut index = 0usize;
        for sw in &ALPH_SWITCH_LIST {
            if sw.episode == 0 {
                numswitches = (index / 2) as c_int;
                switchlist[index] = -1;
                break;
            }
            if sw.episode <= episode {
                switchlist[index] = R_TextureNumForName(sw.name1.as_ptr() as *mut c_char);
                index += 1;
                switchlist[index] = R_TextureNumForName(sw.name2.as_ptr() as *mut c_char);
                index += 1;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// P_StartButton
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_StartButton(line: *mut line_t, w: c_int, texture: c_int, time: c_int) {
    // See if button is already pressed
    for i in 0..MAXBUTTONS {
        if buttonlist[i].btimer != 0 && buttonlist[i].line == line {
            return;
        }
    }

    for i in 0..MAXBUTTONS {
        if buttonlist[i].btimer == 0 {
            buttonlist[i].line = line;
            buttonlist[i].where_ = w;
            buttonlist[i].btexture = texture;
            buttonlist[i].btimer = time;
            buttonlist[i].soundorg =
                &(*(*line).frontsector).soundorg as *const [u8; 40] as *mut c_void;
            return;
        }
    }

    i_error!("P_StartButton: no button slots left!");
}

// ---------------------------------------------------------------------------
// P_ChangeSwitchTexture
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_ChangeSwitchTexture(line: *mut line_t, useAgain: c_int) {
    if useAgain == 0 {
        (*line).special = 0;
    }

    let sidenum = (*line).sidenum[0] as isize;
    let texTop = (*sides.offset(sidenum)).toptexture;
    let texMid = (*sides.offset(sidenum)).midtexture;
    let texBot = (*sides.offset(sidenum)).bottomtexture;

    let mut sound = sfx_swtchn;

    // EXIT SWITCH?
    if (*line).special == 11 {
        sound = sfx_swtchx;
    }

    for i in 0..(numswitches * 2) as usize {
        if switchlist[i] == texTop as c_int {
            S_StartSound(buttonlist[0].soundorg, sound);
            (*sides.offset(sidenum)).toptexture = switchlist[i ^ 1] as c_short;
            if useAgain != 0 {
                P_StartButton(line, top, switchlist[i], BUTTONTIME);
            }
            return;
        } else if switchlist[i] == texMid as c_int {
            S_StartSound(buttonlist[0].soundorg, sound);
            (*sides.offset(sidenum)).midtexture = switchlist[i ^ 1] as c_short;
            if useAgain != 0 {
                P_StartButton(line, middle, switchlist[i], BUTTONTIME);
            }
            return;
        } else if switchlist[i] == texBot as c_int {
            S_StartSound(buttonlist[0].soundorg, sound);
            (*sides.offset(sidenum)).bottomtexture = switchlist[i ^ 1] as c_short;
            if useAgain != 0 {
                P_StartButton(line, bottom, switchlist[i], BUTTONTIME);
            }
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// P_UseSpecialLine
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_UseSpecialLine(
    thing: *mut c_void,
    line: *mut line_t,
    side: c_int,
) -> c_int {
    // Err...
    // Use the back sides of VERY SPECIAL lines...
    if side != 0 {
        match (*line).special {
            124 => {
                // Sliding door open&close
                // UNUSED?
            }
            _ => return 0,
        }
    }

    // Switches that other things can activate.
    let mobj = thing as *mut crate::doom::c_ffi::mobj_t;
    if (*mobj).player.is_null() {
        // never open secret doors
        if (*line).flags & ML_SECRET != 0 {
            return 0;
        }
        match (*line).special {
            1 | 32 | 33 | 34 => {}
            _ => return 0,
        }
    }

    // do something
    match (*line).special {
        // MANUALS
        1 | 26 | 27 | 28 | 31 | 32 | 33 | 34 | 117 | 118 => {
            EV_VerticalDoor(line, thing);
        }

        // SWITCHES
        7 => {
            if EV_BuildStairs(line, stair_build8) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        9 => {
            if EV_DoDonut(line) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        11 => {
            P_ChangeSwitchTexture(line, 0);
            G_ExitLevel();
        }
        14 => {
            if EV_DoPlat(line, plat_raiseAndChange, 32) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        15 => {
            if EV_DoPlat(line, plat_raiseAndChange, 24) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        18 => {
            if EV_DoFloor(line, floor_raiseFloorToNearest) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        20 => {
            if EV_DoPlat(line, plat_raiseToNearestAndChange, 0) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        21 => {
            if EV_DoPlat(line, plat_downWaitUpStay, 0) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        23 => {
            if EV_DoFloor(line, floor_lowerFloorToLowest) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        29 => {
            if EV_DoDoor(line, vld_normal) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        41 => {
            if EV_DoCeiling(line, ceiling_lowerToFloor) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        71 => {
            if EV_DoFloor(line, floor_turboLower) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        49 => {
            if EV_DoCeiling(line, ceiling_crushAndRaise) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        50 => {
            if EV_DoDoor(line, vld_close) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        51 => {
            P_ChangeSwitchTexture(line, 0);
            G_SecretExitLevel();
        }
        55 => {
            if EV_DoFloor(line, floor_raiseFloorCrush) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        101 => {
            if EV_DoFloor(line, floor_raiseFloor) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        102 => {
            if EV_DoFloor(line, floor_lowerFloor) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        103 => {
            if EV_DoDoor(line, vld_open) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        111 => {
            if EV_DoDoor(line, vld_blazeRaise) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        112 => {
            if EV_DoDoor(line, vld_blazeOpen) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        113 => {
            if EV_DoDoor(line, vld_blazeClose) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        122 => {
            if EV_DoPlat(line, plat_blazeDWUS, 0) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        127 => {
            if EV_BuildStairs(line, stair_turbo16) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        131 => {
            if EV_DoFloor(line, floor_raiseFloorTurbo) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        133 | 135 | 137 => {
            if EV_DoLockedDoor(line, vld_blazeOpen, thing) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        140 => {
            if EV_DoFloor(line, floor_raiseFloor512) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }

        // BUTTONS
        42 => {
            if EV_DoDoor(line, vld_close) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        43 => {
            if EV_DoCeiling(line, ceiling_lowerToFloor) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        45 => {
            if EV_DoFloor(line, floor_lowerFloor) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        60 => {
            if EV_DoFloor(line, floor_lowerFloorToLowest) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        61 => {
            if EV_DoDoor(line, vld_open) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        62 => {
            if EV_DoPlat(line, plat_downWaitUpStay, 1) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        63 => {
            if EV_DoDoor(line, vld_normal) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        64 => {
            if EV_DoFloor(line, floor_raiseFloor) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        66 => {
            if EV_DoPlat(line, plat_raiseAndChange, 24) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        67 => {
            if EV_DoPlat(line, plat_raiseAndChange, 32) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        65 => {
            if EV_DoFloor(line, floor_raiseFloorCrush) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        68 => {
            if EV_DoPlat(line, plat_raiseToNearestAndChange, 0) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        69 => {
            if EV_DoFloor(line, floor_raiseFloorToNearest) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        70 => {
            if EV_DoFloor(line, floor_turboLower) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        114 => {
            if EV_DoDoor(line, vld_blazeRaise) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        115 => {
            if EV_DoDoor(line, vld_blazeOpen) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        116 => {
            if EV_DoDoor(line, vld_blazeClose) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        123 => {
            if EV_DoPlat(line, plat_blazeDWUS, 0) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        132 => {
            if EV_DoFloor(line, floor_raiseFloorTurbo) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        99 | 134 | 136 => {
            if EV_DoLockedDoor(line, vld_blazeOpen, thing) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        138 => {
            EV_LightTurnOn(line, 255);
            P_ChangeSwitchTexture(line, 1);
        }
        139 => {
            EV_LightTurnOn(line, 35);
            P_ChangeSwitchTexture(line, 1);
        }

        _ => {}
    }

    1 // true
}

// ---------------------------------------------------------------------------
// Link anchor
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_Switch_Link_Anchor() {
    let _ = P_InitSwitchList as *const () as usize;
    let _ = P_StartButton as *const () as usize;
    let _ = P_ChangeSwitchTexture as *const () as usize;
    let _ = P_UseSpecialLine as *const () as usize;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const BUTTON_T_SIZEOF: usize = 32;
    const BUTTON_T_LINE: usize = 0;
    const BUTTON_T_WHERE: usize = 8;
    const BUTTON_T_BTEXTURE: usize = 12;
    const BUTTON_T_BTIMER: usize = 16;
    const BUTTON_T_SOUNDORG: usize = 24;

    #[test]
    fn button_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<button_t>(), BUTTON_T_SIZEOF);
        assert_eq!(std::mem::offset_of!(button_t, line), BUTTON_T_LINE);
        assert_eq!(std::mem::offset_of!(button_t, where_), BUTTON_T_WHERE);
        assert_eq!(std::mem::offset_of!(button_t, btexture), BUTTON_T_BTEXTURE);
        assert_eq!(std::mem::offset_of!(button_t, btimer), BUTTON_T_BTIMER);
        assert_eq!(std::mem::offset_of!(button_t, soundorg), BUTTON_T_SOUNDORG);
    }

    #[test]
    fn constants_match_c() {
        assert_eq!(MAXSWITCHES, 50);
        assert_eq!(MAXBUTTONS, 16);
        assert_eq!(BUTTONTIME, 35);
        assert_eq!(top, 0);
        assert_eq!(middle, 1);
        assert_eq!(bottom, 2);
    }

    #[test]
    fn globals_are_zero_initialized() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(numswitches, 0);
            for (i, &v) in switchlist.iter().enumerate() {
                assert_eq!(v, 0, "switchlist[{i}] should be 0 before P_InitSwitchList");
            }
            for (i, b) in buttonlist.iter().enumerate() {
                assert!(b.line.is_null(), "buttonlist[{i}].line should be null");
                assert_eq!(b.btimer, 0, "buttonlist[{i}].btimer should be 0");
            }
        }
    }

    #[test]
    fn switchlist_length_is_maxswitches_times_2() {
        unsafe {
            assert_eq!(switchlist.len(), MAXSWITCHES * 2);
            assert_eq!(switchlist.len(), 100);
        }
    }
}
