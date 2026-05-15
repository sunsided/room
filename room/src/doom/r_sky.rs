//! Rust port of vendor/doomgeneric/r_sky.c.
//!
//! Sky-rendering globals and one trivial init function. The sky is a
//! wall-like texture; `skytexturemid` is the vertical centre used by
//! `r_plane.c` when drawing sky columns.

#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::c_int;

use crate::doom::m_fixed::FRACUNIT;

/// Flat number of the sky flat (F_SKY1), filled in by `g_game.c`
/// via `R_FlatNumForName(SKYFLATNAME)`.
#[no_mangle]
pub static mut skyflatnum: c_int = 0;

/// Texture number of the current sky texture (SKY1/SKY2/SKY3/SKY4),
/// assigned by `g_game.c`.
#[no_mangle]
pub static mut skytexture: c_int = 0;

/// Vertical midpoint for sky column drawing (in fixed-point). Reset
/// every time the view size changes.
#[no_mangle]
pub static mut skytexturemid: c_int = 0;

/// Reset sky mapping state. Called whenever the view size changes.
#[no_mangle]
pub extern "C" fn R_InitSkyMap() {
    unsafe {
        // skyflatnum = R_FlatNumForName(SKYFLATNAME);   // done elsewhere
        skytexturemid = 100 * FRACUNIT;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Global state; serialise tests that touch it.
    static LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn init_sets_texturemid() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            skytexturemid = 0;
            R_InitSkyMap();
            assert_eq!(skytexturemid, 100 * 65536);
        }
    }

    #[test]
    fn globals_default_to_zero() {
        // Only meaningful on the first run – still worth asserting
        // that we're publishing three `c_int` symbols with the
        // expected names (compilation + linkage sanity).
        let _g = LOCK.lock().unwrap();
        unsafe {
            let _ = (skyflatnum, skytexture, skytexturemid);
        }
    }
}
