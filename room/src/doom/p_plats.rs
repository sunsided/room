//! Rust port of vendor/doomgeneric/p_plats.c.
//!
//! Plats (elevator platforms) code: raising/lowering.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_void;
use std::os::raw::c_int;

use crate::doom::c_ffi as cffi;
use crate::doom::i_timer::TICRATE;
use crate::doom::m_fixed::{fixed_t, FRACUNIT};
use crate::doom::m_random::P_Random;
use crate::doom::p_floor::{side_t, T_MovePlane};
use crate::doom::p_lights::{line_t, sector_t};
use crate::doom::p_setup::{sectors, sides};
use crate::doom::p_spec::{
    P_FindHighestFloorSurrounding, P_FindLowestFloorSurrounding, P_FindNextHighestFloor,
    P_FindSectorFromLineTag,
};
use crate::doom::p_tick::{leveltime, thinker_t, P_AddThinker, P_RemoveThinker};
use crate::doom::s_sound::S_StartSound;
use crate::doom::z_zone::{PU_LEVSPEC, Z_Malloc};
use crate::i_error;

const PLATSPEED: fixed_t = FRACUNIT;
const PLATWAIT: c_int = 3;
const MAXPLATS: usize = 30;

// result_e enum values
const result_ok: c_int = 0;
const result_crushed: c_int = 1;
const result_pastdest: c_int = 2;

// plat_e enum values
const up: c_int = 0;
const down: c_int = 1;
const waiting: c_int = 2;
const in_stasis: c_int = 3;

// plattype_e enum values
const perpetualRaise: c_int = 0;
const downWaitUpStay: c_int = 1;
const raiseAndChange: c_int = 2;
const raiseToNearestAndChange: c_int = 3;
const blazeDWUS: c_int = 4;

// sfx enum values
const sfx_pstart: c_int = 18;
const sfx_pstop: c_int = 19;
const sfx_stnmov: c_int = 22;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct plat_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub speed: fixed_t,
    pub low: fixed_t,
    pub high: fixed_t,
    pub wait: c_int,
    pub count: c_int,
    pub status: c_int,
    pub oldstatus: c_int,
    pub crush: c_int,
    pub tag: c_int,
    pub r#type: c_int,
}

#[cfg(target_pointer_width = "64")]
mod layout_checks {
    use super::*;
    const _: () = assert!(std::mem::size_of::<plat_t>() == 72);
    const _: () = assert!(std::mem::offset_of!(plat_t, thinker) == 0);
    const _: () = assert!(std::mem::offset_of!(plat_t, sector) == 24);
    const _: () = assert!(std::mem::offset_of!(plat_t, speed) == 32);
    const _: () = assert!(std::mem::offset_of!(plat_t, low) == 36);
    const _: () = assert!(std::mem::offset_of!(plat_t, high) == 40);
    const _: () = assert!(std::mem::offset_of!(plat_t, wait) == 44);
    const _: () = assert!(std::mem::offset_of!(plat_t, count) == 48);
    const _: () = assert!(std::mem::offset_of!(plat_t, status) == 52);
    const _: () = assert!(std::mem::offset_of!(plat_t, oldstatus) == 56);
    const _: () = assert!(std::mem::offset_of!(plat_t, crush) == 60);
    const _: () = assert!(std::mem::offset_of!(plat_t, tag) == 64);
    const _: () = assert!(std::mem::offset_of!(plat_t, r#type) == 68);
}

#[no_mangle]
pub static mut activeplats: [*mut plat_t; MAXPLATS] = [std::ptr::null_mut(); MAXPLATS];


#[no_mangle]
pub unsafe extern "C" fn T_PlatRaise(plat: *mut plat_t) {
    match (*plat).status {
        x if x == up => {
            let res = T_MovePlane(
                (*plat).sector,
                (*plat).speed,
                (*plat).high,
                (*plat).crush,
                0,
                1,
            );

            if (*plat).r#type == raiseAndChange || (*plat).r#type == raiseToNearestAndChange {
                if (leveltime & 7) == 0 {
                    S_StartSound(
                        &(*(*plat).sector).soundorg as *const [u8; 40] as *mut c_void,
                        sfx_stnmov,
                    );
                }
            }

            if res == result_crushed && (*plat).crush == 0 {
                (*plat).count = (*plat).wait;
                (*plat).status = down;
                S_StartSound(
                    &(*(*plat).sector).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_pstart,
                );
            } else if res == result_pastdest {
                (*plat).count = (*plat).wait;
                (*plat).status = waiting;
                S_StartSound(
                    &(*(*plat).sector).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_pstop,
                );

                match (*plat).r#type {
                    x if x == blazeDWUS
                        || x == downWaitUpStay
                        || x == raiseAndChange
                        || x == raiseToNearestAndChange =>
                    {
                        P_RemoveActivePlat(plat);
                    }
                    _ => {}
                }
            }
        }
        x if x == down => {
            let res = T_MovePlane((*plat).sector, (*plat).speed, (*plat).low, 0, 0, -1);

            if res == result_pastdest {
                (*plat).count = (*plat).wait;
                (*plat).status = waiting;
                S_StartSound(
                    &(*(*plat).sector).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_pstop,
                );
            }
        }
        x if x == waiting => {
            (*plat).count -= 1;
            if (*plat).count == 0 {
                if (*(*plat).sector).floorheight == (*plat).low {
                    (*plat).status = up;
                } else {
                    (*plat).status = down;
                }
                S_StartSound(
                    &(*(*plat).sector).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_pstart,
                );
            }
        }
        x if x == in_stasis => {}
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn EV_DoPlat(line: *mut line_t, plattype: c_int, amount: c_int) -> c_int {
    let mut secnum: c_int = -1;
    let mut rtn: c_int = 0;

    match plattype {
        x if x == perpetualRaise => {
            P_ActivateInStasis((*line).tag as c_int);
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
        let plat = Z_Malloc(
            std::mem::size_of::<plat_t>() as c_int,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut plat_t;
        P_AddThinker(&mut (*plat).thinker);

        (*plat).r#type = plattype;
        (*plat).sector = sec as *mut sector_t;
        (*plat).sector.as_mut().unwrap().specialdata = plat as *mut c_void;
        (*plat).thinker.function.acp1 = Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut plat_t),
            unsafe extern "C" fn(*mut c_void),
        >(T_PlatRaise));
        (*plat).crush = 0;
        (*plat).tag = (*line).tag as c_int;

        match plattype {
            x if x == raiseToNearestAndChange => {
                (*plat).speed = PLATSPEED / 2;
                let sidenum = (*line).sidenum[0] as isize;
                (*sec).floorpic = (*sides.offset(sidenum)).sector.as_mut().unwrap().floorpic;
                (*plat).high = P_FindNextHighestFloor(sec, (*sec).floorheight);
                (*plat).wait = 0;
                (*plat).status = up;
                (*sec).special = 0;
                S_StartSound(
                    &(*sec).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_stnmov,
                );
            }
            x if x == raiseAndChange => {
                (*plat).speed = PLATSPEED / 2;
                let sidenum = (*line).sidenum[0] as isize;
                (*sec).floorpic = (*sides.offset(sidenum)).sector.as_mut().unwrap().floorpic;
                (*plat).high = (*sec).floorheight + amount * FRACUNIT;
                (*plat).wait = 0;
                (*plat).status = up;
                S_StartSound(
                    &(*sec).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_stnmov,
                );
            }
            x if x == downWaitUpStay => {
                (*plat).speed = PLATSPEED * 4;
                (*plat).low = P_FindLowestFloorSurrounding(sec);
                if (*plat).low > (*sec).floorheight {
                    (*plat).low = (*sec).floorheight;
                }
                (*plat).high = (*sec).floorheight;
                (*plat).wait = TICRATE * PLATWAIT;
                (*plat).status = down;
                S_StartSound(
                    &(*sec).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_pstart,
                );
            }
            x if x == blazeDWUS => {
                (*plat).speed = PLATSPEED * 8;
                (*plat).low = P_FindLowestFloorSurrounding(sec);
                if (*plat).low > (*sec).floorheight {
                    (*plat).low = (*sec).floorheight;
                }
                (*plat).high = (*sec).floorheight;
                (*plat).wait = TICRATE * PLATWAIT;
                (*plat).status = down;
                S_StartSound(
                    &(*sec).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_pstart,
                );
            }
            x if x == perpetualRaise => {
                (*plat).speed = PLATSPEED;
                (*plat).low = P_FindLowestFloorSurrounding(sec);
                if (*plat).low > (*sec).floorheight {
                    (*plat).low = (*sec).floorheight;
                }
                (*plat).high = P_FindHighestFloorSurrounding(sec);
                if (*plat).high < (*sec).floorheight {
                    (*plat).high = (*sec).floorheight;
                }
                (*plat).wait = TICRATE * PLATWAIT;
                (*plat).status = P_Random() & 1;
                S_StartSound(
                    &(*sec).soundorg as *const [u8; 40] as *mut c_void,
                    sfx_pstart,
                );
            }
            _ => {}
        }
        P_AddActivePlat(plat);
    }
    rtn
}

#[no_mangle]
pub extern "C" fn P_ActivateInStasis(tag: c_int) {
    unsafe {
        for i in 0..MAXPLATS {
            if !activeplats[i].is_null()
                && (*activeplats[i]).tag == tag
                && (*activeplats[i]).status == in_stasis
            {
                (*activeplats[i]).status = (*activeplats[i]).oldstatus;
                (*activeplats[i]).thinker.function.acp1 = Some(core::mem::transmute::<
                    unsafe extern "C" fn(*mut plat_t),
                    unsafe extern "C" fn(*mut c_void),
                >(T_PlatRaise));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn EV_StopPlat(line: *mut line_t) {
    unsafe {
        for j in 0..MAXPLATS {
            if !activeplats[j].is_null()
                && (*activeplats[j]).status != in_stasis
                && (*activeplats[j]).tag == (*line).tag as c_int
            {
                (*activeplats[j]).oldstatus = (*activeplats[j]).status;
                (*activeplats[j]).status = in_stasis;
                (*activeplats[j]).thinker.function.acv = None;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn P_AddActivePlat(plat: *mut plat_t) {
    unsafe {
        for i in 0..MAXPLATS {
            if activeplats[i].is_null() {
                activeplats[i] = plat;
                return;
            }
        }
        i_error!("P_AddActivePlat: no more plats!");
    }
}

#[no_mangle]
pub extern "C" fn P_RemoveActivePlat(plat: *mut plat_t) {
    unsafe {
        for i in 0..MAXPLATS {
            if plat == activeplats[i] {
                (*activeplats[i]).sector.as_mut().unwrap().specialdata = std::ptr::null_mut();
                P_RemoveThinker(&mut (*activeplats[i]).thinker);
                activeplats[i] = std::ptr::null_mut();
                return;
            }
        }
        i_error!("P_RemoveActivePlat: can't find plat!");
    }
}

#[no_mangle]
pub extern "C" fn P_Plats_Link_Anchor() {
    let _ = T_PlatRaise as *const () as usize;
    let _ = EV_DoPlat as *const () as usize;
    let _ = P_ActivateInStasis as *const () as usize;
    let _ = EV_StopPlat as *const () as usize;
    let _ = P_AddActivePlat as *const () as usize;
    let _ = P_RemoveActivePlat as *const () as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const PLAT_T_SIZEOF: usize = 72;
    const PLAT_T_THINKER: usize = 0;
    const PLAT_T_SECTOR: usize = 24;
    const PLAT_T_SPEED: usize = 32;
    const PLAT_T_LOW: usize = 36;
    const PLAT_T_HIGH: usize = 40;
    const PLAT_T_WAIT: usize = 44;
    const PLAT_T_COUNT: usize = 48;
    const PLAT_T_STATUS: usize = 52;
    const PLAT_T_OLDSTATUS: usize = 56;
    const PLAT_T_CRUSH: usize = 60;
    const PLAT_T_TAG: usize = 64;
    const PLAT_T_TYPE: usize = 68;

    #[test]
    fn plat_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<plat_t>(), PLAT_T_SIZEOF);
        assert_eq!(std::mem::offset_of!(plat_t, thinker), PLAT_T_THINKER);
        assert_eq!(std::mem::offset_of!(plat_t, sector), PLAT_T_SECTOR);
        assert_eq!(std::mem::offset_of!(plat_t, speed), PLAT_T_SPEED);
        assert_eq!(std::mem::offset_of!(plat_t, low), PLAT_T_LOW);
        assert_eq!(std::mem::offset_of!(plat_t, high), PLAT_T_HIGH);
        assert_eq!(std::mem::offset_of!(plat_t, wait), PLAT_T_WAIT);
        assert_eq!(std::mem::offset_of!(plat_t, count), PLAT_T_COUNT);
        assert_eq!(std::mem::offset_of!(plat_t, status), PLAT_T_STATUS);
        assert_eq!(std::mem::offset_of!(plat_t, oldstatus), PLAT_T_OLDSTATUS);
        assert_eq!(std::mem::offset_of!(plat_t, crush), PLAT_T_CRUSH);
        assert_eq!(std::mem::offset_of!(plat_t, tag), PLAT_T_TAG);
        assert_eq!(std::mem::offset_of!(plat_t, r#type), PLAT_T_TYPE);
    }
}
