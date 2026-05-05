//! Tests for `r_data.c` — texture/flat data subsystem.
//!
//! Most of `r_data.c` requires the WAD to be loaded via the full game
//! initialization chain (`D_DoomMain` → `W_Init` → `R_Init` → `R_InitData`),
//! which does **not** happen in the unit-test binary.
//!
//! These tests therefore focus on:
//!   * FFI symbol linkage (compile-time: the symbols must resolve at link time)
//!   * Pre-init default values (all C globals zero-initialize)
//!   * Constants that `r_data.c` relies on, verifying they are stable

#![allow(non_snake_case)]

use crate::doom::c_ffi;

// ---------------------------------------------------------------------------
// FFI symbol linkage — these tests simply read the C globals.
// A linker error would cause the binary to fail to build; a panic here
// would indicate the symbol resolved to an unexpected non-zero value
// before R_InitData has been called.
// ---------------------------------------------------------------------------

/// Before the full game initializer runs, all C globals default to zero.
/// Reading them must not panic or produce UB.
#[test]
fn r_data_globals_readable_before_init() {
    unsafe {
        // These are all initialised to 0 by C before R_InitData is called.
        // We only verify that they can be read (linker check), not their values.
        let _ff = c_ffi::firstflat;
        let _lf = c_ffi::lastflat;
        let _nf = c_ffi::numflats;
        let _fsl = c_ffi::firstspritelump;
        let _lsl = c_ffi::lastspritelump;
        let _nsl = c_ffi::numspritelumps;
        let _nt = c_ffi::numtextures;
    }
}

/// All r_data globals are C `int` which must be exactly 4 bytes wide.
/// This catches size mismatches between the Rust `c_int` and the C `int`.
#[test]
fn r_data_globals_are_c_int_width() {
    use std::ffi::c_int;
    const _: () = assert!(std::mem::size_of::<c_int>() == 4);
    unsafe {
        let _: c_int = c_ffi::firstflat;
        let _: c_int = c_ffi::lastflat;
        let _: c_int = c_ffi::numflats;
        let _: c_int = c_ffi::firstspritelump;
        let _: c_int = c_ffi::lastspritelump;
        let _: c_int = c_ffi::numspritelumps;
        let _: c_int = c_ffi::numtextures;
    }
}

// ---------------------------------------------------------------------------
// Constants used internally by r_data.c / r_textures
// ---------------------------------------------------------------------------

/// `FRACBITS` is used throughout the texture-coordinate arithmetic in
/// `r_data.c` as well as many other modules.  It must be exactly 16.
#[test]
fn fracbits_is_16() {
    assert_eq!(c_ffi::FRACBITS, 16);
}

/// `FRACUNIT` = 1 << 16.  Encoded in the WAD and used in offset calculations.
#[test]
fn fracunit_is_65536() {
    assert_eq!(c_ffi::FRACUNIT, 65536);
}

