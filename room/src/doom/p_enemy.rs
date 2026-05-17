//! Rust port of vendor/doomgeneric/p_enemy.c.
//!
//! Enemy thinking, AI, and action pointer functions.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_void;
use std::os::raw::{c_int, c_uint};

use crate::types::Boolean;
type size_t = usize;
type angle_t = c_uint;
type statenum_t = c_int;
type c_short = i16;
type mobjtype_t = c_int;
type mobjinfo_t = MobjInfo;

#[inline]
fn abs(x: c_int) -> c_int {
    if x < 0 {
        -x
    } else {
        x
    }
}

use crate::doom::c_ffi::{line_t, sector_t, vertex_t, MAPBLOCKSHIFT};
use crate::doom::d_loop::gametic;
use crate::doom::d_player::{players, PlayerT, PspdefT, MAXPLAYERS};
use crate::doom::doomstat::{gamemode, gameversion};
use crate::doom::info::{
    mobjinfo, MobjInfo, State, MF_AMBUSH, MF_CORPSE, MF_FLOAT, MF_INFLOAT, MF_JUSTATTACKED,
    MF_JUSTHIT, MF_SHADOW, MF_SHOOTABLE, MF_SKULLFLY, MF_SOLID, MT_ARACHPLAZ, MT_BABY,
    MT_BOSSTARGET, MT_BRUISER, MT_BRUISERSHOT, MT_CYBORG, MT_FATSHOT, MT_FATSO, MT_FIRE, MT_HEAD,
    MT_HEADSHOT, MT_KNIGHT, MT_PAIN, MT_ROCKET, MT_SERGEANT, MT_SHADOWS, MT_SKULL, MT_SMOKE,
    MT_SPAWNFIRE, MT_SPAWNSHOT, MT_SPIDER, MT_TRACER, MT_TROOP, MT_TROOPSHOT, MT_UNDEAD, MT_VILE,
    S_BRAINEXPLODE1, S_NULL, S_VILE_HEAL1,
};
use crate::doom::m_fixed::FRACUNIT;
use crate::doom::m_fixed::{fixed_t, FixedMul};
use crate::doom::m_random::P_Random;
use crate::doom::p_inter::P_DamageMobj;
use crate::doom::p_map::{
    floatok, numspechit, spechit, tmfloorz, P_AimLineAttack, P_CheckPosition, P_LineAttack,
    P_RadiusAttack, P_TeleportMove, P_TryMove,
};
use crate::doom::p_maputl::{
    openrange, P_AproxDistance, P_BlockThingsIterator, P_LineOpening, P_SetThingPosition,
    P_UnsetThingPosition,
};
use crate::doom::p_mobj::{
    P_MobjThinker, P_RemoveMobj, P_SetMobjState, P_SpawnMissile, P_SpawnMobj, P_SpawnPuff,
    P_SubstNullMobj,
};
use crate::doom::p_setup::{bmaporgx, bmaporgy, sides};
use crate::doom::p_sight::P_CheckSight;
use crate::doom::p_switch::P_UseSpecialLine;
use crate::doom::p_telept::mobj_t;
use crate::doom::tables::{ANG180, ANG270, ANG90, ANGLETOFINESHIFT};
use crate::i_error;
type CffiMobj = crate::doom::c_ffi::mobj_t;
use crate::doom::p_tick::{thinker_t, thinkercap};
use crate::doom::r_main::{validcount, R_PointToAngle2};
use crate::doom::s_sound::S_StartSound;
use crate::doom::tables::{finecosine, finesine};

// Local constants
const MELEERANGE: c_int = 64 * FRACUNIT;
const MISSILERANGE: c_int = 32 * 64 * FRACUNIT;
const FLOATSPEED: c_int = FRACUNIT * 4;
const MAXRADIUS: c_int = 32 * FRACUNIT;
const ML_TWOSIDED: c_int = 4;
const ML_SOUNDBLOCK: c_int = 64;

// Direction type constants
type dirtype_t = c_int;
const DI_EAST: c_int = 0;
const DI_NORTHEAST: c_int = 1;
const DI_NORTH: c_int = 2;
const DI_NORTHWEST: c_int = 3;
const DI_WEST: c_int = 4;
const DI_SOUTHWEST: c_int = 5;
const DI_SOUTH: c_int = 6;
const DI_SOUTHEAST: c_int = 7;
const DI_NODIR: c_int = 8;
const NUMDIRS: c_int = 9;

// Sound effect constants
const sfx_posit1: c_int = 36;
const sfx_posit2: c_int = 37;
const sfx_posit3: c_int = 38;
const sfx_bgsit1: c_int = 39;
const sfx_bgsit2: c_int = 40;
const sfx_pistol: c_int = 1;
const sfx_shotgn: c_int = 2;
const sfx_claw: c_int = 41;
const sfx_skeswg: c_int = 42;
const sfx_skepch: c_int = 43;
const sfx_vilatk: c_int = 44;
const sfx_flamst: c_int = 45;
const sfx_flame: c_int = 46;
const sfx_barexp: c_int = 49;
const sfx_manatk: c_int = 50;
const sfx_slop: c_int = 52;
const sfx_hoof: c_int = 70;
const sfx_metal: c_int = 71;
const sfx_bspwlk: c_int = 72;
const sfx_dbopn: c_int = 73;
const sfx_dbload: c_int = 74;
const sfx_dbcls: c_int = 75;
const sfx_bossit: c_int = 76;
const sfx_bospn: c_int = 77;
const sfx_bosdth: c_int = 78;
const sfx_bospit: c_int = 79;
const sfx_boscub: c_int = 80;
const sfx_telept: c_int = 81;
const sfx_pldeth: c_int = 82;
const sfx_pdiehi: c_int = 83;
const sfx_podth1: c_int = 59;
const sfx_podth2: c_int = 60;
const sfx_podth3: c_int = 61;
const sfx_bgdth1: c_int = 62;
const sfx_bgdth2: c_int = 63;

// Game version constants
const exe_ultimate: c_int = 6;

// Game mode constants
const commercial: c_int = 2;

// Skill constants
const sk_nightmare: c_int = 4;
const sk_easy: c_int = 1;

// Floor/door type constants
const lowerFloorToLowest: c_int = 5;
const raiseToTexture: c_int = 8;
const vld_blazeOpen: c_int = 5;
const vld_open: c_int = 0;

const MT_PLAYER: c_int = 0;
const FATSPREAD: c_int = ANG90 as c_int / 8;
const SKULLSPEED: c_int = 20 * FRACUNIT;

use crate::doom::d_main::fastparm;
use crate::doom::g_game::{gameepisode, gamemap, gameskill, netgame, playeringame, G_ExitLevel};
use crate::doom::p_doors::EV_DoDoor;
use crate::doom::p_floor::EV_DoFloor;
use crate::doom::p_pspr::A_ReFire;

// Type alias for cross-module pointer cast (all #[repr(C)] identical layouts).
type PLineThing = crate::doom::p_lights::line_t;

#[no_mangle]
pub static mut opposite: [dirtype_t; 9] = [
    DI_WEST,
    DI_SOUTHWEST,
    DI_SOUTH,
    DI_SOUTHEAST,
    DI_EAST,
    DI_NORTHEAST,
    DI_NORTH,
    DI_NORTHWEST,
    DI_NODIR,
];
#[no_mangle]
pub static mut diags: [dirtype_t; 4] = [DI_NORTHWEST, DI_NORTHEAST, DI_SOUTHWEST, DI_SOUTHEAST];
#[no_mangle]
pub static mut soundtarget: *mut mobj_t = std::ptr::null_mut::<mobj_t>();
#[no_mangle]
pub unsafe extern "C" fn P_RecursiveSound(sec: *mut sector_t, soundblocks: c_int) {
    let mut i: c_int;

    let mut check: *mut line_t;

    let mut other: *mut sector_t;

    if (*sec).validcount == validcount && (*sec).soundtraversed <= soundblocks + 1 as c_int {
        return;
    }
    (*sec).validcount = validcount;
    (*sec).soundtraversed = soundblocks + 1 as c_int;
    (*sec).soundtarget = soundtarget as *mut c_void;
    i = 0 as c_int;
    while i < (*sec).linecount {
        check = *(*sec).lines.offset(i as isize) as *mut line_t;
        if (*check).flags as c_int & ML_TWOSIDED != 0 {
            P_LineOpening(check);
            if openrange > 0 as c_int {
                if std::ptr::eq(
                    (*sides.offset((*check).sidenum[0 as c_int as usize] as isize)).sector,
                    sec,
                ) {
                    other = (*sides.offset((*check).sidenum[1 as c_int as usize] as isize)).sector;
                } else {
                    other = (*sides.offset((*check).sidenum[0 as c_int as usize] as isize)).sector;
                }
                if (*check).flags as c_int & ML_SOUNDBLOCK != 0 {
                    if soundblocks == 0 {
                        P_RecursiveSound(other, 1 as c_int);
                    }
                } else {
                    P_RecursiveSound(other, soundblocks);
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_NoiseAlert(target: *mut mobj_t, emmiter: *mut mobj_t) {
    soundtarget = target;
    validcount += 1;
    P_RecursiveSound((*(*emmiter).subsector).sector as *mut sector_t, 0 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckMeleeRange(actor: *mut mobj_t) -> Boolean {
    if (*actor).target.is_null() {
        return Boolean::FALSE;
    }
    let pl: *mut mobj_t = (*actor).target;
    let dist: fixed_t = P_AproxDistance((*pl).x - (*actor).x, (*pl).y - (*actor).y);
    if dist >= MELEERANGE - 20 as c_int * FRACUNIT + (*((*pl).info as *mut MobjInfo)).radius {
        return Boolean::FALSE;
    }
    if P_CheckSight(actor, (*actor).target) == 0 {
        return Boolean::FALSE;
    }
    Boolean::TRUE
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckMissileRange(actor: *mut mobj_t) -> Boolean {
    let mut dist: fixed_t;

    if P_CheckSight(actor, (*actor).target) == 0 {
        return Boolean::FALSE;
    }
    if (*actor).flags & MF_JUSTHIT as c_int != 0 {
        (*actor).flags &= !(MF_JUSTHIT as c_int);
        return Boolean::TRUE;
    }
    if (*actor).reactiontime != 0 {
        return Boolean::FALSE;
    }
    dist = (P_AproxDistance(
        (*actor).x - (*(*actor).target).x,
        (*actor).y - (*(*actor).target).y,
    ) as c_int
        - 64 as c_int * FRACUNIT) as fixed_t;
    if (*((*actor).info as *mut MobjInfo)).meleestate == 0 {
        dist -= 128 as c_int * FRACUNIT;
    }
    dist >>= 16 as c_int;
    if (*actor).mobjtype as c_uint == MT_VILE as c_int as c_uint && dist > 14 as c_int * 64 as c_int
    {
        return Boolean::FALSE;
    }
    if (*actor).mobjtype as c_uint == MT_UNDEAD as c_int as c_uint {
        if dist < 196 as c_int {
            return Boolean::FALSE;
        }
        dist >>= 1 as c_int;
    }
    if (*actor).mobjtype as c_uint == MT_CYBORG as c_int as c_uint
        || (*actor).mobjtype as c_uint == MT_SPIDER as c_int as c_uint
        || (*actor).mobjtype as c_uint == MT_SKULL as c_int as c_uint
    {
        dist >>= 1 as c_int;
    }
    if dist > 200 as c_int {
        dist = 200 as c_int as fixed_t;
    }
    if (*actor).mobjtype as c_uint == MT_CYBORG as c_int as c_uint && dist > 160 as c_int {
        dist = 160 as c_int as fixed_t;
    }
    if P_Random() < dist {
        return Boolean::FALSE;
    }
    Boolean::TRUE
}
#[no_mangle]
pub static mut xspeed: [fixed_t; 8] = [
    FRACUNIT,
    47000 as c_int,
    0 as c_int,
    -47000 as c_int,
    -FRACUNIT,
    -47000 as c_int,
    0 as c_int,
    47000 as c_int,
];
#[no_mangle]
pub static mut yspeed: [fixed_t; 8] = [
    0 as c_int,
    47000 as c_int,
    FRACUNIT,
    47000 as c_int,
    0 as c_int,
    -47000 as c_int,
    -FRACUNIT,
    -47000 as c_int,
];
#[no_mangle]
pub unsafe extern "C" fn P_Move(actor: *mut mobj_t) -> Boolean {
    let mut ld: *mut line_t;

    let mut good: Boolean;

    if (*actor).movedir == DI_NODIR as c_int {
        return Boolean::FALSE;
    }
    if (*actor).movedir as c_uint >= 8 as c_uint {
        i_error!("Weird actor->movedir!");
    }
    let tryx: fixed_t = (*actor).x
        + (*((*actor).info as *mut MobjInfo)).speed as fixed_t * xspeed[(*actor).movedir as usize];
    let tryy: fixed_t = (*actor).y
        + (*((*actor).info as *mut MobjInfo)).speed as fixed_t * yspeed[(*actor).movedir as usize];
    let try_ok: Boolean = Boolean::from_raw(P_TryMove(actor as *mut CffiMobj, tryx, tryy));
    if try_ok.is_false() {
        if (*actor).flags & MF_FLOAT as c_int != 0 && floatok != 0 {
            if (*actor).z < tmfloorz {
                (*actor).z += FLOATSPEED;
            } else {
                (*actor).z -= FLOATSPEED;
            }
            (*actor).flags |= MF_INFLOAT as c_int;
            return Boolean::TRUE;
        }
        if numspechit == 0 {
            return Boolean::FALSE;
        }
        (*actor).movedir = DI_NODIR as c_int;
        good = Boolean::FALSE;
        loop {
            let c2rust_fresh0 = numspechit;
            numspechit -= 1;
            if c2rust_fresh0 == 0 {
                break;
            }
            ld = spechit[numspechit as usize];
            if P_UseSpecialLine(
                actor as *mut c_void,
                ld as *mut crate::doom::p_lights::line_t,
                0 as c_int,
            ) != 0
            {
                good = Boolean::TRUE;
            }
        }
        return good;
    } else {
        (*actor).flags &= !(MF_INFLOAT as c_int);
    }
    if (*actor).flags & MF_FLOAT as c_int == 0 {
        (*actor).z = (*actor).floorz;
    }
    Boolean::TRUE
}
#[no_mangle]
pub unsafe extern "C" fn P_TryWalk(actor: *mut mobj_t) -> Boolean {
    if P_Move(actor).is_false() {
        return Boolean::FALSE;
    }
    (*actor).movecount = P_Random() & 15 as c_int;
    Boolean::TRUE
}
#[no_mangle]
pub unsafe extern "C" fn P_NewChaseDir(actor: *mut mobj_t) {
    let mut d: [dirtype_t; 3] = [DI_EAST; 3];
    let mut tdir: c_int;

    if (*actor).target.is_null() {
        i_error!("P_NewChaseDir: called with no target");
    }
    let olddir: dirtype_t = (*actor).movedir as dirtype_t;
    let turnaround: dirtype_t = opposite[olddir as usize];
    let deltax: fixed_t = (*(*actor).target).x - (*actor).x;
    let deltay: fixed_t = (*(*actor).target).y - (*actor).y;
    if deltax > 10 as c_int * FRACUNIT {
        d[1 as c_int as usize] = DI_EAST;
    } else if deltax < -10 as c_int * FRACUNIT {
        d[1 as c_int as usize] = DI_WEST;
    } else {
        d[1 as c_int as usize] = DI_NODIR;
    }
    if deltay < -10 as c_int * FRACUNIT {
        d[2 as c_int as usize] = DI_SOUTH;
    } else if deltay > 10 as c_int * FRACUNIT {
        d[2 as c_int as usize] = DI_NORTH;
    } else {
        d[2 as c_int as usize] = DI_NODIR;
    }
    if d[1 as c_int as usize] as c_uint != DI_NODIR as c_int as c_uint
        && d[2 as c_int as usize] as c_uint != DI_NODIR as c_int as c_uint
    {
        (*actor).movedir = diags[((((deltay < 0 as c_int) as c_int) << 1 as c_int)
            + (deltax > 0 as c_int) as c_int) as usize] as c_int;
        if (*actor).movedir != turnaround as c_int && P_TryWalk(actor).is_truthy() {
            return;
        }
    }
    if P_Random() > 200 as c_int || (deltay as c_int).abs() > (deltax as c_int).abs() {
        tdir = d[1 as c_int as usize] as c_int;
        d[1 as c_int as usize] = d[2 as c_int as usize];
        d[2 as c_int as usize] = tdir as dirtype_t;
    }
    if d[1 as c_int as usize] as c_uint == turnaround as c_uint {
        d[1 as c_int as usize] = DI_NODIR;
    }
    if d[2 as c_int as usize] as c_uint == turnaround as c_uint {
        d[2 as c_int as usize] = DI_NODIR;
    }
    if d[1 as c_int as usize] as c_uint != DI_NODIR as c_int as c_uint {
        (*actor).movedir = d[1 as c_int as usize] as c_int;
        if P_TryWalk(actor).is_truthy() {
            return;
        }
    }
    if d[2 as c_int as usize] as c_uint != DI_NODIR as c_int as c_uint {
        (*actor).movedir = d[2 as c_int as usize] as c_int;
        if P_TryWalk(actor).is_truthy() {
            return;
        }
    }
    if olddir as c_uint != DI_NODIR as c_int as c_uint {
        (*actor).movedir = olddir as c_int;
        if P_TryWalk(actor).is_truthy() {
            return;
        }
    }
    if P_Random() & 1 as c_int != 0 {
        tdir = DI_EAST as c_int;
        while tdir <= DI_SOUTHEAST as c_int {
            if tdir != turnaround as c_int {
                (*actor).movedir = tdir;
                if P_TryWalk(actor).is_truthy() {
                    return;
                }
            }
            tdir += 1;
        }
    } else {
        tdir = DI_SOUTHEAST as c_int;
        while tdir != DI_EAST as c_int - 1 as c_int {
            if tdir != turnaround as c_int {
                (*actor).movedir = tdir;
                if P_TryWalk(actor).is_truthy() {
                    return;
                }
            }
            tdir -= 1;
        }
    }
    if turnaround as c_uint != DI_NODIR as c_int as c_uint {
        (*actor).movedir = turnaround as c_int;
        if P_TryWalk(actor).is_truthy() {
            return;
        }
    }
    (*actor).movedir = DI_NODIR as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_LookForPlayers(actor: *mut mobj_t, allaround: Boolean) -> Boolean {
    let mut c: c_int;

    let mut player: *mut PlayerT;

    let mut an: angle_t;

    let mut dist: fixed_t;

    c = 0 as c_int;
    let stop: c_int = ((*actor).lastlook - 1 as c_int) & 3 as c_int;
    loop {
        's_20: {
            if playeringame[(*actor).lastlook as usize] != 0 {
                let c2rust_fresh1 = c;
                c += 1;
                if c2rust_fresh1 == 2 as c_int || (*actor).lastlook == stop {
                    return Boolean::FALSE;
                }
                player = (&raw mut players as *mut PlayerT).offset((*actor).lastlook as isize);
                if (*player).health > 0 as c_int {
                    let sight = P_CheckSight(actor, (*player).mo as *mut mobj_t);
                    if sight != 0 {
                        if allaround.is_false() {
                            an = R_PointToAngle2(
                                (*actor).x,
                                (*actor).y,
                                (*((*player).mo as *mut mobj_t)).x,
                                (*((*player).mo as *mut mobj_t)).y,
                            )
                            .wrapping_sub((*actor).angle);
                            if an > ANG90 as angle_t && an < ANG270 {
                                dist = P_AproxDistance(
                                    (*((*player).mo as *mut mobj_t)).x - (*actor).x,
                                    (*((*player).mo as *mut mobj_t)).y - (*actor).y,
                                );
                                if dist > MELEERANGE {
                                    break 's_20;
                                }
                            }
                        }
                        (*actor).target = (*player).mo as *mut mobj_t;
                        return Boolean::TRUE;
                    }
                }
            }
        }
        (*actor).lastlook = ((*actor).lastlook + 1 as c_int) & 3 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_KeenDie(mo: *mut mobj_t) {
    let mut th: *mut thinker_t;

    let mut mo2: *mut mobj_t;

    let mut junk: line_t = line_t {
        v1: std::ptr::null_mut::<vertex_t>(),
        v2: std::ptr::null_mut::<vertex_t>(),
        dx: 0,
        dy: 0,
        flags: 0,
        special: 0,
        tag: 0,
        sidenum: [0; 2],
        bbox: [0; 4],
        slopetype: 0,
        frontsector: std::ptr::null_mut::<c_void>(),
        backsector: std::ptr::null_mut::<c_void>(),
        validcount: 0,
        specialdata: std::ptr::null_mut::<c_void>(),
    };
    A_Fall(mo);
    th = thinkercap.next;
    while !std::ptr::eq(th, &raw const thinkercap) {
        if (*th).function.acp1
            == core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut mobj_t) -> ()>,
                Option<unsafe extern "C" fn(*mut c_void) -> ()>,
            >(Some(
                P_MobjThinker as unsafe extern "C" fn(*mut mobj_t) -> (),
            ))
        {
            mo2 = th as *mut mobj_t;
            if mo2 != mo
                && (*mo2).mobjtype as c_uint == (*mo).mobjtype as c_uint
                && (*mo2).health > 0 as c_int
            {
                return;
            }
        }
        th = (*th).next;
    }
    junk.tag = 666 as c_short;
    EV_DoDoor(&mut junk as *mut line_t as *mut PLineThing, vld_open);
}
#[no_mangle]
pub unsafe extern "C" fn A_Look(actor: *mut mobj_t) {
    (*actor).threshold = 0 as c_int;
    let targ: *mut mobj_t = (*(*(*actor).subsector).sector).soundtarget as *mut mobj_t;
    '_seeyou: {
        if !targ.is_null() && (*targ).flags & MF_SHOOTABLE as c_int != 0 {
            (*actor).target = targ;
            if (*actor).flags & MF_AMBUSH as c_int != 0 {
                if P_CheckSight(actor, (*actor).target) != 0 {
                    break '_seeyou;
                }
            } else {
                break '_seeyou;
            }
        }
        if P_LookForPlayers(actor, Boolean::FALSE).is_false() {
            return;
        }
    }
    if (*((*actor).info as *mut MobjInfo)).seesound != 0 {
        let sound = match (*((*actor).info as *mut MobjInfo)).seesound {
            36..=38 => sfx_posit1 as c_int + P_Random() % 3 as c_int,
            39 | 40 => sfx_bgsit1 as c_int + P_Random() % 2 as c_int,
            s => s,
        };
        if (*actor).mobjtype as c_uint == MT_SPIDER as c_int as c_uint
            || (*actor).mobjtype as c_uint == MT_CYBORG as c_int as c_uint
        {
            S_StartSound(std::ptr::null_mut::<c_void>(), sound);
        } else {
            S_StartSound(actor as *mut c_void, sound);
        }
    }
    P_SetMobjState(
        actor,
        (*((*actor).info as *mut MobjInfo)).seestate as statenum_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn A_Chase(actor: *mut mobj_t) {
    let delta: c_int;

    if (*actor).reactiontime != 0 {
        (*actor).reactiontime -= 1;
    }
    if (*actor).threshold != 0 {
        if (*actor).target.is_null() || (*(*actor).target).health <= 0 as c_int {
            (*actor).threshold = 0 as c_int;
        } else {
            (*actor).threshold -= 1;
        }
    }
    if (*actor).movedir < 8 as c_int {
        (*actor).angle &= ((7 as c_int) << 29 as c_int) as angle_t;
        delta = (*actor)
            .angle
            .wrapping_sub(((*actor).movedir << 29 as c_int) as angle_t) as c_int;
        if delta > 0 as c_int {
            (*actor).angle = (*actor).angle.wrapping_sub((ANG90 / 2) as angle_t);
        } else if delta < 0 as c_int {
            (*actor).angle = (*actor).angle.wrapping_add((ANG90 / 2) as angle_t);
        }
    }
    if (*actor).target.is_null() || (*(*actor).target).flags & MF_SHOOTABLE as c_int == 0 {
        if P_LookForPlayers(actor, Boolean::TRUE).is_truthy() {
            return;
        }
        P_SetMobjState(
            actor,
            (*((*actor).info as *mut MobjInfo)).spawnstate as statenum_t,
        );
        return;
    }
    if (*actor).flags & MF_JUSTATTACKED as c_int != 0 {
        (*actor).flags &= !(MF_JUSTATTACKED as c_int);
        if gameskill as c_int != sk_nightmare as c_int && fastparm == 0 {
            P_NewChaseDir(actor);
        }
        return;
    }
    if (*((*actor).info as *mut MobjInfo)).meleestate != 0 && P_CheckMeleeRange(actor).is_truthy() {
        if (*((*actor).info as *mut MobjInfo)).attacksound != 0 {
            S_StartSound(
                actor as *mut c_void,
                (*((*actor).info as *mut MobjInfo)).attacksound,
            );
        }
        P_SetMobjState(
            actor,
            (*((*actor).info as *mut MobjInfo)).meleestate as statenum_t,
        );
        return;
    }
    if (*((*actor).info as *mut MobjInfo)).missilestate != 0
        && !((gameskill as c_int) < sk_nightmare as c_int
            && fastparm == 0
            && (*actor).movecount != 0)
        && P_CheckMissileRange(actor).is_truthy()
    {
        P_SetMobjState(
            actor,
            (*((*actor).info as *mut MobjInfo)).missilestate as statenum_t,
        );
        (*actor).flags |= MF_JUSTATTACKED as c_int;
        return;
    }
    if netgame != 0
        && (*actor).threshold == 0
        && P_CheckSight(actor, (*actor).target) == 0
        && P_LookForPlayers(actor, Boolean::TRUE).is_truthy()
    {
        return;
    }
    (*actor).movecount -= 1;
    if (*actor).movecount < 0 as c_int || P_Move(actor).is_false() {
        P_NewChaseDir(actor);
    }
    if (*((*actor).info as *mut MobjInfo)).activesound != 0 && P_Random() < 3 as c_int {
        S_StartSound(
            actor as *mut c_void,
            (*((*actor).info as *mut MobjInfo)).activesound,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_FaceTarget(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    (*actor).flags &= !(MF_AMBUSH as c_int);
    (*actor).angle = R_PointToAngle2(
        (*actor).x,
        (*actor).y,
        (*(*actor).target).x,
        (*(*actor).target).y,
    );
    if (*(*actor).target).flags & MF_SHADOW as c_int != 0 {
        (*actor).angle = (*actor)
            .angle
            .wrapping_add(((P_Random() - P_Random()) << 21 as c_int) as angle_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_PosAttack(actor: *mut mobj_t) {
    let mut angle: c_int;

    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    angle = (*actor).angle as c_int;
    let slope: c_int =
        P_AimLineAttack(actor as *mut CffiMobj, angle as angle_t, MISSILERANGE) as c_int;
    S_StartSound(actor as *mut c_void, sfx_pistol as c_int);
    angle = angle.wrapping_add((P_Random() - P_Random()) << 20 as c_int);
    let damage: c_int = (P_Random() % 5 as c_int + 1 as c_int) * 3 as c_int;
    P_LineAttack(
        actor as *mut CffiMobj,
        angle as angle_t,
        MISSILERANGE,
        slope as fixed_t,
        damage,
    );
}
#[no_mangle]
pub unsafe extern "C" fn A_SPosAttack(actor: *mut mobj_t) {
    let mut i: c_int;

    let mut angle: c_int;

    let mut damage: c_int;

    if (*actor).target.is_null() {
        return;
    }
    S_StartSound(actor as *mut c_void, sfx_shotgn as c_int);
    A_FaceTarget(actor);
    let bangle: c_int = (*actor).angle as c_int;
    let slope: c_int =
        P_AimLineAttack(actor as *mut CffiMobj, bangle as angle_t, MISSILERANGE) as c_int;
    i = 0 as c_int;
    while i < 3 as c_int {
        angle = bangle.wrapping_add((P_Random() - P_Random()) << 20 as c_int);
        damage = (P_Random() % 5 as c_int + 1 as c_int) * 3 as c_int;
        P_LineAttack(
            actor as *mut CffiMobj,
            angle as angle_t,
            MISSILERANGE,
            slope as fixed_t,
            damage,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_CPosAttack(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    S_StartSound(actor as *mut c_void, sfx_shotgn as c_int);
    A_FaceTarget(actor);
    let bangle: c_int = (*actor).angle as c_int;
    let slope: c_int =
        P_AimLineAttack(actor as *mut CffiMobj, bangle as angle_t, MISSILERANGE) as c_int;
    let angle: c_int = bangle.wrapping_add((P_Random() - P_Random()) << 20 as c_int);
    let damage: c_int = (P_Random() % 5 as c_int + 1 as c_int) * 3 as c_int;
    P_LineAttack(
        actor as *mut CffiMobj,
        angle as angle_t,
        MISSILERANGE,
        slope as fixed_t,
        damage,
    );
}
#[no_mangle]
pub unsafe extern "C" fn A_CPosRefire(actor: *mut mobj_t) {
    A_FaceTarget(actor);
    if P_Random() < 40 as c_int {
        return;
    }
    if (*actor).target.is_null()
        || (*(*actor).target).health <= 0 as c_int
        || P_CheckSight(actor, (*actor).target) == 0
    {
        P_SetMobjState(
            actor,
            (*((*actor).info as *mut MobjInfo)).seestate as statenum_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_SpidRefire(actor: *mut mobj_t) {
    A_FaceTarget(actor);
    if P_Random() < 10 as c_int {
        return;
    }
    if (*actor).target.is_null()
        || (*(*actor).target).health <= 0 as c_int
        || P_CheckSight(actor, (*actor).target) == 0
    {
        P_SetMobjState(
            actor,
            (*((*actor).info as *mut MobjInfo)).seestate as statenum_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_BspiAttack(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    P_SpawnMissile(actor, (*actor).target, MT_ARACHPLAZ);
}
#[no_mangle]
pub unsafe extern "C" fn A_TroopAttack(actor: *mut mobj_t) {
    let damage: c_int;

    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckMeleeRange(actor).is_truthy() {
        S_StartSound(actor as *mut c_void, sfx_claw as c_int);
        damage = (P_Random() % 8 as c_int + 1 as c_int) * 3 as c_int;
        P_DamageMobj((*actor).target, actor, actor, damage);
        return;
    }
    P_SpawnMissile(actor, (*actor).target, MT_TROOPSHOT);
}
#[no_mangle]
pub unsafe extern "C" fn A_SargAttack(actor: *mut mobj_t) {
    let damage: c_int;

    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckMeleeRange(actor).is_truthy() {
        damage = (P_Random() % 10 as c_int + 1 as c_int) * 4 as c_int;
        P_DamageMobj((*actor).target, actor, actor, damage);
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_HeadAttack(actor: *mut mobj_t) {
    let damage: c_int;

    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckMeleeRange(actor).is_truthy() {
        damage = (P_Random() % 6 as c_int + 1 as c_int) * 10 as c_int;
        P_DamageMobj((*actor).target, actor, actor, damage);
        return;
    }
    P_SpawnMissile(actor, (*actor).target, MT_HEADSHOT);
}
#[no_mangle]
pub unsafe extern "C" fn A_CyberAttack(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    P_SpawnMissile(actor, (*actor).target, MT_ROCKET);
}
#[no_mangle]
pub unsafe extern "C" fn A_BruisAttack(actor: *mut mobj_t) {
    let damage: c_int;

    if (*actor).target.is_null() {
        return;
    }
    if P_CheckMeleeRange(actor).is_truthy() {
        S_StartSound(actor as *mut c_void, sfx_claw as c_int);
        damage = (P_Random() % 8 as c_int + 1 as c_int) * 10 as c_int;
        P_DamageMobj((*actor).target, actor, actor, damage);
        return;
    }
    P_SpawnMissile(actor, (*actor).target, MT_BRUISERSHOT);
}
#[no_mangle]
pub unsafe extern "C" fn A_SkelMissile(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    (*actor).z += 16 as c_int * FRACUNIT;
    let mo: *mut mobj_t = P_SpawnMissile(actor, (*actor).target, MT_TRACER);
    (*actor).z -= 16 as c_int * FRACUNIT;
    (*mo).x += (*mo).momx;
    (*mo).y += (*mo).momy;
    (*mo).tracer = (*actor).target;
}
#[no_mangle]
pub static mut TRACEANGLE: c_int = 0xc000000 as c_int;
#[no_mangle]
pub unsafe extern "C" fn A_Tracer(actor: *mut mobj_t) {
    let mut exact: angle_t;

    let mut dist: fixed_t;

    if gametic & 3 as c_int != 0 {
        return;
    }
    P_SpawnPuff((*actor).x, (*actor).y, (*actor).z);
    let th: *mut mobj_t = P_SpawnMobj(
        (*actor).x - (*actor).momx,
        (*actor).y - (*actor).momy,
        (*actor).z,
        MT_SMOKE,
    );
    (*th).momz = FRACUNIT as fixed_t;
    (*th).tics -= P_Random() & 3 as c_int;
    if (*th).tics < 1 as c_int {
        (*th).tics = 1 as c_int;
    }
    let dest: *mut mobj_t = (*actor).tracer;
    if dest.is_null() || (*dest).health <= 0 as c_int {
        return;
    }
    exact = R_PointToAngle2((*actor).x, (*actor).y, (*dest).x, (*dest).y);
    if exact != (*actor).angle {
        if exact.wrapping_sub((*actor).angle) > 0x80000000 as c_uint {
            (*actor).angle = (*actor).angle.wrapping_sub(TRACEANGLE as angle_t);
            if exact.wrapping_sub((*actor).angle) < 0x80000000 as c_uint {
                (*actor).angle = exact;
            }
        } else {
            (*actor).angle = (*actor).angle.wrapping_add(TRACEANGLE as angle_t);
            if exact.wrapping_sub((*actor).angle) > 0x80000000 as c_uint {
                (*actor).angle = exact;
            }
        }
    }
    exact = (*actor).angle >> ANGLETOFINESHIFT;
    (*actor).momx = FixedMul(
        (*((*actor).info as *mut MobjInfo)).speed as fixed_t,
        *finecosine.0.add(exact as usize),
    );
    (*actor).momy = FixedMul(
        (*((*actor).info as *mut MobjInfo)).speed as fixed_t,
        finesine[exact as usize],
    );
    dist = P_AproxDistance((*dest).x - (*actor).x, (*dest).y - (*actor).y);
    dist = (dist as c_int / (*((*actor).info as *mut MobjInfo)).speed) as fixed_t;
    if dist < 1 as c_int {
        dist = 1 as c_int as fixed_t;
    }
    let slope: fixed_t = ((*dest).z + 40 as fixed_t * FRACUNIT - (*actor).z) / dist;
    if slope < (*actor).momz {
        (*actor).momz -= FRACUNIT / 8 as c_int;
    } else {
        (*actor).momz += FRACUNIT / 8 as c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn A_SkelWhoosh(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    S_StartSound(actor as *mut c_void, sfx_skeswg as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_SkelFist(actor: *mut mobj_t) {
    let damage: c_int;

    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckMeleeRange(actor).is_truthy() {
        damage = (P_Random() % 10 as c_int + 1 as c_int) * 6 as c_int;
        S_StartSound(actor as *mut c_void, sfx_skepch as c_int);
        P_DamageMobj((*actor).target, actor, actor, damage);
    }
}
#[no_mangle]
pub static mut corpsehit: *mut mobj_t = std::ptr::null_mut::<mobj_t>();
#[no_mangle]
pub static mut vileobj: *mut mobj_t = std::ptr::null_mut::<mobj_t>();
#[no_mangle]
pub static mut viletryx: fixed_t = 0;
#[no_mangle]
pub static mut viletryy: fixed_t = 0;
#[no_mangle]
pub unsafe extern "C" fn PIT_VileCheck(thing: *mut mobj_t) -> Boolean {
    if (*thing).flags & MF_CORPSE as c_int == 0 {
        return Boolean::TRUE;
    }
    if (*thing).tics != -1 as c_int {
        return Boolean::TRUE;
    }
    if (*((*thing).info as *mut MobjInfo)).raisestate == S_NULL as c_int {
        return Boolean::TRUE;
    }
    let maxdist: c_int =
        (*((*thing).info as *mut MobjInfo)).radius + mobjinfo[MT_VILE as c_int as usize].radius;
    if ((*thing).x as c_int - viletryx as c_int).abs() > maxdist
        || ((*thing).y as c_int - viletryy as c_int).abs() > maxdist
    {
        return Boolean::TRUE;
    }
    corpsehit = thing;
    (*corpsehit).momy = 0 as c_int as fixed_t;
    (*corpsehit).momx = (*corpsehit).momy;
    (*corpsehit).height <<= 2 as c_int;
    let check: Boolean = Boolean::from_raw(P_CheckPosition(
        corpsehit as *mut CffiMobj,
        (*corpsehit).x,
        (*corpsehit).y,
    ));
    (*corpsehit).height >>= 2 as c_int;
    if check.is_false() {
        return Boolean::TRUE;
    }
    Boolean::FALSE
}
#[no_mangle]
pub unsafe extern "C" fn A_VileChase(actor: *mut mobj_t) {
    let xl: c_int;

    let xh: c_int;

    let yl: c_int;

    let yh: c_int;

    let mut bx: c_int;

    let mut by: c_int;

    let info: *mut MobjInfo;

    let temp: *mut mobj_t;

    if (*actor).movedir != DI_NODIR as c_int {
        viletryx = (*actor).x
            + (*((*actor).info as *mut MobjInfo)).speed as fixed_t
                * xspeed[(*actor).movedir as usize];
        viletryy = (*actor).y
            + (*((*actor).info as *mut MobjInfo)).speed as fixed_t
                * yspeed[(*actor).movedir as usize];
        xl = (viletryx as c_int - bmaporgx as c_int - 32 as c_int * FRACUNIT * 2 as c_int)
            >> MAPBLOCKSHIFT;
        xh = (viletryx as c_int - bmaporgx as c_int + 32 as c_int * FRACUNIT * 2 as c_int)
            >> MAPBLOCKSHIFT;
        yl = (viletryy as c_int - bmaporgy as c_int - 32 as c_int * FRACUNIT * 2 as c_int)
            >> MAPBLOCKSHIFT;
        yh = (viletryy as c_int - bmaporgy as c_int + 32 as c_int * FRACUNIT * 2 as c_int)
            >> MAPBLOCKSHIFT;
        vileobj = actor;
        bx = xl;
        while bx <= xh {
            by = yl;
            while by <= yh {
                if P_BlockThingsIterator(
                    bx,
                    by,
                    Some(core::mem::transmute::<
                        unsafe extern "C" fn(*mut mobj_t) -> Boolean,
                        unsafe extern "C" fn(*mut CffiMobj) -> c_uint,
                    >(PIT_VileCheck)),
                ) == 0
                {
                    temp = (*actor).target;
                    (*actor).target = corpsehit;
                    A_FaceTarget(actor);
                    (*actor).target = temp;
                    P_SetMobjState(actor, S_VILE_HEAL1);
                    S_StartSound(corpsehit as *mut c_void, sfx_slop as c_int);
                    info = (*corpsehit).info as *mut MobjInfo;
                    P_SetMobjState(corpsehit, (*info).raisestate as statenum_t);
                    (*corpsehit).height <<= 2 as c_int;
                    (*corpsehit).flags = (*info).flags;
                    (*corpsehit).health = (*info).spawnhealth;
                    (*corpsehit).target = ::core::ptr::null_mut::<mobj_t>();
                    return;
                }
                by += 1;
            }
            bx += 1;
        }
    }
    A_Chase(actor);
}
#[no_mangle]
pub unsafe extern "C" fn A_VileStart(actor: *mut mobj_t) {
    S_StartSound(actor as *mut c_void, sfx_vilatk as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_StartFire(actor: *mut mobj_t) {
    S_StartSound(actor as *mut c_void, sfx_flamst as c_int);
    A_Fire(actor);
}
#[no_mangle]
pub unsafe extern "C" fn A_FireCrackle(actor: *mut mobj_t) {
    S_StartSound(actor as *mut c_void, sfx_flame as c_int);
    A_Fire(actor);
}
#[no_mangle]
pub unsafe extern "C" fn A_Fire(actor: *mut mobj_t) {
    let dest: *mut mobj_t = (*actor).tracer;
    if dest.is_null() {
        return;
    }
    let target: *mut mobj_t = P_SubstNullMobj((*actor).target);
    if P_CheckSight(target, dest) == 0 {
        return;
    }
    let an: c_uint = ((*dest).angle >> ANGLETOFINESHIFT) as c_uint;
    P_UnsetThingPosition(actor as *mut CffiMobj);
    (*actor).x = (*dest).x + FixedMul(24 as fixed_t * FRACUNIT, *finecosine.0.add(an as usize));
    (*actor).y = (*dest).y + FixedMul(24 as fixed_t * FRACUNIT, finesine[an as usize]);
    (*actor).z = (*dest).z;
    P_SetThingPosition(actor as *mut CffiMobj);
}
#[no_mangle]
pub unsafe extern "C" fn A_VileTarget(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    let fog: *mut mobj_t = P_SpawnMobj(
        (*(*actor).target).x,
        (*(*actor).target).x,
        (*(*actor).target).z,
        MT_FIRE,
    );
    (*actor).tracer = fog;
    (*fog).target = actor;
    (*fog).tracer = (*actor).target;
    A_Fire(fog);
}
#[no_mangle]
pub unsafe extern "C" fn A_VileAttack(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    if P_CheckSight(actor, (*actor).target) == 0 {
        return;
    }
    S_StartSound(actor as *mut c_void, sfx_barexp as c_int);
    P_DamageMobj((*actor).target, actor, actor, 20 as c_int);
    (*(*actor).target).momz =
        (1000 as c_int * FRACUNIT / (*((*(*actor).target).info as *mut MobjInfo)).mass) as fixed_t;
    let an: c_int = ((*actor).angle >> ANGLETOFINESHIFT) as c_int;
    let fire: *mut mobj_t = (*actor).tracer;
    if fire.is_null() {
        return;
    }
    (*fire).x =
        (*(*actor).target).x - FixedMul(24 as fixed_t * FRACUNIT, *finecosine.0.add(an as usize));
    (*fire).y = (*(*actor).target).y - FixedMul(24 as fixed_t * FRACUNIT, finesine[an as usize]);
    P_RadiusAttack(fire as *mut CffiMobj, actor as *mut CffiMobj, 70 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_FatRaise(actor: *mut mobj_t) {
    A_FaceTarget(actor);
    S_StartSound(actor as *mut c_void, sfx_manatk as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_FatAttack1(actor: *mut mobj_t) {
    A_FaceTarget(actor);
    (*actor).angle = (*actor).angle.wrapping_add(FATSPREAD as angle_t);
    let target: *mut mobj_t = P_SubstNullMobj((*actor).target);
    P_SpawnMissile(actor, target, MT_FATSHOT);
    let mo: *mut mobj_t = P_SpawnMissile(actor, target, MT_FATSHOT);
    (*mo).angle = (*mo).angle.wrapping_add(FATSPREAD as angle_t);
    let an: c_int = ((*mo).angle >> ANGLETOFINESHIFT) as c_int;
    (*mo).momx = FixedMul(
        (*((*mo).info as *mut MobjInfo)).speed as fixed_t,
        *finecosine.0.add(an as usize),
    );
    (*mo).momy = FixedMul(
        (*((*mo).info as *mut MobjInfo)).speed as fixed_t,
        finesine[an as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn A_FatAttack2(actor: *mut mobj_t) {
    A_FaceTarget(actor);
    (*actor).angle = (*actor).angle.wrapping_sub(FATSPREAD as angle_t);
    let target: *mut mobj_t = P_SubstNullMobj((*actor).target);
    P_SpawnMissile(actor, target, MT_FATSHOT);
    let mo: *mut mobj_t = P_SpawnMissile(actor, target, MT_FATSHOT);
    (*mo).angle = (*mo)
        .angle
        .wrapping_sub((FATSPREAD * 2 as c_int) as angle_t);
    let an: c_int = ((*mo).angle >> ANGLETOFINESHIFT) as c_int;
    (*mo).momx = FixedMul(
        (*((*mo).info as *mut MobjInfo)).speed as fixed_t,
        *finecosine.0.add(an as usize),
    );
    (*mo).momy = FixedMul(
        (*((*mo).info as *mut MobjInfo)).speed as fixed_t,
        finesine[an as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn A_FatAttack3(actor: *mut mobj_t) {
    let mut mo: *mut mobj_t;

    let mut an: c_int;

    A_FaceTarget(actor);
    let target: *mut mobj_t = P_SubstNullMobj((*actor).target);
    mo = P_SpawnMissile(actor, target, MT_FATSHOT);
    (*mo).angle = (*mo)
        .angle
        .wrapping_sub((FATSPREAD / 2 as c_int) as angle_t);
    an = ((*mo).angle >> ANGLETOFINESHIFT) as c_int;
    (*mo).momx = FixedMul(
        (*((*mo).info as *mut MobjInfo)).speed as fixed_t,
        *finecosine.0.add(an as usize),
    );
    (*mo).momy = FixedMul(
        (*((*mo).info as *mut MobjInfo)).speed as fixed_t,
        finesine[an as usize],
    );
    mo = P_SpawnMissile(actor, target, MT_FATSHOT);
    (*mo).angle = (*mo)
        .angle
        .wrapping_add((FATSPREAD / 2 as c_int) as angle_t);
    an = ((*mo).angle >> ANGLETOFINESHIFT) as c_int;
    (*mo).momx = FixedMul(
        (*((*mo).info as *mut MobjInfo)).speed as fixed_t,
        *finecosine.0.add(an as usize),
    );
    (*mo).momy = FixedMul(
        (*((*mo).info as *mut MobjInfo)).speed as fixed_t,
        finesine[an as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn A_SkullAttack(actor: *mut mobj_t) {
    let mut dist: c_int;

    if (*actor).target.is_null() {
        return;
    }
    let dest: *mut mobj_t = (*actor).target;
    (*actor).flags |= MF_SKULLFLY as c_int;
    S_StartSound(
        actor as *mut c_void,
        (*((*actor).info as *mut MobjInfo)).attacksound,
    );
    A_FaceTarget(actor);
    let an: angle_t = (*actor).angle >> ANGLETOFINESHIFT;
    (*actor).momx = FixedMul(SKULLSPEED, *finecosine.0.add(an as usize));
    (*actor).momy = FixedMul(SKULLSPEED, finesine[an as usize]);
    dist = P_AproxDistance((*dest).x - (*actor).x, (*dest).y - (*actor).y) as c_int;
    dist /= SKULLSPEED;
    if dist < 1 as c_int {
        dist = 1 as c_int;
    }
    (*actor).momz = (((*dest).z as c_int + ((*dest).height as c_int >> 1 as c_int)
        - (*actor).z as c_int)
        / dist) as fixed_t;
}
#[no_mangle]
pub unsafe extern "C" fn A_PainShootSkull(actor: *mut mobj_t, angle: angle_t) {
    let mut count: c_int;

    let mut currentthinker: *mut thinker_t;

    count = 0 as c_int;
    currentthinker = thinkercap.next;
    while !std::ptr::eq(currentthinker, &raw const thinkercap) {
        if (*currentthinker).function.acp1
            == core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut mobj_t) -> ()>,
                Option<unsafe extern "C" fn(*mut c_void) -> ()>,
            >(Some(
                P_MobjThinker as unsafe extern "C" fn(*mut mobj_t) -> (),
            ))
            && (*(currentthinker as *mut mobj_t)).mobjtype as c_uint == MT_SKULL as c_int as c_uint
        {
            count += 1;
        }
        currentthinker = (*currentthinker).next;
    }
    if count > 20 as c_int {
        return;
    }
    let an: angle_t = angle >> ANGLETOFINESHIFT;
    let prestep: c_int = 4 as c_int * FRACUNIT
        + 3 as c_int
            * ((*((*actor).info as *mut MobjInfo)).radius
                + mobjinfo[MT_SKULL as c_int as usize].radius)
            / 2 as c_int;
    let x: fixed_t = (*actor).x + FixedMul(prestep as fixed_t, *finecosine.0.add(an as usize));
    let y: fixed_t = (*actor).y + FixedMul(prestep as fixed_t, finesine[an as usize]);
    let z: fixed_t = ((*actor).z as c_int + 8 as c_int * FRACUNIT) as fixed_t;
    let newmobj: *mut mobj_t = P_SpawnMobj(x, y, z, MT_SKULL);
    if P_TryMove(newmobj as *mut CffiMobj, (*newmobj).x, (*newmobj).y) == 0 {
        P_DamageMobj(newmobj, actor, actor, 10000 as c_int);
        return;
    }
    (*newmobj).target = (*actor).target;
    A_SkullAttack(newmobj);
}
#[no_mangle]
pub unsafe extern "C" fn A_PainAttack(actor: *mut mobj_t) {
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor);
    A_PainShootSkull(actor, (*actor).angle);
}
#[no_mangle]
pub unsafe extern "C" fn A_PainDie(actor: *mut mobj_t) {
    A_Fall(actor);
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG90 as angle_t));
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG180));
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG270));
}
#[no_mangle]
pub unsafe extern "C" fn A_Scream(actor: *mut mobj_t) {
    let sound = match (*((*actor).info as *mut MobjInfo)).deathsound {
        0 => return,
        59..=61 => sfx_podth1 as c_int + P_Random() % 3 as c_int,
        62 | 63 => sfx_bgdth1 as c_int + P_Random() % 2 as c_int,
        s => s,
    };
    if (*actor).mobjtype as c_uint == MT_SPIDER as c_int as c_uint
        || (*actor).mobjtype as c_uint == MT_CYBORG as c_int as c_uint
    {
        S_StartSound(std::ptr::null_mut::<c_void>(), sound);
    } else {
        S_StartSound(actor as *mut c_void, sound);
    };
}
#[no_mangle]
pub unsafe extern "C" fn A_XScream(actor: *mut mobj_t) {
    S_StartSound(actor as *mut c_void, sfx_slop as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_Pain(actor: *mut mobj_t) {
    if (*((*actor).info as *mut MobjInfo)).painsound != 0 {
        S_StartSound(
            actor as *mut c_void,
            (*((*actor).info as *mut MobjInfo)).painsound,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_Fall(actor: *mut mobj_t) {
    (*actor).flags &= !(MF_SOLID as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_Explode(thingy: *mut mobj_t) {
    P_RadiusAttack(
        thingy as *mut CffiMobj,
        (*thingy).target as *mut CffiMobj,
        128 as c_int,
    );
}
unsafe extern "C" fn CheckBossEnd(motype: mobjtype_t) -> Boolean {
    if (gameversion as c_uint) < exe_ultimate as c_int as c_uint {
        if gamemap != 8 as c_int {
            return Boolean::FALSE;
        }
        if motype == MT_BRUISER && gameepisode != 1 as c_int {
            return Boolean::FALSE;
        }
        Boolean::TRUE
    } else {
        match gameepisode {
            1 => Boolean::from(gamemap == 8 as c_int && motype == MT_BRUISER),
            2 => Boolean::from(gamemap == 8 as c_int && motype == MT_CYBORG),
            3 => Boolean::from(gamemap == 8 as c_int && motype == MT_SPIDER),
            4 => Boolean::from(
                gamemap == 6 as c_int && motype == MT_CYBORG
                    || gamemap == 8 as c_int && motype == MT_SPIDER,
            ),
            _ => Boolean::from(gamemap == 8 as c_int),
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_BossDeath(mo: *mut mobj_t) {
    let mut th: *mut thinker_t;

    let mut mo2: *mut mobj_t;

    let mut junk: line_t = line_t {
        v1: std::ptr::null_mut::<vertex_t>(),
        v2: std::ptr::null_mut::<vertex_t>(),
        dx: 0,
        dy: 0,
        flags: 0,
        special: 0,
        tag: 0,
        sidenum: [0; 2],
        bbox: [0; 4],
        slopetype: 0,
        frontsector: std::ptr::null_mut::<c_void>(),
        backsector: std::ptr::null_mut::<c_void>(),
        validcount: 0,
        specialdata: std::ptr::null_mut::<c_void>(),
    };
    let _i: c_int = 0;
    if gamemode as c_uint == commercial as c_int as c_uint {
        if gamemap != 7 as c_int {
            return;
        }
        if (*mo).mobjtype as c_uint != MT_FATSO as c_int as c_uint
            && (*mo).mobjtype as c_uint != MT_BABY as c_int as c_uint
        {
            return;
        }
    } else if CheckBossEnd((*mo).mobjtype).is_false() {
        return;
    }
    let mut i: usize = 0;
    while i < MAXPLAYERS {
        if playeringame[i] != 0 && players[i].health > 0 as c_int {
            break;
        }
        i += 1;
    }
    if i == MAXPLAYERS {
        return;
    }
    th = thinkercap.next;
    while !std::ptr::eq(th, &raw const thinkercap) {
        if (*th).function.acp1
            == core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut mobj_t) -> ()>,
                Option<unsafe extern "C" fn(*mut c_void) -> ()>,
            >(Some(
                P_MobjThinker as unsafe extern "C" fn(*mut mobj_t) -> (),
            ))
        {
            mo2 = th as *mut mobj_t;
            if mo2 != mo
                && (*mo2).mobjtype as c_uint == (*mo).mobjtype as c_uint
                && (*mo2).health > 0 as c_int
            {
                return;
            }
        }
        th = (*th).next;
    }
    if gamemode as c_uint == commercial as c_int as c_uint {
        if gamemap == 7 as c_int {
            if (*mo).mobjtype as c_uint == MT_FATSO as c_int as c_uint {
                junk.tag = 666 as c_short;
                EV_DoFloor(
                    &mut junk as *mut line_t as *mut PLineThing,
                    lowerFloorToLowest,
                );
                return;
            }
            if (*mo).mobjtype as c_uint == MT_BABY as c_int as c_uint {
                junk.tag = 667 as c_short;
                EV_DoFloor(&mut junk as *mut line_t as *mut PLineThing, raiseToTexture);
                return;
            }
        }
    } else {
        match gameepisode {
            1 => {
                junk.tag = 666 as c_short;
                EV_DoFloor(
                    &mut junk as *mut line_t as *mut PLineThing,
                    lowerFloorToLowest,
                );
                return;
            }
            4 => match gamemap {
                6 => {
                    junk.tag = 666 as c_short;
                    EV_DoDoor(&mut junk as *mut line_t as *mut PLineThing, vld_blazeOpen);
                    return;
                }
                8 => {
                    junk.tag = 666 as c_short;
                    EV_DoFloor(
                        &mut junk as *mut line_t as *mut PLineThing,
                        lowerFloorToLowest,
                    );
                    return;
                }
                _ => {}
            },
            _ => {}
        }
    }
    G_ExitLevel();
}
#[no_mangle]
pub unsafe extern "C" fn A_Hoof(mo: *mut mobj_t) {
    S_StartSound(mo as *mut c_void, sfx_hoof as c_int);
    A_Chase(mo);
}
#[no_mangle]
pub unsafe extern "C" fn A_Metal(mo: *mut mobj_t) {
    S_StartSound(mo as *mut c_void, sfx_metal as c_int);
    A_Chase(mo);
}
#[no_mangle]
pub unsafe extern "C" fn A_BabyMetal(mo: *mut mobj_t) {
    S_StartSound(mo as *mut c_void, sfx_bspwlk as c_int);
    A_Chase(mo);
}
#[no_mangle]
pub unsafe extern "C" fn A_OpenShotgun2(player: *mut PlayerT, _psp: *mut PspdefT) {
    S_StartSound((*player).mo as *mut c_void, sfx_dbopn as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_LoadShotgun2(player: *mut PlayerT, _psp: *mut PspdefT) {
    S_StartSound((*player).mo as *mut c_void, sfx_dbload as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_CloseShotgun2(player: *mut PlayerT, psp: *mut PspdefT) {
    S_StartSound((*player).mo as *mut c_void, sfx_dbcls as c_int);
    A_ReFire(player, psp);
}
#[no_mangle]
pub static mut braintargets: [*mut mobj_t; 32] = [std::ptr::null_mut::<mobj_t>(); 32];
#[no_mangle]
pub static mut numbraintargets: c_int = 0;
#[no_mangle]
pub static mut braintargeton: c_int = 0 as c_int;
#[no_mangle]
pub unsafe extern "C" fn A_BrainAwake(_mo: *mut mobj_t) {
    let mut thinker: *mut thinker_t;

    let mut m: *mut mobj_t;

    numbraintargets = 0 as c_int;
    braintargeton = 0 as c_int;
    thinker = thinkercap.next;
    while !std::ptr::eq(thinker, &raw const thinkercap) {
        if (*thinker).function.acp1
            == core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut mobj_t) -> ()>,
                Option<unsafe extern "C" fn(*mut c_void) -> ()>,
            >(Some(
                P_MobjThinker as unsafe extern "C" fn(*mut mobj_t) -> (),
            ))
        {
            m = thinker as *mut mobj_t;
            if (*m).mobjtype as c_uint == MT_BOSSTARGET as c_int as c_uint {
                braintargets[numbraintargets as usize] = m;
                numbraintargets += 1;
            }
        }
        thinker = (*thinker).next;
    }
    S_StartSound(std::ptr::null_mut::<c_void>(), sfx_bossit as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_BrainPain(_mo: *mut mobj_t) {
    S_StartSound(std::ptr::null_mut::<c_void>(), sfx_bospn as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_BrainScream(mo: *mut mobj_t) {
    let mut x: c_int;

    let mut y: c_int;

    let mut z: c_int;

    let mut th: *mut mobj_t;

    x = (*mo).x as c_int - 196 as c_int * FRACUNIT;
    while x < (*mo).x as c_int + 320 as c_int * FRACUNIT {
        y = (*mo).y as c_int - 320 as c_int * FRACUNIT;
        z = 128 as c_int + P_Random() * 2 as c_int * FRACUNIT;
        th = P_SpawnMobj(x as fixed_t, y as fixed_t, z as fixed_t, MT_ROCKET);
        (*th).momz = (P_Random() * 512 as c_int) as fixed_t;
        P_SetMobjState(th, S_BRAINEXPLODE1);
        (*th).tics -= P_Random() & 7 as c_int;
        if (*th).tics < 1 as c_int {
            (*th).tics = 1 as c_int;
        }
        x += FRACUNIT * 8 as c_int;
    }
    S_StartSound(std::ptr::null_mut::<c_void>(), sfx_bosdth as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_BrainExplode(mo: *mut mobj_t) {
    let x: c_int = (*mo).x as c_int + (P_Random() - P_Random()) * 2048 as c_int;
    let y: c_int = (*mo).y as c_int;
    let z: c_int = 128 as c_int + P_Random() * 2 as c_int * FRACUNIT;
    let th: *mut mobj_t = P_SpawnMobj(x as fixed_t, y as fixed_t, z as fixed_t, MT_ROCKET);
    (*th).momz = (P_Random() * 512 as c_int) as fixed_t;
    P_SetMobjState(th, S_BRAINEXPLODE1);
    (*th).tics -= P_Random() & 7 as c_int;
    if (*th).tics < 1 as c_int {
        (*th).tics = 1 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_BrainDie(_mo: *mut mobj_t) {
    G_ExitLevel();
}
#[no_mangle]
pub unsafe extern "C" fn A_BrainSpit(mo: *mut mobj_t) {
    static mut easy: c_int = 0 as c_int;
    easy ^= 1 as c_int;
    if gameskill as c_int <= sk_easy as c_int && easy == 0 {
        return;
    }
    let targ: *mut mobj_t = braintargets[braintargeton as usize];
    braintargeton = (braintargeton + 1 as c_int) % numbraintargets;
    let newmobj: *mut mobj_t = P_SpawnMissile(mo, targ, MT_SPAWNSHOT);
    (*newmobj).target = targ;
    (*newmobj).reactiontime = ((*targ).y as c_int - (*mo).y as c_int)
        / (*newmobj).momy as c_int
        / (*((*newmobj).state as *mut State)).tics;
    S_StartSound(std::ptr::null_mut::<c_void>(), sfx_bospit as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_SpawnSound(mo: *mut mobj_t) {
    S_StartSound(mo as *mut c_void, sfx_boscub as c_int);
    A_SpawnFly(mo);
}
#[no_mangle]
pub unsafe extern "C" fn A_SpawnFly(mo: *mut mobj_t) {
    let type_0: mobjtype_t;

    (*mo).reactiontime -= 1;
    if (*mo).reactiontime != 0 {
        return;
    }
    let targ: *mut mobj_t = P_SubstNullMobj((*mo).target);
    let fog: *mut mobj_t = P_SpawnMobj((*targ).x, (*targ).y, (*targ).z, MT_SPAWNFIRE);
    S_StartSound(fog as *mut c_void, sfx_telept as c_int);
    let r: c_int = P_Random();
    if r < 50 as c_int {
        type_0 = MT_TROOP;
    } else if r < 90 as c_int {
        type_0 = MT_SERGEANT;
    } else if r < 120 as c_int {
        type_0 = MT_SHADOWS;
    } else if r < 130 as c_int {
        type_0 = MT_PAIN;
    } else if r < 160 as c_int {
        type_0 = MT_HEAD;
    } else if r < 162 as c_int {
        type_0 = MT_VILE;
    } else if r < 172 as c_int {
        type_0 = MT_UNDEAD;
    } else if r < 192 as c_int {
        type_0 = MT_BABY;
    } else if r < 222 as c_int {
        type_0 = MT_FATSO;
    } else if r < 246 as c_int {
        type_0 = MT_KNIGHT;
    } else {
        type_0 = MT_BRUISER;
    }
    let newmobj: *mut mobj_t = P_SpawnMobj((*targ).x, (*targ).y, (*targ).z, type_0);
    if P_LookForPlayers(newmobj, Boolean::TRUE).is_truthy() {
        P_SetMobjState(
            newmobj,
            (*((*newmobj).info as *mut MobjInfo)).seestate as statenum_t,
        );
    }
    P_TeleportMove(newmobj as *mut CffiMobj, (*newmobj).x, (*newmobj).y);
    P_RemoveMobj(mo);
}
#[no_mangle]
pub unsafe extern "C" fn A_PlayerScream(mo: *mut mobj_t) {
    let mut sound: c_int = sfx_pldeth as c_int;
    if gamemode as c_uint == commercial as c_int as c_uint && (*mo).health < -50 as c_int {
        sound = sfx_pdiehi as c_int;
    }
    S_StartSound(mo as *mut c_void, sound);
}
