//! Rust port of vendor/doomgeneric/p_sight.c.
//!
//! LineOfSight/visibility checks using REJECT lookup table and BSP traversal.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use crate::i_error;
use std::os::raw::c_int;

use crate::doom::m_fixed::{FixedDiv, FixedMul};

use crate::doom::m_fixed::FRACBITS;

const NF_SUBSECTOR: u32 = 0x8000;
const ML_TWOSIDED: i16 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct divline_t {
    pub x: c_int,
    pub y: c_int,
    pub dx: c_int,
    pub dy: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct vertex_t {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct subsector_t {
    pub sector: *mut sector_t,
    pub numlines: i16,
    pub firstline: i16,
    _pad: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct seg_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub offset: c_int,
    pub angle: u32,
    pub sidedef: *mut c_void,
    pub linedef: *mut line_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
}

// NOTE: The first 4 fields of node_t have the same layout as divline_t.
// C code casts (divline_t*)node for P_DivlineSide calls. We replicate
// the divline_t header here so we can safely transmute.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct node_t {
    pub x: c_int,
    pub y: c_int,
    pub dx: c_int,
    pub dy: c_int,
    pub bbox: [[c_int; 4]; 2],
    pub children: [u16; 2],
}

use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sector_t {
    pub floorheight: c_int,
    pub ceilingheight: c_int,
    pub floorpic: i16,
    pub ceilingpic: i16,
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
    _pad0: [u8; 2],
    pub soundtraversed: c_int,
    pub soundtarget: *mut c_void,
    pub blockbox: [c_int; 4],
    pub soundorg: [u8; 40],
    pub validcount: c_int,
    pub thinglist: *mut c_void,
    pub specialdata: *mut c_void,
    pub linecount: c_int,
    pub lines: *mut *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct line_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub dx: c_int,
    pub dy: c_int,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
    pub bbox: [c_int; 4],
    pub slopetype: c_int,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub validcount: c_int,
    pub specialdata: *mut c_void,
}

#[no_mangle]
pub static mut sightzstart: c_int = 0;
#[no_mangle]
pub static mut topslope: c_int = 0;
#[no_mangle]
pub static mut bottomslope: c_int = 0;
#[no_mangle]
pub static mut strace: divline_t = divline_t {
    x: 0,
    y: 0,
    dx: 0,
    dy: 0,
};
#[no_mangle]
pub static mut t2x: c_int = 0;
#[no_mangle]
pub static mut t2y: c_int = 0;
#[no_mangle]
pub static mut sightcounts: [c_int; 2] = [0, 0];

extern "C" {
    static mut numsubsectors: c_int;
    static mut subsectors: *mut subsector_t;
    static mut segs: *mut seg_t;
    static mut validcount: c_int;
    static mut nodes: *mut node_t;
    static mut numnodes: c_int;
    static mut rejectmatrix: *mut u8;
    static mut numsectors: c_int;
    static mut sectors: *mut sector_t;
}
// Reuse the authoritative mobj_t mirror from p_telept.rs to guarantee
// field offsets match the C layout (x=24, subsector=88, height=108).
pub use crate::doom::p_telept::mobj_t;
// Returns side 0 (front), 1 (back), or 2 (on).

fn P_DivlineSide(x: c_int, y: c_int, node: &divline_t) -> c_int {
    if node.dx == 0 {
        if x == node.x {
            return 2;
        }
        return if x <= node.x {
            (node.dy > 0) as c_int
        } else {
            (node.dy < 0) as c_int
        };
    }

    if node.dy == 0 {
        // NOTE: The original C code reads `if (x==node->y)` here — NOT
        // `y==node->y`. This is a long-standing quirk in the Doom source
        // that demos rely on for deterministic playback. DO NOT "fix" it.
        if x == node.y {
            return 2;
        }
        return if y <= node.y {
            (node.dx < 0) as c_int
        } else {
            (node.dx > 0) as c_int
        };
    }

    let dx = x - node.x;
    let dy = y - node.y;

    let left = (node.dy >> FRACBITS) * (dx >> FRACBITS);
    let right = (dy >> FRACBITS) * (node.dx >> FRACBITS);

    if right < left {
        return 0; // front side
    }
    if left == right {
        return 2;
    }
    1 // back side
}
// Returns the fractional intercept point along the first divline.

fn P_InterceptVector2(v2: &divline_t, v1: &divline_t) -> c_int {
    let den = unsafe { FixedMul(v1.dy >> 8, v2.dx) - FixedMul(v1.dx >> 8, v2.dy) };

    if den == 0 {
        return 0;
    }

    let num = unsafe { FixedMul((v1.x - v2.x) >> 8, v1.dy) + FixedMul((v2.y - v1.y) >> 8, v1.dx) };

    unsafe { FixedDiv(num, den) }
}

fn P_CrossSubsector(num: c_int) -> bool {
    unsafe {
        if num >= numsubsectors {
            i_error!(
                "P_CrossSubsector: ss {} with numss = {}",
                num,
                numsubsectors
            );
        }

        let sub = &*subsectors.add(num as usize);

        let mut count = sub.numlines as c_int;
        let mut seg_ptr = segs.add(sub.firstline as usize);

        while count > 0 {
            let seg = &*seg_ptr;
            seg_ptr = seg_ptr.add(1);
            count -= 1;

            let line = &mut *seg.linedef;

            // Already checked other side?
            if line.validcount == validcount {
                continue;
            }

            line.validcount = validcount;

            let v1 = &*line.v1;
            let v2 = &*line.v2;

            let s1 = P_DivlineSide(v1.x, v1.y, &strace);
            let s2 = P_DivlineSide(v2.x, v2.y, &strace);

            // Line isn't crossed?
            if s1 == s2 {
                continue;
            }

            let divl = divline_t {
                x: v1.x,
                y: v1.y,
                dx: v2.x - v1.x,
                dy: v2.y - v1.y,
            };

            let s1 = P_DivlineSide(strace.x, strace.y, &divl);
            let s2 = P_DivlineSide(t2x, t2y, &divl);

            // Line isn't crossed?
            if s1 == s2 {
                continue;
            }

            // Backsector may be NULL if this is an "impassible glass" hack line.
            // IMPORTANT: C uses line->backsector (the original linedef side), NOT
            // seg->backsector (which is the BSP-split sub-side and may differ).
            if line.backsector.is_null() {
                return false;
            }

            // Stop because it is not two sided anyway.
            // Also must use line->flags, not a re-read through seg->linedef.
            if line.flags & ML_TWOSIDED == 0 {
                return false;
            }

            let front = seg.frontsector;
            let back = seg.backsector;

            let front_floor = (*front).floorheight;
            let front_ceil = (*front).ceilingheight;
            let back_floor = (*back).floorheight;
            let back_ceil = (*back).ceilingheight;

            // No wall to block sight with?
            if front_floor == back_floor && front_ceil == back_ceil {
                continue;
            }

            // Possible occluder.
            let opentop = if front_ceil < back_ceil {
                front_ceil
            } else {
                back_ceil
            };

            let openbottom = if front_floor > back_floor {
                front_floor
            } else {
                back_floor
            };

            // Quick test for totally closed doors.
            if openbottom >= opentop {
                return false;
            }

            let frac = P_InterceptVector2(&strace, &divl);

            if front_floor != back_floor {
                let slope = FixedDiv(openbottom - sightzstart, frac);
                if slope > bottomslope {
                    bottomslope = slope;
                }
            }

            if front_ceil != back_ceil {
                let slope = FixedDiv(opentop - sightzstart, frac);
                if slope < topslope {
                    topslope = slope;
                }
            }

            if topslope <= bottomslope {
                return false;
            }
        }

        true
    }
}

/// Extract the first 4 fields of a node_t as a divline_t.
/// The C code casts `(divline_t*)node` for P_DivlineSide calls;
/// since both structs share the same header layout (x, y, dx, dy),
/// this produces identical results.
#[inline]
fn node_as_divline(node: &node_t) -> divline_t {
    divline_t {
        x: node.x,
        y: node.y,
        dx: node.dx,
        dy: node.dy,
    }
}

fn P_CrossBSPNode(bspnum: c_int) -> bool {
    unsafe {
        if bspnum & NF_SUBSECTOR as c_int != 0 {
            if bspnum == -1 {
                return P_CrossSubsector(0);
            } else {
                return P_CrossSubsector(bspnum & !(NF_SUBSECTOR as c_int));
            }
        }

        let bsp = &*nodes.add(bspnum as usize);
        let bsp_div = node_as_divline(bsp);

        let side = P_DivlineSide(strace.x, strace.y, &bsp_div);
        let side = if side == 2 { 0 } else { side };

        // Cross the starting side.
        if !P_CrossBSPNode(bsp.children[side as usize] as c_int) {
            return false;
        }

        // The partition plane is crossed here.
        let bsp_div2 = node_as_divline(bsp);
        let t2_side = P_DivlineSide(t2x, t2y, &bsp_div2);
        if side == t2_side {
            return true;
        }

        // Cross the ending side.
        P_CrossBSPNode(bsp.children[(side ^ 1) as usize] as c_int)
    }
}

/// Returns C `boolean` (= `unsigned int`, 4 bytes), not Rust `bool` (1 byte).
/// Doom's `typedef unsigned int boolean` means the C caller reads a 32-bit
/// return value; returning Rust `bool` would leave 24 bits undefined and
/// cause sight checks to randomly succeed/fail, desynchronising demos and
/// RNG.
#[no_mangle]
pub extern "C" fn P_CheckSight(t1: *mut mobj_t, t2: *mut mobj_t) -> c_int {
    unsafe {
        // Cast both pointers through *const u8 to sidestep the fact that
        // mobj_t (from p_telept) references a different sector_t type than
        // the one declared here. Sector size is the same in both.
        let sec_size = std::mem::size_of::<sector_t>() as isize;
        let s1 = (((*(*t1).subsector).sector as *const u8).offset_from(sectors as *const u8)
            / sec_size) as c_int;
        let s2 = (((*(*t2).subsector).sector as *const u8).offset_from(sectors as *const u8)
            / sec_size) as c_int;
        let pnum = s1 * numsectors + s2;
        let bytenum = pnum >> 3;
        let bitnum = 1 << (pnum & 7);

        // Check in REJECT table.
        if *rejectmatrix.add(bytenum as usize) & bitnum != 0 {
            sightcounts[0] += 1;
            return 0;
        }

        sightcounts[1] += 1;
        validcount += 1;

        sightzstart = (*t1).z + (*t1).height - ((*t1).height >> 2);
        topslope = (*t2).z + (*t2).height - sightzstart;
        bottomslope = (*t2).z - sightzstart;

        strace.x = (*t1).x;
        strace.y = (*t1).y;
        t2x = (*t2).x;
        t2y = (*t2).y;
        strace.dx = (*t2).x - (*t1).x;
        strace.dy = (*t2).y - (*t1).y;

        P_CrossBSPNode(numnodes - 1) as c_int
    }
}

/// Anchor function to ensure exports survive linker dead-code elimination.
#[no_mangle]
pub extern "C" fn P_Sight_Link_Anchor() {
    let _ = P_CheckSight as *const () as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const DIVLINE_T_SIZEOF: usize = 16;
    const VERTEX_T_SIZEOF: usize = 8;
    const SUBSECTOR_T_SIZEOF: usize = 16;
    const SEG_T_SIZEOF: usize = 56;
    const NODE_T_SIZEOF: usize = 52;
    const SECTOR_T_SIZEOF: usize = 128;
    const LINE_T_SIZEOF: usize = 88;

    #[test]
    fn divline_t_size() {
        assert_eq!(std::mem::size_of::<divline_t>(), DIVLINE_T_SIZEOF);
    }

    #[test]
    fn vertex_t_size() {
        assert_eq!(std::mem::size_of::<vertex_t>(), VERTEX_T_SIZEOF);
    }

    #[test]
    fn subsector_t_size() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<subsector_t>(), SUBSECTOR_T_SIZEOF,);
    }

    #[test]
    fn seg_t_size() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<seg_t>(), SEG_T_SIZEOF,);
    }

    #[test]
    fn node_t_size() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<node_t>(), NODE_T_SIZEOF,);
    }

    #[test]
    fn sector_t_layout() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<sector_t>(), SECTOR_T_SIZEOF);
        assert_eq!(std::mem::offset_of!(sector_t, floorheight), 0);
        assert_eq!(std::mem::offset_of!(sector_t, ceilingheight), 4);
        assert_eq!(std::mem::offset_of!(sector_t, floorpic), 8);
        assert_eq!(std::mem::offset_of!(sector_t, ceilingpic), 10);
        assert_eq!(std::mem::offset_of!(sector_t, lightlevel), 12);
        assert_eq!(std::mem::offset_of!(sector_t, special), 14);
        assert_eq!(std::mem::offset_of!(sector_t, tag), 16);
        assert_eq!(std::mem::offset_of!(sector_t, soundtraversed), 20);
        assert_eq!(std::mem::offset_of!(sector_t, soundtarget), 24);
        assert_eq!(std::mem::offset_of!(sector_t, blockbox), 32);
        assert_eq!(std::mem::offset_of!(sector_t, soundorg), 48);
        assert_eq!(std::mem::offset_of!(sector_t, validcount), 88);
        assert_eq!(std::mem::offset_of!(sector_t, thinglist), 96);
        assert_eq!(std::mem::offset_of!(sector_t, specialdata), 104);
        assert_eq!(std::mem::offset_of!(sector_t, linecount), 112);
        assert_eq!(std::mem::offset_of!(sector_t, lines), 120);
    }

    #[test]
    fn line_t_layout() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<line_t>(), LINE_T_SIZEOF);
        assert_eq!(std::mem::offset_of!(line_t, v1), 0);
        assert_eq!(std::mem::offset_of!(line_t, v2), 8);
        assert_eq!(std::mem::offset_of!(line_t, dx), 16);
        assert_eq!(std::mem::offset_of!(line_t, dy), 20);
        assert_eq!(std::mem::offset_of!(line_t, flags), 24);
        assert_eq!(std::mem::offset_of!(line_t, special), 26);
        assert_eq!(std::mem::offset_of!(line_t, tag), 28);
        assert_eq!(std::mem::offset_of!(line_t, sidenum), 30);
        assert_eq!(std::mem::offset_of!(line_t, bbox), 36);
        assert_eq!(std::mem::offset_of!(line_t, slopetype), 52);
        assert_eq!(std::mem::offset_of!(line_t, frontsector), 56);
        assert_eq!(std::mem::offset_of!(line_t, backsector), 64);
        assert_eq!(std::mem::offset_of!(line_t, validcount), 72);
        assert_eq!(std::mem::offset_of!(line_t, specialdata), 80);
    }
}
