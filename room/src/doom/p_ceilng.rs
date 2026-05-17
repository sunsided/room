//! Rust port of vendor/doomgeneric/p_ceilng.c.
//!
//! Ceiling animation: lowering, crushing, raising.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use crate::doom::sounds::Sfx;
use std::ffi::c_void;
use std::os::raw::c_int;

use crate::doom::c_ffi as cffi;
use crate::doom::m_fixed::{fixed_t, FRACUNIT};
use crate::doom::p_floor::T_MovePlane;
use crate::doom::p_lights::{line_t, sector_t};
use crate::doom::p_setup::sectors;
use crate::doom::p_spec::{P_FindHighestCeilingSurrounding, P_FindSectorFromLineTag};
use crate::doom::p_tick::{leveltime, thinker_t, P_AddThinker, P_RemoveThinker};
use crate::doom::s_sound::S_StartSound;
use crate::doom::z_zone::{Z_Malloc, PU_LEVSPEC};

const CEILSPEED: fixed_t = FRACUNIT;
pub const MAXCEILINGS: usize = 30;

// result_e enum values
const result_ok: c_int = 0;
const result_crushed: c_int = 1;
const result_pastdest: c_int = 2;

// ceiling_e enum values
const lowerToFloor: c_int = 0;
const raiseToHighest: c_int = 1;
const lowerAndCrush: c_int = 2;
const crushAndRaise: c_int = 3;
const fastCrushAndRaise: c_int = 4;
const silentCrushAndRaise: c_int = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ceiling_t {
    pub thinker: thinker_t,
    pub r#type: c_int,
    _pad0: [u8; 4],
    pub sector: *mut sector_t,
    pub bottomheight: fixed_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub crush: c_int,
    pub direction: c_int,
    pub tag: c_int,
    pub olddirection: c_int,
    _pad1: [u8; 4],
}

#[cfg(target_pointer_width = "64")]
mod layout_checks {
    use super::*;
    const _: () = assert!(std::mem::size_of::<ceiling_t>() == 72);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, thinker) == 0);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, r#type) == 24);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, sector) == 32);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, bottomheight) == 40);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, topheight) == 44);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, speed) == 48);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, crush) == 52);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, direction) == 56);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, tag) == 60);
    const _: () = assert!(std::mem::offset_of!(ceiling_t, olddirection) == 64);
}

#[no_mangle]
pub static mut activeceilings: [*mut ceiling_t; MAXCEILINGS] = [std::ptr::null_mut(); MAXCEILINGS];

#[no_mangle]
pub unsafe extern "C" fn T_MoveCeiling(ceiling: *mut ceiling_t) {
    let res = T_MovePlane(
        (*ceiling).sector,
        (*ceiling).speed,
        if (*ceiling).direction == 1 {
            (*ceiling).topheight
        } else {
            (*ceiling).bottomheight
        },
        (*ceiling).crush,
        1,
        (*ceiling).direction,
    );

    if (leveltime & 7) == 0 {
        match (*ceiling).r#type {
            x if x == silentCrushAndRaise => {}
            _ => {
                S_StartSound(
                    &(*(*ceiling).sector).soundorg as *const [u8; 40] as *mut c_void,
                    Sfx::Stnmov as c_int,
                );
            }
        }
    }

    if res == result_pastdest {
        match (*ceiling).r#type {
            x if x == raiseToHighest => {
                P_RemoveActiveCeiling(ceiling);
            }
            x if x == silentCrushAndRaise => {
                S_StartSound(
                    &(*(*ceiling).sector).soundorg as *const [u8; 40] as *mut c_void,
                    Sfx::Pstop as c_int,
                );
            }
            x if x == fastCrushAndRaise || x == crushAndRaise => {
                (*ceiling).direction = -1;
            }
            _ => {}
        }
    } else if res == result_crushed {
        match (*ceiling).r#type {
            x if x == silentCrushAndRaise || x == crushAndRaise || x == lowerAndCrush => {
                (*ceiling).speed = CEILSPEED / 8;
            }
            _ => {}
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn EV_DoCeiling(line: *mut line_t, ceilingtype: c_int) -> c_int {
    let mut secnum: c_int = -1;
    let mut rtn: c_int = 0;

    // Reactivate in-stasis ceilings for certain types.
    match ceilingtype {
        x if x == fastCrushAndRaise || x == silentCrushAndRaise || x == crushAndRaise => {
            P_ActivateInStasisCeiling(line);
        }
        _ => {}
    }

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
        let ceiling = Z_Malloc(
            std::mem::size_of::<ceiling_t>() as c_int,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut ceiling_t;
        P_AddThinker(&mut (*ceiling).thinker);
        (*sec).specialdata = ceiling as *mut c_void;
        (*ceiling).thinker.function.acp1 = Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut ceiling_t),
            unsafe extern "C" fn(*mut c_void),
        >(T_MoveCeiling));
        (*ceiling).sector = sec as *mut sector_t;
        (*ceiling).crush = 0;

        match ceilingtype {
            x if x == fastCrushAndRaise => {
                (*ceiling).crush = 1;
                (*ceiling).topheight = (*sec).ceilingheight;
                (*ceiling).bottomheight = (*sec).floorheight + (8 * FRACUNIT);
                (*ceiling).direction = -1;
                (*ceiling).speed = CEILSPEED * 2;
            }
            x if x == silentCrushAndRaise || x == crushAndRaise => {
                (*ceiling).crush = 1;
                (*ceiling).topheight = (*sec).ceilingheight;
            }
            x if x == lowerAndCrush || x == lowerToFloor => {
                (*ceiling).bottomheight = (*sec).floorheight;
                if ceilingtype != lowerToFloor {
                    (*ceiling).bottomheight += 8 * FRACUNIT;
                }
                (*ceiling).direction = -1;
                (*ceiling).speed = CEILSPEED;
            }
            x if x == raiseToHighest => {
                (*ceiling).topheight = P_FindHighestCeilingSurrounding(sec);
                (*ceiling).direction = 1;
                (*ceiling).speed = CEILSPEED;
            }
            _ => {}
        }

        (*ceiling).tag = (*sec).tag as c_int;
        (*ceiling).r#type = ceilingtype;
        P_AddActiveCeiling(ceiling);
    }
    rtn
}

#[no_mangle]
pub extern "C" fn P_AddActiveCeiling(c: *mut ceiling_t) {
    unsafe {
        for i in 0..MAXCEILINGS {
            if activeceilings[i].is_null() {
                activeceilings[i] = c;
                return;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn P_RemoveActiveCeiling(c: *mut ceiling_t) {
    unsafe {
        for i in 0..MAXCEILINGS {
            if activeceilings[i] == c {
                (*activeceilings[i]).sector.as_mut().unwrap().specialdata = std::ptr::null_mut();
                P_RemoveThinker(&mut (*activeceilings[i]).thinker);
                activeceilings[i] = std::ptr::null_mut();
                break;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn P_ActivateInStasisCeiling(line: *mut line_t) {
    unsafe {
        for i in 0..MAXCEILINGS {
            if !activeceilings[i].is_null()
                && (*activeceilings[i]).tag == (*line).tag as c_int
                && (*activeceilings[i]).direction == 0
            {
                (*activeceilings[i]).direction = (*activeceilings[i]).olddirection;
                (*activeceilings[i]).thinker.function.acp1 = Some(core::mem::transmute::<
                    unsafe extern "C" fn(*mut ceiling_t),
                    unsafe extern "C" fn(*mut c_void),
                >(T_MoveCeiling));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn EV_CeilingCrushStop(line: *mut line_t) -> c_int {
    unsafe {
        let mut rtn: c_int = 0;
        for i in 0..MAXCEILINGS {
            if !activeceilings[i].is_null()
                && (*activeceilings[i]).tag == (*line).tag as c_int
                && (*activeceilings[i]).direction != 0
            {
                (*activeceilings[i]).olddirection = (*activeceilings[i]).direction;
                (*activeceilings[i]).thinker.function.acv = None;
                (*activeceilings[i]).direction = 0;
                rtn = 1;
            }
        }
        rtn
    }
}

#[no_mangle]
pub extern "C" fn P_Ceilng_Link_Anchor() {
    let _ = T_MoveCeiling as *const () as usize;
    let _ = EV_DoCeiling as *const () as usize;
    let _ = P_AddActiveCeiling as *const () as usize;
    let _ = P_RemoveActiveCeiling as *const () as usize;
    let _ = P_ActivateInStasisCeiling as *const () as usize;
    let _ = EV_CeilingCrushStop as *const () as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const CEILING_T_SIZEOF: usize = 72;
    const CEILING_T_THINKER: usize = 0;
    const CEILING_T_TYPE: usize = 24;
    const CEILING_T_SECTOR: usize = 32;
    const CEILING_T_BOTTOMHEIGHT: usize = 40;
    const CEILING_T_TOPHEIGHT: usize = 44;
    const CEILING_T_SPEED: usize = 48;
    const CEILING_T_CRUSH: usize = 52;
    const CEILING_T_DIRECTION: usize = 56;
    const CEILING_T_TAG: usize = 60;
    const CEILING_T_OLDDIRECTION: usize = 64;

    #[test]
    fn ceiling_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<ceiling_t>(), CEILING_T_SIZEOF);
        assert_eq!(std::mem::offset_of!(ceiling_t, thinker), CEILING_T_THINKER);
        assert_eq!(std::mem::offset_of!(ceiling_t, r#type), CEILING_T_TYPE);
        assert_eq!(std::mem::offset_of!(ceiling_t, sector), CEILING_T_SECTOR);
        assert_eq!(
            std::mem::offset_of!(ceiling_t, bottomheight),
            CEILING_T_BOTTOMHEIGHT
        );
        assert_eq!(
            std::mem::offset_of!(ceiling_t, topheight),
            CEILING_T_TOPHEIGHT
        );
        assert_eq!(std::mem::offset_of!(ceiling_t, speed), CEILING_T_SPEED);
        assert_eq!(std::mem::offset_of!(ceiling_t, crush), CEILING_T_CRUSH);
        assert_eq!(
            std::mem::offset_of!(ceiling_t, direction),
            CEILING_T_DIRECTION
        );
        assert_eq!(std::mem::offset_of!(ceiling_t, tag), CEILING_T_TAG);
        assert_eq!(
            std::mem::offset_of!(ceiling_t, olddirection),
            CEILING_T_OLDDIRECTION
        );
    }
}
