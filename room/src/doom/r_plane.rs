//! Rust port of vendor/doomgeneric/r_plane.c.
//!
//! Core visplane rendering: drawing floors and ceilings while maintaining
//! a per-column clipping list. Also handles sky areas.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_int, c_short, c_uchar, c_void};
use std::ptr;

use super::m_fixed::{angle_t, fixed_t, FixedDiv, FixedMul};
use super::r_sky;
use super::tables;
use super::tables::{ANG90, ANGLETOFINESHIFT, FINEMASK};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const SCREENWIDTH: usize = crate::doom::i_video::SCREENWIDTH as usize;
const SCREENHEIGHT: usize = crate::doom::i_video::SCREENHEIGHT as usize;

const MAXVISPLANES: usize = 128;
const MAXOPENINGS: usize = SCREENWIDTH * 64;

const LIGHTLEVELS: usize = 16;
const LIGHTSEGSHIFT: u32 = 4;
const MAXLIGHTZ: usize = 128;
const LIGHTZSHIFT: u32 = 20;

const ANGLETOSKYSHIFT: u32 = 22;

/// Safe accessor for finecosine table (it's a pointer into finesine).
#[inline]
fn finecosine(idx: usize) -> c_int {
    unsafe { *tables::finecosine.0.add(idx & FINEMASK as usize) }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

type lighttable_t = c_uchar;

pub type planefunction_t = unsafe extern "C" fn(c_int, c_int);

// ---------------------------------------------------------------------------
// visplane_t — mirrors the C struct including pad bytes
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct visplane_t {
    pub height: fixed_t,
    pub picnum: c_int,
    pub lightlevel: c_int,
    pub minx: c_int,
    pub maxx: c_int,
    pub pad1: c_uchar,
    pub top: [c_uchar; SCREENWIDTH],
    pub pad2: c_uchar,
    pub pad3: c_uchar,
    pub bottom: [c_uchar; SCREENWIDTH],
    pub pad4: c_uchar,
}

#[cfg(target_pointer_width = "64")]
const _: () = assert!(
    std::mem::size_of::<visplane_t>() == 664,
    "visplane_t size mismatch"
);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, height) == 0);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, picnum) == 4);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, lightlevel) == 8);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, minx) == 12);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, maxx) == 16);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, pad1) == 20);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, top) == 21);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, pad2) == 341);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, pad3) == 342);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, bottom) == 343);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::offset_of!(visplane_t, pad4) == 663);

impl Default for visplane_t {
    fn default() -> Self {
        Self {
            height: 0,
            picnum: 0,
            lightlevel: 0,
            minx: 0,
            maxx: 0,
            pad1: 0,
            top: [0xFF; SCREENWIDTH],
            pad2: 0,
            pad3: 0,
            bottom: [0; SCREENWIDTH],
            pad4: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Global mutable state (exported for C consumers)
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut floorfunc: Option<planefunction_t> = None;

#[no_mangle]
pub static mut ceilingfunc: Option<planefunction_t> = None;

#[no_mangle]
pub static mut visplanes: [visplane_t; MAXVISPLANES] = {
    // Can't use Default::default() in const context, so we transmute
    // from zeroed bytes. Each visplane_t is initialized at runtime.
    const ZERO: visplane_t = visplane_t {
        height: 0,
        picnum: 0,
        lightlevel: 0,
        minx: 0,
        maxx: 0,
        pad1: 0,
        top: [0; SCREENWIDTH],
        pad2: 0,
        pad3: 0,
        bottom: [0; SCREENWIDTH],
        pad4: 0,
    };
    let arr: [visplane_t; MAXVISPLANES] = [ZERO; MAXVISPLANES];
    // Initialize top to 0xFF for each visplane at runtime via memset-like
    // approach. For now leave as 0; R_ClearPlanes will reset them.
    arr
};

#[no_mangle]
pub static mut lastvisplane: *mut visplane_t = ptr::null_mut();

#[no_mangle]
pub static mut floorplane: *mut visplane_t = ptr::null_mut();

#[no_mangle]
pub static mut ceilingplane: *mut visplane_t = ptr::null_mut();

#[no_mangle]
pub static mut openings: [c_short; MAXOPENINGS] = [0; MAXOPENINGS];

#[no_mangle]
pub static mut lastopening: *mut c_short = ptr::null_mut();

#[no_mangle]
pub static mut floorclip: [c_short; SCREENWIDTH] = [0; SCREENWIDTH];

#[no_mangle]
pub static mut ceilingclip: [c_short; SCREENWIDTH] = [0; SCREENWIDTH];

#[no_mangle]
pub static mut spanstart: [c_int; SCREENHEIGHT] = [0; SCREENHEIGHT];

#[no_mangle]
pub static mut spanstop: [c_int; SCREENHEIGHT] = [0; SCREENHEIGHT];

#[no_mangle]
pub static mut planezlight: *const *const lighttable_t = ptr::null();

#[no_mangle]
pub static mut planeheight: fixed_t = 0;

#[no_mangle]
pub static mut yslope: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];

#[no_mangle]
pub static mut distscale: [fixed_t; SCREENWIDTH] = [0; SCREENWIDTH];

#[no_mangle]
pub static mut basexscale: fixed_t = 0;

#[no_mangle]
pub static mut baseyscale: fixed_t = 0;

#[no_mangle]
pub static mut cachedheight: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];

#[no_mangle]
pub static mut cacheddistance: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];

#[no_mangle]
pub static mut cachedxstep: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];

#[no_mangle]
pub static mut cachedystep: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];

// ---------------------------------------------------------------------------
// Imports from other modules
// ---------------------------------------------------------------------------

use crate::doom::r_data::{colormaps, firstflat, flattranslation, R_GetColumn};
use crate::doom::r_draw::{
    dc_colormap, dc_iscale, dc_source, dc_texturemid, dc_x, dc_yh, dc_yl, ds_colormap, ds_source,
    ds_x1, ds_x2, ds_xfrac, ds_xstep, ds_y, ds_yfrac, ds_ystep, viewheight, viewwidth,
};
use crate::doom::r_main::{
    centerxfrac, colfunc, detailshift, extralight, fixedcolormap, spanfunc, viewangle, viewx,
    viewy, viewz, xtoviewangle, zlight,
};
use crate::doom::r_sky::{skytexture, skytexturemid};
use crate::doom::r_things::pspriteiscale;
use crate::doom::w_wad::{W_CacheLumpNum, W_ReleaseLumpNum};

// ---------------------------------------------------------------------------
// R_InitPlanes — called once at game startup
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_InitPlanes() {
    // Doh! — nothing to do here, matching original C
}

// ---------------------------------------------------------------------------
// R_MapPlane — maps a single plane span
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_MapPlane(y: c_int, x1: c_int, x2: c_int) {
    unsafe {
        #[cfg(feature = "rangecheck")]
        {
            if x2 < x1 || x1 < 0 || x2 >= viewwidth || y > viewheight {
                // Would call I_Error — skip in Rust for now
                return;
            }
        }

        let y_usize = y as usize;

        let distance = if planeheight != cachedheight[y_usize] {
            cachedheight[y_usize] = planeheight;
            let d = FixedMul(planeheight, yslope[y_usize]);
            cacheddistance[y_usize] = d;
            cachedxstep[y_usize] = FixedMul(d, basexscale);
            cachedystep[y_usize] = FixedMul(d, baseyscale);
            d
        } else {
            cacheddistance[y_usize]
        };

        ds_xstep = cachedxstep[y_usize];
        ds_ystep = cachedystep[y_usize];

        let length = FixedMul(distance, distscale[x1 as usize]);
        let angle = viewangle.wrapping_add(xtoviewangle[x1 as usize]) >> ANGLETOFINESHIFT;
        ds_xfrac = viewx.wrapping_add(FixedMul(finecosine(angle as usize), length));
        ds_yfrac = (-viewy).wrapping_sub(FixedMul(
            tables::finesine[angle as usize & FINEMASK as usize],
            length,
        ));

        if !fixedcolormap.is_null() {
            ds_colormap = fixedcolormap;
        } else {
            let mut index = (distance >> LIGHTZSHIFT) as usize;
            if index >= MAXLIGHTZ {
                index = MAXLIGHTZ - 1;
            }
            ds_colormap = *planezlight.add(index) as *mut u8;
        }

        ds_y = y;
        ds_x1 = x1;
        ds_x2 = x2;

        if let Some(func) = spanfunc {
            func();
        }
    }
}

// ---------------------------------------------------------------------------
// R_ClearPlanes — called at beginning of each frame
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_ClearPlanes() {
    unsafe {
        let vw = viewwidth as usize;
        let vh = viewheight as c_short;

        for i in 0..vw {
            floorclip[i] = vh;
            ceilingclip[i] = -1;
        }

        lastvisplane = visplanes.as_mut_ptr();
        lastopening = openings.as_mut_ptr();

        // Reset cache heights
        for h in cachedheight.iter_mut() {
            *h = 0;
        }

        // Left to right mapping
        let angle = (viewangle.wrapping_sub(ANG90)) >> ANGLETOFINESHIFT;
        basexscale = FixedDiv(finecosine(angle as usize), centerxfrac);
        baseyscale = 0i32.wrapping_sub(FixedDiv(
            tables::finesine[angle as usize & FINEMASK as usize],
            centerxfrac,
        ));
    }
}

// ---------------------------------------------------------------------------
// R_FindPlane — find or create a visplane matching the given parameters
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_FindPlane(
    height: fixed_t,
    picnum: c_int,
    lightlevel: c_int,
) -> *mut visplane_t {
    unsafe {
        let mut h = height;
        let mut ll = lightlevel;

        // All sky planes map together
        if picnum == r_sky::skyflatnum {
            h = 0;
            ll = 0;
        }

        // Search existing visplanes
        let mut check = visplanes.as_mut_ptr();
        let end = lastvisplane;

        while check < end {
            if h == (*check).height && picnum == (*check).picnum && ll == (*check).lightlevel {
                return check;
            }
            check = check.add(1);
        }

        // Need a new visplane
        if (lastvisplane as usize - visplanes.as_ptr() as usize) / std::mem::size_of::<visplane_t>()
            >= MAXVISPLANES
        {
            // Would call I_Error — for now just return null
            return ptr::null_mut();
        }

        let new_vp = lastvisplane;
        lastvisplane = lastvisplane.add(1);

        (*new_vp).height = h;
        (*new_vp).picnum = picnum;
        (*new_vp).lightlevel = ll;
        (*new_vp).minx = SCREENWIDTH as c_int;
        (*new_vp).maxx = -1;

        // memset top to 0xFF
        for t in (*new_vp).top.iter_mut() {
            *t = 0xFF;
        }

        new_vp
    }
}

// ---------------------------------------------------------------------------
// R_CheckPlane — check if a visplane can cover [start..stop] without overlap
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_CheckPlane(pl: *mut visplane_t, start: c_int, stop: c_int) -> *mut visplane_t {
    unsafe {
        let intrl = if start < (*pl).minx {
            (*pl).minx
        } else {
            start
        };
        let unionl = if start < (*pl).minx {
            start
        } else {
            (*pl).minx
        };

        let intrh = if stop > (*pl).maxx { (*pl).maxx } else { stop };
        let unionh = if stop > (*pl).maxx { stop } else { (*pl).maxx };

        // Check for overlap in the intersection range
        let mut x = intrl;
        while x <= intrh {
            if (*pl).top[x as usize] != 0xFF {
                break;
            }
            x += 1;
        }

        if x > intrh {
            // No overlap — extend the visplane
            (*pl).minx = unionl;
            (*pl).maxx = unionh;
            return pl;
        }

        // Make a new visplane
        let new_vp = lastvisplane;
        lastvisplane = lastvisplane.add(1);

        (*new_vp).height = (*pl).height;
        (*new_vp).picnum = (*pl).picnum;
        (*new_vp).lightlevel = (*pl).lightlevel;
        (*new_vp).minx = start;
        (*new_vp).maxx = stop;

        for t in (*new_vp).top.iter_mut() {
            *t = 0xFF;
        }

        new_vp
    }
}

// ---------------------------------------------------------------------------
// R_MakeSpans — generate spans from ceiling/floor clip differences
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_MakeSpans(x: c_int, t1: c_int, b1: c_int, t2: c_int, b2: c_int) {
    unsafe {
        let mut t1 = t1;
        let mut b1 = b1;
        let mut t2 = t2;
        let mut b2 = b2;

        while t1 < t2 && t1 <= b1 {
            R_MapPlane(t1, spanstart[t1 as usize], x - 1);
            t1 += 1;
        }
        while b1 > b2 && b1 >= t1 {
            R_MapPlane(b1, spanstart[b1 as usize], x - 1);
            b1 -= 1;
        }

        while t2 < t1 && t2 <= b2 {
            spanstart[t2 as usize] = x;
            t2 += 1;
        }
        while b2 > b1 && b2 >= t2 {
            spanstart[b2 as usize] = x;
            b2 -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawPlanes — draw all visplanes at end of frame
// ---------------------------------------------------------------------------

use crate::doom::z_zone::PU_STATIC;

#[no_mangle]
pub extern "C" fn R_DrawPlanes() {
    unsafe {
        let mut pl = visplanes.as_mut_ptr();
        let end = lastvisplane;

        while pl < end {
            if (*pl).minx > (*pl).maxx {
                pl = pl.add(1);
                continue;
            }

            // Sky flat
            if (*pl).picnum == r_sky::skyflatnum {
                dc_iscale = pspriteiscale >> detailshift;
                // Sky is always drawn full bright,
                // i.e. colormaps[0] is used.
                // Because of this hack, sky is not affected
                // by INVUL inverse mapping.
                dc_colormap = colormaps;
                dc_texturemid = skytexturemid;

                for x in (*pl).minx..=(*pl).maxx {
                    dc_yl = (*pl).top[x as usize] as c_int;
                    dc_yh = (*pl).bottom[x as usize] as c_int;

                    if dc_yl <= dc_yh {
                        let angle =
                            viewangle.wrapping_add(xtoviewangle[x as usize]) >> ANGLETOSKYSHIFT;
                        dc_x = x;
                        dc_source = R_GetColumn(skytexture, angle as c_int);
                        if let Some(func) = colfunc {
                            func();
                        }
                    }
                }
                pl = pl.add(1);
                continue;
            }

            // Regular flat
            let flat_idx = *flattranslation.offset((*pl).picnum as isize);
            let lumpnum = firstflat + flat_idx;
            ds_source = W_CacheLumpNum(lumpnum, PU_STATIC) as *mut u8;

            planeheight = ((*pl).height.wrapping_sub(viewz)).abs();
            let mut light = ((*pl).lightlevel >> LIGHTSEGSHIFT) as c_int + extralight;

            if light >= LIGHTLEVELS as c_int {
                light = LIGHTLEVELS as c_int - 1;
            }
            if light < 0 {
                light = 0;
            }

            // Set planezlight — in the full implementation this would be
            // a pointer into zlight[light]
            let light_usize = light as usize;
            if light_usize < LIGHTLEVELS {
                planezlight = zlight[light_usize].as_ptr() as *const *const lighttable_t;
            }

            // Set sentinel values at the visplane boundaries. The C code does
            // pl->top[pl->maxx+1] = 0xff and pl->top[pl->minx-1] = 0xff, which
            // relies on padding bytes when at screen edges.
            if (*pl).minx > 0 {
                (*pl).top[((*pl).minx - 1) as usize] = 0xFF;
            } else {
                (*pl).pad1 = 0xFF;
            }
            if (*pl).maxx < SCREENWIDTH as c_int - 1 {
                (*pl).top[((*pl).maxx + 1) as usize] = 0xFF;
            } else {
                (*pl).pad2 = 0xFF;
            }

            let stop = (*pl).maxx + 1;
            for x in (*pl).minx..=stop {
                let prev_x = x - 1;
                let t_top = if prev_x < 0 {
                    (*pl).pad1
                } else {
                    (*pl).top[prev_x as usize]
                };
                let t_bottom = if prev_x < 0 {
                    (*pl).pad3
                } else {
                    (*pl).bottom[prev_x as usize]
                };
                let b_top = if x == SCREENWIDTH as c_int {
                    (*pl).pad2
                } else {
                    (*pl).top[x as usize]
                };
                let b_bottom = if x == SCREENWIDTH as c_int {
                    (*pl).pad4
                } else {
                    (*pl).bottom[x as usize]
                };
                R_MakeSpans(
                    x,
                    t_top as c_int,
                    t_bottom as c_int,
                    b_top as c_int,
                    b_bottom as c_int,
                );
            }

            W_ReleaseLumpNum(lumpnum);
            pl = pl.add(1);
        }
    }
}
