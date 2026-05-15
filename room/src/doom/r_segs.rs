//! Rust port of vendor/doomgeneric/r_segs.c.
//!
//! Wall-segment rendering: R_StoreWallRange sets up the per-seg globals that
//! drive the low-level column drawers, and R_RenderMaskedSegRange draws the
//! transparent middle textures on two-sided lines.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_int, c_short, c_uchar, c_void};
use std::ptr;

use super::m_fixed::{angle_t, fixed_t, FixedMul, FRACBITS};
use super::r_bsp::{drawseg_t, line_t, sector_t, seg_t, side_t};
use super::r_plane::visplane_t;
use super::tables::{self, ANG180, ANG90, ANGLETOFINESHIFT};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const HEIGHTBITS: u32 = 12;
const HEIGHTUNIT: c_int = 1 << HEIGHTBITS;

const SIL_BOTTOM: c_int = 1;
const SIL_TOP: c_int = 2;
const SIL_BOTH: c_int = 3;

const LIGHTLEVELS: usize = 16;
const LIGHTSEGSHIFT: u32 = 4;
const MAXLIGHTSCALE: usize = 48;
const LIGHTSCALESHIFT: u32 = 12;

const FINEMASK: usize = 0x1FFF;

const SCREENWIDTH: usize = crate::doom::i_video::SCREENWIDTH as usize;
const MAXDRAWSEGS: usize = 256;

// ---------------------------------------------------------------------------
// Externs from other modules
// ---------------------------------------------------------------------------

extern "C" {
    static mut viewz: fixed_t;
    static mut viewangle: angle_t;
    static mut viewheight: c_int;
    static mut centeryfrac: fixed_t;
    static mut extralight: c_int;
    static mut fixedcolormap: *mut c_uchar;
    static mut scalelight: [[*mut c_uchar; MAXLIGHTSCALE]; LIGHTLEVELS];
    static mut texturetranslation: *mut c_int;
    static mut textureheight: *mut fixed_t;
    static mut xtoviewangle: [angle_t; SCREENWIDTH + 1];
    static mut skyflatnum: c_int;
    static mut screenheightarray: [c_short; SCREENWIDTH];
    static mut negonearray: [c_short; SCREENWIDTH];

    static mut ceilingclip: [c_short; SCREENWIDTH];
    static mut floorclip: [c_short; SCREENWIDTH];
    static mut floorplane: *mut visplane_t;
    static mut ceilingplane: *mut visplane_t;
    static mut lastopening: *mut c_short;

    static mut drawsegs: [drawseg_t; MAXDRAWSEGS];
    static mut ds_p: *mut drawseg_t;
    static mut curline: *mut seg_t;
    static mut frontsector: *mut sector_t;
    static mut backsector: *mut sector_t;
    static mut sidedef: *mut side_t;
    static mut linedef: *mut line_t;

    static mut dc_x: c_int;
    static mut dc_yl: c_int;
    static mut dc_yh: c_int;
    static mut dc_iscale: fixed_t;
    static mut dc_texturemid: fixed_t;
    static mut dc_source: *mut c_uchar;
    static mut dc_colormap: *mut c_uchar;

    static mut colfunc: Option<unsafe extern "C" fn()>;

    fn R_GetColumn(tex: c_int, col: c_int) -> *mut c_uchar;
    fn R_CheckPlane(pl: *mut visplane_t, start: c_int, stop: c_int) -> *mut visplane_t;
    fn R_ScaleFromGlobalAngle(visangle: angle_t) -> fixed_t;
    fn R_PointToDist(x: fixed_t, y: fixed_t) -> fixed_t;
    fn R_DrawMaskedColumn(column: *mut c_void);
}

// ---------------------------------------------------------------------------
// Globals defined by this module
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut segtextured: c_int = 0;

#[no_mangle]
pub static mut markfloor: c_int = 0;

#[no_mangle]
pub static mut markceiling: c_int = 0;

#[no_mangle]
pub static mut maskedtexture: c_int = 0;

#[no_mangle]
pub static mut toptexture: c_int = 0;

#[no_mangle]
pub static mut bottomtexture: c_int = 0;

#[no_mangle]
pub static mut midtexture: c_int = 0;

#[no_mangle]
pub static mut rw_normalangle: angle_t = 0;

#[no_mangle]
pub static mut rw_angle1: angle_t = 0;

#[no_mangle]
pub static mut rw_x: c_int = 0;

#[no_mangle]
pub static mut rw_stopx: c_int = 0;

#[no_mangle]
pub static mut rw_centerangle: angle_t = 0;

#[no_mangle]
pub static mut rw_offset: fixed_t = 0;

#[no_mangle]
pub static mut rw_distance: fixed_t = 0;

#[no_mangle]
pub static mut rw_scale: fixed_t = 0;

#[no_mangle]
pub static mut rw_scalestep: fixed_t = 0;

#[no_mangle]
pub static mut rw_midtexturemid: fixed_t = 0;

#[no_mangle]
pub static mut rw_toptexturemid: fixed_t = 0;

#[no_mangle]
pub static mut rw_bottomtexturemid: fixed_t = 0;

#[no_mangle]
pub static mut worldtop: c_int = 0;

#[no_mangle]
pub static mut worldbottom: c_int = 0;

#[no_mangle]
pub static mut worldhigh: c_int = 0;

#[no_mangle]
pub static mut worldlow: c_int = 0;

#[no_mangle]
pub static mut pixhigh: fixed_t = 0;

#[no_mangle]
pub static mut pixlow: fixed_t = 0;

#[no_mangle]
pub static mut pixhighstep: fixed_t = 0;

#[no_mangle]
pub static mut pixlowstep: fixed_t = 0;

#[no_mangle]
pub static mut topfrac: fixed_t = 0;

#[no_mangle]
pub static mut topstep: fixed_t = 0;

#[no_mangle]
pub static mut bottomfrac: fixed_t = 0;

#[no_mangle]
pub static mut bottomstep: fixed_t = 0;

#[no_mangle]
pub static mut walllights: *mut *mut c_uchar = ptr::null_mut();

#[no_mangle]
pub static mut maskedtexturecol: *mut c_short = ptr::null_mut();

// ---------------------------------------------------------------------------
// R_RenderMaskedSegRange
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_RenderMaskedSegRange(ds: *mut drawseg_t, x1: c_int, x2: c_int) {
    let texnum: c_int = *texturetranslation.add((*(*(*ds).curline).sidedef).midtexture as usize);

    curline = (*ds).curline;
    frontsector = (*curline).frontsector;
    backsector = (*curline).backsector;

    let mut lightnum: c_int =
        (((*frontsector).lightlevel as u32) >> LIGHTSEGSHIFT) as c_int + extralight;

    if (*(*curline).v1).y == (*(*curline).v2).y {
        lightnum -= 1;
    } else if (*(*curline).v1).x == (*(*curline).v2).x {
        lightnum += 1;
    }

    if lightnum < 0 {
        walllights = scalelight[0].as_mut_ptr();
    } else if lightnum >= LIGHTLEVELS as c_int {
        walllights = scalelight[LIGHTLEVELS - 1].as_mut_ptr();
    } else {
        walllights = scalelight[lightnum as usize].as_mut_ptr();
    }

    maskedtexturecol = (*ds).maskedtexturecol;
    rw_scalestep = (*ds).scalestep;
    crate::doom::r_things::spryscale = (*ds).scale1 + (x1 - (*ds).x1) * rw_scalestep;
    crate::doom::r_things::mfloorclip = (*ds).sprbottomclip;
    crate::doom::r_things::mceilingclip = (*ds).sprtopclip;

    if (*(*curline).linedef).flags & (super::c_ffi::ML_DONTPEGBOTTOM as c_short) != 0 {
        dc_texturemid = if (*frontsector).floorheight > (*backsector).floorheight {
            (*frontsector).floorheight
        } else {
            (*backsector).floorheight
        };
        dc_texturemid = dc_texturemid + *textureheight.add(texnum as usize) - viewz;
    } else {
        dc_texturemid = if (*frontsector).ceilingheight < (*backsector).ceilingheight {
            (*frontsector).ceilingheight
        } else {
            (*backsector).ceilingheight
        };
        dc_texturemid = dc_texturemid - viewz;
    }
    dc_texturemid += (*(*curline).sidedef).rowoffset;

    if !fixedcolormap.is_null() {
        dc_colormap = fixedcolormap;
    }

    dc_x = x1;
    while dc_x <= x2 {
        if *maskedtexturecol.add(dc_x as usize) != c_short::MAX {
            if fixedcolormap.is_null() {
                let mut index = (crate::doom::r_things::spryscale as u32) >> LIGHTSCALESHIFT;
                if index >= MAXLIGHTSCALE as u32 {
                    index = (MAXLIGHTSCALE - 1) as u32;
                }
                dc_colormap = *walllights.add(index as usize);
            }

            crate::doom::r_things::sprtopscreen =
                centeryfrac - FixedMul(dc_texturemid, crate::doom::r_things::spryscale);
            dc_iscale = (0xffffffff_u32 / (crate::doom::r_things::spryscale as u32)) as c_int;

            let raw = R_GetColumn(texnum, *maskedtexturecol.add(dc_x as usize) as c_int);
            let col = raw.sub(3);
            R_DrawMaskedColumn(col as *mut c_void);

            *maskedtexturecol.add(dc_x as usize) = c_short::MAX;
        }
        crate::doom::r_things::spryscale += rw_scalestep;
        dc_x += 1;
    }
}

// ---------------------------------------------------------------------------
// R_RenderSegLoop
// ---------------------------------------------------------------------------

unsafe fn R_RenderSegLoop() {
    while rw_x < rw_stopx {
        let clip_ceil = ceilingclip[rw_x as usize] as c_int;
        let clip_floor = floorclip[rw_x as usize] as c_int;

        let mut yl = ((topfrac + HEIGHTUNIT - 1) >> HEIGHTBITS) as c_int;
        if yl < clip_ceil + 1 {
            yl = clip_ceil + 1;
        }

        if markceiling != 0 {
            let top = clip_ceil + 1;
            let mut bottom = yl - 1;
            if bottom >= clip_floor {
                bottom = clip_floor - 1;
            }
            if top <= bottom {
                (*ceilingplane).top[rw_x as usize] = top as u8;
                (*ceilingplane).bottom[rw_x as usize] = bottom as u8;
            }
        }

        let mut yh = (bottomfrac >> HEIGHTBITS) as c_int;
        if yh >= clip_floor {
            yh = clip_floor - 1;
        }

        if markfloor != 0 {
            let mut top = yh + 1;
            let bottom = clip_floor - 1;
            if top <= clip_ceil {
                top = clip_ceil + 1;
            }
            if top <= bottom {
                (*floorplane).top[rw_x as usize] = top as u8;
                (*floorplane).bottom[rw_x as usize] = bottom as u8;
            }
        }

        let texturecolumn: fixed_t;
        if segtextured != 0 {
            let angle =
                (rw_centerangle.wrapping_add(xtoviewangle[rw_x as usize])) >> ANGLETOFINESHIFT;
            texturecolumn =
                rw_offset - FixedMul(tables::finetangent[angle as usize & FINEMASK], rw_distance);

            let mut index = (rw_scale as u32) >> LIGHTSCALESHIFT;
            if index >= MAXLIGHTSCALE as u32 {
                index = (MAXLIGHTSCALE - 1) as u32;
            }
            dc_colormap = *walllights.add(index as usize);
            dc_x = rw_x;
            dc_iscale = (0xffffffff_u32 / (rw_scale as u32)) as c_int;
        } else {
            texturecolumn = 0;
        }

        if midtexture != 0 {
            dc_yl = yl;
            dc_yh = yh;
            dc_texturemid = rw_midtexturemid;
            dc_source = R_GetColumn(midtexture, texturecolumn >> FRACBITS);
            if let Some(func) = colfunc {
                func();
            }
            ceilingclip[rw_x as usize] = viewheight as c_short;
            floorclip[rw_x as usize] = -1;
        } else {
            if toptexture != 0 {
                let mut mid = (pixhigh >> HEIGHTBITS) as c_int;
                pixhigh += pixhighstep;

                let clip_floor = floorclip[rw_x as usize] as c_int;
                if mid >= clip_floor {
                    mid = clip_floor - 1;
                }

                if mid >= yl {
                    dc_yl = yl;
                    dc_yh = mid;
                    dc_texturemid = rw_toptexturemid;
                    dc_source = R_GetColumn(toptexture, texturecolumn >> FRACBITS);
                    if let Some(func) = colfunc {
                        func();
                    }
                    ceilingclip[rw_x as usize] = mid as c_short;
                } else {
                    ceilingclip[rw_x as usize] = (yl - 1) as c_short;
                }
            } else {
                if markceiling != 0 {
                    ceilingclip[rw_x as usize] = (yl - 1) as c_short;
                }
            }

            if bottomtexture != 0 {
                let mut mid = ((pixlow + HEIGHTUNIT - 1) >> HEIGHTBITS) as c_int;
                pixlow += pixlowstep;

                let clip_ceil = ceilingclip[rw_x as usize] as c_int;
                if mid <= clip_ceil {
                    mid = clip_ceil + 1;
                }

                if mid <= yh {
                    dc_yl = mid;
                    dc_yh = yh;
                    dc_texturemid = rw_bottomtexturemid;
                    dc_source = R_GetColumn(bottomtexture, texturecolumn >> FRACBITS);
                    if let Some(func) = colfunc {
                        func();
                    }
                    floorclip[rw_x as usize] = mid as c_short;
                } else {
                    floorclip[rw_x as usize] = (yh + 1) as c_short;
                }
            } else {
                if markfloor != 0 {
                    floorclip[rw_x as usize] = (yh + 1) as c_short;
                }
            }

            if maskedtexture != 0 {
                *maskedtexturecol.add(rw_x as usize) = (texturecolumn >> FRACBITS) as c_short;
            }
        }

        rw_scale += rw_scalestep;
        topfrac += topstep;
        bottomfrac += bottomstep;
        rw_x += 1;
    }
}

// ---------------------------------------------------------------------------
// R_StoreWallRange
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_StoreWallRange(start: c_int, stop: c_int) {
    if ds_p == drawsegs.as_mut_ptr().add(MAXDRAWSEGS) {
        return;
    }

    #[cfg(feature = "rangecheck")]
    {
        if start >= viewwidth || start > stop {
            // Would call I_Error — skip in Rust for now
            return;
        }
    }

    sidedef = (*curline).sidedef;
    linedef = (*curline).linedef;

    (*linedef).flags |= super::c_ffi::ML_MAPPED as c_short;

    rw_normalangle = (*curline).angle.wrapping_add(ANG90);
    let mut offsetangle = (rw_normalangle.wrapping_sub(rw_angle1) as i32).abs() as u32;

    if offsetangle > ANG90 {
        offsetangle = ANG90;
    }

    let distangle = ANG90 - offsetangle;
    let hyp = R_PointToDist((*(*curline).v1).x, (*(*curline).v1).y);
    let sineval = tables::finesine[(distangle >> ANGLETOFINESHIFT) as usize];
    rw_distance = FixedMul(hyp, sineval);

    (*ds_p).x1 = start;
    rw_x = start;
    (*ds_p).x2 = stop;
    (*ds_p).curline = curline;
    rw_stopx = stop + 1;

    (*ds_p).scale1 = rw_scale;
    rw_scale = R_ScaleFromGlobalAngle(viewangle.wrapping_add(xtoviewangle[start as usize]));
    (*ds_p).scale1 = rw_scale;

    if stop > start {
        (*ds_p).scale2 =
            R_ScaleFromGlobalAngle(viewangle.wrapping_add(xtoviewangle[stop as usize]));
        (*ds_p).scalestep = rw_scalestep;
        rw_scalestep = ((*ds_p).scale2 - rw_scale) / (stop - start);
        (*ds_p).scalestep = rw_scalestep;
    } else {
        (*ds_p).scale2 = (*ds_p).scale1;
    }

    worldtop = (*frontsector).ceilingheight - viewz;
    worldbottom = (*frontsector).floorheight - viewz;

    midtexture = 0;
    toptexture = 0;
    bottomtexture = 0;
    maskedtexture = 0;
    (*ds_p).maskedtexturecol = ptr::null_mut();

    if backsector.is_null() {
        // single sided line
        midtexture = *texturetranslation.add((*sidedef).midtexture as usize);
        markfloor = 1;
        markceiling = 1;

        if (*linedef).flags & (super::c_ffi::ML_DONTPEGBOTTOM as c_short) != 0 {
            let vtop =
                (*frontsector).floorheight + *textureheight.add((*sidedef).midtexture as usize);
            rw_midtexturemid = vtop - viewz;
        } else {
            rw_midtexturemid = worldtop;
        }
        rw_midtexturemid += (*sidedef).rowoffset;

        (*ds_p).silhouette = SIL_BOTH;
        (*ds_p).sprtopclip = screenheightarray.as_mut_ptr();
        (*ds_p).sprbottomclip = negonearray.as_mut_ptr();
        (*ds_p).bsilheight = c_int::MAX;
        (*ds_p).tsilheight = c_int::MIN;
    } else {
        // two sided line
        (*ds_p).sprtopclip = ptr::null_mut();
        (*ds_p).sprbottomclip = ptr::null_mut();
        (*ds_p).silhouette = 0;

        if (*frontsector).floorheight > (*backsector).floorheight {
            (*ds_p).silhouette = SIL_BOTTOM;
            (*ds_p).bsilheight = (*frontsector).floorheight;
        } else if (*backsector).floorheight > viewz {
            (*ds_p).silhouette = SIL_BOTTOM;
            (*ds_p).bsilheight = c_int::MAX;
        }

        if (*frontsector).ceilingheight < (*backsector).ceilingheight {
            (*ds_p).silhouette |= SIL_TOP;
            (*ds_p).tsilheight = (*frontsector).ceilingheight;
        } else if (*backsector).ceilingheight < viewz {
            (*ds_p).silhouette |= SIL_TOP;
            (*ds_p).tsilheight = c_int::MIN;
        }

        if (*backsector).ceilingheight <= (*frontsector).floorheight {
            (*ds_p).sprbottomclip = negonearray.as_mut_ptr();
            (*ds_p).bsilheight = c_int::MAX;
            (*ds_p).silhouette |= SIL_BOTTOM;
        }

        if (*backsector).floorheight >= (*frontsector).ceilingheight {
            (*ds_p).sprtopclip = screenheightarray.as_mut_ptr();
            (*ds_p).tsilheight = c_int::MIN;
            (*ds_p).silhouette |= SIL_TOP;
        }

        worldhigh = (*backsector).ceilingheight - viewz;
        worldlow = (*backsector).floorheight - viewz;

        if (*frontsector).ceilingpic as c_int == skyflatnum
            && (*backsector).ceilingpic as c_int == skyflatnum
        {
            worldtop = worldhigh;
        }

        if worldlow != worldbottom
            || (*backsector).floorpic != (*frontsector).floorpic
            || (*backsector).lightlevel != (*frontsector).lightlevel
        {
            markfloor = 1;
        } else {
            markfloor = 0;
        }

        if worldhigh != worldtop
            || (*backsector).ceilingpic != (*frontsector).ceilingpic
            || (*backsector).lightlevel != (*frontsector).lightlevel
        {
            markceiling = 1;
        } else {
            markceiling = 0;
        }

        if (*backsector).ceilingheight <= (*frontsector).floorheight
            || (*backsector).floorheight >= (*frontsector).ceilingheight
        {
            markceiling = 1;
            markfloor = 1;
        }

        if worldhigh < worldtop {
            toptexture = *texturetranslation.add((*sidedef).toptexture as usize);
            if (*linedef).flags & (super::c_ffi::ML_DONTPEGTOP as c_short) != 0 {
                rw_toptexturemid = worldtop;
            } else {
                let vtop = (*backsector).ceilingheight
                    + *textureheight.add((*sidedef).toptexture as usize);
                rw_toptexturemid = vtop - viewz;
            }
        }

        if worldlow > worldbottom {
            bottomtexture = *texturetranslation.add((*sidedef).bottomtexture as usize);
            if (*linedef).flags & (super::c_ffi::ML_DONTPEGBOTTOM as c_short) != 0 {
                rw_bottomtexturemid = worldtop;
            } else {
                rw_bottomtexturemid = worldlow;
            }
        }

        rw_toptexturemid += (*sidedef).rowoffset;
        rw_bottomtexturemid += (*sidedef).rowoffset;

        if (*sidedef).midtexture != 0 {
            maskedtexture = 1;
            (*ds_p).maskedtexturecol = lastopening.sub(rw_x as usize);
            maskedtexturecol = (*ds_p).maskedtexturecol;
            lastopening = lastopening.add((rw_stopx - rw_x) as usize);
        }
    }

    segtextured = midtexture | toptexture | bottomtexture | maskedtexture;

    if segtextured != 0 {
        let mut offsetangle = rw_normalangle.wrapping_sub(rw_angle1);
        if offsetangle > ANG180 {
            offsetangle = offsetangle.wrapping_neg();
        }
        if offsetangle > ANG90 {
            offsetangle = ANG90;
        }

        let sineval = tables::finesine[(offsetangle >> ANGLETOFINESHIFT) as usize];
        rw_offset = FixedMul(hyp, sineval);

        if rw_normalangle.wrapping_sub(rw_angle1) < ANG180 {
            rw_offset = -rw_offset;
        }

        rw_offset += (*sidedef).textureoffset + (*curline).offset;
        rw_centerangle = ANG90.wrapping_add(viewangle).wrapping_sub(rw_normalangle);

        if fixedcolormap.is_null() {
            let mut lightnum =
                (((*frontsector).lightlevel as u32) >> LIGHTSEGSHIFT) as c_int + extralight;

            if (*(*curline).v1).y == (*(*curline).v2).y {
                lightnum -= 1;
            } else if (*(*curline).v1).x == (*(*curline).v2).x {
                lightnum += 1;
            }

            if lightnum < 0 {
                walllights = scalelight[0].as_mut_ptr();
            } else if lightnum >= LIGHTLEVELS as c_int {
                walllights = scalelight[LIGHTLEVELS - 1].as_mut_ptr();
            } else {
                walllights = scalelight[lightnum as usize].as_mut_ptr();
            }
        }
    }

    if (*frontsector).floorheight >= viewz {
        markfloor = 0;
    }

    if (*frontsector).ceilingheight <= viewz && (*frontsector).ceilingpic as c_int != skyflatnum {
        markceiling = 0;
    }

    worldtop >>= 4;
    worldbottom >>= 4;

    topstep = -FixedMul(rw_scalestep, worldtop);
    topfrac = (centeryfrac >> 4) - FixedMul(worldtop, rw_scale);

    bottomstep = -FixedMul(rw_scalestep, worldbottom);
    bottomfrac = (centeryfrac >> 4) - FixedMul(worldbottom, rw_scale);

    if !backsector.is_null() {
        worldhigh >>= 4;
        worldlow >>= 4;

        if worldhigh < worldtop {
            pixhigh = (centeryfrac >> 4) - FixedMul(worldhigh, rw_scale);
            pixhighstep = -FixedMul(rw_scalestep, worldhigh);
        }

        if worldlow > worldbottom {
            pixlow = (centeryfrac >> 4) - FixedMul(worldlow, rw_scale);
            pixlowstep = -FixedMul(rw_scalestep, worldlow);
        }
    }

    if markceiling != 0 {
        ceilingplane = R_CheckPlane(ceilingplane, rw_x, rw_stopx - 1);
    }
    if markfloor != 0 {
        floorplane = R_CheckPlane(floorplane, rw_x, rw_stopx - 1);
    }

    R_RenderSegLoop();

    if ((*ds_p).silhouette & SIL_TOP) != 0 || maskedtexture != 0 {
        if (*ds_p).sprtopclip.is_null() {
            for i in 0..(rw_stopx - start) {
                *lastopening.add(i as usize) = ceilingclip[(start + i) as usize];
            }
            (*ds_p).sprtopclip = lastopening.sub(start as usize);
            lastopening = lastopening.add((rw_stopx - start) as usize);
        }
    }

    if ((*ds_p).silhouette & SIL_BOTTOM) != 0 || maskedtexture != 0 {
        if (*ds_p).sprbottomclip.is_null() {
            for i in 0..(rw_stopx - start) {
                *lastopening.add(i as usize) = floorclip[(start + i) as usize];
            }
            (*ds_p).sprbottomclip = lastopening.sub(start as usize);
            lastopening = lastopening.add((rw_stopx - start) as usize);
        }
    }

    if maskedtexture != 0 && ((*ds_p).silhouette & SIL_TOP) == 0 {
        (*ds_p).silhouette |= SIL_TOP;
        (*ds_p).tsilheight = c_int::MIN;
    }
    if maskedtexture != 0 && ((*ds_p).silhouette & SIL_BOTTOM) == 0 {
        (*ds_p).silhouette |= SIL_BOTTOM;
        (*ds_p).bsilheight = c_int::MAX;
    }

    ds_p = ds_p.add(1);
}
