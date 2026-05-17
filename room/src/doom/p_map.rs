//! Rust port of vendor/doomgeneric/p_map.c.
//!
//! Movement, collision handling, shooting and aiming.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_int, c_uint, c_void};
use std::ptr;

use crate::doom::c_ffi::{
    intercept_t, line_t, mobj_t, sector_t, subsector_t, LinedefFlag, DEFAULT_SPECHIT_MAGIC,
    MAPBLOCKSHIFT,
};
use crate::doom::info::MobjInfo;
use crate::doom::m_bbox::BBox;
use crate::doom::m_fixed::{fixed_t, FixedDiv, FixedMul, FRACBITS, FRACUNIT};
use crate::doom::m_random::P_Random;
use crate::doom::p_maputl::{
    lowfloor, openbottom, openrange, opentop, P_AproxDistance, P_BlockLinesIterator,
    P_BlockThingsIterator, P_BoxOnLineSide, P_LineOpening, P_PathTraverse, P_PointOnLineSide,
    P_SetThingPosition, P_UnsetThingPosition, PT_ADDLINES, PT_ADDTHINGS,
};
use crate::doom::p_setup::{bmaporgx, bmaporgy, lines};
use crate::doom::p_sight::{bottomslope, topslope, P_CheckSight};
use crate::doom::r_main::{validcount, R_PointInSubsector, R_PointToAngle2};
use crate::doom::tables::{finecosine, finesine, ANG180, ANGLETOFINESHIFT};
use crate::i_error;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const MAXSPECIALCROSS: usize = 20;
const MAXSPECIALCROSS_ORIGINAL: c_int = 8;
const MAXRADIUS: c_int = 32 * FRACUNIT;
const USERANGE: c_int = 64 * FRACUNIT;

const MF_SPECIAL: c_int = 1;
const MF_SOLID: c_int = 2;
const MF_SHOOTABLE: c_int = 4;
const MF_NOSECTOR: c_int = 8;
const MF_NOBLOCKMAP: c_int = 16;
const MF_MISSILE: c_int = 65536;
const MF_DROPOFF: c_int = 1024;
const MF_NOCLIP: c_int = 4096;
const MF_FLOAT: c_int = 16384;
const MF_TELEPORT: c_int = 32768;
const MF_SKULLFLY: c_int = 16777216;
const MF_PICKUP: c_int = 2048;
const MF_NOBLOOD: c_int = 524288;
const MF_DROPPED: c_int = 131072;

const MT_PLAYER: c_int = 0;
const MT_KNIGHT: c_int = 17;
const MT_BRUISER: c_int = 15;
const MT_CYBORG: c_int = 21;
const MT_SPIDER: c_int = 19;
const MT_BLOOD: c_int = 38;

const S_GIBS: c_int = 895;

const ST_HORIZONTAL: c_int = 0;
const ST_VERTICAL: c_int = 1;

const DEH_DEFAULT_SPECIES_INFIGHTING: c_int = 0;

// ---------------------------------------------------------------------------
// Extern functions from other modules
// ---------------------------------------------------------------------------

use crate::doom::g_game::gamemap;
use crate::doom::m_argv::{myargv, M_CheckParmWithArgs};
use crate::doom::m_misc::M_StrToInt;
use crate::doom::p_inter::{P_DamageMobj, P_TouchSpecialThing};
use crate::doom::p_mobj::{
    P_RemoveMobj, P_SetMobjState, P_SpawnBlood, P_SpawnMobj, P_SpawnPuff, P_SubstNullMobj,
};
use crate::doom::p_spec::{P_CrossSpecialLine, P_ShootSpecialLine};
use crate::doom::p_switch::P_UseSpecialLine;
use crate::doom::p_tick::leveltime;
use crate::doom::r_sky::skyflatnum;
use crate::doom::s_sound::S_StartSound;
use crate::doom::sounds::Sfx;

// Type aliases for cross-module pointer casts (all #[repr(C)] identical layouts).
type TeleptMobj = crate::doom::p_telept::mobj_t;

// ---------------------------------------------------------------------------
// Movement scratchpad globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut tmbbox: [fixed_t; 4] = [0; 4];
#[no_mangle]
pub static mut tmthing: *mut mobj_t = ptr::null_mut();
#[no_mangle]
pub static mut tmflags: c_int = 0;
#[no_mangle]
pub static mut tmx: fixed_t = 0;
#[no_mangle]
pub static mut tmy: fixed_t = 0;

#[no_mangle]
pub static mut floatok: c_int = 0; // boolean
#[no_mangle]
pub static mut tmfloorz: fixed_t = 0;
#[no_mangle]
pub static mut tmceilingz: fixed_t = 0;
#[no_mangle]
pub static mut tmdropoffz: fixed_t = 0;

#[no_mangle]
pub static mut ceilingline: *mut line_t = ptr::null_mut();

#[no_mangle]
pub static mut spechit: [*mut line_t; MAXSPECIALCROSS] = [ptr::null_mut(); MAXSPECIALCROSS];
#[no_mangle]
pub static mut numspechit: c_int = 0;

// ---------------------------------------------------------------------------
// TELEPORT MOVE
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn PIT_StompThing(thing: *mut mobj_t) -> c_uint {
    let thing = &*thing;
    if thing.flags & MF_SHOOTABLE == 0 {
        return 1;
    }
    let blockdist = thing.radius + (*tmthing).radius;
    if (thing.x - tmx).wrapping_abs() >= blockdist || (thing.y - tmy).wrapping_abs() >= blockdist {
        return 1;
    }
    if std::ptr::eq(thing, tmthing) {
        return 1;
    }
    if (*tmthing).player.is_null() && gamemap != 30 {
        return 0;
    }
    P_DamageMobj(
        thing as *const _ as *mut TeleptMobj,
        tmthing as *mut TeleptMobj,
        tmthing as *mut TeleptMobj,
        10000,
    );
    1
}

#[no_mangle]
pub unsafe extern "C" fn P_TeleportMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> c_uint {
    tmthing = thing;
    tmflags = (*thing).flags;
    tmx = x;
    tmy = y;

    tmbbox[BBox::TOP] = y + (*tmthing).radius;
    tmbbox[BBox::BOTTOM] = y - (*tmthing).radius;
    tmbbox[BBox::RIGHT] = x + (*tmthing).radius;
    tmbbox[BBox::LEFT] = x - (*tmthing).radius;

    let newsubsec = R_PointInSubsector(x, y) as *mut subsector_t;
    ceilingline = ptr::null_mut();
    let sec = (*newsubsec).sector as *mut sector_t;
    tmfloorz = (*sec).floorheight;
    tmdropoffz = tmfloorz;
    tmceilingz = (*sec).ceilingheight;

    validcount = validcount.wrapping_add(1);
    numspechit = 0;

    let xl = (tmbbox[BBox::LEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
    let xh = (tmbbox[BBox::RIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
    let yl = (tmbbox[BBox::BOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
    let yh = (tmbbox[BBox::TOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;

    for bx in xl..=xh {
        for by in yl..=yh {
            if P_BlockThingsIterator(bx, by, Some(PIT_StompThing)) == 0 {
                return 0;
            }
        }
    }

    P_UnsetThingPosition(thing);
    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;
    (*thing).x = x;
    (*thing).y = y;
    P_SetThingPosition(thing);

    1
}

// ---------------------------------------------------------------------------
// MOVEMENT ITERATOR FUNCTIONS
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn PIT_CheckLine(ld: *mut line_t) -> c_uint {
    let ld = &*ld;
    if tmbbox[BBox::RIGHT] <= ld.bbox[BBox::LEFT]
        || tmbbox[BBox::LEFT] >= ld.bbox[BBox::RIGHT]
        || tmbbox[BBox::TOP] <= ld.bbox[BBox::BOTTOM]
        || tmbbox[BBox::BOTTOM] >= ld.bbox[BBox::TOP]
    {
        return 1;
    }
    if P_BoxOnLineSide(std::ptr::addr_of_mut!(tmbbox[0]), ld as *const _ as *mut _) != -1 {
        return 1;
    }
    if ld.backsector.is_null() {
        return 0;
    }
    if (*tmthing).flags & MF_MISSILE == 0 {
        if (ld.flags as c_int) & (LinedefFlag::BLOCKING as c_int) != 0 {
            return 0;
        }
        if (*tmthing).player.is_null()
            && (ld.flags as c_int) & (LinedefFlag::BLOCKMONSTERS as c_int) != 0
        {
            return 0;
        }
    }
    P_LineOpening(ld as *const _ as *mut _);
    if opentop < tmceilingz {
        tmceilingz = opentop;
        ceilingline = ld as *const _ as *mut _;
    }
    if openbottom > tmfloorz {
        tmfloorz = openbottom;
    }
    if lowfloor < tmdropoffz {
        tmdropoffz = lowfloor;
    }
    if ld.special != 0 {
        spechit[numspechit as usize] = ld as *const _ as *mut _;
        numspechit += 1;
        if numspechit > MAXSPECIALCROSS_ORIGINAL {
            SpechitOverrun(ld as *const _ as *mut _);
        }
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn PIT_CheckThing(thing: *mut mobj_t) -> c_uint {
    let thing = &*thing;
    if thing.flags & (MF_SOLID | MF_SPECIAL | MF_SHOOTABLE) == 0 {
        return 1;
    }
    let blockdist = thing.radius + (*tmthing).radius;
    if (thing.x - tmx).wrapping_abs() >= blockdist || (thing.y - tmy).wrapping_abs() >= blockdist {
        return 1;
    }
    if std::ptr::eq(thing, tmthing) {
        return 1;
    }

    // check for skulls slamming into things
    if (*tmthing).flags & MF_SKULLFLY != 0 {
        let damage = ((P_Random() % 8) + 1) * (*((*tmthing).info as *mut MobjInfo)).damage;
        P_DamageMobj(
            thing as *const _ as *mut TeleptMobj,
            tmthing as *mut TeleptMobj,
            tmthing as *mut TeleptMobj,
            damage,
        );
        (*tmthing).flags &= !MF_SKULLFLY;
        (*tmthing).momx = 0;
        (*tmthing).momy = 0;
        (*tmthing).momz = 0;
        P_SetMobjState(
            tmthing as *mut TeleptMobj,
            (*((*tmthing).info as *mut MobjInfo)).spawnstate,
        );
        return 0;
    }

    // missiles can hit other things
    if (*tmthing).flags & MF_MISSILE != 0 {
        if (*tmthing).z > thing.z + thing.height {
            return 1;
        }
        if (*tmthing).z + (*tmthing).height < thing.z {
            return 1;
        }
        if !(*tmthing).target.is_null() {
            let target = (*tmthing).target as *mut mobj_t;
            let target_type = (*target).type_;
            let thing_type = thing.type_;
            if target_type == thing_type
                || (target_type == MT_KNIGHT && thing_type == MT_BRUISER)
                || (target_type == MT_BRUISER && thing_type == MT_KNIGHT)
            {
                if std::ptr::eq(thing, target) {
                    return 1;
                }
                if thing_type != MT_PLAYER && DEH_DEFAULT_SPECIES_INFIGHTING == 0 {
                    return 0;
                }
            }
        }
        if thing.flags & MF_SHOOTABLE == 0 {
            return (thing.flags & MF_SOLID == 0) as c_uint;
        }
        let damage = ((P_Random() % 8) + 1) * (*((*tmthing).info as *mut MobjInfo)).damage;
        let target = (*tmthing).target as *mut TeleptMobj;
        P_DamageMobj(
            thing as *const _ as *mut TeleptMobj,
            tmthing as *mut TeleptMobj,
            target,
            damage,
        );
        return 0;
    }

    // check for special pickup
    if thing.flags & MF_SPECIAL != 0 {
        let solid = thing.flags & MF_SOLID;
        if tmflags & MF_PICKUP != 0 {
            P_TouchSpecialThing(
                thing as *const _ as *mut TeleptMobj,
                tmthing as *mut TeleptMobj,
            );
        }
        return (solid == 0) as c_uint;
    }

    (thing.flags & MF_SOLID == 0) as c_uint
}

// ---------------------------------------------------------------------------
// MOVEMENT CLIPPING
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_CheckPosition(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> c_uint {
    tmthing = thing;
    tmflags = (*thing).flags;
    tmx = x;
    tmy = y;

    tmbbox[BBox::TOP] = y + (*tmthing).radius;
    tmbbox[BBox::BOTTOM] = y - (*tmthing).radius;
    tmbbox[BBox::RIGHT] = x + (*tmthing).radius;
    tmbbox[BBox::LEFT] = x - (*tmthing).radius;

    let newsubsec = R_PointInSubsector(x, y) as *mut subsector_t;
    ceilingline = ptr::null_mut();
    let sec = (*newsubsec).sector as *mut sector_t;
    tmfloorz = (*sec).floorheight;
    tmdropoffz = tmfloorz;
    tmceilingz = (*sec).ceilingheight;

    validcount = validcount.wrapping_add(1);
    numspechit = 0;

    if tmflags & MF_NOCLIP != 0 {
        return 1;
    }

    let xl = (tmbbox[BBox::LEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
    let xh = (tmbbox[BBox::RIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
    let yl = (tmbbox[BBox::BOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
    let yh = (tmbbox[BBox::TOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;

    for bx in xl..=xh {
        for by in yl..=yh {
            if P_BlockThingsIterator(bx, by, Some(PIT_CheckThing)) == 0 {
                return 0;
            }
        }
    }

    let xl = (tmbbox[BBox::LEFT] - bmaporgx) >> MAPBLOCKSHIFT;
    let xh = (tmbbox[BBox::RIGHT] - bmaporgx) >> MAPBLOCKSHIFT;
    let yl = (tmbbox[BBox::BOTTOM] - bmaporgy) >> MAPBLOCKSHIFT;
    let yh = (tmbbox[BBox::TOP] - bmaporgy) >> MAPBLOCKSHIFT;

    for bx in xl..=xh {
        for by in yl..=yh {
            if P_BlockLinesIterator(bx, by, Some(PIT_CheckLine)) == 0 {
                return 0;
            }
        }
    }

    1
}

#[no_mangle]
pub unsafe extern "C" fn P_TryMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> c_uint {
    floatok = 0;
    if P_CheckPosition(thing, x, y) == 0 {
        return 0;
    }
    if (*thing).flags & MF_NOCLIP == 0 {
        if tmceilingz - tmfloorz < (*thing).height {
            return 0;
        }
        floatok = 1;
        if (*thing).flags & MF_TELEPORT == 0 && tmceilingz - (*thing).z < (*thing).height {
            return 0;
        }
        if (*thing).flags & MF_TELEPORT == 0 && tmfloorz - (*thing).z > 24 * FRACUNIT {
            return 0;
        }
        if (*thing).flags & (MF_DROPOFF | MF_FLOAT) == 0 && tmfloorz - tmdropoffz > 24 * FRACUNIT {
            return 0;
        }
    }

    P_UnsetThingPosition(thing);
    let oldx = (*thing).x;
    let oldy = (*thing).y;
    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;
    (*thing).x = x;
    (*thing).y = y;
    P_SetThingPosition(thing);

    if (*thing).flags & (MF_TELEPORT | MF_NOCLIP) == 0 {
        while numspechit > 0 {
            numspechit -= 1;
            let ld = spechit[numspechit as usize];
            let side = P_PointOnLineSide((*thing).x, (*thing).y, ld);
            let oldside = P_PointOnLineSide(oldx, oldy, ld);
            if side != oldside && (*ld).special != 0 {
                let linenum = ld.offset_from(lines);
                P_CrossSpecialLine(linenum as c_int, oldside, thing);
            }
        }
    }

    1
}

#[no_mangle]
pub unsafe extern "C" fn P_ThingHeightClip(thing: *mut mobj_t) -> c_uint {
    let onfloor = (*thing).z == (*thing).floorz;
    P_CheckPosition(thing, (*thing).x, (*thing).y);
    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;
    if onfloor {
        (*thing).z = (*thing).floorz;
    } else if (*thing).z + (*thing).height > (*thing).ceilingz {
        (*thing).z = (*thing).ceilingz - (*thing).height;
    }
    if (*thing).ceilingz - (*thing).floorz < (*thing).height {
        return 0;
    }
    1
}

// ---------------------------------------------------------------------------
// SLIDE MOVE
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut bestslidefrac: fixed_t = 0;
#[no_mangle]
pub static mut secondslidefrac: fixed_t = 0;
#[no_mangle]
pub static mut bestslideline: *mut line_t = ptr::null_mut();
#[no_mangle]
pub static mut secondslideline: *mut line_t = ptr::null_mut();
#[no_mangle]
pub static mut slidemo: *mut mobj_t = ptr::null_mut();
#[no_mangle]
pub static mut tmxmove: fixed_t = 0;
#[no_mangle]
pub static mut tmymove: fixed_t = 0;

#[no_mangle]
pub unsafe extern "C" fn P_HitSlideLine(ld: *mut line_t) {
    let ld = &*ld;
    if ld.slopetype == ST_HORIZONTAL {
        tmymove = 0;
        return;
    }
    if ld.slopetype == ST_VERTICAL {
        tmxmove = 0;
        return;
    }
    let side = P_PointOnLineSide((*slidemo).x, (*slidemo).y, ld as *const _ as *mut _);
    let mut lineangle = R_PointToAngle2(0, 0, ld.dx, ld.dy);
    if side == 1 {
        lineangle = lineangle.wrapping_add(ANG180);
    }
    let moveangle = R_PointToAngle2(0, 0, tmxmove, tmymove);
    let mut deltaangle = moveangle.wrapping_sub(lineangle);
    if deltaangle > ANG180 {
        deltaangle = deltaangle.wrapping_add(ANG180);
    }
    let lineangle = (lineangle >> ANGLETOFINESHIFT) as usize;
    let deltaangle = (deltaangle >> ANGLETOFINESHIFT) as usize;
    let movelen = P_AproxDistance(tmxmove, tmymove);
    let newlen = FixedMul(movelen, *finecosine.0.add(deltaangle));
    tmxmove = FixedMul(newlen, *finecosine.0.add(lineangle));
    tmymove = FixedMul(newlen, finesine[lineangle]);
}

#[no_mangle]
pub unsafe extern "C" fn PTR_SlideTraverse(in_: *mut intercept_t) -> c_uint {
    let in_ = &*in_;
    if in_.isaline == 0 {
        i_error!("PTR_SlideTraverse: not a line?");
    }
    let li = in_.d.line;
    if (*li).flags as c_int & LinedefFlag::TWOSIDED as c_int == 0 {
        if P_PointOnLineSide((*slidemo).x, (*slidemo).y, li) != 0 {
            return 1;
        }
    } else {
        P_LineOpening(li);
        if openrange >= (*slidemo).height
            && opentop - (*slidemo).z >= (*slidemo).height
            && openbottom - (*slidemo).z <= 24 * FRACUNIT
        {
            return 1;
        }
    }
    if in_.frac < bestslidefrac {
        secondslidefrac = bestslidefrac;
        secondslideline = bestslideline;
        bestslidefrac = in_.frac;
        bestslideline = li;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn P_SlideMove(mo: *mut mobj_t) {
    slidemo = mo;
    let mut hitcount = 0;

    'retry: loop {
        hitcount += 1;
        if hitcount == 3 {
            // stairstep
            if P_TryMove(mo, (*mo).x, (*mo).y + (*mo).momy) == 0 {
                P_TryMove(mo, (*mo).x + (*mo).momx, (*mo).y);
            }
            return;
        }

        let (leadx, trailx) = if (*mo).momx > 0 {
            ((*mo).x + (*mo).radius, (*mo).x - (*mo).radius)
        } else {
            ((*mo).x - (*mo).radius, (*mo).x + (*mo).radius)
        };
        let (leady, traily) = if (*mo).momy > 0 {
            ((*mo).y + (*mo).radius, (*mo).y - (*mo).radius)
        } else {
            ((*mo).y - (*mo).radius, (*mo).y + (*mo).radius)
        };

        bestslidefrac = FRACUNIT + 1;

        P_PathTraverse(
            leadx,
            leady,
            leadx + (*mo).momx,
            leady + (*mo).momy,
            PT_ADDLINES,
            Some(PTR_SlideTraverse),
        );
        P_PathTraverse(
            trailx,
            leady,
            trailx + (*mo).momx,
            leady + (*mo).momy,
            PT_ADDLINES,
            Some(PTR_SlideTraverse),
        );
        P_PathTraverse(
            leadx,
            traily,
            leadx + (*mo).momx,
            traily + (*mo).momy,
            PT_ADDLINES,
            Some(PTR_SlideTraverse),
        );

        if bestslidefrac == FRACUNIT + 1 {
            // stairstep
            if P_TryMove(mo, (*mo).x, (*mo).y + (*mo).momy) == 0 {
                P_TryMove(mo, (*mo).x + (*mo).momx, (*mo).y);
            }
            return;
        }

        bestslidefrac -= 0x800;
        if bestslidefrac > 0 {
            let newx = FixedMul((*mo).momx, bestslidefrac);
            let newy = FixedMul((*mo).momy, bestslidefrac);
            if P_TryMove(mo, (*mo).x + newx, (*mo).y + newy) == 0 {
                continue 'retry;
            }
        }

        bestslidefrac = FRACUNIT - (bestslidefrac + 0x800);
        if bestslidefrac > FRACUNIT {
            bestslidefrac = FRACUNIT;
        }
        if bestslidefrac <= 0 {
            return;
        }
        tmxmove = FixedMul((*mo).momx, bestslidefrac);
        tmymove = FixedMul((*mo).momy, bestslidefrac);
        P_HitSlideLine(bestslideline);
        (*mo).momx = tmxmove;
        (*mo).momy = tmymove;
        if P_TryMove(mo, (*mo).x + tmxmove, (*mo).y + tmymove) == 0 {
            continue 'retry;
        }
        return;
    }
}

// ---------------------------------------------------------------------------
// LINE ATTACK
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut linetarget: *mut mobj_t = ptr::null_mut();
#[no_mangle]
pub static mut shootthing: *mut mobj_t = ptr::null_mut();
#[no_mangle]
pub static mut shootz: fixed_t = 0;
#[no_mangle]
pub static mut la_damage: c_int = 0;
#[no_mangle]
pub static mut attackrange: fixed_t = 0;
#[no_mangle]
pub static mut aimslope: fixed_t = 0;

#[no_mangle]
pub unsafe extern "C" fn PTR_AimTraverse(in_: *mut intercept_t) -> c_uint {
    let in_ = &*in_;
    if in_.isaline != 0 {
        let li = in_.d.line;
        if (*li).flags as c_int & LinedefFlag::TWOSIDED as c_int == 0 {
            return 0;
        }
        P_LineOpening(li);
        if openbottom >= opentop {
            return 0;
        }
        let dist = FixedMul(attackrange, in_.frac);
        let front = (*li).frontsector as *mut sector_t;
        let back = (*li).backsector as *mut sector_t;
        if back.is_null() || (*front).floorheight != (*back).floorheight {
            let slope = FixedDiv(openbottom - shootz, dist);
            if slope > bottomslope {
                bottomslope = slope;
            }
        }
        if back.is_null() || (*front).ceilingheight != (*back).ceilingheight {
            let slope = FixedDiv(opentop - shootz, dist);
            if slope < topslope {
                topslope = slope;
            }
        }
        if topslope <= bottomslope {
            return 0;
        }
        return 1;
    }

    let th = in_.d.thing as *mut mobj_t;
    if th == shootthing {
        return 1;
    }
    if (*th).flags & MF_SHOOTABLE == 0 {
        return 1;
    }
    let dist = FixedMul(attackrange, in_.frac);
    let thingtopslope = FixedDiv((*th).z + (*th).height - shootz, dist);
    if thingtopslope < bottomslope {
        return 1;
    }
    let thingbottomslope = FixedDiv((*th).z - shootz, dist);
    if thingbottomslope > topslope {
        return 1;
    }
    let mut thingtopslope = thingtopslope;
    let mut thingbottomslope = thingbottomslope;
    if thingtopslope > topslope {
        thingtopslope = topslope;
    }
    if thingbottomslope < bottomslope {
        thingbottomslope = bottomslope;
    }
    aimslope = (thingtopslope + thingbottomslope) / 2;
    linetarget = th;
    0
}

#[no_mangle]
pub unsafe extern "C" fn PTR_ShootTraverse(in_: *mut intercept_t) -> c_uint {
    let in_ = &*in_;
    if in_.isaline != 0 {
        let li = in_.d.line;
        if (*li).special != 0 {
            P_ShootSpecialLine(shootthing, li);
        }
        if (*li).flags as c_int & LinedefFlag::TWOSIDED as c_int != 0 {
            P_LineOpening(li);
            let dist = FixedMul(attackrange, in_.frac);
            let back = (*li).backsector as *mut sector_t;
            if back.is_null() {
                let slope = FixedDiv(openbottom - shootz, dist);
                if slope > aimslope {
                    goto_hitline(li, in_);
                    return 0;
                }
                let slope = FixedDiv(opentop - shootz, dist);
                if slope < aimslope {
                    goto_hitline(li, in_);
                    return 0;
                }
            } else {
                let front = (*li).frontsector as *mut sector_t;
                if (*front).floorheight != (*back).floorheight {
                    let slope = FixedDiv(openbottom - shootz, dist);
                    if slope > aimslope {
                        goto_hitline(li, in_);
                        return 0;
                    }
                }
                if (*front).ceilingheight != (*back).ceilingheight {
                    let slope = FixedDiv(opentop - shootz, dist);
                    if slope < aimslope {
                        goto_hitline(li, in_);
                        return 0;
                    }
                }
            }
            return 1;
        }
        goto_hitline(li, in_);
        return 0;
    }

    let th = in_.d.thing as *mut mobj_t;
    if th == shootthing {
        return 1;
    }
    if (*th).flags & MF_SHOOTABLE == 0 {
        return 1;
    }
    let dist = FixedMul(attackrange, in_.frac);
    let thingtopslope = FixedDiv((*th).z + (*th).height - shootz, dist);
    if thingtopslope < aimslope {
        return 1;
    }
    let thingbottomslope = FixedDiv((*th).z - shootz, dist);
    if thingbottomslope > aimslope {
        return 1;
    }

    let frac = in_.frac - FixedDiv(10 * FRACUNIT, attackrange);
    let x = crate::doom::p_maputl::trace.x + FixedMul(crate::doom::p_maputl::trace.dx, frac);
    let y = crate::doom::p_maputl::trace.y + FixedMul(crate::doom::p_maputl::trace.dy, frac);
    let z = shootz + FixedMul(aimslope, FixedMul(frac, attackrange));
    if (*th).flags & MF_NOBLOOD != 0 {
        P_SpawnPuff(x, y, z);
    } else {
        P_SpawnBlood(x, y, z, la_damage);
    }
    if la_damage != 0 {
        P_DamageMobj(
            th as *mut TeleptMobj,
            shootthing as *mut TeleptMobj,
            shootthing as *mut TeleptMobj,
            la_damage,
        );
    }
    0
}

unsafe fn goto_hitline(li: *mut line_t, in_: &intercept_t) {
    let frac = in_.frac - FixedDiv(4 * FRACUNIT, attackrange);
    let x = crate::doom::p_maputl::trace.x + FixedMul(crate::doom::p_maputl::trace.dx, frac);
    let y = crate::doom::p_maputl::trace.y + FixedMul(crate::doom::p_maputl::trace.dy, frac);
    let z = shootz + FixedMul(aimslope, FixedMul(frac, attackrange));
    let front = (*li).frontsector as *mut sector_t;
    if (*front).ceilingpic as c_int == skyflatnum {
        if z > (*front).ceilingheight {
            return;
        }
        let back = (*li).backsector as *mut sector_t;
        if !back.is_null() && (*back).ceilingpic as c_int == skyflatnum {
            return;
        }
    }
    P_SpawnPuff(x, y, z);
}

#[no_mangle]
pub unsafe extern "C" fn P_AimLineAttack(
    t1: *mut mobj_t,
    angle: c_uint,
    distance: fixed_t,
) -> fixed_t {
    let t1 = P_SubstNullMobj(t1 as *mut TeleptMobj) as *mut mobj_t;
    let angle = (angle >> ANGLETOFINESHIFT) as usize;
    shootthing = t1;
    let x2 = (*t1).x + ((distance >> FRACBITS) as c_int) * *finecosine.0.add(angle);
    let y2 = (*t1).y + ((distance >> FRACBITS) as c_int) * finesine[angle];
    shootz = (*t1).z + ((*t1).height >> 1) + 8 * FRACUNIT;
    topslope = 100 * FRACUNIT / 160;
    bottomslope = -(100 * FRACUNIT / 160);
    attackrange = distance;
    linetarget = ptr::null_mut();
    P_PathTraverse(
        (*t1).x,
        (*t1).y,
        x2,
        y2,
        PT_ADDLINES | PT_ADDTHINGS,
        Some(PTR_AimTraverse),
    );
    if !linetarget.is_null() {
        return aimslope;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn P_LineAttack(
    t1: *mut mobj_t,
    angle: c_uint,
    distance: fixed_t,
    slope: fixed_t,
    damage: c_int,
) {
    let angle = (angle >> ANGLETOFINESHIFT) as usize;
    shootthing = t1;
    la_damage = damage;
    let x2 = (*t1).x + ((distance >> FRACBITS) as c_int) * *finecosine.0.add(angle);
    let y2 = (*t1).y + ((distance >> FRACBITS) as c_int) * finesine[angle];
    shootz = (*t1).z + ((*t1).height >> 1) + 8 * FRACUNIT;
    attackrange = distance;
    aimslope = slope;
    P_PathTraverse(
        (*t1).x,
        (*t1).y,
        x2,
        y2,
        PT_ADDLINES | PT_ADDTHINGS,
        Some(PTR_ShootTraverse),
    );
}

// ---------------------------------------------------------------------------
// USE LINES
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut usething: *mut mobj_t = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn PTR_UseTraverse(in_: *mut intercept_t) -> c_uint {
    let in_ = &*in_;
    if (*in_.d.line).special == 0 {
        P_LineOpening(in_.d.line);
        if openrange <= 0 {
            S_StartSound(usething as *mut c_void, Sfx::Noway as c_int);
            return 0;
        }
        return 1;
    }
    let mut side = 0;
    if P_PointOnLineSide((*usething).x, (*usething).y, in_.d.line) == 1 {
        side = 1;
    }
    P_UseSpecialLine(
        usething as *mut c_void,
        in_.d.line as *mut crate::doom::p_lights::line_t,
        side,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn P_UseLines(player: *mut c_void) {
    let player = player as *mut crate::doom::d_player::PlayerT;
    let mo = (*player).mo as *mut mobj_t;
    usething = mo;
    let angle = ((*mo).angle >> ANGLETOFINESHIFT) as usize;
    let x1 = (*mo).x;
    let y1 = (*mo).y;
    let x2 = x1 + ((USERANGE >> FRACBITS) as c_int) * *finecosine.0.add(angle);
    let y2 = y1 + ((USERANGE >> FRACBITS) as c_int) * finesine[angle];
    P_PathTraverse(x1, y1, x2, y2, PT_ADDLINES, Some(PTR_UseTraverse));
}

// ---------------------------------------------------------------------------
// RADIUS ATTACK
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut bombsource: *mut mobj_t = ptr::null_mut();
#[no_mangle]
pub static mut bombspot: *mut mobj_t = ptr::null_mut();
#[no_mangle]
pub static mut bombdamage: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn PIT_RadiusAttack(thing: *mut mobj_t) -> c_uint {
    let thing = &*thing;
    if thing.flags & MF_SHOOTABLE == 0 {
        return 1;
    }
    if thing.type_ == MT_CYBORG || thing.type_ == MT_SPIDER {
        return 1;
    }
    let dx = (thing.x - (*bombspot).x).wrapping_abs();
    let dy = (thing.y - (*bombspot).y).wrapping_abs();
    let mut dist = if dx > dy { dx } else { dy };
    dist = (dist - thing.radius) >> FRACBITS;
    if dist < 0 {
        dist = 0;
    }
    if dist >= bombdamage {
        return 1;
    }
    let pt_mobj_t = thing as *const _ as *mut crate::doom::p_telept::mobj_t;
    let pt_bombspot = bombspot as *mut crate::doom::p_telept::mobj_t;
    if P_CheckSight(pt_mobj_t, pt_bombspot) != 0 {
        P_DamageMobj(
            thing as *const _ as *mut TeleptMobj,
            bombspot as *mut TeleptMobj,
            bombsource as *mut TeleptMobj,
            bombdamage - dist,
        );
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn P_RadiusAttack(spot: *mut mobj_t, source: *mut mobj_t, damage: c_int) {
    let dist = (damage + MAXRADIUS) << FRACBITS;
    let yh = ((*spot).y + dist - bmaporgy) >> MAPBLOCKSHIFT;
    let yl = ((*spot).y - dist - bmaporgy) >> MAPBLOCKSHIFT;
    let xh = ((*spot).x + dist - bmaporgx) >> MAPBLOCKSHIFT;
    let xl = ((*spot).x - dist - bmaporgx) >> MAPBLOCKSHIFT;
    bombspot = spot;
    bombsource = source;
    bombdamage = damage;
    for y in yl..=yh {
        for x in xl..=xh {
            P_BlockThingsIterator(x, y, Some(PIT_RadiusAttack));
        }
    }
}

// ---------------------------------------------------------------------------
// SECTOR HEIGHT CHANGING
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut crushchange: c_int = 0; // boolean
#[no_mangle]
pub static mut nofit: c_int = 0; // boolean

#[no_mangle]
pub unsafe extern "C" fn PIT_ChangeSector(thing: *mut mobj_t) -> c_uint {
    if P_ThingHeightClip(thing) != 0 {
        return 1;
    }
    let thing = &mut *thing;
    if thing.health <= 0 {
        P_SetMobjState(thing as *const _ as *mut TeleptMobj, S_GIBS);
        thing.flags &= !MF_SOLID;
        thing.height = 0;
        thing.radius = 0;
        return 1;
    }
    if thing.flags & MF_DROPPED != 0 {
        P_RemoveMobj(thing as *const _ as *mut TeleptMobj);
        return 1;
    }
    if thing.flags & MF_SHOOTABLE == 0 {
        return 1;
    }
    nofit = 1;
    if crushchange != 0 && leveltime & 3 == 0 {
        P_DamageMobj(
            thing as *const _ as *mut TeleptMobj,
            ptr::null_mut(),
            ptr::null_mut(),
            10,
        );
        let mo = P_SpawnMobj(thing.x, thing.y, thing.z + thing.height / 2, MT_BLOOD);
        (*mo).momx = (P_Random() - P_Random()) << 12;
        (*mo).momy = (P_Random() - P_Random()) << 12;
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn P_ChangeSector(sector: *mut sector_t, crunch: c_int) -> c_int {
    nofit = 0;
    crushchange = crunch;
    let sec = &*sector;
    for x in sec.blockbox[BBox::LEFT]..=sec.blockbox[BBox::RIGHT] {
        for y in sec.blockbox[BBox::BOTTOM]..=sec.blockbox[BBox::TOP] {
            P_BlockThingsIterator(x, y, Some(PIT_ChangeSector));
        }
    }
    nofit
}

// ---------------------------------------------------------------------------
// Spechit overrun emulation
// ---------------------------------------------------------------------------

unsafe fn SpechitOverrun(ld: *mut line_t) {
    static mut baseaddr: c_uint = 0;
    if baseaddr == 0 {
        let p = M_CheckParmWithArgs(c"-spechit".as_ptr().cast_mut(), 1);
        if p > 0 {
            M_StrToInt(
                *myargv.add((p + 1) as usize),
                &raw mut baseaddr as *mut c_int,
            );
        } else {
            baseaddr = DEFAULT_SPECHIT_MAGIC;
        }
    }
    let addr = baseaddr as c_int + (ld.offset_from(lines) * 0x3e) as c_int;
    match numspechit {
        9..=12 => {
            tmbbox[(numspechit - 9) as usize] = addr;
        }
        13 => {
            crushchange = addr;
        }
        14 => {
            nofit = addr;
        }
        _ => {
            eprintln!(
                "SpechitOverrun: Warning: unable to emulate an overrun where numspechit={}",
                numspechit as c_int
            );
        }
    }
}
