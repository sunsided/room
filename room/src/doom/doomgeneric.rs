#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int};
use std::ptr;

// Pull in the anchors so all #[no_mangle] functions survive link-time dead-code elimination
// (they are only called from C).
use super::hu_lib::HUlib_init;
use super::i_input::I_Input_Link_Anchor;
use super::p_ceilng::P_Ceilng_Link_Anchor;
use super::p_doors::P_Doors_Link_Anchor;
use super::p_floor::P_Floor_Link_Anchor;
use super::p_lights::P_Lights_Link_Anchor;
use super::p_plats::P_Plats_Link_Anchor;
use super::p_pspr::P_Pspr_Link_Anchor;
use super::p_sight::P_Sight_Link_Anchor;
use super::p_switch::P_Switch_Link_Anchor;
use super::p_telept::P_Telept_Link_Anchor;
use super::p_user::P_User_Link_Anchor;
use super::r_main::R_Main_Link_Anchor;

pub const DOOMGENERIC_RESX: usize = 640;
pub const DOOMGENERIC_RESY: usize = 400;
pub const DOOMGENERIC_PIXELS: usize = DOOMGENERIC_RESX * DOOMGENERIC_RESY;

#[no_mangle]
pub static mut DG_ScreenBuffer: *mut u32 = ptr::null_mut();

extern "C" {
    fn M_FindResponseFile();
    fn DG_Init();
    fn D_DoomMain();
}

extern "C" {
    static mut myargc: c_int;
    static mut myargv: *mut *mut c_char;
}

#[no_mangle]
pub unsafe extern "C" fn doomgeneric_Create(argc: c_int, argv: *mut *mut c_char) {
    // Anchor all ported module symbols so they survive LTO (called only from C).
    P_Ceilng_Link_Anchor();
    P_Doors_Link_Anchor();
    P_Floor_Link_Anchor();
    P_Lights_Link_Anchor();
    P_Plats_Link_Anchor();
    P_Pspr_Link_Anchor();
    P_Sight_Link_Anchor();
    P_Switch_Link_Anchor();
    P_Telept_Link_Anchor();
    P_User_Link_Anchor();
    I_Input_Link_Anchor();
    R_Main_Link_Anchor();
    let _ = HUlib_init as *const () as usize;

    myargc = argc;
    myargv = argv;

    M_FindResponseFile();

    let total_pixels = DOOMGENERIC_RESX * DOOMGENERIC_RESY;
    let mut buffer = vec![0u32; total_pixels];
    DG_ScreenBuffer = buffer.as_mut_ptr();
    std::mem::forget(buffer);

    DG_Init();
    D_DoomMain();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dg_screenbuffer_is_null_init() {
        unsafe {
            assert!(DG_ScreenBuffer.is_null());
        }
    }
}
