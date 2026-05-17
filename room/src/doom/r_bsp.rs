//! Rust port of vendor/doomgeneric/r_bsp.c.
//!
//! BSP traversal, handling of LineSegs for rendering.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_int, c_short, c_void};
use std::ptr;

use super::m_bbox::BBox;
use super::m_fixed::{angle_t, fixed_t};
use super::tables::{ANG90, ANGLETOFINESHIFT};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const MAXDRAWSEGS: usize = 256;
const MAXSEGS: usize = 32;
const NF_SUBSECTOR: u32 = 0x8000;

// ---------------------------------------------------------------------------
// Diagnostic probes for the "walls disappear / different room appears" bug
// ---------------------------------------------------------------------------
//
// Enable with `RUST_LOG=room::doom::r_bsp=trace`. These are gated at TRACE so
// the existing debug log stays usable; terminate the app the moment the
// glitch appears and we inspect the tail of the trace.
static mut PROBE_FRAME: u64 = 0;

#[inline]
unsafe fn seg_index(line: *const seg_t) -> isize {
    if segs.is_null() || line.is_null() {
        -1
    } else {
        (line as isize - segs as isize) / std::mem::size_of::<seg_t>() as isize
    }
}

// ---------------------------------------------------------------------------
// Mirrored C structs (from r_defs.h) — only fields read by r_bsp.c
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct vertex_t {
    pub x: fixed_t,
    pub y: fixed_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mobj_s {
    _opaque: [u8; 0],
}
pub type mobj_t = mobj_s;

// thinker_t is 24 bytes in C (prev/next/function pointers).
// r_bsp.rs never dereferences it, but it must have the correct size
// so that sector_t (which contains degenmobj_t, which contains thinker_t)
// matches the C layout of 128 bytes.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct thinker_s {
    _prev: *mut c_void,
    _next: *mut c_void,
    _function: *mut c_void,
}
pub type thinker_t = thinker_s;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct degenmobj_s {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sector_t {
    pub floorheight: fixed_t,
    pub ceilingheight: fixed_t,
    pub floorpic: c_short,
    pub ceilingpic: c_short,
    pub lightlevel: c_short,
    pub special: c_short,
    pub tag: c_short,
    pub soundtraversed: c_int,
    pub soundtarget: *mut mobj_t,
    pub blockbox: [c_int; 4],
    pub soundorg: degenmobj_s,
    pub validcount: c_int,
    pub thinglist: *mut mobj_t,
    pub specialdata: *mut c_void,
    pub linecount: c_int,
    pub lines: *mut *mut line_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct side_t {
    pub textureoffset: fixed_t,
    pub rowoffset: fixed_t,
    pub toptexture: c_short,
    pub bottomtexture: c_short,
    pub midtexture: c_short,
    pub sector: *mut sector_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum slopetype_t {
    ST_HORIZONTAL,
    ST_VERTICAL,
    ST_POSITIVE,
    ST_NEGATIVE,
}

#[repr(C)]
pub struct line_s {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub flags: c_short,
    pub special: c_short,
    pub tag: c_short,
    pub sidenum: [c_short; 2],
    pub bbox: [fixed_t; 4],
    pub slopetype: slopetype_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub validcount: c_int,
    pub specialdata: *mut c_void,
}
pub type line_t = line_s;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct subsector_s {
    pub sector: *mut sector_t,
    pub numlines: c_short,
    pub firstline: c_short,
}
pub type subsector_t = subsector_s;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct seg_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub offset: fixed_t,
    pub angle: angle_t,
    pub sidedef: *mut side_t,
    pub linedef: *mut line_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct node_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub bbox: [[fixed_t; 4]; 2],
    pub children: [u16; 2],
}

// ---------------------------------------------------------------------------
// visplane_t — already exported from r_plane.rs, mirror locally for field access
// ---------------------------------------------------------------------------

const SCREENWIDTH_RP: usize = 320;

#[repr(C)]
#[derive(Clone, Copy)]
struct visplane_t {
    pub height: fixed_t,
    pub picnum: c_int,
    pub lightlevel: c_int,
    pub minx: c_int,
    pub maxx: c_int,
    pub pad1: u8,
    pub top: [u8; SCREENWIDTH_RP],
    pub pad2: u8,
    pub pad3: u8,
    pub bottom: [u8; SCREENWIDTH_RP],
    pub pad4: u8,
}

// ---------------------------------------------------------------------------
// drawseg_t — MUST match C layout exactly (r_segs.c writes every field)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct drawseg_t {
    pub curline: *mut seg_t,
    pub x1: c_int,
    pub x2: c_int,
    pub scale1: fixed_t,
    pub scale2: fixed_t,
    pub scalestep: fixed_t,
    pub silhouette: c_int,
    pub bsilheight: fixed_t,
    pub tsilheight: fixed_t,
    pub sprtopclip: *mut c_short,
    pub sprbottomclip: *mut c_short,
    pub maskedtexturecol: *mut c_short,
}

const ZERO_DRAWSEG: drawseg_t = drawseg_t {
    curline: ptr::null_mut(),
    x1: 0,
    x2: 0,
    scale1: 0,
    scale2: 0,
    scalestep: 0,
    silhouette: 0,
    bsilheight: 0,
    tsilheight: 0,
    sprtopclip: ptr::null_mut(),
    sprbottomclip: ptr::null_mut(),
    maskedtexturecol: ptr::null_mut(),
};

// Compile-time size and offset checks for structs that must match C layout
// (values verified against vendor/doomgeneric C structs on x86_64 Linux)
#[cfg(target_pointer_width = "64")]
mod layout_checks {
    use super::*;
    const _: () = assert!(std::mem::size_of::<vertex_t>() == 8);
    const _: () = assert!(std::mem::size_of::<sector_t>() == 128);
    const _: () = assert!(std::mem::offset_of!(sector_t, floorheight) == 0);
    const _: () = assert!(std::mem::offset_of!(sector_t, ceilingheight) == 4);
    const _: () = assert!(std::mem::offset_of!(sector_t, floorpic) == 8);
    const _: () = assert!(std::mem::offset_of!(sector_t, ceilingpic) == 10);
    const _: () = assert!(std::mem::offset_of!(sector_t, lightlevel) == 12);
    const _: () = assert!(std::mem::offset_of!(sector_t, special) == 14);
    const _: () = assert!(std::mem::offset_of!(sector_t, tag) == 16);
    const _: () = assert!(std::mem::offset_of!(sector_t, soundtraversed) == 20);
    const _: () = assert!(std::mem::offset_of!(sector_t, soundtarget) == 24);
    const _: () = assert!(std::mem::offset_of!(sector_t, blockbox) == 32);
    const _: () = assert!(std::mem::offset_of!(sector_t, soundorg) == 48);
    const _: () = assert!(std::mem::offset_of!(sector_t, validcount) == 88);
    const _: () = assert!(std::mem::offset_of!(sector_t, thinglist) == 96);
    const _: () = assert!(std::mem::offset_of!(sector_t, specialdata) == 104);
    const _: () = assert!(std::mem::offset_of!(sector_t, linecount) == 112);
    const _: () = assert!(std::mem::offset_of!(sector_t, lines) == 120);

    const _: () = assert!(std::mem::size_of::<side_t>() == 24);
    const _: () = assert!(std::mem::offset_of!(side_t, textureoffset) == 0);
    const _: () = assert!(std::mem::offset_of!(side_t, rowoffset) == 4);
    const _: () = assert!(std::mem::offset_of!(side_t, toptexture) == 8);
    const _: () = assert!(std::mem::offset_of!(side_t, bottomtexture) == 10);
    const _: () = assert!(std::mem::offset_of!(side_t, midtexture) == 12);
    const _: () = assert!(std::mem::offset_of!(side_t, sector) == 16);

    const _: () = assert!(std::mem::size_of::<slopetype_t>() == 4);

    const _: () = assert!(std::mem::size_of::<line_t>() == 88);
    const _: () = assert!(std::mem::offset_of!(line_t, v1) == 0);
    const _: () = assert!(std::mem::offset_of!(line_t, v2) == 8);
    const _: () = assert!(std::mem::offset_of!(line_t, dx) == 16);
    const _: () = assert!(std::mem::offset_of!(line_t, dy) == 20);
    const _: () = assert!(std::mem::offset_of!(line_t, flags) == 24);
    const _: () = assert!(std::mem::offset_of!(line_t, special) == 26);
    const _: () = assert!(std::mem::offset_of!(line_t, tag) == 28);
    const _: () = assert!(std::mem::offset_of!(line_t, sidenum) == 30);
    const _: () = assert!(std::mem::offset_of!(line_t, bbox) == 36);
    const _: () = assert!(std::mem::offset_of!(line_t, slopetype) == 52);
    const _: () = assert!(std::mem::offset_of!(line_t, frontsector) == 56);
    const _: () = assert!(std::mem::offset_of!(line_t, backsector) == 64);
    const _: () = assert!(std::mem::offset_of!(line_t, validcount) == 72);
    const _: () = assert!(std::mem::offset_of!(line_t, specialdata) == 80);

    const _: () = assert!(std::mem::size_of::<subsector_t>() == 16);
    const _: () = assert!(std::mem::offset_of!(subsector_t, sector) == 0);
    const _: () = assert!(std::mem::offset_of!(subsector_t, numlines) == 8);
    const _: () = assert!(std::mem::offset_of!(subsector_t, firstline) == 10);

    const _: () = assert!(std::mem::size_of::<seg_t>() == 56);
    const _: () = assert!(std::mem::offset_of!(seg_t, v1) == 0);
    const _: () = assert!(std::mem::offset_of!(seg_t, v2) == 8);
    const _: () = assert!(std::mem::offset_of!(seg_t, offset) == 16);
    const _: () = assert!(std::mem::offset_of!(seg_t, angle) == 20);
    const _: () = assert!(std::mem::offset_of!(seg_t, sidedef) == 24);
    const _: () = assert!(std::mem::offset_of!(seg_t, linedef) == 32);
    const _: () = assert!(std::mem::offset_of!(seg_t, frontsector) == 40);
    const _: () = assert!(std::mem::offset_of!(seg_t, backsector) == 48);

    const _: () = assert!(std::mem::size_of::<node_t>() == 52);
    const _: () = assert!(std::mem::offset_of!(node_t, x) == 0);
    const _: () = assert!(std::mem::offset_of!(node_t, y) == 4);
    const _: () = assert!(std::mem::offset_of!(node_t, dx) == 8);
    const _: () = assert!(std::mem::offset_of!(node_t, dy) == 12);
    const _: () = assert!(std::mem::offset_of!(node_t, bbox) == 16);
    const _: () = assert!(std::mem::offset_of!(node_t, children) == 48);

    const _: () = assert!(std::mem::size_of::<drawseg_t>() == 64);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, curline) == 0);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, x1) == 8);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, x2) == 12);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, scale1) == 16);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, scale2) == 20);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, scalestep) == 24);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, silhouette) == 28);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, bsilheight) == 32);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, tsilheight) == 36);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, sprtopclip) == 40);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, sprbottomclip) == 48);
    const _: () = assert!(std::mem::offset_of!(drawseg_t, maskedtexturecol) == 56);

    const _: () = assert!(std::mem::size_of::<thinker_t>() == 24);
    const _: () = assert!(std::mem::size_of::<degenmobj_s>() == 40);
}

// ---------------------------------------------------------------------------
// cliprange_t (internal)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
struct cliprange_t {
    first: c_int,
    last: c_int,
}

// ---------------------------------------------------------------------------
// Globals exported with #[no_mangle]
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut curline: *mut seg_t = ptr::null_mut();

#[no_mangle]
pub static mut sidedef: *mut side_t = ptr::null_mut();

#[no_mangle]
pub static mut linedef: *mut line_t = ptr::null_mut();

#[no_mangle]
pub static mut frontsector: *mut sector_t = ptr::null_mut();

#[no_mangle]
pub static mut backsector: *mut sector_t = ptr::null_mut();

#[no_mangle]
pub static mut drawsegs: [drawseg_t; MAXDRAWSEGS] = [ZERO_DRAWSEG; MAXDRAWSEGS];

#[no_mangle]
pub static mut ds_p: *mut drawseg_t = ptr::null_mut();

// ---------------------------------------------------------------------------
// Module-local state
// ---------------------------------------------------------------------------

static mut solidsegs: [cliprange_t; MAXSEGS] = [cliprange_t { first: 0, last: 0 }; MAXSEGS];
static mut newend: *mut cliprange_t = ptr::null_mut();

// checkcoord — 12 rows, only 11 initialised in C (rows 3 and 7 are {0})
static CHECKCOORD: [[c_int; 4]; 12] = [
    [3, 0, 2, 1],
    [3, 0, 2, 0],
    [3, 1, 2, 0],
    [0, 0, 0, 0],
    [2, 0, 2, 1],
    [0, 0, 0, 0],
    [3, 1, 3, 0],
    [0, 0, 0, 0],
    [2, 0, 3, 1],
    [2, 1, 3, 1],
    [2, 1, 3, 0],
    [0, 0, 0, 0], // C has no 12th row; zero-padded
];

// ---------------------------------------------------------------------------
// Imports from other modules
// ---------------------------------------------------------------------------

use crate::doom::p_setup::{nodes, segs, subsectors};
use crate::doom::r_draw::viewwidth;
use crate::doom::r_main::{
    clipangle, sscount, viewangle, viewangletox, viewx, viewy, viewz, R_PointOnSide, R_PointToAngle,
};
use crate::doom::r_plane::{ceilingplane, floorplane, R_FindPlane};
use crate::doom::r_segs::{rw_angle1, R_StoreWallRange};
use crate::doom::r_sky::skyflatnum;
use crate::doom::r_things::R_AddSprites;

// ---------------------------------------------------------------------------
// R_ClearDrawSegs
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_ClearDrawSegs() {
    ds_p = std::ptr::addr_of_mut!(drawsegs[0]);
}

// ---------------------------------------------------------------------------
// R_ClearClipSegs
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_ClearClipSegs() {
    solidsegs[0].first = -0x7fffffff;
    solidsegs[0].last = -1;
    solidsegs[1].first = viewwidth;
    solidsegs[1].last = 0x7fffffff;
    newend = std::ptr::addr_of_mut!(solidsegs[0]).add(2);

    // Diagnostic: frame marker. R_ClearClipSegs is called once per frame by
    // R_RenderPlayerView before descending the BSP.
    PROBE_FRAME = PROBE_FRAME.wrapping_add(1);
    log::trace!(
        "=== FRAME {} === viewx={:#x} viewy={:#x} viewangle={:#x} viewwidth={}",
        { PROBE_FRAME },
        { viewx },
        { viewy },
        { viewangle },
        { viewwidth }
    );
}

// ---------------------------------------------------------------------------
// R_ClipSolidWallSegment
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_ClipSolidWallSegment(first: c_int, last: c_int) {
    if first > last {
        log::debug!(
            "R_ClipSolidWallSegment INVALID: first={} > last={}",
            first,
            last
        );
    }
    let solidsegs_base = std::ptr::addr_of_mut!(solidsegs[0]);

    let mut start = solidsegs_base;
    while (*start).last < first - 1 {
        start = start.add(1);
    }

    if first < (*start).first {
        if last < (*start).first - 1 {
            R_StoreWallRange(first, last);
            let mut next = newend;
            newend = next.add(1);

            while next != start {
                *next = *next.sub(1);
                next = next.sub(1);
            }
            (*next).first = first;
            (*next).last = last;
            return;
        }

        R_StoreWallRange(first, (*start).first - 1);
        (*start).first = first;
    }

    if last <= (*start).last {
        return;
    }

    let mut next = start;
    loop {
        let next_plus_1 = next.add(1);
        if last < (*next_plus_1).first - 1 {
            break;
        }

        R_StoreWallRange((*next).last + 1, (*next_plus_1).first - 1);
        next = next.add(1);

        if last <= (*next).last {
            (*start).last = (*next).last;
            // "goto crunch"
            if next == start {
                return;
            }
            // C: while (next++ != newend) { *++start = *next; }
            loop {
                let old_next = next;
                next = next.add(1);
                if old_next == newend {
                    break;
                }
                start = start.add(1);
                *start = *next;
            }
            newend = start.add(1);
            return;
        }
    }

    R_StoreWallRange((*next).last + 1, last);
    (*start).last = last;

    // crunch:
    if next == start {
        return;
    }

    // C: while (next++ != newend) { *++start = *next; }
    loop {
        let old_next = next;
        next = next.add(1);
        if old_next == newend {
            break;
        }
        start = start.add(1);
        *start = *next;
    }
    newend = start.add(1);
}

// ---------------------------------------------------------------------------
// R_ClipPassWallSegment
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_ClipPassWallSegment(first: c_int, last: c_int) {
    if first > last {
        log::debug!(
            "R_ClipPassWallSegment INVALID: first={} > last={}",
            first,
            last
        );
    }
    let solidsegs_base = std::ptr::addr_of_mut!(solidsegs[0]);

    let mut start = solidsegs_base;
    while (*start).last < first - 1 {
        start = start.add(1);
    }

    if first < (*start).first {
        if last < (*start).first - 1 {
            R_StoreWallRange(first, last);
            return;
        }

        R_StoreWallRange(first, (*start).first - 1);
    }

    if last <= (*start).last {
        return;
    }

    loop {
        let start_plus_1 = start.add(1);
        if last < (*start_plus_1).first - 1 {
            break;
        }

        R_StoreWallRange((*start).last + 1, (*start_plus_1).first - 1);
        start = start.add(1);

        if last <= (*start).last {
            return;
        }
    }

    R_StoreWallRange((*start).last + 1, last);
}

// ---------------------------------------------------------------------------
// R_AddLine
// ---------------------------------------------------------------------------

unsafe fn R_AddLine(line: *mut seg_t) {
    curline = line;

    let orig_angle1 = R_PointToAngle((*(*line).v1).x, (*(*line).v1).y);
    let orig_angle2 = R_PointToAngle((*(*line).v2).x, (*(*line).v2).y);

    let span = orig_angle1.wrapping_sub(orig_angle2);

    // Back side?
    if span >= 0x8000_0000 {
        log::trace!(
            "R_AddLine SKIP backface: orig_a1={:#x} orig_a2={:#x} va={:#x}",
            orig_angle1,
            orig_angle2,
            { viewangle }
        );
        return;
    }

    rw_angle1 = orig_angle1;
    let mut angle1 = orig_angle1.wrapping_sub(viewangle);
    let mut angle2 = orig_angle2.wrapping_sub(viewangle);

    let clipangle_d2 = clipangle.wrapping_mul(2);

    let mut tspan = angle1.wrapping_add(clipangle);
    if tspan > clipangle_d2 {
        tspan = tspan.wrapping_sub(clipangle_d2);
        if tspan >= span {
            log::trace!("R_AddLine SKIP off-left: orig_a1={:#x} orig_a2={:#x} va={:#x} a1={:#x} a2={:#x} span={:#x} tspan={:#x}", orig_angle1, orig_angle2, { viewangle }, angle1, angle2, span, tspan);
            return;
        }
        angle1 = clipangle;
    }

    tspan = clipangle.wrapping_sub(angle2);
    if tspan > clipangle_d2 {
        tspan = tspan.wrapping_sub(clipangle_d2);
        if tspan >= span {
            log::trace!("R_AddLine SKIP off-right: orig_a1={:#x} orig_a2={:#x} va={:#x} a1={:#x} a2={:#x} span={:#x} tspan={:#x}", orig_angle1, orig_angle2, { viewangle }, angle1, angle2, span, tspan);
            return;
        }
        angle2 = 0u32.wrapping_sub(clipangle);
    }

    let idx1 = ((angle1.wrapping_add(ANG90)) >> ANGLETOFINESHIFT) as usize;
    let idx2 = ((angle2.wrapping_add(ANG90)) >> ANGLETOFINESHIFT) as usize;
    let x1 = viewangletox[idx1];
    let x2 = viewangletox[idx2];

    // Log walls that land in the right portion of the screen
    if x2 >= 200 || x1 >= 200 {
        log::trace!(
            "R_AddLine wall: orig_a1={:#x} orig_a2={:#x} va={:#x} clip={:#x} a1={:#x} a2={:#x} idx1={} idx2={} x1={} x2={}",
            orig_angle1, orig_angle2, { viewangle }, { clipangle }, angle1, angle2, idx1, idx2, x1, x2
        );
    }
    // Diagnostic: log every surviving wall (full screen) at TRACE.
    log::trace!(
        "R_AddLine projected: frame={} seg={} x1={} x2={} a1={:#x} a2={:#x}",
        { PROBE_FRAME },
        seg_index(line),
        x1,
        x2,
        angle1,
        angle2
    );

    if x1 == x2 {
        log::trace!(
            "R_AddLine SKIP x1==x2: orig_a1={:#x} orig_a2={:#x} x1={} x2={}",
            orig_angle1,
            orig_angle2,
            x1,
            x2
        );
        return;
    }

    backsector = (*line).backsector;

    // Single sided line?
    if backsector.is_null() {
        log::trace!(
            "R_AddLine classify: frame={} seg={} x1={} x2={} decision=SOLID(1-sided) back=null front={:p}",
            { PROBE_FRAME }, seg_index(line), x1, x2, frontsector as *const _
        );
        R_ClipSolidWallSegment(x1, x2 - 1);
        return;
    }

    // Closed door
    if (*backsector).ceilingheight <= (*frontsector).floorheight
        || (*backsector).floorheight >= (*frontsector).ceilingheight
    {
        log::trace!(
            "R_AddLine classify: frame={} seg={} x1={} x2={} decision=SOLID(closed-door) back={:p} f.ch={} f.fh={} b.ch={} b.fh={}",
            { PROBE_FRAME }, seg_index(line), x1, x2, backsector as *const _,
            (*frontsector).ceilingheight, (*frontsector).floorheight,
            (*backsector).ceilingheight, (*backsector).floorheight
        );
        R_ClipSolidWallSegment(x1, x2 - 1);
        return;
    }

    // Window
    if (*backsector).ceilingheight != (*frontsector).ceilingheight
        || (*backsector).floorheight != (*frontsector).floorheight
    {
        log::trace!(
            "R_AddLine classify: frame={} seg={} x1={} x2={} decision=PASS(window) back={:p} f.ch={} f.fh={} b.ch={} b.fh={}",
            { PROBE_FRAME }, seg_index(line), x1, x2, backsector as *const _,
            (*frontsector).ceilingheight, (*frontsector).floorheight,
            (*backsector).ceilingheight, (*backsector).floorheight
        );
        R_ClipPassWallSegment(x1, x2 - 1);
        return;
    }

    // Reject empty lines
    if (*backsector).ceilingpic == (*frontsector).ceilingpic
        && (*backsector).floorpic == (*frontsector).floorpic
        && (*backsector).lightlevel == (*frontsector).lightlevel
        && (*(*curline).sidedef).midtexture == 0
    {
        log::trace!(
            "R_AddLine classify: frame={} seg={} x1={} x2={} decision=REJECT(empty)",
            { PROBE_FRAME },
            seg_index(line),
            x1,
            x2
        );
        return;
    }

    log::trace!(
        "R_AddLine classify: frame={} seg={} x1={} x2={} decision=PASS(midtex/lighting) midtex={}",
        { PROBE_FRAME },
        seg_index(line),
        x1,
        x2,
        (*(*curline).sidedef).midtexture
    );
    R_ClipPassWallSegment(x1, x2 - 1);
}

// ---------------------------------------------------------------------------
// R_CheckBBox
// ---------------------------------------------------------------------------

// NOTE: These indices MUST match the C enum in vendor/doomgeneric/m_bbox.h
// (and the matching Rust constants in m_bbox.rs) which stores bbox fields as
// [TOP, BOTTOM, LEFT, RIGHT]. An earlier version had LEFT/RIGHT/TOP/BOTTOM
// ordering here which made R_CheckBBox compare viewx against y-coordinates
// (and viewy against x-coordinates), producing axis-scrambled visibility
// tests — far BSP subtrees were not pruned and rendering descended into the
// wrong parts of the map, causing walls to "disappear" and a different room
// to show through (classic Doom HOM variant).
#[no_mangle]
pub unsafe extern "C" fn R_CheckBBox(bspcoord: *mut fixed_t) -> c_int {
    let boxx = if viewx <= *bspcoord.add(BBox::LEFT) {
        0
    } else if viewx < *bspcoord.add(BBox::RIGHT) {
        1
    } else {
        2
    };

    let boxy = if viewy >= *bspcoord.add(BBox::TOP) {
        0
    } else if viewy > *bspcoord.add(BBox::BOTTOM) {
        1
    } else {
        2
    };

    let boxpos = (boxy << 2) + boxx;
    if boxpos == 5 {
        return 1;
    }

    let cc = CHECKCOORD[boxpos as usize];
    let x1 = *bspcoord.add(cc[0] as usize);
    let y1 = *bspcoord.add(cc[1] as usize);
    let x2 = *bspcoord.add(cc[2] as usize);
    let y2 = *bspcoord.add(cc[3] as usize);

    let mut angle1 = R_PointToAngle(x1, y1).wrapping_sub(viewangle);
    let mut angle2 = R_PointToAngle(x2, y2).wrapping_sub(viewangle);

    let span = angle1.wrapping_sub(angle2);

    if span >= 0x8000_0000 {
        return 1;
    }

    let clipangle_d2 = clipangle.wrapping_mul(2);

    let mut tspan = angle1.wrapping_add(clipangle);
    if tspan > clipangle_d2 {
        tspan = tspan.wrapping_sub(clipangle_d2);
        if tspan >= span {
            return 0;
        }
        angle1 = clipangle;
    }

    tspan = clipangle.wrapping_sub(angle2);
    if tspan > clipangle_d2 {
        tspan = tspan.wrapping_sub(clipangle_d2);
        if tspan >= span {
            return 0;
        }
        angle2 = 0u32.wrapping_sub(clipangle);
    }

    let sx1 = viewangletox[((angle1.wrapping_add(ANG90)) >> ANGLETOFINESHIFT) as usize];
    let sx2 = viewangletox[((angle2.wrapping_add(ANG90)) >> ANGLETOFINESHIFT) as usize];

    if sx1 == sx2 {
        return 0;
    }
    let sx2 = sx2 - 1;

    let solidsegs_base = std::ptr::addr_of!(solidsegs[0]);
    let mut start = solidsegs_base;
    while (*start).last < sx2 {
        start = start.add(1);
    }

    if sx1 >= (*start).first && sx2 <= (*start).last {
        return 0;
    }

    1
}

// ---------------------------------------------------------------------------
// R_Subsector
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_Subsector(num: c_int) {
    let num_usize = num as usize;

    sscount += 1;

    let sub = &*(subsectors.add(num_usize) as *mut subsector_t);
    frontsector = sub.sector;
    let mut count = sub.numlines as c_int;
    let mut line = segs.add(sub.firstline as usize) as *mut seg_t;

    if (*frontsector).floorheight < viewz {
        floorplane = R_FindPlane(
            (*frontsector).floorheight,
            (*frontsector).floorpic as c_int,
            (*frontsector).lightlevel as c_int,
        );
    } else {
        floorplane = ptr::null_mut();
    }

    if (*frontsector).ceilingheight > viewz || (*frontsector).ceilingpic as c_int == skyflatnum {
        ceilingplane = R_FindPlane(
            (*frontsector).ceilingheight,
            (*frontsector).ceilingpic as c_int,
            (*frontsector).lightlevel as c_int,
        );
    } else {
        ceilingplane = ptr::null_mut();
    }

    R_AddSprites(frontsector);

    while count > 0 {
        R_AddLine(line);
        line = line.add(1);
        count -= 1;
    }
}

// ---------------------------------------------------------------------------
// R_RenderBSPNode
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_RenderBSPNode(bspnum: c_int) {
    // Found a subsector?
    if (bspnum as u32) & NF_SUBSECTOR != 0 {
        let sub_num = if bspnum == -1 {
            0
        } else {
            (bspnum as u32 & !NF_SUBSECTOR) as c_int
        };
        log::trace!(
            "R_RenderBSPNode: frame={} leaf subsector={} (bspnum={:#x})",
            { PROBE_FRAME },
            sub_num,
            bspnum as u32
        );
        R_Subsector(sub_num);
        return;
    }

    let bsp = &*(nodes.add(bspnum as usize) as *const node_t);

    let side = R_PointOnSide(viewx, viewy, bsp);

    log::trace!(
        "R_RenderBSPNode: frame={} node={} side={} front_child={:#x} back_child={:#x}",
        { PROBE_FRAME },
        bspnum,
        side,
        bsp.children[side as usize] as u32,
        bsp.children[(side ^ 1) as usize] as u32,
    );

    R_RenderBSPNode(bsp.children[side as usize] as c_int);

    let back_visible = R_CheckBBox(bsp.bbox[(side ^ 1) as usize].as_ptr() as *mut fixed_t) != 0;
    let back_box = bsp.bbox[(side ^ 1) as usize];
    log::trace!(
        "R_RenderBSPNode: frame={} node={} back_visible={} back_bbox=[L={} R={} T={} B={}]",
        { PROBE_FRAME },
        bspnum,
        back_visible,
        back_box[BBox::LEFT],
        back_box[BBox::RIGHT],
        back_box[BBox::TOP],
        back_box[BBox::BOTTOM],
    );
    if back_visible {
        R_RenderBSPNode(bsp.children[(side ^ 1) as usize] as c_int);
    }
}

// ---------------------------------------------------------------------------
// Anchor so linker doesn't discard
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_Bsp_Link_Anchor() {
    R_ClearDrawSegs();
    R_ClearClipSegs();
    R_RenderBSPNode(0);
    R_Subsector(0);
    R_ClipSolidWallSegment(0, 0);
    R_ClipPassWallSegment(0, 0);
    R_CheckBBox(ptr::null_mut());
}
