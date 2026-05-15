//! Rust port of vendor/doomgeneric/st_lib.c.
//!
//! Status bar widget library: number, percent, multi-icon, and binary-icon widgets.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_char;
use std::ffi::c_void;
use std::os::raw::c_int;

use crate::doom::v_video::patch_t;
use crate::doom::z_zone::PU_STATIC;
use crate::i_error;
const ST_HEIGHT: c_int = 32;
const ST_Y: c_int = 200 - ST_HEIGHT; // 168

/// Byte-swap for little-endian (SHORT macro from i_swap.h).
/// On x86_64 this is an identity cast.
#[inline(always)]
fn short_swap(v: i16) -> i16 {
    v
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct st_number_t {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub oldnum: c_int,
    pub num: *mut c_int,
    pub on: *mut c_int, // boolean* (c_int in C)
    pub p: *mut *mut patch_t,
    pub data: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct st_percent_t {
    pub n: st_number_t,
    pub p: *mut patch_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct st_multicon_t {
    pub x: c_int,
    pub y: c_int,
    pub oldinum: c_int,
    pub inum: *mut c_int,
    pub on: *mut c_int,
    pub p: *mut *mut patch_t,
    pub data: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct st_binicon_t {
    pub x: c_int,
    pub y: c_int,
    pub oldval: c_int,
    pub val: *mut c_int,
    pub on: *mut c_int,
    pub p: *mut patch_t,
    pub data: c_int,
}

#[no_mangle]
pub static mut sttminus: *mut patch_t = std::ptr::null_mut();

extern "C" {
    fn W_CacheLumpName(name: *const c_char, tag: c_int) -> *mut c_void;
    fn V_CopyRect(
        srcx: c_int,
        srcy: c_int,
        srcscr: *mut u8,
        width: c_int,
        height: c_int,
        destx: c_int,
        desty: c_int,
    );
    fn V_DrawPatch(x: c_int, y: c_int, patch: *mut patch_t);
    static mut st_backing_screen: *mut u8;
    static mut automapactive: c_int;
}

#[no_mangle]
pub extern "C" fn STlib_init() {
    unsafe {
        // DEH_String("STTMINUS") is identity — just pass the string.
        sttminus =
            W_CacheLumpName(b"STTMINUS\0".as_ptr() as *const c_char, PU_STATIC) as *mut patch_t;
    }
}

#[no_mangle]
pub extern "C" fn STlib_initNum(
    n: *mut st_number_t,
    x: c_int,
    y: c_int,
    pl: *mut *mut patch_t,
    num: *mut c_int,
    on: *mut c_int,
    width: c_int,
) {
    unsafe {
        (*n).x = x;
        (*n).y = y;
        (*n).oldnum = 0;
        (*n).width = width;
        (*n).num = num;
        (*n).on = on;
        (*n).p = pl;
    }
}

#[no_mangle]
pub extern "C" fn STlib_drawNum(n: *mut st_number_t, _refresh: c_int) {
    unsafe {
        let mut numdigits = (*n).width;
        let num = *(*n).num;

        let w = short_swap((**(*n).p).width) as c_int;
        let h = short_swap((**(*n).p).height) as c_int;
        let mut x = (*n).x;

        (*n).oldnum = num;

        let neg = num < 0;
        let mut num = num;

        if neg {
            if numdigits == 2 && num < -9 {
                num = -9;
            } else if numdigits == 3 && num < -99 {
                num = -99;
            }
            num = -num;
        }

        // clear the area
        x = (*n).x - numdigits * w;

        if (*n).y - ST_Y < 0 {
            i_error!("drawNum: n->y - ST_Y < 0");
        }

        V_CopyRect(
            x,
            (*n).y - ST_Y,
            st_backing_screen,
            w * numdigits,
            h,
            x,
            (*n).y,
        );

        // if non-number, do not draw it
        if num == 1994 {
            return;
        }

        x = (*n).x;

        // in the special case of 0, you draw 0
        if num == 0 {
            V_DrawPatch(x - w, (*n).y, *(*n).p);
        }

        // draw the new number
        while num != 0 && numdigits > 0 {
            x -= w;
            V_DrawPatch(x, (*n).y, *(*n).p.offset((num % 10) as isize));
            num /= 10;
            numdigits -= 1;
        }

        // draw a minus sign if necessary
        if neg {
            V_DrawPatch(x - 8, (*n).y, sttminus);
        }
    }
}

#[no_mangle]
pub extern "C" fn STlib_updateNum(n: *mut st_number_t, refresh: c_int) {
    unsafe {
        if *(*n).on != 0 {
            STlib_drawNum(n, refresh);
        }
    }
}

#[no_mangle]
pub extern "C" fn STlib_initPercent(
    p: *mut st_percent_t,
    x: c_int,
    y: c_int,
    pl: *mut *mut patch_t,
    num: *mut c_int,
    on: *mut c_int,
    percent: *mut patch_t,
) {
    unsafe {
        STlib_initNum(&mut (*p).n, x, y, pl, num, on, 3);
    }
    unsafe {
        (*p).p = percent;
    }
}

#[no_mangle]
pub extern "C" fn STlib_updatePercent(per: *mut st_percent_t, refresh: c_int) {
    unsafe {
        if refresh != 0 && *(*per).n.on != 0 {
            V_DrawPatch((*per).n.x, (*per).n.y, (*per).p);
        }
        STlib_updateNum(&mut (*per).n, refresh);
    }
}

#[no_mangle]
pub extern "C" fn STlib_initMultIcon(
    i: *mut st_multicon_t,
    x: c_int,
    y: c_int,
    il: *mut *mut patch_t,
    inum: *mut c_int,
    on: *mut c_int,
) {
    unsafe {
        (*i).x = x;
        (*i).y = y;
        (*i).oldinum = -1;
        (*i).inum = inum;
        (*i).on = on;
        (*i).p = il;
    }
}

#[no_mangle]
pub extern "C" fn STlib_updateMultIcon(mi: *mut st_multicon_t, refresh: c_int) {
    unsafe {
        if *(*mi).on != 0 && ((*mi).oldinum != *(*mi).inum || refresh != 0) && *(*mi).inum != -1 {
            if (*mi).oldinum != -1 {
                let old_patch = *(*mi).p.offset((*mi).oldinum as isize);
                let x = (*mi).x - short_swap((*old_patch).leftoffset) as c_int;
                let y = (*mi).y - short_swap((*old_patch).topoffset) as c_int;
                let w = short_swap((*old_patch).width) as c_int;
                let h = short_swap((*old_patch).height) as c_int;

                if y - ST_Y < 0 {
                    i_error!("updateMultIcon: y - ST_Y < 0");
                }

                V_CopyRect(x, y - ST_Y, st_backing_screen, w, h, x, y);
            }
            V_DrawPatch((*mi).x, (*mi).y, *(*mi).p.offset(*(*mi).inum as isize));
            (*mi).oldinum = *(*mi).inum;
        }
    }
}

#[no_mangle]
pub extern "C" fn STlib_initBinIcon(
    b: *mut st_binicon_t,
    x: c_int,
    y: c_int,
    i: *mut patch_t,
    val: *mut c_int,
    on: *mut c_int,
) {
    unsafe {
        (*b).x = x;
        (*b).y = y;
        (*b).oldval = 0;
        (*b).val = val;
        (*b).on = on;
        (*b).p = i;
    }
}

#[no_mangle]
pub extern "C" fn STlib_updateBinIcon(bi: *mut st_binicon_t, refresh: c_int) {
    unsafe {
        if *(*bi).on != 0 && ((*bi).oldval != *(*bi).val || refresh != 0) {
            let p = (*bi).p;
            let x = (*bi).x - short_swap((*p).leftoffset) as c_int;
            let y = (*bi).y - short_swap((*p).topoffset) as c_int;
            let w = short_swap((*p).width) as c_int;
            let h = short_swap((*p).height) as c_int;

            if y - ST_Y < 0 {
                i_error!("updateBinIcon: y - ST_Y < 0");
            }

            if *(*bi).val != 0 {
                V_DrawPatch((*bi).x, (*bi).y, p);
            } else {
                V_CopyRect(x, y - ST_Y, st_backing_screen, w, h, x, y);
            }

            (*bi).oldval = *(*bi).val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const ST_NUMBER_T_SIZEOF: usize = 48;
    const ST_PERCENT_T_SIZEOF: usize = 56;
    const ST_MULTICON_T_SIZEOF: usize = 48;
    const ST_BINICON_T_SIZEOF: usize = 48;

    #[test]
    fn st_number_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<st_number_t>(),
            ST_NUMBER_T_SIZEOF,
            "st_number_t size mismatch: Rust={}, expected={}",
            std::mem::size_of::<st_number_t>(),
            ST_NUMBER_T_SIZEOF,
        );
    }

    #[test]
    fn st_percent_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<st_percent_t>(),
            ST_PERCENT_T_SIZEOF,
            "st_percent_t size mismatch: Rust={}, expected={}",
            std::mem::size_of::<st_percent_t>(),
            ST_PERCENT_T_SIZEOF,
        );
    }

    #[test]
    fn st_multicon_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<st_multicon_t>(),
            ST_MULTICON_T_SIZEOF,
            "st_multicon_t size mismatch: Rust={}, expected={}",
            std::mem::size_of::<st_multicon_t>(),
            ST_MULTICON_T_SIZEOF,
        );
    }

    #[test]
    fn st_binicon_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<st_binicon_t>(),
            ST_BINICON_T_SIZEOF,
            "st_binicon_t size mismatch: Rust={}, expected={}",
            std::mem::size_of::<st_binicon_t>(),
            ST_BINICON_T_SIZEOF,
        );
    }
}
