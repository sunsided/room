//! Rust port of vendor/doomgeneric/f_wipe.c.
//!
//! Mission begin melt / wipe screen special effect.

#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::c_void;
use std::os::raw::c_int;

use crate::doom::m_random::M_Random;

use crate::doom::i_video::{SCREENHEIGHT, SCREENWIDTH};
use crate::doom::z_zone::PU_STATIC;

extern "C" {
    fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void;
    fn Z_Free(ptr: *mut c_void);
    fn I_ReadScreen(scr: *mut u8);
    fn V_DrawBlock(x: c_int, y: c_int, width: c_int, height: c_int, src: *mut u8);
    fn V_MarkRect(x: c_int, y: c_int, width: c_int, height: c_int);
    static mut I_VideoBuffer: *mut u8;
}

static mut GO: c_int = 0;
static mut WIPE_SCR_START: *mut u8 = std::ptr::null_mut();
static mut WIPE_SCR_END: *mut u8 = std::ptr::null_mut();
static mut WIPE_SCR: *mut u8 = std::ptr::null_mut();
static mut Y: *mut c_int = std::ptr::null_mut();

/// Transpose a width×height array of i16 from row-major to column-major
/// in-place.  Equivalent to `wipe_shittyColMajorXform`.
unsafe fn wipe_shitty_col_major_xform(array: *mut i16, width: c_int, height: c_int) {
    let total = (width * height) as usize;
    let dest = Z_Malloc(total as c_int * 2, PU_STATIC, std::ptr::null_mut()) as *mut i16;

    for y in 0..height {
        for x in 0..width {
            *dest.add((x * height + y) as usize) = *array.add((y * width + x) as usize);
        }
    }

    std::ptr::copy(dest, array, total);
    Z_Free(dest as *mut c_void);
}

unsafe extern "C" fn wipe_init_color_xform(width: c_int, height: c_int, _ticks: c_int) -> c_int {
    let len = (width * height) as usize;
    std::ptr::copy(WIPE_SCR_START, WIPE_SCR, len);
    0
}

unsafe extern "C" fn wipe_do_color_xform(width: c_int, height: c_int, ticks: c_int) -> c_int {
    let len = (width * height) as usize;
    let mut changed = false;

    for i in 0..len {
        let w = *WIPE_SCR.add(i);
        let e = *WIPE_SCR_END.add(i);

        if w != e {
            let newval: c_int;
            if w > e {
                newval = (w as c_int) - ticks;
                if newval < e as c_int {
                    *WIPE_SCR.add(i) = e;
                } else {
                    *WIPE_SCR.add(i) = newval as u8;
                }
                changed = true;
            } else {
                newval = (w as c_int) + ticks;
                if newval > e as c_int {
                    *WIPE_SCR.add(i) = e;
                } else {
                    *WIPE_SCR.add(i) = newval as u8;
                }
                changed = true;
            }
        }
    }

    if changed {
        0
    } else {
        1
    }
}

unsafe extern "C" fn wipe_exit_color_xform(_width: c_int, _height: c_int, _ticks: c_int) -> c_int {
    0
}

unsafe extern "C" fn wipe_init_melt(width: c_int, height: c_int, _ticks: c_int) -> c_int {
    let len = (width * height) as usize;
    std::ptr::copy(WIPE_SCR_START, WIPE_SCR, len);

    wipe_shitty_col_major_xform(WIPE_SCR_START as *mut i16, width / 2, height);
    wipe_shitty_col_major_xform(WIPE_SCR_END as *mut i16, width / 2, height);

    Y = Z_Malloc(
        width * std::mem::size_of::<c_int>() as c_int,
        PU_STATIC,
        std::ptr::null_mut(),
    ) as *mut c_int;

    *Y.add(0) = -((M_Random() % 16) as c_int);
    for i in 1..width as usize {
        let r = (M_Random() % 3) as c_int - 1;
        *Y.add(i) = *Y.add(i - 1) + r;
        if *Y.add(i) > 0 {
            *Y.add(i) = 0;
        } else if *Y.add(i) == -16 {
            *Y.add(i) = -15;
        }
    }

    0
}

unsafe extern "C" fn wipe_do_melt(width: c_int, height: c_int, ticks: c_int) -> c_int {
    let width = width / 2;
    let mut done = true;
    let mut ticks = ticks;

    while ticks > 0 {
        ticks -= 1;
        for i in 0..width as usize {
            if *Y.add(i) < 0 {
                *Y.add(i) += 1;
                done = false;
            } else if *Y.add(i) < height {
                let mut dy: c_int;
                if *Y.add(i) < 16 {
                    dy = *Y.add(i) + 1;
                } else {
                    dy = 8;
                }
                if *Y.add(i) + dy >= height {
                    dy = height - *Y.add(i);
                }

                let yi = *Y.add(i) as usize;
                let src = (WIPE_SCR_END as *mut i16).add(i * height as usize + yi);
                let dst = (WIPE_SCR as *mut i16).add(yi * width as usize + i);
                for j in 0..dy as usize {
                    *dst.add(j * width as usize) = *src.add(j);
                }

                *Y.add(i) += dy;

                let src = (WIPE_SCR_START as *mut i16).add(i * height as usize);
                let dst = (WIPE_SCR as *mut i16).add((*Y.add(i) as usize) * width as usize + i);
                for j in 0..(height as usize - *Y.add(i) as usize) {
                    *dst.add(j * width as usize) = *src.add(j);
                }

                done = false;
            }
        }
    }

    if done {
        1
    } else {
        0
    }
}

unsafe extern "C" fn wipe_exit_melt(_width: c_int, _height: c_int, _ticks: c_int) -> c_int {
    Z_Free(Y as *mut c_void);
    Z_Free(WIPE_SCR_START as *mut c_void);
    Z_Free(WIPE_SCR_END as *mut c_void);
    Y = std::ptr::null_mut();
    WIPE_SCR_START = std::ptr::null_mut();
    WIPE_SCR_END = std::ptr::null_mut();
    0
}

type WipeFn = unsafe extern "C" fn(c_int, c_int, c_int) -> c_int;

const WIPES: [WipeFn; 6] = [
    wipe_init_color_xform,
    wipe_do_color_xform,
    wipe_exit_color_xform,
    wipe_init_melt,
    wipe_do_melt,
    wipe_exit_melt,
];

#[no_mangle]
pub extern "C" fn wipe_StartScreen(_x: c_int, _y: c_int, _width: c_int, _height: c_int) -> c_int {
    unsafe {
        WIPE_SCR_START =
            Z_Malloc(SCREENWIDTH * SCREENHEIGHT, PU_STATIC, std::ptr::null_mut()) as *mut u8;
        I_ReadScreen(WIPE_SCR_START);
    }
    0
}

#[no_mangle]
pub extern "C" fn wipe_EndScreen(x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
    unsafe {
        WIPE_SCR_END =
            Z_Malloc(SCREENWIDTH * SCREENHEIGHT, PU_STATIC, std::ptr::null_mut()) as *mut u8;
        I_ReadScreen(WIPE_SCR_END);
        V_DrawBlock(x, y, width, height, WIPE_SCR_START);
    }
    0
}

#[no_mangle]
pub extern "C" fn wipe_ScreenWipe(
    wipeno: c_int,
    _x: c_int,
    _y: c_int,
    width: c_int,
    height: c_int,
    ticks: c_int,
) -> c_int {
    unsafe {
        if GO == 0 {
            GO = 1;
            WIPE_SCR = I_VideoBuffer;
            WIPES[(wipeno * 3) as usize](width, height, ticks);
        }

        V_MarkRect(0, 0, width, height);
        let rc = WIPES[(wipeno * 3 + 1) as usize](width, height, ticks);

        if rc != 0 {
            GO = 0;
            WIPES[(wipeno * 3 + 2) as usize](width, height, ticks);
        }

        if GO == 0 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wipe_fns_are_callable() {
        // Sanity: verify all six function pointers are distinct and
        // non-null at compile-time.
        let fns: [*const (); 6] = WIPES.map(|f| f as *const ());
        for i in 0..6 {
            for j in (i + 1)..6 {
                assert_ne!(fns[i], fns[j], "WIPES[{i}] == WIPES[{j}]");
            }
        }
    }
}
