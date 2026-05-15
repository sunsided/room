//! Rust port of vendor/doomgeneric/r_main.c.
//!
//! Renderer main loop, view setup, and utility functions (BSP geometry,
//! trigonometry). Also hosts the bulk of renderer global state.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_short, c_uint};
use std::ptr;

use crate::doom::d_player::PlayerT;
use crate::doom::i_video::{SCREENHEIGHT as SCREENHEIGHT_IV, SCREENWIDTH as SCREENWIDTH_IV};
use crate::doom::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::doom::m_fixed::{angle_t, fixed_t, FixedDiv, FixedMul};
use crate::doom::m_fixed::{FRACBITS, FRACUNIT};
use crate::doom::p_telept::mobj_t;
use crate::doom::r_bsp::{node_t, seg_t, subsector_t};
use crate::doom::tables::{self, SlopeDiv};
use crate::doom::tables::{ANG180, ANG270, ANG90, ANGLETOFINESHIFT};
use crate::types::Boolean;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const FIELDOFVIEW: c_int = 2048;
const SCREENWIDTH: usize = SCREENWIDTH_IV as usize;
const SCREENHEIGHT: usize = SCREENHEIGHT_IV as usize;
const LIGHTLEVELS: usize = 16;
#[allow(dead_code)]
const LIGHTSEGSHIFT: u32 = 4;
const MAXLIGHTSCALE: usize = 48;
const LIGHTSCALESHIFT: u32 = 12;
const MAXLIGHTZ: usize = 128;
const LIGHTZSHIFT: u32 = 20;
const NUMCOLORMAPS: usize = 32;
const DISTMAP: usize = 2;
// DBITS = FRACBITS - SLOPEBITS = 16 - 11 = 5 (matches vendor/doomgeneric/tables.h).
const DBITS: u32 = 5;

type lighttable_t = u8;

// ---------------------------------------------------------------------------
// Globals defined by this module
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut viewangleoffset: c_int = 0;

#[no_mangle]
pub static mut validcount: c_int = 1;

#[no_mangle]
pub static mut fixedcolormap: *mut lighttable_t = ptr::null_mut();

#[no_mangle]
pub static mut centerx: c_int = 0;

#[no_mangle]
pub static mut centery: c_int = 0;

#[no_mangle]
pub static mut centerxfrac: fixed_t = 0;

#[no_mangle]
pub static mut centeryfrac: fixed_t = 0;

#[no_mangle]
pub static mut projection: fixed_t = 0;

#[no_mangle]
pub static mut framecount: c_int = 0;

#[no_mangle]
pub static mut sscount: c_int = 0;

#[no_mangle]
pub static mut linecount: c_int = 0;

#[no_mangle]
pub static mut loopcount: c_int = 0;

#[no_mangle]
pub static mut viewx: fixed_t = 0;

#[no_mangle]
pub static mut viewy: fixed_t = 0;

#[no_mangle]
pub static mut viewz: fixed_t = 0;

#[no_mangle]
pub static mut viewangle: angle_t = 0;

#[no_mangle]
pub static mut viewcos: fixed_t = 0;

#[no_mangle]
pub static mut viewsin: fixed_t = 0;

#[no_mangle]
pub static mut viewplayer: *mut PlayerT = ptr::null_mut();

#[no_mangle]
pub static mut detailshift: c_int = 0;

#[no_mangle]
pub static mut clipangle: angle_t = 0;

// r_bsp.rs originally declared this as [c_int; FINEANGLES/2 + 1] to guard
// against a potential off-by-one in the original C code. Keep the same
// size so the two modules agree.
#[no_mangle]
pub static mut viewangletox: [c_int; tables::FINEANGLES / 2 + 1] = [0; tables::FINEANGLES / 2 + 1];

#[no_mangle]
pub static mut xtoviewangle: [angle_t; SCREENWIDTH + 1] = [0; SCREENWIDTH + 1];

#[no_mangle]
pub static mut scalelight: [[*mut lighttable_t; MAXLIGHTSCALE]; LIGHTLEVELS] =
    [[ptr::null_mut(); MAXLIGHTSCALE]; LIGHTLEVELS];

#[no_mangle]
pub static mut scalelightfixed: [*mut lighttable_t; MAXLIGHTSCALE] =
    [ptr::null_mut(); MAXLIGHTSCALE];

#[no_mangle]
pub static mut zlight: [[*mut lighttable_t; MAXLIGHTZ]; LIGHTLEVELS] =
    [[ptr::null_mut(); MAXLIGHTZ]; LIGHTLEVELS];

#[no_mangle]
pub static mut extralight: c_int = 0;

#[no_mangle]
pub static mut colfunc: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
pub static mut basecolfunc: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
pub static mut fuzzcolfunc: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
pub static mut transcolfunc: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
pub static mut spanfunc: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
pub static mut setsizeneeded: Boolean = Boolean::FALSE;

#[no_mangle]
pub static mut setblocks: c_int = 0;

#[no_mangle]
pub static mut setdetail: c_int = 0;

// ---------------------------------------------------------------------------
// Imports from other modules
// ---------------------------------------------------------------------------

use crate::doom::d_loop::NetUpdate;
use crate::doom::m_menu::{detailLevel, screenblocks};
use crate::doom::p_setup::{numnodes, nodes, subsectors};
use crate::doom::r_bsp::{R_ClearClipSegs, R_ClearDrawSegs, R_RenderBSPNode};
use crate::doom::r_data::{colormaps, R_InitData};
use crate::doom::r_draw::{
    scaledviewwidth, viewheight, viewwidth, R_DrawColumn, R_DrawColumnLow, R_DrawFuzzColumn,
    R_DrawFuzzColumnLow, R_DrawSpan, R_DrawSpanLow, R_DrawTranslatedColumn,
    R_DrawTranslatedColumnLow, R_InitBuffer, R_InitTranslationTables,
};
use crate::doom::r_plane::{R_ClearPlanes, R_DrawPlanes, R_InitPlanes};
use crate::doom::r_segs::{rw_distance, rw_normalangle, walllights};
use crate::doom::r_things::{
    pspriteiscale, pspritescale, screenheightarray, R_ClearSprites, R_DrawMasked,
};

// ---------------------------------------------------------------------------
// R_AddPointToBox
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_AddPointToBox(x: c_int, y: c_int, box_: *mut fixed_t) {
    if x < *box_.add(BOXLEFT) {
        *box_.add(BOXLEFT) = x;
    }
    if x > *box_.add(BOXRIGHT) {
        *box_.add(BOXRIGHT) = x;
    }
    if y < *box_.add(BOXBOTTOM) {
        *box_.add(BOXBOTTOM) = y;
    }
    if y > *box_.add(BOXTOP) {
        *box_.add(BOXTOP) = y;
    }
}

// ---------------------------------------------------------------------------
// R_PointOnSide
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_PointOnSide(x: fixed_t, y: fixed_t, node: *const node_t) -> c_int {
    if (*node).dx == 0 {
        if x <= (*node).x {
            return ((*node).dy > 0) as c_int;
        }
        return ((*node).dy < 0) as c_int;
    }
    if (*node).dy == 0 {
        if y <= (*node).y {
            return ((*node).dx < 0) as c_int;
        }
        return ((*node).dx > 0) as c_int;
    }

    let dx = x - (*node).x;
    let dy = y - (*node).y;

    // Try to quickly decide by looking at sign bits.
    if (((*node).dy ^ (*node).dx ^ dx ^ dy) as u32) & 0x80000000 != 0 {
        if (((*node).dy ^ dx) as u32) & 0x80000000 != 0 {
            return 1;
        }
        return 0;
    }

    let left = FixedMul((*node).dy >> FRACBITS, dx);
    let right = FixedMul(dy, (*node).dx >> FRACBITS);

    if right < left {
        0
    } else {
        1
    }
}

// ---------------------------------------------------------------------------
// R_PointOnSegSide
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_PointOnSegSide(x: fixed_t, y: fixed_t, line: *const seg_t) -> c_int {
    let lx = (*(*line).v1).x;
    let ly = (*(*line).v1).y;

    let ldx = (*(*line).v2).x - lx;
    let ldy = (*(*line).v2).y - ly;

    if ldx == 0 {
        if x <= lx {
            return (ldy > 0) as c_int;
        }
        return (ldy < 0) as c_int;
    }
    if ldy == 0 {
        if y <= ly {
            return (ldx < 0) as c_int;
        }
        return (ldx > 0) as c_int;
    }

    let dx = x - lx;
    let dy = y - ly;

    // Try to quickly decide by looking at sign bits.
    if ((ldy ^ ldx ^ dx ^ dy) as u32) & 0x80000000 != 0 {
        if ((ldy ^ dx) as u32) & 0x80000000 != 0 {
            return 1;
        }
        return 0;
    }

    let left = FixedMul(ldy >> FRACBITS, dx);
    let right = FixedMul(dy, ldx >> FRACBITS);

    if right < left {
        0
    } else {
        1
    }
}

// ---------------------------------------------------------------------------
// R_PointToAngle
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_PointToAngle(x: fixed_t, y: fixed_t) -> angle_t {
    let mut x = x - viewx;
    let mut y = y - viewy;

    if x == 0 && y == 0 {
        return 0;
    }

    if x >= 0 {
        // x >= 0
        if y >= 0 {
            // y >= 0
            if x > y {
                // octant 0
                return tables::tantoangle[SlopeDiv(y as c_uint, x as c_uint) as usize];
            } else {
                // octant 1
                return ANG90 - 1 - tables::tantoangle[SlopeDiv(x as c_uint, y as c_uint) as usize];
            }
        } else {
            // y < 0
            y = -y;
            if x > y {
                // octant 8
                return 0u32
                    .wrapping_sub(tables::tantoangle[SlopeDiv(y as c_uint, x as c_uint) as usize]);
            } else {
                // octant 7
                return ANG270 + tables::tantoangle[SlopeDiv(x as c_uint, y as c_uint) as usize];
            }
        }
    } else {
        // x < 0
        x = -x;
        if y >= 0 {
            // y >= 0
            if x > y {
                // octant 3
                return ANG180
                    - 1
                    - tables::tantoangle[SlopeDiv(y as c_uint, x as c_uint) as usize];
            } else {
                // octant 2
                return ANG90 + tables::tantoangle[SlopeDiv(x as c_uint, y as c_uint) as usize];
            }
        } else {
            // y < 0
            y = -y;
            if x > y {
                // octant 4
                return ANG180 + tables::tantoangle[SlopeDiv(y as c_uint, x as c_uint) as usize];
            } else {
                // octant 5
                return ANG270
                    - 1
                    - tables::tantoangle[SlopeDiv(x as c_uint, y as c_uint) as usize];
            }
        }
    }
}

// ---------------------------------------------------------------------------
// R_PointToAngle2
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_PointToAngle2(
    x1: fixed_t,
    y1: fixed_t,
    x2: fixed_t,
    y2: fixed_t,
) -> angle_t {
    viewx = x1;
    viewy = y1;
    R_PointToAngle(x2, y2)
}

// ---------------------------------------------------------------------------
// R_PointToDist
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_PointToDist(x: fixed_t, y: fixed_t) -> fixed_t {
    let mut dx = (x - viewx).wrapping_abs();
    let mut dy = (y - viewy).wrapping_abs();

    if dy > dx {
        let temp = dx;
        dx = dy;
        dy = temp;
    }

    let frac = if dx != 0 { FixedDiv(dy, dx) } else { 0 };

    let angle = (tables::tantoangle[(frac as u32 >> DBITS) as usize] + ANG90) >> ANGLETOFINESHIFT;

    FixedDiv(dx, tables::finesine[angle as usize])
}

// ---------------------------------------------------------------------------
// R_InitPointToAngle
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_InitPointToAngle() {
    // UNUSED - now getting from tables.c
}

// ---------------------------------------------------------------------------
// R_ScaleFromGlobalAngle
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_ScaleFromGlobalAngle(visangle: angle_t) -> fixed_t {
    let anglea = ANG90.wrapping_add(visangle.wrapping_sub(viewangle));
    let angleb = ANG90.wrapping_add(visangle.wrapping_sub(rw_normalangle));

    let sinea = tables::finesine[(anglea >> ANGLETOFINESHIFT) as usize];
    let sineb = tables::finesine[(angleb >> ANGLETOFINESHIFT) as usize];
    let num = FixedMul(projection, sineb) << detailshift;
    let den = FixedMul(rw_distance, sinea);

    if den > num >> 16 {
        let mut scale = FixedDiv(num, den);
        if scale > 64 * FRACUNIT {
            scale = 64 * FRACUNIT;
        } else if scale < 256 {
            scale = 256;
        }
        scale
    } else {
        64 * FRACUNIT
    }
}

// ---------------------------------------------------------------------------
// R_InitTables
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_InitTables() {
    // UNUSED: now getting from tables.c
}

// ---------------------------------------------------------------------------
// R_InitTextureMapping
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_InitTextureMapping() {
    // Use tangent table to generate viewangletox:
    //  viewangletox will give the next greatest x after the view angle.
    //
    // Calc focallength so FIELDOFVIEW angles covers SCREENWIDTH.
    let focallength = FixedDiv(
        centerxfrac,
        tables::finetangent[(tables::FINEANGLES / 4 + FIELDOFVIEW as usize / 2) as usize],
    );

    for i in 0..tables::FINEANGLES / 2 {
        let t: c_int;
        if tables::finetangent[i] > FRACUNIT * 2 {
            t = -1;
        } else if tables::finetangent[i] < -FRACUNIT * 2 {
            t = viewwidth + 1;
        } else {
            let mut tt = FixedMul(tables::finetangent[i], focallength);
            tt = (centerxfrac - tt + FRACUNIT - 1) >> FRACBITS;
            if tt < -1 {
                t = -1;
            } else if tt > viewwidth + 1 {
                t = viewwidth + 1;
            } else {
                t = tt;
            }
        }
        viewangletox[i] = t;
    }

    // Scan viewangletox[] to generate xtoviewangle[]:
    //  xtoviewangle will give the smallest view angle that maps to x.
    for x in 0..=viewwidth as usize {
        let mut i = 0usize;
        while viewangletox[i] > x as c_int {
            i += 1;
        }
        xtoviewangle[x] = ((i as u32) << ANGLETOFINESHIFT).wrapping_sub(ANG90);
    }

    // Take out the fencepost cases from viewangletox.
    for i in 0..tables::FINEANGLES / 2 {
        let mut t = FixedMul(tables::finetangent[i], focallength);
        t = centerx - t;

        if viewangletox[i] == -1 {
            viewangletox[i] = 0;
        } else if viewangletox[i] == viewwidth + 1 {
            viewangletox[i] = viewwidth;
        }
    }

    clipangle = xtoviewangle[0];
}

// ---------------------------------------------------------------------------
// R_InitLightTables
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_InitLightTables() {
    for i in 0..LIGHTLEVELS {
        let startmap = (((LIGHTLEVELS - 1 - i) * 2) * NUMCOLORMAPS / LIGHTLEVELS) as c_int;
        for j in 0..MAXLIGHTZ {
            let mut scale = FixedDiv(
                (SCREENWIDTH as c_int / 2) * FRACUNIT,
                ((j + 1) << LIGHTZSHIFT) as c_int,
            );
            scale >>= LIGHTSCALESHIFT;
            let mut level = startmap - scale / DISTMAP as c_int;

            if level < 0 {
                level = 0;
            }
            if level >= NUMCOLORMAPS as c_int {
                level = NUMCOLORMAPS as c_int - 1;
            }

            zlight[i][j] = colormaps.add(level as usize * 256);
        }
    }
}

// ---------------------------------------------------------------------------
// R_SetViewSize
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_SetViewSize(blocks: c_int, detail: c_int) {
    setsizeneeded = Boolean::TRUE;
    setblocks = blocks;
    setdetail = detail;
}

// ---------------------------------------------------------------------------
// R_ExecuteSetViewSize
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_ExecuteSetViewSize() {
    setsizeneeded = Boolean::FALSE;

    if setblocks == 11 {
        scaledviewwidth = SCREENWIDTH as c_int;
        viewheight = SCREENHEIGHT as c_int;
    } else {
        scaledviewwidth = setblocks * 32;
        viewheight = (setblocks * 168 / 10) & !7;
    }

    detailshift = setdetail;
    viewwidth = scaledviewwidth >> detailshift;

    centery = viewheight / 2;
    centerx = viewwidth / 2;
    centerxfrac = centerx << FRACBITS;
    centeryfrac = centery << FRACBITS;
    projection = centerxfrac;

    if detailshift == 0 {
        colfunc = Some(R_DrawColumn);
        basecolfunc = Some(R_DrawColumn);
        fuzzcolfunc = Some(R_DrawFuzzColumn);
        transcolfunc = Some(R_DrawTranslatedColumn);
        spanfunc = Some(R_DrawSpan);
    } else {
        colfunc = Some(R_DrawColumnLow);
        basecolfunc = Some(R_DrawColumnLow);
        fuzzcolfunc = Some(R_DrawFuzzColumnLow);
        transcolfunc = Some(R_DrawTranslatedColumnLow);
        spanfunc = Some(R_DrawSpanLow);
    }

    R_InitBuffer(scaledviewwidth, viewheight);
    R_InitTextureMapping();

    // psprite scales
    pspritescale = FRACUNIT * viewwidth / SCREENWIDTH as c_int;
    pspriteiscale = FRACUNIT * SCREENWIDTH as c_int / viewwidth;

    // thing clipping
    for i in 0..viewwidth as usize {
        screenheightarray[i] = viewheight as c_short;
    }

    // planes
    for i in 0..viewheight as usize {
        let mut dy = ((i as c_int - viewheight / 2) << FRACBITS) + FRACUNIT / 2;
        dy = dy.wrapping_abs();
        crate::doom::r_plane::yslope[i] = FixedDiv((viewwidth << detailshift) / 2 * FRACUNIT, dy);
    }

    for i in 0..viewwidth as usize {
        let cosadj = (*tables::finecosine
            .0
            .add((xtoviewangle[i] >> ANGLETOFINESHIFT) as usize))
        .wrapping_abs();
        crate::doom::r_plane::distscale[i] = FixedDiv(FRACUNIT, cosadj);
    }

    // Calculate the light levels to use for each level / scale combination.
    for i in 0..LIGHTLEVELS {
        let startmap = (((LIGHTLEVELS - 1 - i) * 2) * NUMCOLORMAPS / LIGHTLEVELS) as c_int;
        for j in 0..MAXLIGHTSCALE {
            let mut level = startmap
                - (j as c_int * SCREENWIDTH as c_int)
                    / (viewwidth << detailshift)
                    / DISTMAP as c_int;

            if level < 0 {
                level = 0;
            }
            if level >= NUMCOLORMAPS as c_int {
                level = NUMCOLORMAPS as c_int - 1;
            }

            scalelight[i][j] = colormaps.add(level as usize * 256);
        }
    }
}

// ---------------------------------------------------------------------------
// R_Init
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_Init() {
    R_InitData();
    libc::printf(b".\0".as_ptr() as *const c_char);
    R_InitPointToAngle();
    libc::printf(b".\0".as_ptr() as *const c_char);
    R_InitTables();
    libc::printf(b".\0".as_ptr() as *const c_char);
    R_SetViewSize(screenblocks, detailLevel);
    R_InitPlanes();
    libc::printf(b".\0".as_ptr() as *const c_char);
    R_InitLightTables();
    libc::printf(b".\0".as_ptr() as *const c_char);
    crate::doom::r_sky::R_InitSkyMap();
    R_InitTranslationTables();
    libc::printf(b".\0".as_ptr() as *const c_char);

    framecount = 0;
}

// ---------------------------------------------------------------------------
// R_PointInSubsector
// ---------------------------------------------------------------------------

const NF_SUBSECTOR: u32 = 0x8000;

#[no_mangle]
pub unsafe extern "C" fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t {
    // single subsector is a special case
    if numnodes == 0 {
        return subsectors as *mut subsector_t;
    }

    let mut nodenum = numnodes - 1;

    while (nodenum as u32) & NF_SUBSECTOR == 0 {
        let node = nodes.add(nodenum as usize) as *const node_t;
        let side = R_PointOnSide(x, y, node);
        nodenum = (*node).children[side as usize] as c_int;
    }

    subsectors.add((nodenum as u32 & !NF_SUBSECTOR) as usize) as *mut subsector_t
}

// ---------------------------------------------------------------------------
// R_SetupFrame
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_SetupFrame(player: *mut PlayerT) {
    viewplayer = player;
    let mo = (*player).mo as *mut mobj_t;
    viewx = (*mo).x;
    viewy = (*mo).y;
    viewangle = (*mo).angle.wrapping_add(viewangleoffset as u32);
    extralight = (*player).extralight;

    viewz = (*player).viewz;

    viewsin = tables::finesine[(viewangle >> ANGLETOFINESHIFT) as usize];
    viewcos = *tables::finecosine
        .0
        .add((viewangle >> ANGLETOFINESHIFT) as usize);

    sscount = 0;

    if (*player).fixedcolormap != 0 {
        fixedcolormap = colormaps
            .add((*player).fixedcolormap as usize * 256 * std::mem::size_of::<lighttable_t>());

        for i in 0..MAXLIGHTSCALE {
            scalelightfixed[i] = fixedcolormap;
        }
        walllights = ptr::addr_of_mut!(scalelightfixed[0]);
    } else {
        fixedcolormap = ptr::null_mut();
    }

    framecount += 1;
    validcount += 1;
}

// ---------------------------------------------------------------------------
// R_RenderPlayerView
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_RenderPlayerView(player: *mut PlayerT) {
    R_SetupFrame(player);

    R_ClearClipSegs();
    R_ClearDrawSegs();
    R_ClearPlanes();
    R_ClearSprites();

    NetUpdate();
    R_RenderBSPNode(numnodes - 1);
    NetUpdate();
    R_DrawPlanes();
    NetUpdate();
    R_DrawMasked();
    NetUpdate();
}

// ---------------------------------------------------------------------------
// Anchor
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_Main_Link_Anchor() {
    let _ = R_AddPointToBox as *const () as usize;
    let _ = R_PointOnSide as *const () as usize;
    let _ = R_PointOnSegSide as *const () as usize;
    let _ = R_PointToAngle as *const () as usize;
    let _ = R_PointToAngle2 as *const () as usize;
    let _ = R_PointToDist as *const () as usize;
    let _ = R_InitPointToAngle as *const () as usize;
    let _ = R_ScaleFromGlobalAngle as *const () as usize;
    let _ = R_InitTables as *const () as usize;
    let _ = R_InitTextureMapping as *const () as usize;
    let _ = R_InitLightTables as *const () as usize;
    let _ = R_SetViewSize as *const () as usize;
    let _ = R_ExecuteSetViewSize as *const () as usize;
    let _ = R_Init as *const () as usize;
    let _ = R_PointInSubsector as *const () as usize;
    let _ = R_SetupFrame as *const () as usize;
    let _ = R_RenderPlayerView as *const () as usize;
}

// ---------------------------------------------------------------------------
// Regression tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doom::m_fixed::FixedDiv;

    /// The DBITS constant was once incorrectly set to 15 instead of
    /// FRACBITS - SLOPEBITS = 16 - 11 = 5.  With the wrong value,
    /// R_PointToDist indexes tantoangle with the wrong shift and
    /// computes completely wrong distances, causing wall-offset /
    /// sprite-clipping glitches.
    #[test]
    fn dbits_is_five() {
        assert_eq!(DBITS, 5, "DBITS must be FRACBITS - SLOPEBITS = 5");
    }

    /// For a 45-degree line (dy == dx), FixedDiv(dy, dx) returns FRACUNIT.
    /// With DBITS = 5 the index into tantoangle is FRACUNIT >> 5 == 2048,
    /// the top of the table.  With DBITS = 15 the index would be 2,
    /// which produces a garbage distance.
    #[test]
    fn r_point_to_dist_45_degree_index() {
        let frac = FixedDiv(FRACUNIT, FRACUNIT); // dy == dx
        let index = (frac as u32 >> DBITS) as usize;
        assert_eq!(
            index, 2048,
            "frac>>DBITS for 45-degree case must index tantoangle[2048]"
        );
    }

    #[test]
    fn r_point_to_dist_reasonable_values() {
        unsafe {
            viewx = 0;
            viewy = 0;

            // Straight ahead: distance should be approximately FRACUNIT.
            let dist_ahead = R_PointToDist(FRACUNIT, 0);
            assert!(dist_ahead > 0, "distance straight ahead must be positive");
            assert!(
                (dist_ahead - FRACUNIT).abs() < FRACUNIT / 4,
                "distance straight ahead should be near FRACUNIT, got {}",
                dist_ahead
            );

            // 45-degree diagonal: distance should be larger than straight ahead.
            let dist_diag = R_PointToDist(FRACUNIT, FRACUNIT);
            assert!(
                dist_diag > dist_ahead,
                "diagonal distance ({}) must exceed straight-ahead distance ({})",
                dist_diag,
                dist_ahead
            );

            // Same point: distance must be exactly zero.
            let dist_zero = R_PointToDist(0, 0);
            assert_eq!(dist_zero, 0, "distance to view position must be zero");
        }
    }

    #[test]
    fn r_point_to_angle2_cardinals() {
        unsafe {
            // Due east is exact.
            assert_eq!(R_PointToAngle2(0, 0, FRACUNIT, 0), 0);

            // Due north quantizes to ANG90 - 1 in the fixed-point LUT.
            assert_eq!(R_PointToAngle2(0, 0, 0, FRACUNIT), ANG90 - 1);

            // Due west quantizes to ANG180 - 1.
            assert_eq!(R_PointToAngle2(0, 0, -FRACUNIT, 0), ANG180 - 1);

            // Due south is exact.
            assert_eq!(R_PointToAngle2(0, 0, 0, -FRACUNIT), ANG270);
        }
    }

    /// R_ScaleFromGlobalAngle previously used plain `+` and `-` on
    /// angle_t values, which panics in debug builds on wraparound.
    /// It must use wrapping_add / wrapping_sub.
    #[test]
    fn r_scale_from_global_angle_wraparound_no_panic() {
        unsafe {
            viewangle = 0;
            rw_normalangle = 0;
            projection = FRACUNIT;
            rw_distance = FRACUNIT;
            detailshift = 0;

            // Angles near the u32 boundary must not panic.
            let _ = R_ScaleFromGlobalAngle(u32::MAX);
            let _ = R_ScaleFromGlobalAngle(0);
            let _ = R_ScaleFromGlobalAngle(ANG90);
            let _ = R_ScaleFromGlobalAngle(ANG180);
            let _ = R_ScaleFromGlobalAngle(ANG270);
            let _ = R_ScaleFromGlobalAngle(viewangle.wrapping_sub(1));
            let _ = R_ScaleFromGlobalAngle(viewangle.wrapping_add(1));
        }
    }

    #[test]
    fn r_point_on_side_vertical_line() {
        unsafe {
            // Node with dx == 0, dy > 0 (vertical line, northward).
            let node = node_t {
                x: 0,
                y: 0,
                dx: 0,
                dy: FRACUNIT,
                bbox: [[0; 4]; 2],
                children: [0; 2],
            };
            // Point to the left of the line -> side 1 (from C logic).
            assert_eq!(R_PointOnSide(-FRACUNIT, 0, &node), 1);
            // Point to the right of the line -> side 0.
            assert_eq!(R_PointOnSide(FRACUNIT, 0, &node), 0);
            // Point exactly on the line -> x <= node.x, side depends on dy > 0 -> 1.
            assert_eq!(R_PointOnSide(0, 0, &node), 1);
        }
    }
}
