//! Rust port of vendor/doomgeneric/doomstat.c.
//!
//! Global game state variables.

#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int};
use std::ptr;

use super::d_mode;
use crate::types::Boolean;

// Game Mode - identify IWAD as shareware, retail etc.
#[no_mangle]
pub static mut gamemode: c_int = d_mode::indetermined;

#[no_mangle]
pub static mut gamemission: c_int = d_mode::doom;

#[no_mangle]
pub static mut gameversion: c_int = d_mode::exe_final2;

#[no_mangle]
pub static mut gamedescription: *mut c_char = ptr::null_mut();

// Set if homebrew PWAD stuff has been added.
#[no_mangle]
pub static mut modifiedgame: Boolean = Boolean::FALSE;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn defaults_match_c() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(gamemode, d_mode::indetermined);
            assert_eq!(gamemission, d_mode::doom);
            assert_eq!(gameversion, d_mode::exe_final2);
            assert!(gamedescription.is_null());
            assert_eq!(modifiedgame, Boolean::FALSE);
        }
    }
}
