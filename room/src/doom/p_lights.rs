//! Rust port of vendor/doomgeneric/p_lights.c.
//!
//! Sector lighting effects: fire flicker, light flash, strobe, and glow.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_void;
use std::os::raw::c_int;

use crate::doom::c_ffi as cffi;
use crate::doom::m_random::P_Random;
use crate::doom::p_setup::{numsectors, sectors};
use crate::doom::p_spec::{getNextSector, P_FindMinSurroundingLight, P_FindSectorFromLineTag};
use crate::doom::p_tick::{thinker_t, P_AddThinker};
use crate::doom::z_zone::{Z_Malloc, PU_LEVSPEC};
const GLOWSPEED: c_int = 8;
const STROBEBRIGHT: c_int = 5;
const FASTDARK: c_int = 15;
const SLOWDARK: c_int = 35;
//
// We only need fields up to `specialdata`.  Verified against C layout
// on x86_64 Linux via layout_probe:
//   sector_t size=128, lightlevel@12, special@14, specialdata@104

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sector_t {
    pub floorheight: c_int,
    pub ceilingheight: c_int,
    pub floorpic: i16,
    pub ceilingpic: i16,
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
    _pad0: [u8; 2],
    pub soundtraversed: c_int,
    pub soundtarget: *mut c_void,
    pub blockbox: [c_int; 4],
    pub soundorg: [u8; 40], // degenmobj_t (opaque)
    pub validcount: c_int,
    pub thinglist: *mut c_void,
    pub specialdata: *mut c_void,
    pub linecount: c_int,
    pub lines: *mut *mut line_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct fireflicker_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: c_int,
    pub maxlight: c_int,
    pub minlight: c_int,
    _pad: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct lightflash_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: c_int,
    pub maxlight: c_int,
    pub minlight: c_int,
    pub maxtime: c_int,
    pub mintime: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct strobe_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: c_int,
    pub minlight: c_int,
    pub maxlight: c_int,
    pub darktime: c_int,
    pub brighttime: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct glow_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub minlight: c_int,
    pub maxlight: c_int,
    pub direction: c_int,
    _pad: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct line_t {
    pub v1: *mut c_void,
    pub v2: *mut c_void,
    pub dx: c_int,
    pub dy: c_int,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
    pub bbox: [c_int; 4],
    pub slopetype: c_int,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub validcount: c_int,
    pub specialdata: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn T_FireFlicker(flick: *mut fireflicker_t) {
    (*flick).count -= 1;
    if (*flick).count != 0 {
        return;
    }

    let amount = (P_Random() & 3) * 16;
    let sec = &mut *(*flick).sector;

    if (sec.lightlevel as c_int) - amount < (*flick).minlight {
        sec.lightlevel = (*flick).minlight as i16;
    } else {
        sec.lightlevel = ((*flick).maxlight - amount) as i16;
    }

    (*flick).count = 4;
}

#[no_mangle]
pub extern "C" fn P_SpawnFireFlicker(sector: *mut sector_t) {
    unsafe {
        (*sector).special = 0;

        let flick = Z_Malloc(
            std::mem::size_of::<fireflicker_t>() as c_int,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut fireflicker_t;

        P_AddThinker(&mut (*flick).thinker);

        (*flick).thinker.function.acp1 = Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut fireflicker_t),
            unsafe extern "C" fn(*mut c_void),
        >(T_FireFlicker));
        (*flick).sector = sector;
        (*flick).maxlight = (*sector).lightlevel as c_int;
        (*flick).minlight =
            P_FindMinSurroundingLight(sector as *mut cffi::sector_t, (*sector).lightlevel as c_int)
                + 16;
        (*flick).count = 4;
    }
}

#[no_mangle]
pub unsafe extern "C" fn T_LightFlash(flash: *mut lightflash_t) {
    (*flash).count -= 1;
    if (*flash).count != 0 {
        return;
    }

    let sec = &mut *(*flash).sector;

    if sec.lightlevel as c_int == (*flash).maxlight {
        sec.lightlevel = (*flash).minlight as i16;
        (*flash).count = (P_Random() & (*flash).mintime) + 1;
    } else {
        sec.lightlevel = (*flash).maxlight as i16;
        (*flash).count = (P_Random() & (*flash).maxtime) + 1;
    }
}

#[no_mangle]
pub extern "C" fn P_SpawnLightFlash(sector: *mut sector_t) {
    unsafe {
        (*sector).special = 0;

        let flash = Z_Malloc(
            std::mem::size_of::<lightflash_t>() as c_int,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut lightflash_t;

        P_AddThinker(&mut (*flash).thinker);

        (*flash).thinker.function.acp1 = Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut lightflash_t),
            unsafe extern "C" fn(*mut c_void),
        >(T_LightFlash));
        (*flash).sector = sector;
        (*flash).maxlight = (*sector).lightlevel as c_int;
        (*flash).minlight =
            P_FindMinSurroundingLight(sector as *mut cffi::sector_t, (*sector).lightlevel as c_int);
        (*flash).maxtime = 64;
        (*flash).mintime = 7;
        (*flash).count = (P_Random() & (*flash).maxtime) + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn T_StrobeFlash(flash: *mut strobe_t) {
    (*flash).count -= 1;
    if (*flash).count != 0 {
        return;
    }

    let sec = &mut *(*flash).sector;

    if sec.lightlevel as c_int == (*flash).minlight {
        sec.lightlevel = (*flash).maxlight as i16;
        (*flash).count = (*flash).brighttime;
    } else {
        sec.lightlevel = (*flash).minlight as i16;
        (*flash).count = (*flash).darktime;
    }
}

#[no_mangle]
pub extern "C" fn P_SpawnStrobeFlash(sector: *mut sector_t, fastOrSlow: c_int, inSync: c_int) {
    unsafe {
        let flash = Z_Malloc(
            std::mem::size_of::<strobe_t>() as c_int,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut strobe_t;

        P_AddThinker(&mut (*flash).thinker);

        (*flash).sector = sector;
        (*flash).darktime = fastOrSlow;
        (*flash).brighttime = STROBEBRIGHT;
        (*flash).thinker.function.acp1 = Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut strobe_t),
            unsafe extern "C" fn(*mut c_void),
        >(T_StrobeFlash));
        (*flash).maxlight = (*sector).lightlevel as c_int;
        (*flash).minlight =
            P_FindMinSurroundingLight(sector as *mut cffi::sector_t, (*sector).lightlevel as c_int);

        if (*flash).minlight == (*flash).maxlight {
            (*flash).minlight = 0;
        }

        (*sector).special = 0;

        if inSync == 0 {
            (*flash).count = (P_Random() & 7) + 1;
        } else {
            (*flash).count = 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn EV_StartLightStrobing(line: *mut line_t) {
    unsafe {
        let mut secnum: c_int = -1;
        loop {
            secnum = P_FindSectorFromLineTag(line as *mut cffi::line_t, secnum);
            if secnum < 0 {
                break;
            }
            let sec = sectors.add(secnum as usize);
            if !(*sec).specialdata.is_null() {
                continue;
            }
            P_SpawnStrobeFlash(sec as *mut sector_t, SLOWDARK, 0);
        }
    }
}

#[no_mangle]
pub extern "C" fn EV_TurnTagLightsOff(line: *mut line_t) {
    unsafe {
        for j in 0..numsectors as usize {
            let sec = sectors.add(j);
            if (*sec).tag != (*line).tag {
                continue;
            }

            let mut min = (*sec).lightlevel as c_int;
            for i in 0..(*sec).linecount as usize {
                let templine = *(*sec).lines.add(i);
                let tsec = getNextSector(templine as *mut cffi::line_t, sec);
                if tsec.is_null() {
                    continue;
                }
                let tl = (*tsec).lightlevel as c_int;
                if tl < min {
                    min = tl;
                }
            }
            (*sec).lightlevel = min as i16;
        }
    }
}

#[no_mangle]
pub extern "C" fn EV_LightTurnOn(line: *mut line_t, bright: c_int) {
    unsafe {
        let mut bright = bright;
        for i in 0..numsectors as usize {
            let sec = sectors.add(i);
            if (*sec).tag != (*line).tag {
                continue;
            }

            if bright == 0 {
                for j in 0..(*sec).linecount as usize {
                    let templine = *(*sec).lines.add(j);
                    let temp = getNextSector(templine as *mut cffi::line_t, sec);
                    if temp.is_null() {
                        continue;
                    }
                    let tl = (*temp).lightlevel as c_int;
                    if tl > bright {
                        bright = tl;
                    }
                }
            }
            (*sec).lightlevel = bright as i16;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn T_Glow(g: *mut glow_t) {
    let sec = &mut *(*g).sector;
    match (*g).direction {
        -1 => {
            sec.lightlevel -= GLOWSPEED as i16;
            if (sec.lightlevel as c_int) <= (*g).minlight {
                sec.lightlevel += GLOWSPEED as i16;
                (*g).direction = 1;
            }
        }
        1 => {
            sec.lightlevel += GLOWSPEED as i16;
            if (sec.lightlevel as c_int) >= (*g).maxlight {
                sec.lightlevel -= GLOWSPEED as i16;
                (*g).direction = -1;
            }
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn P_SpawnGlowingLight(sector: *mut sector_t) {
    unsafe {
        let g = Z_Malloc(
            std::mem::size_of::<glow_t>() as c_int,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut glow_t;

        P_AddThinker(&mut (*g).thinker);

        (*g).sector = sector;
        (*g).minlight =
            P_FindMinSurroundingLight(sector as *mut cffi::sector_t, (*sector).lightlevel as c_int);
        (*g).maxlight = (*sector).lightlevel as c_int;
        (*g).thinker.function.acp1 = Some(core::mem::transmute::<
            unsafe extern "C" fn(*mut glow_t),
            unsafe extern "C" fn(*mut c_void),
        >(T_Glow));
        (*g).direction = -1;

        (*sector).special = 0;
    }
}

/// Anchor function referenced from `doomgeneric_Create` to ensure all
/// `#[no_mangle]` light functions survive link-time dead-code elimination.
#[no_mangle]
pub unsafe extern "C" fn P_Lights_Link_Anchor() {
    // Force the linker to include every exported symbol from this module.
    let _ = T_FireFlicker as *const () as usize;
    let _ = T_LightFlash as *const () as usize;
    let _ = T_StrobeFlash as *const () as usize;
    let _ = T_Glow as *const () as usize;
    let _ = P_SpawnFireFlicker as *const () as usize;
    let _ = P_SpawnLightFlash as *const () as usize;
    let _ = P_SpawnStrobeFlash as *const () as usize;
    let _ = EV_StartLightStrobing as *const () as usize;
    let _ = EV_TurnTagLightsOff as *const () as usize;
    let _ = EV_LightTurnOn as *const () as usize;
    let _ = P_SpawnGlowingLight as *const () as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const SECTOR_T_SIZEOF: usize = 128;
    const SECTOR_T_LIGHTLEVEL_OFFSET: usize = 12;
    const SECTOR_T_SPECIAL_OFFSET: usize = 14;
    const SECTOR_T_SPECIALDATA_OFFSET: usize = 104;
    const FIREFLICKER_T_SIZEOF: usize = 48;
    const LIGHTFLASH_T_SIZEOF: usize = 56;
    const STROBE_T_SIZEOF: usize = 56;
    const GLOW_T_SIZEOF: usize = 48;

    #[test]
    fn sector_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<sector_t>(),
            SECTOR_T_SIZEOF,
            "sector_t size mismatch: Rust={}, expected={}",
            std::mem::size_of::<sector_t>(),
            SECTOR_T_SIZEOF,
        );
        assert_eq!(
            std::mem::offset_of!(sector_t, lightlevel),
            SECTOR_T_LIGHTLEVEL_OFFSET,
        );
        assert_eq!(
            std::mem::offset_of!(sector_t, special),
            SECTOR_T_SPECIAL_OFFSET,
        );
        assert_eq!(
            std::mem::offset_of!(sector_t, specialdata),
            SECTOR_T_SPECIALDATA_OFFSET,
        );
    }

    #[test]
    fn fireflicker_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<fireflicker_t>(), FIREFLICKER_T_SIZEOF,);
    }

    #[test]
    fn lightflash_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<lightflash_t>(), LIGHTFLASH_T_SIZEOF,);
    }

    #[test]
    fn strobe_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<strobe_t>(), STROBE_T_SIZEOF,);
    }

    #[test]
    fn glow_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<glow_t>(), GLOW_T_SIZEOF,);
    }
}
