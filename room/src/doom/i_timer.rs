#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::c_int;

pub const TICRATE: c_int = 35;

static mut BASETIME: u32 = 0;

extern "C" {
    fn DG_GetTicksMs() -> u32;
    fn DG_SleepMs(ms: u32);
}

#[no_mangle]
pub extern "C" fn I_GetTicks() -> c_int {
    unsafe { DG_GetTicksMs() as c_int }
}

#[no_mangle]
pub extern "C" fn I_GetTime() -> c_int {
    unsafe {
        let ticks = DG_GetTicksMs();
        if BASETIME == 0 {
            BASETIME = ticks;
        }
        ((ticks - BASETIME) * TICRATE as u32 / 1000) as c_int
    }
}

#[no_mangle]
pub extern "C" fn I_GetTimeMS() -> c_int {
    unsafe {
        let ticks = DG_GetTicksMs();
        if BASETIME == 0 {
            BASETIME = ticks;
        }
        (ticks - BASETIME) as c_int
    }
}

#[no_mangle]
pub extern "C" fn I_Sleep(ms: c_int) {
    unsafe { DG_SleepMs(ms as u32) }
}

#[no_mangle]
pub extern "C" fn I_WaitVBL(_count: c_int) {}

#[no_mangle]
pub extern "C" fn I_InitTimer() {}
