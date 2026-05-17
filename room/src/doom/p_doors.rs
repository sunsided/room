//! Rust port of vendor/doomgeneric/p_doors.c.
//!
//! Door animation: opening/closing vertical doors.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_void};
use std::os::raw::c_int;

use crate::doom::c_ffi as cffi;
use crate::doom::c_ffi::mobj_t;
use crate::doom::d_player::PlayerT;
use crate::doom::i_timer::TICRATE;
use crate::doom::m_fixed::{fixed_t, FRACUNIT};
use crate::doom::p_floor::T_MovePlane;
use crate::doom::p_lights::{line_t, sector_t};
use crate::doom::p_plats::{plat_t, T_PlatRaise};
use crate::doom::p_setup::{sectors, sides};
use crate::doom::p_spec::{P_FindLowestCeilingSurrounding, P_FindSectorFromLineTag};
use crate::doom::p_tick::{thinker_t, P_AddThinker, P_RemoveThinker};
use crate::doom::s_sound::S_StartSound;
use crate::doom::z_zone::{Z_Malloc, PU_LEVSPEC};

const VDOORSPEED: fixed_t = FRACUNIT * 2;
const VDOORWAIT: c_int = 150;

// vldoor_e enum values
const vld_normal: c_int = 0;
const vld_close30ThenOpen: c_int = 1;
const vld_close: c_int = 2;
const vld_open: c_int = 3;
const vld_raiseIn5Mins: c_int = 4;
const vld_blazeRaise: c_int = 5;
const vld_blazeOpen: c_int = 6;
const vld_blazeClose: c_int = 7;

// result_e enum values
const result_ok: c_int = 0;
const result_crushed: c_int = 1;
const result_pastdest: c_int = 2;

// sfx enum values
const sfx_doropn: c_int = 20;
const sfx_dorcls: c_int = 21;
const sfx_oof: c_int = 34;
const sfx_bdopn: c_int = 88;
const sfx_bdcls: c_int = 89;

// card indices
const it_bluecard: usize = 0;
const it_yellowcard: usize = 1;
const it_redcard: usize = 2;
const it_blueskull: usize = 3;
const it_yellowskull: usize = 4;
const it_redskull: usize = 5;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct vldoor_t {
    pub thinker: thinker_t,
    pub r#type: c_int,
    _pad0: [u8; 4],
    pub sector: *mut sector_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub direction: c_int,
    pub topwait: c_int,
    pub topcountdown: c_int,
}

#[cfg(target_pointer_width = "64")]
mod layout_checks {
    use super::*;
    const _: () = assert!(std::mem::size_of::<vldoor_t>() == 64);
    const _: () = assert!(std::mem::offset_of!(vldoor_t, thinker) == 0);
    const _: () = assert!(std::mem::offset_of!(vldoor_t, r#type) == 24);
    const _: () = assert!(std::mem::offset_of!(vldoor_t, sector) == 32);
    const _: () = assert!(std::mem::offset_of!(vldoor_t, topheight) == 40);
    const _: () = assert!(std::mem::offset_of!(vldoor_t, speed) == 44);
    const _: () = assert!(std::mem::offset_of!(vldoor_t, direction) == 48);
    const _: () = assert!(std::mem::offset_of!(vldoor_t, topwait) == 52);
    const _: () = assert!(std::mem::offset_of!(vldoor_t, topcountdown) == 56);
}

#[no_mangle]
pub unsafe extern "C" fn T_VerticalDoor(door: *mut vldoor_t) {
    let res: c_int;

    match (*door).direction {
        0 => {
            // WAITING
            (*door).topcountdown -= 1;
            if (*door).topcountdown == 0 {
                match (*door).r#type {
                    x if x == vld_blazeRaise => {
                        (*door).direction = -1;
                        S_StartSound(
                            &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                            sfx_bdcls,
                        );
                    }
                    x if x == vld_normal => {
                        (*door).direction = -1;
                        S_StartSound(
                            &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                            sfx_dorcls,
                        );
                    }
                    x if x == vld_close30ThenOpen => {
                        (*door).direction = 1;
                        S_StartSound(
                            &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                            sfx_doropn,
                        );
                    }
                    _ => {}
                }
            }
        }
        2 => {
            // INITIAL WAIT
            (*door).topcountdown -= 1;
            if (*door).topcountdown == 0 {
                match (*door).r#type {
                    x if x == vld_raiseIn5Mins => {
                        (*door).direction = 1;
                        (*door).r#type = vld_normal;
                        S_StartSound(
                            &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                            sfx_doropn,
                        );
                    }
                    _ => {}
                }
            }
        }
        -1 => {
            // DOWN
            res = T_MovePlane(
                (*door).sector,
                (*door).speed,
                (*(*door).sector).floorheight,
                0,
                1,
                -1,
            );
            if res == result_pastdest {
                match (*door).r#type {
                    x if x == vld_blazeRaise || x == vld_blazeClose => {
                        (*(*door).sector).specialdata = std::ptr::null_mut();
                        P_RemoveThinker(&mut (*door).thinker);
                        S_StartSound(
                            &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                            sfx_bdcls,
                        );
                    }
                    x if x == vld_normal || x == vld_close => {
                        (*(*door).sector).specialdata = std::ptr::null_mut();
                        P_RemoveThinker(&mut (*door).thinker);
                    }
                    x if x == vld_close30ThenOpen => {
                        (*door).direction = 0;
                        (*door).topcountdown = TICRATE * 30;
                    }
                    _ => {}
                }
            } else if res == result_crushed {
                match (*door).r#type {
                    x if x == vld_blazeClose || x == vld_close => {
                        // DO NOT GO BACK UP!
                    }
                    _ => {
                        (*door).direction = 1;
                        S_StartSound(
                            &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                            sfx_doropn,
                        );
                    }
                }
            }
        }
        1 => {
            // UP
            res = T_MovePlane((*door).sector, (*door).speed, (*door).topheight, 0, 1, 1);
            if res == result_pastdest {
                match (*door).r#type {
                    x if x == vld_blazeRaise || x == vld_normal => {
                        (*door).direction = 0;
                        (*door).topcountdown = (*door).topwait;
                    }
                    x if x == vld_close30ThenOpen || x == vld_blazeOpen || x == vld_open => {
                        (*(*door).sector).specialdata = std::ptr::null_mut();
                        P_RemoveThinker(&mut (*door).thinker);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn EV_DoLockedDoor(
    line: *mut line_t,
    r#type: c_int,
    thing: *mut mobj_t,
) -> c_int {
    let p = (*thing).player as *mut PlayerT;
    if p.is_null() {
        return 0;
    }

    match (*line).special as c_int {
        99 | 133 => {
            if p.is_null() {
                return 0;
            }
            if (*p).cards[it_bluecard] == 0 && (*p).cards[it_blueskull] == 0 {
                (*p).message = cstr!("You need a blue key to activate this object");
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return 0;
            }
        }
        134 | 135 => {
            if p.is_null() {
                return 0;
            }
            if (*p).cards[it_redcard] == 0 && (*p).cards[it_redskull] == 0 {
                (*p).message = cstr!("You need a red key to activate this object");
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return 0;
            }
        }
        136 | 137 => {
            if p.is_null() {
                return 0;
            }
            if (*p).cards[it_yellowcard] == 0 && (*p).cards[it_yellowskull] == 0 {
                (*p).message = cstr!("You need a yellow key to activate this object");
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return 0;
            }
        }
        _ => {}
    }

    EV_DoDoor(line, r#type)
}

#[no_mangle]
pub unsafe extern "C" fn EV_DoDoor(line: *mut line_t, r#type: c_int) -> c_int {
    let mut secnum: c_int = -1;
    let mut rtn: c_int = 0;

    while {
        secnum = P_FindSectorFromLineTag(line as *mut cffi::line_t, secnum);
        secnum
    } >= 0
    {
        let sec = sectors.add(secnum as usize);
        if !(*sec).specialdata.is_null() {
            continue;
        }

        rtn = 1;
        let door = Z_Malloc(
            std::mem::size_of::<vldoor_t>() as c_int,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut vldoor_t;
        P_AddThinker(&mut (*door).thinker);
        (*sec).specialdata = door as *mut c_void;

        (*door).thinker.function.acp1 = Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut vldoor_t),
            unsafe extern "C" fn(*mut c_void),
        >(T_VerticalDoor));
        (*door).sector = sec as *mut sector_t;
        (*door).r#type = r#type;
        (*door).topwait = VDOORWAIT;
        (*door).speed = VDOORSPEED;

        match r#type {
            x if x == vld_blazeClose => {
                (*door).topheight = P_FindLowestCeilingSurrounding(sec);
                (*door).topheight -= 4 * FRACUNIT;
                (*door).direction = -1;
                (*door).speed = VDOORSPEED * 4;
                S_StartSound(
                    &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_bdcls,
                );
            }
            x if x == vld_close => {
                (*door).topheight = P_FindLowestCeilingSurrounding(sec);
                (*door).topheight -= 4 * FRACUNIT;
                (*door).direction = -1;
                S_StartSound(
                    &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_dorcls,
                );
            }
            x if x == vld_close30ThenOpen => {
                (*door).topheight = (*sec).ceilingheight;
                (*door).direction = -1;
                S_StartSound(
                    &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_dorcls,
                );
            }
            x if x == vld_blazeRaise || x == vld_blazeOpen => {
                (*door).direction = 1;
                (*door).topheight = P_FindLowestCeilingSurrounding(sec);
                (*door).topheight -= 4 * FRACUNIT;
                (*door).speed = VDOORSPEED * 4;
                if (*door).topheight != (*sec).ceilingheight {
                    S_StartSound(
                        &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                        sfx_bdopn,
                    );
                }
            }
            x if x == vld_normal || x == vld_open => {
                (*door).direction = 1;
                (*door).topheight = P_FindLowestCeilingSurrounding(sec);
                (*door).topheight -= 4 * FRACUNIT;
                if (*door).topheight != (*sec).ceilingheight {
                    S_StartSound(
                        &(*(*door).sector).soundorg as *const [u8; 40] as *mut c_void,
                        sfx_doropn,
                    );
                }
            }
            _ => {}
        }
    }
    rtn
}

#[no_mangle]
pub unsafe extern "C" fn EV_VerticalDoor(line: *mut line_t, thing: *mut mobj_t) {
    let side = 0;

    // Check for locks
    let player = (*thing).player as *mut PlayerT;

    match (*line).special as c_int {
        26 | 32 => {
            if player.is_null() {
                return;
            }
            if (*player).cards[it_bluecard] == 0 && (*player).cards[it_blueskull] == 0 {
                (*player).message = cstr!("You need a blue key to open this door");
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return;
            }
        }
        27 | 34 => {
            if player.is_null() {
                return;
            }
            if (*player).cards[it_yellowcard] == 0 && (*player).cards[it_yellowskull] == 0 {
                (*player).message = cstr!("You need a yellow key to open this door");
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return;
            }
        }
        28 | 33 => {
            if player.is_null() {
                return;
            }
            if (*player).cards[it_redcard] == 0 && (*player).cards[it_redskull] == 0 {
                (*player).message = cstr!("You need a red key to open this door");
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return;
            }
        }
        _ => {}
    }

    let sec = (*sides.offset((*line).sidenum[(side ^ 1) as usize] as isize)).sector;

    if !(*sec).specialdata.is_null() {
        let door = (*sec).specialdata as *mut vldoor_t;
        match (*line).special as c_int {
            1 | 26 | 27 | 28 | 117 => {
                if (*door).direction == -1 {
                    (*door).direction = 1;
                } else {
                    if (*thing).player.is_null() {
                        return;
                    }
                    let t_vdoor = Some(core::mem::transmute::<
                        unsafe extern "C" fn(*mut vldoor_t),
                        unsafe extern "C" fn(*mut c_void),
                    >(T_VerticalDoor));
                    let t_plat = Some(core::mem::transmute::<
                        unsafe extern "C" fn(*mut plat_t),
                        unsafe extern "C" fn(*mut c_void),
                    >(T_PlatRaise));
                    if (*door).thinker.function.acp1 == t_vdoor {
                        (*door).direction = -1;
                    } else if (*door).thinker.function.acp1 == t_plat {
                        let plat = door as *mut plat_t;
                        (*plat).wait = -1;
                    } else {
                        eprintln!("EV_VerticalDoor: Tried to close something that wasn't a door.");
                        (*door).direction = -1;
                    }
                }
                return;
            }
            _ => {}
        }
    }

    // for proper sound
    match (*line).special as c_int {
        117 | 118 => {
            S_StartSound(
                &(*sec).soundorg as *const [u8; 40] as *mut c_void,
                sfx_bdopn,
            );
        }
        1 | 31 => {
            S_StartSound(
                &(*sec).soundorg as *const [u8; 40] as *mut c_void,
                sfx_doropn,
            );
        }
        _ => {
            S_StartSound(
                &(*sec).soundorg as *const [u8; 40] as *mut c_void,
                sfx_doropn,
            );
        }
    }

    // new door thinker
    let door = Z_Malloc(
        std::mem::size_of::<vldoor_t>() as c_int,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut vldoor_t;
    P_AddThinker(&mut (*door).thinker);
    (*sec).specialdata = door as *mut c_void;
    (*door).thinker.function.acp1 = Some(core::mem::transmute::<
        unsafe extern "C" fn(*mut vldoor_t),
        unsafe extern "C" fn(*mut c_void),
    >(T_VerticalDoor));
    (*door).sector = sec as *mut sector_t;
    (*door).direction = 1;
    (*door).speed = VDOORSPEED;
    (*door).topwait = VDOORWAIT;

    match (*line).special as c_int {
        1 | 26 | 27 | 28 => {
            (*door).r#type = vld_normal;
        }
        31..=34 => {
            (*door).r#type = vld_open;
            (*line).special = 0;
        }
        117 => {
            (*door).r#type = vld_blazeRaise;
            (*door).speed = VDOORSPEED * 4;
        }
        118 => {
            (*door).r#type = vld_blazeOpen;
            (*line).special = 0;
            (*door).speed = VDOORSPEED * 4;
        }
        _ => {}
    }

    // find the top and bottom of the movement range
    (*door).topheight = P_FindLowestCeilingSurrounding(sec);
    (*door).topheight -= 4 * FRACUNIT;
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnDoorCloseIn30(sec: *mut sector_t) {
    let door = Z_Malloc(
        std::mem::size_of::<vldoor_t>() as c_int,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut vldoor_t;

    P_AddThinker(&mut (*door).thinker);

    (*sec).specialdata = door as *mut c_void;
    (*sec).special = 0;

    (*door).thinker.function.acp1 = Some(core::mem::transmute::<
        unsafe extern "C" fn(*mut vldoor_t),
        unsafe extern "C" fn(*mut c_void),
    >(T_VerticalDoor));
    (*door).sector = sec;
    (*door).direction = 0;
    (*door).r#type = vld_normal;
    (*door).speed = VDOORSPEED;
    (*door).topcountdown = 30 * TICRATE;
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnDoorRaiseIn5Mins(sec: *mut sector_t, _secnum: c_int) {
    let door = Z_Malloc(
        std::mem::size_of::<vldoor_t>() as c_int,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut vldoor_t;

    P_AddThinker(&mut (*door).thinker);

    (*sec).specialdata = door as *mut c_void;
    (*sec).special = 0;

    (*door).thinker.function.acp1 = Some(core::mem::transmute::<
        unsafe extern "C" fn(*mut vldoor_t),
        unsafe extern "C" fn(*mut c_void),
    >(T_VerticalDoor));
    (*door).sector = sec;
    (*door).direction = 2;
    (*door).r#type = vld_raiseIn5Mins;
    (*door).speed = VDOORSPEED;
    (*door).topheight = P_FindLowestCeilingSurrounding(sec as *mut cffi::sector_t);
    (*door).topheight -= 4 * FRACUNIT;
    (*door).topwait = VDOORWAIT;
    (*door).topcountdown = 5 * 60 * TICRATE;
}

/// Anchor function referenced from `doomgeneric_Create` to ensure all
/// `#[no_mangle]` door functions survive link-time dead-code elimination.
#[no_mangle]
pub unsafe extern "C" fn P_Doors_Link_Anchor() {
    let _ = T_VerticalDoor as *const () as usize;
    let _ = EV_DoLockedDoor as *const () as usize;
    let _ = EV_DoDoor as *const () as usize;
    let _ = EV_VerticalDoor as *const () as usize;
    let _ = P_SpawnDoorCloseIn30 as *const () as usize;
    let _ = P_SpawnDoorRaiseIn5Mins as *const () as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const VLDOOR_T_SIZEOF: usize = 64;
    const VLDOOR_T_THINKER: usize = 0;
    const VLDOOR_T_TYPE: usize = 24;
    const VLDOOR_T_SECTOR: usize = 32;
    const VLDOOR_T_TOPHEIGHT: usize = 40;
    const VLDOOR_T_SPEED: usize = 44;
    const VLDOOR_T_DIRECTION: usize = 48;
    const VLDOOR_T_TOPWAIT: usize = 52;
    const VLDOOR_T_TOPCOUNTDOWN: usize = 56;

    #[test]
    fn vldoor_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<vldoor_t>(),
            VLDOOR_T_SIZEOF,
            "vldoor_t size mismatch: Rust={}, expected={}",
            std::mem::size_of::<vldoor_t>(),
            VLDOOR_T_SIZEOF,
        );
        assert_eq!(std::mem::offset_of!(vldoor_t, thinker), VLDOOR_T_THINKER);
        assert_eq!(std::mem::offset_of!(vldoor_t, r#type), VLDOOR_T_TYPE);
        assert_eq!(std::mem::offset_of!(vldoor_t, sector), VLDOOR_T_SECTOR);
        assert_eq!(
            std::mem::offset_of!(vldoor_t, topheight),
            VLDOOR_T_TOPHEIGHT
        );
        assert_eq!(std::mem::offset_of!(vldoor_t, speed), VLDOOR_T_SPEED);
        assert_eq!(
            std::mem::offset_of!(vldoor_t, direction),
            VLDOOR_T_DIRECTION
        );
        assert_eq!(std::mem::offset_of!(vldoor_t, topwait), VLDOOR_T_TOPWAIT);
        assert_eq!(
            std::mem::offset_of!(vldoor_t, topcountdown),
            VLDOOR_T_TOPCOUNTDOWN
        );
    }
}
