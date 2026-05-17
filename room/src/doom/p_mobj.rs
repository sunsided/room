//! Rust port of vendor/doomgeneric/p_mobj.c.
//!
//! Map object (mobj) creation, movement, spawning, and thinker.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use crate::i_error;
use std::ffi::c_void;
use std::os::raw::c_int;
use std::ptr;

use crate::doom::c_ffi::ITEMQUESIZE;
use crate::doom::d_player::{PlayerT, CF_NOMOMENTUM, MAXPLAYERS};
use crate::doom::hu_stuff::HU_Start;
use crate::doom::i_timer::TICRATE;
use crate::doom::info::{self, *};
use crate::doom::m_fixed::{FixedMul, FRACBITS, FRACUNIT};
use crate::doom::m_random::P_Random;
use crate::doom::p_maputl::{P_AproxDistance, P_SetThingPosition, P_UnsetThingPosition};
use crate::doom::p_pspr::P_SetupPsprites;
// deathmatch_p, deathmatchstarts, playerstarts are accessed via extern "C"
// below to avoid type mismatches between p_setup::mapthing_t and
// p_telept::mapthing_t.
use crate::doom::p_telept::{line_t, mapthing_t, mobj_t, subsector_t};
use crate::doom::p_tick::{actionf_t, thinker_t, P_AddThinker, P_RemoveThinker};
use crate::doom::r_main::{R_PointInSubsector, R_PointToAngle2};
use crate::doom::s_sound::{MobjStub, S_StartSound, S_StopSound};
use crate::doom::tables::{finecosine, finesine, ANG45, ANGLETOFINESHIFT};
use crate::doom::z_zone::PU_LEVEL;

const STOPSPEED: c_int = 0x1000;
const FRICTION: c_int = 0xe800;
const MAXMOVE: c_int = 30 * FRACUNIT;
const GRAVITY: c_int = FRACUNIT;
const FLOATSPEED: c_int = FRACUNIT * 4;
const ONFLOORZ: c_int = i32::MIN;
const ONCEILINGZ: c_int = i32::MAX;
const VIEWHEIGHT: c_int = 41 * FRACUNIT;
const MELEERANGE: c_int = 64 * FRACUNIT;

const MTF_AMBUSH: c_int = 8;

const sfx_oof: c_int = 34;
const sfx_telept: c_int = 35;
const sfx_itmbk: c_int = 90;

const PST_LIVE: c_int = 0;
const PST_REBORN: c_int = 2;

const exe_ultimate: c_int = 6;

use crate::doom::d_main::nomonsters;
use crate::doom::doomstat::gameversion;
use crate::doom::g_game::G_PlayerReborn;
use crate::doom::g_game::{
    consoleplayer, deathmatch, gameskill, netgame, playeringame, players, respawnmonsters,
    totalitems, totalkills,
};
use crate::doom::p_map::{
    attackrange, ceilingline, linetarget, P_AimLineAttack, P_CheckPosition, P_SlideMove, P_TryMove,
};
use crate::doom::p_setup::{deathmatch_p, deathmatchstarts, playerstarts};
use crate::doom::p_tick::leveltime;
use crate::doom::r_sky::skyflatnum;
use crate::doom::st_stuff::ST_Start;
use crate::doom::z_zone::Z_Malloc;

// Type aliases for cross-module pointer casts (all #[repr(C)] identical layouts).
type CffiMobj = crate::doom::c_ffi::mobj_t;
type SetupMapThing = crate::doom::p_setup::mapthing_t;

#[no_mangle]
pub static mut itemrespawnque: [mapthing_t; ITEMQUESIZE] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    r#type: 0,
    options: 0,
}; ITEMQUESIZE];

#[no_mangle]
pub static mut itemrespawntime: [c_int; ITEMQUESIZE] = [0; ITEMQUESIZE];

#[no_mangle]
pub static mut iquehead: c_int = 0;

#[no_mangle]
pub static mut iquetail: c_int = 0;

fn sentinel_ac() -> Option<unsafe extern "C" fn()> {
    Some(unsafe { core::mem::transmute::<usize, unsafe extern "C" fn()>(usize::MAX) })
}

fn is_sentinel(f: actionf_t) -> bool {
    unsafe { f.acv == sentinel_ac() }
}

#[no_mangle]
pub unsafe extern "C" fn P_SetMobjState(mobj: *mut mobj_t, state: c_int) -> c_int {
    let mobj = &mut *mobj;
    let mut state = state;
    loop {
        if state == S_NULL {
            mobj.state = S_NULL as *mut State as *mut crate::doom::p_telept::state_t;
            P_RemoveMobj(mobj as *mut mobj_t);
            return 0;
        }
        let st = &mut info::states[state as usize] as *mut State;
        mobj.state = st as *mut crate::doom::p_telept::state_t;
        mobj.tics = (*st).tics;
        mobj.sprite = (*st).sprite;
        mobj.frame = (*st).frame;
        if let Some(action) = (*st).action {
            let action: unsafe extern "C" fn(*mut c_void) = core::mem::transmute(action);
            action(mobj as *mut mobj_t as *mut c_void);
        }
        state = (*st).nextstate;
        if mobj.tics != 0 {
            break;
        }
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn P_ExplodeMissile(mo: *mut mobj_t) {
    let mo = &mut *mo;
    let info = mo.info as *mut MobjInfo;
    mo.momx = 0;
    mo.momy = 0;
    mo.momz = 0;
    P_SetMobjState(mo as *mut mobj_t, (*info).deathstate);
    mo.tics -= P_Random() & 3;
    if mo.tics < 1 {
        mo.tics = 1;
    }
    mo.flags &= !MF_MISSILE;
    if (*info).deathsound != 0 {
        S_StartSound(mo as *mut mobj_t as *mut c_void, (*info).deathsound);
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_XYMovement(mo: *mut mobj_t) {
    let mo = &mut *mo;
    let mut ptryx: c_int;
    let mut ptryy: c_int;
    let player = mo.player as *mut PlayerT;
    let mut xmove: c_int;
    let mut ymove: c_int;

    if mo.momx == 0 && mo.momy == 0 {
        if mo.flags & MF_SKULLFLY != 0 {
            let info = mo.info as *mut MobjInfo;
            mo.flags &= !MF_SKULLFLY;
            mo.momx = 0;
            mo.momy = 0;
            mo.momz = 0;
            P_SetMobjState(mo as *mut mobj_t, (*info).spawnstate);
        }
        return;
    }

    mo.momx = mo.momx.clamp(-MAXMOVE, MAXMOVE);
    mo.momy = mo.momy.clamp(-MAXMOVE, MAXMOVE);

    xmove = mo.momx;
    ymove = mo.momy;

    loop {
        if xmove > MAXMOVE / 2 || ymove > MAXMOVE / 2 {
            ptryx = mo.x + xmove / 2;
            ptryy = mo.y + ymove / 2;
            xmove >>= 1;
            ymove >>= 1;
        } else {
            ptryx = mo.x + xmove;
            ptryy = mo.y + ymove;
            xmove = 0;
            ymove = 0;
        }

        if P_TryMove(mo as *mut _ as *mut CffiMobj, ptryx, ptryy) == 0 {
            if !mo.player.is_null() {
                P_SlideMove(mo as *mut _ as *mut CffiMobj);
            } else if mo.flags & MF_MISSILE != 0 {
                let cl = ceilingline;
                if !cl.is_null() {
                    let line = cl as *mut line_t;
                    if !(*line).backsector.is_null()
                        && (*(*line).backsector).ceilingpic == skyflatnum as i16
                    {
                        P_RemoveMobj(mo as *mut mobj_t);
                        return;
                    }
                }
                P_ExplodeMissile(mo as *mut mobj_t);
            } else {
                mo.momx = 0;
                mo.momy = 0;
            }
        }
        if xmove == 0 && ymove == 0 {
            break;
        }
    }

    if !player.is_null() && (*player).cheats & CF_NOMOMENTUM != 0 {
        mo.momx = 0;
        mo.momy = 0;
        return;
    }

    if mo.flags & (MF_MISSILE | MF_SKULLFLY) != 0 {
        return;
    }

    if mo.z > mo.floorz {
        return;
    }

    if mo.flags & MF_CORPSE != 0
        && (mo.momx > FRACUNIT / 4
            || mo.momx < -FRACUNIT / 4
            || mo.momy > FRACUNIT / 4
            || mo.momy < -FRACUNIT / 4)
        && mo.floorz != (*(*mo.subsector).sector).floorheight
    {
        return;
    }

    if mo.momx > -STOPSPEED
        && mo.momx < STOPSPEED
        && mo.momy > -STOPSPEED
        && mo.momy < STOPSPEED
        && (player.is_null() || ((*player).cmd.forwardmove == 0 && (*player).cmd.sidemove == 0))
    {
        if !player.is_null() {
            let state_ptr = mo.state as *mut State;
            let states_ptr = std::ptr::addr_of!(info::states) as *const State;
            let state_idx =
                (state_ptr as usize - states_ptr as usize) / std::mem::size_of::<State>();
            let run_offset = state_idx as c_int - S_PLAY_RUN1;
            if (0..4).contains(&run_offset) {
                P_SetMobjState(mo as *mut mobj_t, S_PLAY);
            }
        }
        mo.momx = 0;
        mo.momy = 0;
    } else {
        mo.momx = FixedMul(mo.momx, FRICTION);
        mo.momy = FixedMul(mo.momy, FRICTION);
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_ZMovement(mo: *mut mobj_t) {
    let mo = &mut *mo;
    let dist: c_int;
    let delta: c_int;

    if !mo.player.is_null() && mo.z < mo.floorz {
        let player = mo.player as *mut PlayerT;
        (*player).viewheight -= mo.floorz - mo.z;
        (*player).deltaviewheight = (VIEWHEIGHT - (*player).viewheight) >> 3;
    }

    mo.z += mo.momz;

    if mo.flags & MF_FLOAT != 0
        && !mo.target.is_null()
        && mo.flags & MF_SKULLFLY == 0
        && mo.flags & MF_INFLOAT == 0
    {
        let target = mo.target;
        dist = P_AproxDistance(mo.x - (*target).x, mo.y - (*target).y);
        delta = ((*target).z + (mo.height >> 1)) - mo.z;
        if delta < 0 && dist < -(delta * 3) {
            mo.z -= FLOATSPEED;
        } else if delta > 0 && dist < delta * 3 {
            mo.z += FLOATSPEED;
        }
    }

    if mo.z <= mo.floorz {
        let correct_lost_soul_bounce = gameversion >= exe_ultimate;
        if correct_lost_soul_bounce && mo.flags & MF_SKULLFLY != 0 {
            mo.momz = -mo.momz;
        }
        if mo.momz < 0 {
            if !mo.player.is_null() && mo.momz < -GRAVITY * 8 {
                let player = mo.player as *mut PlayerT;
                (*player).deltaviewheight = mo.momz >> 3;
                S_StartSound(mo as *mut mobj_t as *mut c_void, sfx_oof);
            }
            mo.momz = 0;
        }
        mo.z = mo.floorz;
        if !correct_lost_soul_bounce && mo.flags & MF_SKULLFLY != 0 {
            mo.momz = -mo.momz;
        }
        if mo.flags & MF_MISSILE != 0 && mo.flags & MF_NOCLIP == 0 {
            P_ExplodeMissile(mo as *mut mobj_t);
            return;
        }
    } else if mo.flags & MF_NOGRAVITY == 0 {
        if mo.momz == 0 {
            mo.momz = -GRAVITY * 2;
        } else {
            mo.momz -= GRAVITY;
        }
    }

    if mo.z + mo.height > mo.ceilingz {
        if mo.momz > 0 {
            mo.momz = 0;
        }
        mo.z = mo.ceilingz - mo.height;
        if mo.flags & MF_SKULLFLY != 0 {
            mo.momz = -mo.momz;
        }
        if mo.flags & MF_MISSILE != 0 && mo.flags & MF_NOCLIP == 0 {
            P_ExplodeMissile(mo as *mut mobj_t);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_NightmareRespawn(mobj: *mut mobj_t) {
    let mobj = &mut *mobj;
    let x = (mobj.spawnpoint.x as c_int) << FRACBITS;
    let y = (mobj.spawnpoint.y as c_int) << FRACBITS;
    if P_CheckPosition(mobj as *mut _ as *mut CffiMobj, x, y) == 0 {
        return;
    }

    let mut mo = P_SpawnMobj(
        mobj.x,
        mobj.y,
        (*(*mobj.subsector).sector).floorheight,
        MT_TFOG,
    );
    S_StartSound(mo as *mut c_void, sfx_telept);

    let ss = R_PointInSubsector(x, y) as *mut subsector_t;
    mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, MT_TFOG);
    S_StartSound(mo as *mut c_void, sfx_telept);

    let mthing = &mobj.spawnpoint;
    let info = mobj.info as *mut MobjInfo;
    let z = if (*info).flags & MF_SPAWNCEILING != 0 {
        ONCEILINGZ
    } else {
        ONFLOORZ
    };

    mo = P_SpawnMobj(x, y, z, mobj.mobjtype);
    (*mo).spawnpoint = mobj.spawnpoint;
    (*mo).angle = ANG45.wrapping_mul((mthing.angle as u32) / 45);
    if mthing.options & MTF_AMBUSH as i16 != 0 {
        (*mo).flags |= MF_AMBUSH;
    }
    (*mo).reactiontime = 18;
    P_RemoveMobj(mobj as *mut mobj_t);
}

#[no_mangle]
pub unsafe extern "C" fn P_MobjThinker(mobj: *mut mobj_t) {
    let mobj = &mut *mobj;
    if mobj.momx != 0 || mobj.momy != 0 || mobj.flags & MF_SKULLFLY != 0 {
        P_XYMovement(mobj as *mut mobj_t);
        if is_sentinel(mobj.thinker.function) {
            return;
        }
    }
    if mobj.z != mobj.floorz || mobj.momz != 0 {
        P_ZMovement(mobj as *mut mobj_t);
        if is_sentinel(mobj.thinker.function) {
            return;
        }
    }

    if mobj.tics != -1 {
        mobj.tics -= 1;
        if mobj.tics == 0 {
            let state_ptr = mobj.state as *mut State;
            P_SetMobjState(mobj as *mut mobj_t, (*state_ptr).nextstate);
        }
    } else {
        if mobj.flags & MF_COUNTKILL == 0 {
            return;
        }
        if respawnmonsters == 0 {
            return;
        }
        mobj.movecount += 1;
        if mobj.movecount < 12 * TICRATE {
            return;
        }
        if leveltime & 31 != 0 {
            return;
        }
        if P_Random() > 4 {
            return;
        }
        P_NightmareRespawn(mobj as *mut mobj_t);
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnMobj(x: c_int, y: c_int, z: c_int, type_: c_int) -> *mut mobj_t {
    let mobj = Z_Malloc(
        std::mem::size_of::<mobj_t>() as c_int,
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut mobj_t;
    // Z_Malloc already zeroes the allocation internally.
    let mobj_ref = &mut *mobj;
    let info = &mut info::mobjinfo[type_ as usize] as *mut MobjInfo;

    mobj_ref.mobjtype = type_;
    mobj_ref.info = info as *mut crate::doom::p_telept::mobjinfo_t;
    mobj_ref.x = x;
    mobj_ref.y = y;
    mobj_ref.radius = (*info).radius;
    mobj_ref.height = (*info).height;
    mobj_ref.flags = (*info).flags;
    mobj_ref.health = (*info).spawnhealth;

    if gameskill != 4 {
        // sk_nightmare = 4
        mobj_ref.reactiontime = (*info).reactiontime;
    }

    mobj_ref.lastlook = P_Random() % MAXPLAYERS as c_int;

    let st = &mut info::states[(*info).spawnstate as usize] as *mut State;
    mobj_ref.state = st as *mut crate::doom::p_telept::state_t;
    mobj_ref.tics = (*st).tics;
    mobj_ref.sprite = (*st).sprite;
    mobj_ref.frame = (*st).frame;

    P_SetThingPosition(mobj as *mut crate::doom::c_ffi::mobj_t);

    mobj_ref.floorz = (*(*mobj_ref.subsector).sector).floorheight;
    mobj_ref.ceilingz = (*(*mobj_ref.subsector).sector).ceilingheight;

    if z == ONFLOORZ {
        mobj_ref.z = mobj_ref.floorz;
    } else if z == ONCEILINGZ {
        mobj_ref.z = mobj_ref.ceilingz - (*info).height;
    } else {
        mobj_ref.z = z;
    }

    mobj_ref.thinker.function.acp1 = Some(core::mem::transmute::<
        unsafe extern "C" fn(*mut mobj_t),
        unsafe extern "C" fn(*mut c_void),
    >(P_MobjThinker));

    P_AddThinker(&mut mobj_ref.thinker as *mut thinker_t);

    mobj
}

#[no_mangle]
pub unsafe extern "C" fn P_RemoveMobj(mobj: *mut mobj_t) {
    let mobj = &mut *mobj;
    if mobj.flags & MF_SPECIAL != 0
        && mobj.flags & MF_DROPPED == 0
        && mobj.mobjtype != MT_INV
        && mobj.mobjtype != MT_INS
    {
        itemrespawnque[iquehead as usize] = mobj.spawnpoint;
        itemrespawntime[iquehead as usize] = leveltime;
        iquehead = (iquehead + 1) & (ITEMQUESIZE as c_int - 1);
        if iquehead == iquetail {
            iquetail = (iquetail + 1) & (ITEMQUESIZE as c_int - 1);
        }
    }

    P_UnsetThingPosition(mobj as *mut mobj_t as *mut crate::doom::c_ffi::mobj_t);
    S_StopSound(mobj as *mut mobj_t as *mut MobjStub);
    P_RemoveThinker(&mut mobj.thinker as *mut thinker_t);
}

#[no_mangle]
pub unsafe extern "C" fn P_RespawnSpecials() {
    if deathmatch != 2 {
        return;
    }
    if iquehead == iquetail {
        return;
    }
    if leveltime - itemrespawntime[iquetail as usize] < 30 * TICRATE {
        return;
    }

    let mthing = &itemrespawnque[iquetail as usize];
    let x = (mthing.x as c_int) << FRACBITS;
    let y = (mthing.y as c_int) << FRACBITS;

    let ss = R_PointInSubsector(x, y) as *mut subsector_t;
    let mut mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, MT_IFOG);
    S_StartSound(mo as *mut c_void, sfx_itmbk);

    let mut i = 0;
    while i < NUMMOBJTYPES {
        if mthing.r#type as c_int == info::mobjinfo[i].doomednum {
            break;
        }
        i += 1;
    }

    let z = if info::mobjinfo[i].flags & MF_SPAWNCEILING != 0 {
        ONCEILINGZ
    } else {
        ONFLOORZ
    };

    mo = P_SpawnMobj(x, y, z, i as c_int);
    (*mo).spawnpoint = *mthing;
    (*mo).angle = ANG45.wrapping_mul((mthing.angle as u32) / 45);

    iquetail = (iquetail + 1) & (ITEMQUESIZE as c_int - 1);
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnPlayer(mthing: *mut mapthing_t) {
    let mthing = &mut *mthing;
    if mthing.r#type as c_int == 0 {
        return;
    }
    if playeringame[(mthing.r#type - 1) as usize] == 0 {
        return;
    }

    let p = &mut players[(mthing.r#type - 1) as usize] as *mut PlayerT;

    if (*p).playerstate == PST_REBORN {
        G_PlayerReborn(mthing.r#type as c_int - 1);
    }

    let x = (mthing.x as c_int) << FRACBITS;
    let y = (mthing.y as c_int) << FRACBITS;
    let z = ONFLOORZ;
    let mobj = P_SpawnMobj(x, y, z, MT_PLAYER);

    if mthing.r#type > 1 {
        (*mobj).flags |= ((mthing.r#type - 1) as c_int) << MF_TRANSSHIFT;
    }

    (*mobj).angle = ANG45.wrapping_mul((mthing.angle as u32) / 45);
    (*mobj).player = p as *mut crate::doom::p_telept::player_s;
    (*mobj).health = (*p).health;

    (*p).mo = mobj as *mut crate::doom::d_player::mobj_t;
    (*p).playerstate = PST_LIVE;
    (*p).refire = 0;
    (*p).message = ptr::null_mut();
    (*p).damagecount = 0;
    (*p).bonuscount = 0;
    (*p).extralight = 0;
    (*p).fixedcolormap = 0;
    (*p).viewheight = VIEWHEIGHT;

    P_SetupPsprites(p);

    if deathmatch != 0 {
        for i in 0..crate::doom::d_player::NUMCARDS {
            (*p).cards[i] = 1;
        }
    }

    if mthing.r#type as c_int - 1 == consoleplayer {
        ST_Start();
        HU_Start();
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnMapThing(mthing: *mut mapthing_t) {
    let mthing = &mut *mthing;
    let mut i: usize;

    if mthing.r#type as c_int == 11 {
        if deathmatch_p < std::ptr::addr_of_mut!(deathmatchstarts[0]).add(10) {
            *deathmatch_p = *(mthing as *mut _ as *mut SetupMapThing);
            deathmatch_p = deathmatch_p.add(1);
        }
        return;
    }

    if mthing.r#type as c_int <= 0 {
        return;
    }

    if mthing.r#type as c_int <= 4 {
        let ps = std::ptr::addr_of_mut!(playerstarts[0]) as *mut mapthing_t;
        *ps.add((mthing.r#type - 1) as usize) = *mthing;
        if deathmatch == 0 {
            P_SpawnPlayer(mthing as *mut mapthing_t);
        }
        return;
    }

    if netgame == 0 && mthing.options & 16i16 != 0 {
        return;
    }

    let bit = if gameskill == 0 {
        // sk_baby
        1
    } else if gameskill == 4 {
        // sk_nightmare
        4
    } else {
        1 << (gameskill - 1)
    };

    if mthing.options & bit as i16 == 0 {
        return;
    }

    i = 0;
    while i < NUMMOBJTYPES {
        if mthing.r#type as c_int == info::mobjinfo[i].doomednum {
            break;
        }
        i += 1;
    }

    if i == NUMMOBJTYPES {
        i_error!(
            "P_SpawnMapThing: Unknown type {} at ({}, {})",
            mthing.r#type as c_int,
            mthing.x as c_int,
            mthing.y as c_int
        );
    }

    if deathmatch != 0 && info::mobjinfo[i].flags & MF_NOTDMATCH != 0 {
        return;
    }

    if nomonsters != 0 && (i == MT_SKULL as usize || info::mobjinfo[i].flags & MF_COUNTKILL != 0) {
        return;
    }

    let x = (mthing.x as c_int) << FRACBITS;
    let y = (mthing.y as c_int) << FRACBITS;

    let z = if info::mobjinfo[i].flags & MF_SPAWNCEILING != 0 {
        ONCEILINGZ
    } else {
        ONFLOORZ
    };

    let mobj = P_SpawnMobj(x, y, z, i as c_int);
    (*mobj).spawnpoint = *mthing;

    if (*mobj).tics > 0 {
        (*mobj).tics = 1 + (P_Random() % (*mobj).tics);
    }
    if (*mobj).flags & MF_COUNTKILL != 0 {
        totalkills += 1;
    }
    if (*mobj).flags & MF_COUNTITEM != 0 {
        totalitems += 1;
    }

    (*mobj).angle = ANG45.wrapping_mul((mthing.angle as u32) / 45);
    if mthing.options & MTF_AMBUSH as i16 != 0 {
        (*mobj).flags |= MF_AMBUSH;
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnPuff(x: c_int, y: c_int, z: c_int) {
    let z = z + ((P_Random() - P_Random()) << 10);
    let th = P_SpawnMobj(x, y, z, MT_PUFF);
    (*th).momz = FRACUNIT;
    (*th).tics -= P_Random() & 3;
    if (*th).tics < 1 {
        (*th).tics = 1;
    }
    if attackrange == MELEERANGE {
        P_SetMobjState(th, S_PUFF3);
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnBlood(x: c_int, y: c_int, z: c_int, damage: c_int) {
    let z = z + ((P_Random() - P_Random()) << 10);
    let th = P_SpawnMobj(x, y, z, MT_BLOOD);
    (*th).momz = FRACUNIT * 2;
    (*th).tics -= P_Random() & 3;
    if (*th).tics < 1 {
        (*th).tics = 1;
    }
    if (9..=12).contains(&damage) {
        P_SetMobjState(th, S_BLOOD2);
    } else if damage < 9 {
        P_SetMobjState(th, S_BLOOD3);
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_CheckMissileSpawn(th: *mut mobj_t) {
    let th = &mut *th;
    th.tics -= P_Random() & 3;
    if th.tics < 1 {
        th.tics = 1;
    }
    th.x += th.momx >> 1;
    th.y += th.momy >> 1;
    th.z += th.momz >> 1;
    if P_TryMove(th as *mut _ as *mut CffiMobj, th.x, th.y) == 0 {
        P_ExplodeMissile(th as *mut mobj_t);
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_SubstNullMobj(mobj: *mut mobj_t) -> *mut mobj_t {
    if mobj.is_null() {
        static mut DUMMY_MOBJ: mobj_t = unsafe { std::mem::zeroed() };
        DUMMY_MOBJ.x = 0;
        DUMMY_MOBJ.y = 0;
        DUMMY_MOBJ.z = 0;
        DUMMY_MOBJ.flags = 0;
        return std::ptr::addr_of_mut!(DUMMY_MOBJ);
    }
    mobj
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnMissile(
    source: *mut mobj_t,
    dest: *mut mobj_t,
    type_: c_int,
) -> *mut mobj_t {
    let source = &*source;
    let dest = &*dest;
    let th = P_SpawnMobj(source.x, source.y, source.z + 4 * 8 * FRACUNIT, type_);
    let info = (*th).info as *mut MobjInfo;
    if (*info).seesound != 0 {
        S_StartSound(th as *mut c_void, (*info).seesound);
    }
    (*th).target = source as *const mobj_t as *mut mobj_t;
    let mut an = R_PointToAngle2(source.x, source.y, dest.x, dest.y);
    if dest.flags & MF_SHADOW != 0 {
        an = an.wrapping_add(((P_Random() - P_Random()) << 20) as u32);
    }
    (*th).angle = an;
    an >>= ANGLETOFINESHIFT;
    (*th).momx = FixedMul((*info).speed, *finecosine.0.add(an as usize));
    (*th).momy = FixedMul((*info).speed, finesine[an as usize]);

    let mut dist = P_AproxDistance(dest.x - source.x, dest.y - source.y);
    dist /= (*info).speed;
    if dist < 1 {
        dist = 1;
    }
    (*th).momz = (dest.z - source.z) / dist;
    P_CheckMissileSpawn(th);
    th
}

#[no_mangle]
pub unsafe extern "C" fn P_SpawnPlayerMissile(source: *mut mobj_t, type_: c_int) {
    let source = &mut *source;
    let mut an = source.angle;
    let mut slope = P_AimLineAttack(source as *mut _ as *mut CffiMobj, an, 16 * 64 * FRACUNIT);

    if linetarget.is_null() {
        an = an.wrapping_add(1 << 26);
        slope = P_AimLineAttack(source as *mut _ as *mut CffiMobj, an, 16 * 64 * FRACUNIT);
        if linetarget.is_null() {
            an = an.wrapping_sub(2 << 26);
            slope = P_AimLineAttack(source as *mut _ as *mut CffiMobj, an, 16 * 64 * FRACUNIT);
            if linetarget.is_null() {
                an = source.angle;
                slope = 0;
            }
        }
    }

    let x = source.x;
    let y = source.y;
    let z = source.z + 4 * 8 * FRACUNIT;

    let th = P_SpawnMobj(x, y, z, type_);
    let info = (*th).info as *mut MobjInfo;
    if (*info).seesound != 0 {
        S_StartSound(th as *mut c_void, (*info).seesound);
    }
    (*th).target = source as *const mobj_t as *mut mobj_t;
    (*th).angle = an;
    let fine_idx = (an >> ANGLETOFINESHIFT) as usize;
    (*th).momx = FixedMul((*info).speed, *finecosine.0.add(fine_idx));
    (*th).momy = FixedMul((*info).speed, finesine[fine_idx]);
    (*th).momz = FixedMul((*info).speed, slope);

    P_CheckMissileSpawn(th);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stopspeed_is_0x1000() {
        assert_eq!(STOPSPEED, 0x1000);
        assert_eq!(STOPSPEED, 4096);
    }

    #[test]
    fn friction_is_0xe800() {
        assert_eq!(FRICTION, 0xe800_u32 as i32);
        assert_eq!(FRICTION, 59392_u32 as i32);
    }

    #[test]
    fn friction_is_less_than_fracunit() {
        assert!(
            FRICTION < FRACUNIT,
            "FRICTION must be < FRACUNIT for deceleration"
        );
    }

    #[test]
    fn stopspeed_is_small_fraction_of_fracunit() {
        assert!(STOPSPEED < FRACUNIT);
        assert_eq!(FRACUNIT / STOPSPEED, 16);
    }

    #[test]
    fn itemquesize_is_128() {
        assert_eq!(ITEMQUESIZE, 128);
    }

    #[test]
    fn itemrespawntime_length_is_itemquesize() {
        unsafe {
            assert_eq!(itemrespawntime.len(), ITEMQUESIZE);
        }
    }

    #[test]
    fn itemrespawnque_length_is_itemquesize() {
        unsafe {
            assert_eq!(itemrespawnque.len(), ITEMQUESIZE);
        }
    }

    #[test]
    fn itemque_starts_empty() {
        unsafe {
            assert_eq!(
                iquehead, iquetail,
                "item queue must be empty (head == tail) at startup"
            );
        }
    }

    #[test]
    fn itemrespawntime_starts_zeroed() {
        unsafe {
            for (i, &t) in itemrespawntime.iter().enumerate() {
                assert_eq!(t, 0, "itemrespawntime[{i}] should be 0 before level load");
            }
        }
    }

    #[test]
    fn queue_indices_are_c_int_width() {
        const _: () = assert!(std::mem::size_of::<c_int>() == 4);
        unsafe {
            let _: c_int = iquehead;
            let _: c_int = iquetail;
        }
    }
}
