//! Tests for `p_setup.c` — level-loading globals and constants.
//!
//! `p_setup.c` reads all BSP and geometry lumps from the WAD into C globals
//! (`numvertexes`, `numsegs`, etc.).  Before a level is loaded (i.e. in the
//! unit-test binary that never calls `D_DoomMain`), all of these globals are
//! zero.  The blockmap grid globals (`bmapwidth`, `bmapheight`, `bmaporgx`,
//! `bmaporgy`) are also zero until `P_LoadBlockMap` fills them.
//!
//! These tests verify:
//!   * `MAX_DEATHMATCH_STARTS` constant
//!   * All level-dimension globals are zero before a level is loaded
//!   * Type widths match the C declarations

#![allow(non_snake_case)]

use std::ffi::c_int;

use crate::doom::c_ffi;

// ---------------------------------------------------------------------------
// MAX_DEATHMATCH_STARTS constant
// ---------------------------------------------------------------------------

/// `MAX_DEATHMATCH_STARTS = 10`: the maximum number of deathmatch start spots
/// a level can have.  Any extras are silently ignored.
#[test]
fn max_deathmatch_starts_is_10() {
    assert_eq!(c_ffi::MAX_DEATHMATCH_STARTS, 10);
}

// ---------------------------------------------------------------------------
// Level geometry count globals — default zero
// ---------------------------------------------------------------------------

/// `numvertexes` is the number of vertexes in the loaded level; zero before
/// `P_LoadVertexes` has run.
#[test]
fn numvertexes_default_zero() {
    unsafe {
        assert_eq!(c_ffi::numvertexes, 0, "numvertexes should be 0 before level load");
    }
}

/// `numsegs` is the number of BSP line segments; zero before `P_LoadSegs`.
#[test]
fn numsegs_default_zero() {
    unsafe {
        assert_eq!(c_ffi::numsegs, 0, "numsegs should be 0 before level load");
    }
}

/// `numsectors` is the number of sectors; zero before `P_LoadSectors`.
#[test]
fn numsectors_default_zero() {
    unsafe {
        assert_eq!(c_ffi::numsectors, 0, "numsectors should be 0 before level load");
    }
}

/// `numsubsectors` is the BSP subsector count; zero before `P_LoadSubsectors`.
#[test]
fn numsubsectors_default_zero() {
    unsafe {
        assert_eq!(
            c_ffi::numsubsectors,
            0,
            "numsubsectors should be 0 before level load"
        );
    }
}

/// `numnodes` is the number of BSP nodes; zero before `P_LoadNodes`.
#[test]
fn numnodes_default_zero() {
    unsafe {
        assert_eq!(c_ffi::numnodes, 0, "numnodes should be 0 before level load");
    }
}

/// `numlines` is the number of linedefs; zero before `P_LoadLineDefs`.
#[test]
fn numlines_default_zero() {
    unsafe {
        assert_eq!(c_ffi::numlines, 0, "numlines should be 0 before level load");
    }
}

/// `numsides` is the number of sidedefs; zero before `P_LoadSideDefs`.
#[test]
fn numsides_default_zero() {
    unsafe {
        assert_eq!(c_ffi::numsides, 0, "numsides should be 0 before level load");
    }
}

// ---------------------------------------------------------------------------
// Blockmap grid globals — default zero
// ---------------------------------------------------------------------------

/// `bmapwidth` and `bmapheight` are the blockmap grid dimensions in 128-unit
/// blocks; zero before `P_LoadBlockMap` runs.
#[test]
fn bmap_dimensions_default_zero() {
    unsafe {
        assert_eq!(c_ffi::bmapwidth, 0, "bmapwidth");
        assert_eq!(c_ffi::bmapheight, 0, "bmapheight");
    }
}

/// `bmaporgx` and `bmaporgy` are the fixed-point world-space origin of the
/// blockmap; zero before `P_LoadBlockMap` runs.
#[test]
fn bmap_origin_default_zero() {
    unsafe {
        assert_eq!(c_ffi::bmaporgx, 0, "bmaporgx");
        assert_eq!(c_ffi::bmaporgy, 0, "bmaporgy");
    }
}

// ---------------------------------------------------------------------------
// Type-width checks
// ---------------------------------------------------------------------------

/// All level count and blockmap globals are C `int` (4 bytes).
#[test]
fn setup_globals_are_c_int_width() {
    const _: () = assert!(std::mem::size_of::<c_int>() == 4);
    unsafe {
        let _: c_int = c_ffi::numvertexes;
        let _: c_int = c_ffi::numsegs;
        let _: c_int = c_ffi::numsectors;
        let _: c_int = c_ffi::numsubsectors;
        let _: c_int = c_ffi::numnodes;
        let _: c_int = c_ffi::numlines;
        let _: c_int = c_ffi::numsides;
        let _: c_int = c_ffi::bmapwidth;
        let _: c_int = c_ffi::bmapheight;
        let _: c_int = c_ffi::bmaporgx;
        let _: c_int = c_ffi::bmaporgy;
    }
}
