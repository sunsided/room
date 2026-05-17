//! Rust port of vendor/doomgeneric/p_maputl.c.
//!
//! Movement/collision utility functions, BLOCKMAP iterator functions,
//! and intercept routines used by p_map.c.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_int, c_uint, c_void};
use std::ptr;

use crate::doom::c_ffi::{
    divline_t, intercept_t, intercept_t_d, line_t, mobj_t, sector_t, subsector_t, MAPBLOCKSHIFT,
    MAPBLOCKSIZE, MAPBTOFRAC,
};
use crate::doom::info::*;
use crate::doom::m_fixed::{fixed_t, FixedDiv, FixedMul};
use crate::doom::m_fixed::{FRACBITS, FRACUNIT};

pub const MAXINTERCEPTS_ORIGINAL: usize = 128;
pub const MAXINTERCEPTS: usize = MAXINTERCEPTS_ORIGINAL + 61;

pub const PT_ADDLINES: c_int = 1;
pub const PT_ADDTHINGS: c_int = 2;
pub const PT_EARLYOUT: c_int = 4;

// ---------------------------------------------------------------------------
// Globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut opentop: fixed_t = 0;
#[no_mangle]
pub static mut openbottom: fixed_t = 0;
#[no_mangle]
pub static mut openrange: fixed_t = 0;
#[no_mangle]
pub static mut lowfloor: fixed_t = 0;

#[no_mangle]
pub static mut intercepts: [intercept_t; MAXINTERCEPTS] = [intercept_t {
    frac: 0,
    isaline: 0,
    d: intercept_t_d {
        thing: ptr::null_mut(),
    },
}; MAXINTERCEPTS];

#[no_mangle]
pub static mut intercept_p: *mut intercept_t = ptr::null_mut();

#[no_mangle]
pub static mut trace: divline_t = divline_t {
    x: 0,
    y: 0,
    dx: 0,
    dy: 0,
};

static mut earlyout: c_int = 0;

// ---------------------------------------------------------------------------
// P_AproxDistance
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_AproxDistance(dx: fixed_t, dy: fixed_t) -> fixed_t {
    let dx = dx.wrapping_abs();
    let dy = dy.wrapping_abs();
    if dx < dy {
        dx.wrapping_add(dy).wrapping_sub(dx >> 1)
    } else {
        dx.wrapping_add(dy).wrapping_sub(dy >> 1)
    }
}

// ---------------------------------------------------------------------------
// P_PointOnLineSide
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_PointOnLineSide(x: fixed_t, y: fixed_t, line: *mut line_t) -> c_int {
    unsafe {
        let line = &*line;
        if line.dx == 0 {
            if x <= (*line.v1).x {
                return if line.dy > 0 { 1 } else { 0 };
            }
            return if line.dy < 0 { 1 } else { 0 };
        }
        if line.dy == 0 {
            if y <= (*line.v1).y {
                return if line.dx < 0 { 1 } else { 0 };
            }
            return if line.dx > 0 { 1 } else { 0 };
        }
        let dx = x - (*line.v1).x;
        let dy = y - (*line.v1).y;
        let left = FixedMul(line.dy >> FRACBITS, dx);
        let right = FixedMul(dy, line.dx >> FRACBITS);
        if right < left {
            0
        } else {
            1
        }
    }
}

// ---------------------------------------------------------------------------
// P_PointOnDivlineSide
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_PointOnDivlineSide(x: fixed_t, y: fixed_t, line: *mut divline_t) -> c_int {
    unsafe {
        let line = &*line;
        if line.dx == 0 {
            if x <= line.x {
                return if line.dy > 0 { 1 } else { 0 };
            }
            return if line.dy < 0 { 1 } else { 0 };
        }
        if line.dy == 0 {
            if y <= line.y {
                return if line.dx < 0 { 1 } else { 0 };
            }
            return if line.dx > 0 { 1 } else { 0 };
        }
        let dx = x - line.x;
        let dy = y - line.y;
        if ((line.dy ^ line.dx ^ dx ^ dy) & (0x80000000u32 as i32)) != 0 {
            if ((line.dy ^ dx) & (0x80000000u32 as i32)) != 0 {
                return 1;
            }
            return 0;
        }
        let left = FixedMul(line.dy >> 8, dx >> 8);
        let right = FixedMul(dy >> 8, line.dx >> 8);
        if right < left {
            0
        } else {
            1
        }
    }
}

// ---------------------------------------------------------------------------
// P_MakeDivline
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_MakeDivline(li: *mut line_t, dl: *mut divline_t) {
    unsafe {
        let li = &*li;
        let dl = &mut *dl;
        dl.x = (*li.v1).x;
        dl.y = (*li.v1).y;
        dl.dx = li.dx;
        dl.dy = li.dy;
    }
}

// ---------------------------------------------------------------------------
// P_InterceptVector
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_InterceptVector(v2: *mut divline_t, v1: *mut divline_t) -> fixed_t {
    unsafe {
        let v1 = &*v1;
        let v2 = &*v2;
        let den = FixedMul(v1.dy >> 8, v2.dx) - FixedMul(v1.dx >> 8, v2.dy);
        if den == 0 {
            return 0;
        }
        let num = FixedMul((v1.x - v2.x) >> 8, v1.dy) + FixedMul((v2.y - v1.y) >> 8, v1.dx);
        FixedDiv(num, den)
    }
}

// ---------------------------------------------------------------------------
// P_BoxOnLineSide
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_BoxOnLineSide(tmbox: *mut c_int, ld: *mut line_t) -> c_int {
    unsafe {
        let tmbox = std::slice::from_raw_parts_mut(tmbox, 4);
        let ld = &*ld;
        let mut p1: c_int;
        let mut p2: c_int;
        match ld.slopetype {
            0 => {
                // ST_HORIZONTAL
                p1 = (tmbox[0] > (*ld.v1).y) as c_int;
                p2 = (tmbox[1] > (*ld.v1).y) as c_int;
                if ld.dx < 0 {
                    p1 ^= 1;
                    p2 ^= 1;
                }
            }
            1 => {
                // ST_VERTICAL
                p1 = (tmbox[3] < (*ld.v1).x) as c_int;
                p2 = (tmbox[2] < (*ld.v1).x) as c_int;
                if ld.dy < 0 {
                    p1 ^= 1;
                    p2 ^= 1;
                }
            }
            2 => {
                // ST_POSITIVE
                p1 = P_PointOnLineSide(tmbox[2], tmbox[0], ld as *const _ as *mut _);
                p2 = P_PointOnLineSide(tmbox[3], tmbox[1], ld as *const _ as *mut _);
            }
            _ => {
                // ST_NEGATIVE
                p1 = P_PointOnLineSide(tmbox[3], tmbox[0], ld as *const _ as *mut _);
                p2 = P_PointOnLineSide(tmbox[2], tmbox[1], ld as *const _ as *mut _);
            }
        }
        if p1 == p2 {
            p1
        } else {
            -1
        }
    }
}

// ---------------------------------------------------------------------------
// P_LineOpening
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LineOpening(linedef: *mut line_t) {
    unsafe {
        let linedef = &*linedef;
        if linedef.sidenum[1] == -1 {
            openrange = 0;
            return;
        }
        let front = linedef.frontsector as *mut sector_t;
        let back = linedef.backsector as *mut sector_t;
        if (*front).ceilingheight < (*back).ceilingheight {
            opentop = (*front).ceilingheight;
        } else {
            opentop = (*back).ceilingheight;
        }
        if (*front).floorheight > (*back).floorheight {
            openbottom = (*front).floorheight;
            lowfloor = (*back).floorheight;
        } else {
            openbottom = (*back).floorheight;
            lowfloor = (*front).floorheight;
        }
        openrange = opentop - openbottom;
    }
}

// ---------------------------------------------------------------------------
// P_UnsetThingPosition
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_UnsetThingPosition(thing: *mut mobj_t) {
    unsafe {
        let thing = &mut *thing;
        if (thing.flags & MF_NOSECTOR) == 0 {
            if !thing.snext.is_null() {
                let snext = thing.snext as *mut mobj_t;
                (*snext).sprev = thing.sprev;
            }
            if !thing.sprev.is_null() {
                let sprev = thing.sprev as *mut mobj_t;
                (*sprev).snext = thing.snext;
            } else {
                let subsector = thing.subsector as *mut subsector_t;
                let sector = (*subsector).sector as *mut sector_t;
                (*sector).thinglist = thing.snext;
            }
        }
        if (thing.flags & MF_NOBLOCKMAP) == 0 {
            if !thing.bnext.is_null() {
                let bnext = thing.bnext as *mut mobj_t;
                (*bnext).bprev = thing.bprev;
            }
            if !thing.bprev.is_null() {
                let bprev = thing.bprev as *mut mobj_t;
                (*bprev).bnext = thing.bnext;
            } else {
                let blockx = (thing.x - crate::doom::p_setup::bmaporgx) >> MAPBLOCKSHIFT;
                let blocky = (thing.y - crate::doom::p_setup::bmaporgy) >> MAPBLOCKSHIFT;
                if blockx >= 0
                    && blockx < crate::doom::p_setup::bmapwidth
                    && blocky >= 0
                    && blocky < crate::doom::p_setup::bmapheight
                {
                    let idx = (blocky * crate::doom::p_setup::bmapwidth + blockx) as isize;
                    *crate::doom::p_setup::blocklinks.offset(idx) = thing.bnext;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// P_SetThingPosition
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_SetThingPosition(thing: *mut mobj_t) {
    unsafe {
        let thing = &mut *thing;
        let ss = crate::doom::r_main::R_PointInSubsector(thing.x, thing.y) as *mut subsector_t;
        thing.subsector = ss as *mut c_void;
        if (thing.flags & MF_NOSECTOR) == 0 {
            let sec = (*ss).sector as *mut sector_t;
            thing.sprev = ptr::null_mut();
            thing.snext = (*sec).thinglist;
            if !(*sec).thinglist.is_null() {
                let thinglist = (*sec).thinglist as *mut mobj_t;
                (*thinglist).sprev = thing as *mut mobj_t as *mut c_void;
            }
            (*sec).thinglist = thing as *mut mobj_t as *mut c_void;
        }
        if (thing.flags & MF_NOBLOCKMAP) == 0 {
            let blockx = (thing.x - crate::doom::p_setup::bmaporgx) >> MAPBLOCKSHIFT;
            let blocky = (thing.y - crate::doom::p_setup::bmaporgy) >> MAPBLOCKSHIFT;
            if blockx >= 0
                && blockx < crate::doom::p_setup::bmapwidth
                && blocky >= 0
                && blocky < crate::doom::p_setup::bmapheight
            {
                let idx = (blocky * crate::doom::p_setup::bmapwidth + blockx) as isize;
                let link = crate::doom::p_setup::blocklinks.offset(idx);
                thing.bprev = ptr::null_mut();
                thing.bnext = *link;
                if !(*link).is_null() {
                    let l = *link as *mut mobj_t;
                    (*l).bprev = thing as *mut mobj_t as *mut c_void;
                }
                *link = thing as *mut mobj_t as *mut c_void;
            } else {
                thing.bnext = ptr::null_mut();
                thing.bprev = ptr::null_mut();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// P_BlockLinesIterator
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_BlockLinesIterator(
    x: c_int,
    y: c_int,
    func: Option<unsafe extern "C" fn(*mut line_t) -> c_uint>,
) -> c_uint {
    unsafe {
        if x < 0
            || y < 0
            || x >= crate::doom::p_setup::bmapwidth
            || y >= crate::doom::p_setup::bmapheight
        {
            return 1;
        }
        let offset = (y * crate::doom::p_setup::bmapwidth + x) as isize;
        let offset = *crate::doom::p_setup::blockmap.offset(offset) as isize;
        let mut list = crate::doom::p_setup::blockmaplump.offset(offset);
        while *list != -1 {
            let ld = crate::doom::p_setup::lines.offset(*list as isize);
            if (*ld).validcount == crate::doom::r_main::validcount {
                list = list.offset(1);
                continue;
            }
            (*ld).validcount = crate::doom::r_main::validcount;
            if func.unwrap()(ld) == 0 {
                return 0;
            }
            list = list.offset(1);
        }
        1
    }
}

// ---------------------------------------------------------------------------
// P_BlockThingsIterator
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_BlockThingsIterator(
    x: c_int,
    y: c_int,
    func: Option<unsafe extern "C" fn(*mut mobj_t) -> c_uint>,
) -> c_uint {
    unsafe {
        if x < 0
            || y < 0
            || x >= crate::doom::p_setup::bmapwidth
            || y >= crate::doom::p_setup::bmapheight
        {
            return 1;
        }
        let idx = (y * crate::doom::p_setup::bmapwidth + x) as isize;
        let mut mobj = *crate::doom::p_setup::blocklinks.offset(idx) as *mut mobj_t;
        while !mobj.is_null() {
            if func.unwrap()(mobj) == 0 {
                return 0;
            }
            mobj = (*mobj).bnext as *mut mobj_t;
        }
        1
    }
}

// ---------------------------------------------------------------------------
// Intercept routines
// ---------------------------------------------------------------------------

#[allow(unused_assignments)]
unsafe fn InterceptsMemoryOverrun(location: c_int, value: c_int) {
    let mut offset: usize = 0;
    let loc = location as usize;

    macro_rules! skip {
        ($len:expr) => {
            if offset + $len > loc {
                return;
            }
            offset += $len;
        };
    }
    macro_rules! write_i32 {
        ($len:expr, $ptr:expr) => {
            if offset + $len > loc {
                let index = (loc - offset) / 4;
                ($ptr as *mut c_int).add(index).write(value);
                return;
            }
            offset += $len;
        };
    }
    macro_rules! write_i16_arr {
        ($len:expr, $ptr:expr) => {
            if offset + $len > loc {
                let index = (loc - offset) / 2;
                let p = ($ptr as *mut _ as *mut u8) as *mut i16;
                p.add(index).write((value & 0xffff) as i16);
                p.add(index + 1).write(((value >> 16) & 0xffff) as i16);
                return;
            }
            offset += $len;
        };
    }

    skip!(4); // 0
    skip!(4); // 1 earlyout
    skip!(4); // 2 intercept_p
    write_i32!(4, std::ptr::addr_of_mut!(lowfloor)); // 3 lowfloor
    write_i32!(4, std::ptr::addr_of_mut!(openbottom)); // 4 openbottom
    write_i32!(4, std::ptr::addr_of_mut!(opentop)); // 5 opentop
    write_i32!(4, std::ptr::addr_of_mut!(openrange)); // 6 openrange
    skip!(4); // 7
    skip!(120); // 8 activeplats
    skip!(8); // 9
    write_i32!(4, std::ptr::addr_of_mut!(crate::doom::p_pspr::bulletslope)); // 10 bulletslope
    skip!(4); // 11 swingx
    skip!(4); // 12 swingy
    skip!(4); // 13
    write_i16_arr!(
        40,
        std::ptr::addr_of_mut!(crate::doom::p_setup::playerstarts)
    ); // 14 playerstarts
    skip!(4); // 15 blocklinks
    write_i32!(4, std::ptr::addr_of_mut!(crate::doom::p_setup::bmapwidth)); // 16 bmapwidth
    skip!(4); // 17 blockmap
    write_i32!(4, std::ptr::addr_of_mut!(crate::doom::p_setup::bmaporgx)); // 18 bmaporgx
    write_i32!(4, std::ptr::addr_of_mut!(crate::doom::p_setup::bmaporgy)); // 19 bmaporgy
    skip!(4); // 20 blockmaplump
    write_i32!(4, std::ptr::addr_of_mut!(crate::doom::p_setup::bmapheight)); // 21 bmapheight
}

unsafe fn InterceptsOverrun(num_intercepts: c_int, intercept: *mut intercept_t) {
    if num_intercepts <= MAXINTERCEPTS_ORIGINAL as c_int {
        return;
    }
    let location = (num_intercepts - MAXINTERCEPTS_ORIGINAL as c_int - 1) * 12;
    InterceptsMemoryOverrun(location, (*intercept).frac);
    InterceptsMemoryOverrun(location + 4, (*intercept).isaline);
    InterceptsMemoryOverrun(location + 8, (*intercept).d.thing as usize as c_int);
}

// ---------------------------------------------------------------------------
// PIT_AddLineIntercepts
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn PIT_AddLineIntercepts(ld: *mut line_t) -> c_uint {
    unsafe {
        let ld = &*ld;
        let (s1, s2) = if trace.dx > FRACUNIT * 16
            || trace.dy > FRACUNIT * 16
            || trace.dx < -FRACUNIT * 16
            || trace.dy < -FRACUNIT * 16
        {
            (
                P_PointOnDivlineSide(
                    (*ld.v1).x,
                    (*ld.v1).y,
                    &raw const trace as *const _ as *mut _,
                ),
                P_PointOnDivlineSide(
                    (*ld.v2).x,
                    (*ld.v2).y,
                    &raw const trace as *const _ as *mut _,
                ),
            )
        } else {
            (
                P_PointOnLineSide(trace.x, trace.y, ld as *const _ as *mut _),
                P_PointOnLineSide(
                    trace.x + trace.dx,
                    trace.y + trace.dy,
                    ld as *const _ as *mut _,
                ),
            )
        };
        if s1 == s2 {
            return 1;
        }
        let mut dl = divline_t {
            x: 0,
            y: 0,
            dx: 0,
            dy: 0,
        };
        P_MakeDivline(ld as *const _ as *mut _, &mut dl);
        let frac = P_InterceptVector(&raw const trace as *const _ as *mut _, &mut dl);
        if frac < 0 {
            return 1;
        }
        if earlyout != 0 && frac < FRACUNIT && ld.backsector.is_null() {
            return 0;
        }
        (*intercept_p).frac = frac;
        (*intercept_p).isaline = 1;
        (*intercept_p).d.line = ld as *const line_t as *mut line_t;
        InterceptsOverrun(
            intercept_p.offset_from(std::ptr::addr_of_mut!(intercepts[0])) as c_int,
            intercept_p,
        );
        intercept_p = intercept_p.offset(1);
        1
    }
}

// ---------------------------------------------------------------------------
// PIT_AddThingIntercepts
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn PIT_AddThingIntercepts(thing: *mut mobj_t) -> c_uint {
    unsafe {
        let thing = &*thing;
        // C: tracepositive = (trace.dx ^ trace.dy)>0;
        let tracepositive = (trace.dx ^ trace.dy) > 0;
        let (x1, y1, x2, y2) = if tracepositive {
            (
                thing.x - thing.radius,
                thing.y + thing.radius,
                thing.x + thing.radius,
                thing.y - thing.radius,
            )
        } else {
            (
                thing.x - thing.radius,
                thing.y - thing.radius,
                thing.x + thing.radius,
                thing.y + thing.radius,
            )
        };
        let s1 = P_PointOnDivlineSide(x1, y1, &raw const trace as *const _ as *mut _);
        let s2 = P_PointOnDivlineSide(x2, y2, &raw const trace as *const _ as *mut _);
        if s1 == s2 {
            return 1;
        }
        let mut dl = divline_t {
            x: x1,
            y: y1,
            dx: x2 - x1,
            dy: y2 - y1,
        };
        let frac = P_InterceptVector(&raw const trace as *const _ as *mut _, &mut dl);
        if frac < 0 {
            return 1;
        }
        (*intercept_p).frac = frac;
        (*intercept_p).isaline = 0;
        (*intercept_p).d.thing = thing as *const _ as *mut c_void;
        InterceptsOverrun(
            intercept_p.offset_from(std::ptr::addr_of_mut!(intercepts[0])) as c_int,
            intercept_p,
        );
        intercept_p = intercept_p.offset(1);
        1
    }
}

// ---------------------------------------------------------------------------
// P_TraverseIntercepts
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_TraverseIntercepts(
    func: Option<unsafe extern "C" fn(*mut intercept_t) -> c_uint>,
    maxfrac: fixed_t,
) -> c_uint {
    unsafe {
        let count = if intercept_p.is_null() {
            0
        } else {
            intercept_p.offset_from(std::ptr::addr_of_mut!(intercepts[0])) as c_int
        };
        for _ in 0..count {
            let mut dist = c_int::MAX;
            let mut in_ptr: *mut intercept_t = ptr::null_mut();
            let mut scan = std::ptr::addr_of_mut!(intercepts[0]);
            while scan < intercept_p {
                if (*scan).frac < dist {
                    dist = (*scan).frac;
                    in_ptr = scan;
                }
                scan = scan.offset(1);
            }
            if dist > maxfrac {
                return 1;
            }
            if func.unwrap()(in_ptr) == 0 {
                return 0;
            }
            (*in_ptr).frac = c_int::MAX;
        }
        1
    }
}

// ---------------------------------------------------------------------------
// P_PathTraverse
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_PathTraverse(
    x1: fixed_t,
    y1: fixed_t,
    x2: fixed_t,
    y2: fixed_t,
    flags: c_int,
    trav: Option<unsafe extern "C" fn(*mut intercept_t) -> c_uint>,
) -> c_uint {
    unsafe {
        earlyout = flags & PT_EARLYOUT;
        crate::doom::r_main::validcount = crate::doom::r_main::validcount.wrapping_add(1);
        intercept_p = std::ptr::addr_of_mut!(intercepts[0]);
        let mut x1 = x1;
        let mut y1 = y1;
        if ((x1 - crate::doom::p_setup::bmaporgx) & (MAPBLOCKSIZE - 1)) == 0 {
            x1 += FRACUNIT;
        }
        if ((y1 - crate::doom::p_setup::bmaporgy) & (MAPBLOCKSIZE - 1)) == 0 {
            y1 += FRACUNIT;
        }
        trace.x = x1;
        trace.y = y1;
        trace.dx = x2 - x1;
        trace.dy = y2 - y1;
        let x1 = x1 - crate::doom::p_setup::bmaporgx;
        let y1 = y1 - crate::doom::p_setup::bmaporgy;
        let xt1 = x1 >> MAPBLOCKSHIFT;
        let yt1 = y1 >> MAPBLOCKSHIFT;
        let x2 = x2 - crate::doom::p_setup::bmaporgx;
        let y2 = y2 - crate::doom::p_setup::bmaporgy;
        let xt2 = x2 >> MAPBLOCKSHIFT;
        let yt2 = y2 >> MAPBLOCKSHIFT;
        let (mapxstep, partial, ystep) = if xt2 > xt1 {
            (
                1,
                FRACUNIT - ((x1 >> MAPBTOFRAC) & (FRACUNIT - 1)),
                FixedDiv(y2 - y1, (x2 - x1).wrapping_abs()),
            )
        } else if xt2 < xt1 {
            (
                -1,
                (x1 >> MAPBTOFRAC) & (FRACUNIT - 1),
                FixedDiv(y2 - y1, (x2 - x1).wrapping_abs()),
            )
        } else {
            (0, FRACUNIT, 256 * FRACUNIT)
        };
        let mut yintercept = (y1 >> MAPBTOFRAC) + FixedMul(partial, ystep);
        let (mapystep, partial, xstep) = if yt2 > yt1 {
            (
                1,
                FRACUNIT - ((y1 >> MAPBTOFRAC) & (FRACUNIT - 1)),
                FixedDiv(x2 - x1, (y2 - y1).wrapping_abs()),
            )
        } else if yt2 < yt1 {
            (
                -1,
                (y1 >> MAPBTOFRAC) & (FRACUNIT - 1),
                FixedDiv(x2 - x1, (y2 - y1).wrapping_abs()),
            )
        } else {
            (0, FRACUNIT, 256 * FRACUNIT)
        };
        let mut xintercept = (x1 >> MAPBTOFRAC) + FixedMul(partial, xstep);
        let mut mapx = xt1;
        let mut mapy = yt1;
        for _ in 0..64 {
            if flags & PT_ADDLINES != 0
                && P_BlockLinesIterator(
                    mapx,
                    mapy,
                    Some(PIT_AddLineIntercepts as unsafe extern "C" fn(*mut line_t) -> c_uint),
                ) == 0
            {
                return 0;
            }
            if flags & PT_ADDTHINGS != 0
                && P_BlockThingsIterator(
                    mapx,
                    mapy,
                    Some(PIT_AddThingIntercepts as unsafe extern "C" fn(*mut mobj_t) -> c_uint),
                ) == 0
            {
                return 0;
            }
            if mapx == xt2 && mapy == yt2 {
                break;
            }
            if (yintercept >> FRACBITS) == mapy {
                yintercept += ystep;
                mapx += mapxstep;
            } else if (xintercept >> FRACBITS) == mapx {
                xintercept += xstep;
                mapy += mapystep;
            }
        }
        P_TraverseIntercepts(trav, FRACUNIT)
    }
}
