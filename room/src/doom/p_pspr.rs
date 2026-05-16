//! Rust port of vendor/doomgeneric/p_pspr.c.
//!
//! Weapon sprite animation, weapon objects, and action functions for weapons.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_void;
use std::os::raw::c_int;

use crate::doom::c_ffi::{LOWERSPEED, RAISESPEED, WEAPONBOTTOM, WEAPONTOP};
use crate::doom::d_items::weaponinfo;
use crate::doom::d_mode::{commercial, shareware};
use crate::doom::d_player::{PlayerT, PspdefT, NUMAMMO, NUMPSPRITES, NUMWEAPONS};
use crate::doom::doomstat::gamemode;
use crate::doom::info::{self, *};
use crate::doom::m_fixed::{fixed_t, FixedMul};
use crate::doom::m_fixed::{FRACBITS, FRACUNIT};
use crate::doom::m_random::P_Random;
use crate::doom::p_telept::mobj_t;
use crate::doom::p_tick::leveltime;
use crate::doom::s_sound::S_StartSound;
use crate::doom::tables::{finecosine, finesine, FINEANGLES, FINEMASK};

// Weapon type constants (from doomdef.h)
const wp_fist: c_int = 0;
const wp_pistol: c_int = 1;
const wp_shotgun: c_int = 2;
const wp_chaingun: c_int = 3;
const wp_missile: c_int = 4;
const wp_plasma: c_int = 5;
const wp_bfg: c_int = 6;
const wp_chainsaw: c_int = 7;
const wp_supershotgun: c_int = 8;
const wp_nochange: c_int = NUMWEAPONS as c_int;

// Ammo type constants
const am_noammo: c_int = 5;
const am_clip: c_int = 0;
const am_shell: c_int = 1;
const am_cell: c_int = 2;
const am_misl: c_int = 3;

// Power type constants
const pw_strength: usize = 1;

// Button constants
const BT_ATTACK: u8 = 1;

// Player state constants
const PST_DEAD: c_int = 1;

// Angle constants
const ANG90: u32 = 0x40000000;
const ANG180: u32 = 0x80000000;

// Range constants (from p_local.h)
const MELEERANGE: c_int = 64 * FRACUNIT;
const MISSILERANGE: c_int = 32 * 64 * FRACUNIT;

// Dehacked default
const DEH_DEFAULT_BFG_CELLS_PER_SHOT: c_int = 40;

// Sound effect constants
const sfx_sawup: c_int = 10;
const sfx_sawidl: c_int = 11;
const sfx_sawful: c_int = 12;
const sfx_sawhit: c_int = 13;
const sfx_pistol: c_int = 1;
const sfx_shotgn: c_int = 2;
const sfx_dshtgn: c_int = 4;
const sfx_bfg: c_int = 9;
const sfx_punch: c_int = 83;

use crate::doom::p_enemy::P_NoiseAlert;
use crate::doom::p_inter::P_DamageMobj;
use crate::doom::p_map::{linetarget, P_AimLineAttack, P_LineAttack};
use crate::doom::p_mobj::{P_SetMobjState, P_SpawnMobj, P_SpawnPlayerMissile};
use crate::doom::r_main::R_PointToAngle2;

// Type aliases for cross-module pointer casts (all #[repr(C)] identical layouts).
type CffiMobj = crate::doom::c_ffi::mobj_t;

/// Horizontal weapon-bob offset; updated each tic by P_CalcSwing.
#[no_mangle]
pub static mut swingx: fixed_t = 0;

/// Vertical weapon-bob offset; updated each tic by P_CalcSwing.
#[no_mangle]
pub static mut swingy: fixed_t = 0;

/// Slope set by P_BulletSlope for near-miss aiming.
#[no_mangle]
pub static mut bulletslope: fixed_t = 0;

/// Set a psprite to a given state.
#[no_mangle]
pub unsafe extern "C" fn P_SetPsprite(player: *mut PlayerT, position: c_int, stnum: c_int) {
    let psp = (*player).psprites.as_mut_ptr().add(position as usize);

    let mut stnum = stnum;
    loop {
        if stnum == 0 {
            (*psp).state = std::ptr::null_mut();
            break;
        }

        let state = &mut info::states[stnum as usize] as *mut State;
        (*psp).state = state as *mut crate::doom::d_player::state_t;
        (*psp).tics = (*state).tics;

        if (*state).misc1 != 0 {
            (*psp).sx = (*state).misc1 << FRACBITS;
            (*psp).sy = (*state).misc2 << FRACBITS;
        }

        // Call action routine.
        if let Some(action) = (*state).action {
            let action: unsafe extern "C" fn(*mut PlayerT, *mut PspdefT) =
                std::mem::transmute(action);
            action(player, psp);
            if (*psp).state.is_null() {
                break;
            }
        }

        // Read nextstate from psp->state (action may have changed it).
        stnum = ((*psp).state as *mut State).read().nextstate;

        if (*psp).tics != 0 {
            break;
        }
    }
}

/// Calculate weapon swing offsets.
#[no_mangle]
pub unsafe extern "C" fn P_CalcSwing(player: *mut PlayerT) {
    let swing = (*player).bob;

    let mut angle = (FINEANGLES as c_int / 70 * leveltime) & FINEMASK;
    swingx = FixedMul(swing, finesine[angle as usize]);

    angle = (FINEANGLES as c_int / 70 * leveltime + FINEANGLES as c_int / 2) & FINEMASK;
    swingy = -FixedMul(swingx, finesine[angle as usize]);
}

/// Start bringing the pending weapon up from the bottom of the screen.
#[no_mangle]
pub unsafe extern "C" fn P_BringUpWeapon(player: *mut PlayerT) {
    if (*player).pendingweapon == wp_nochange {
        (*player).pendingweapon = (*player).readyweapon;
    }

    if (*player).pendingweapon == wp_chainsaw {
        S_StartSound((*player).mo as *mut c_void, sfx_sawup);
    }

    let newstate = weaponinfo[(*player).pendingweapon as usize].upstate;

    (*player).pendingweapon = wp_nochange;
    (*player).psprites[0].sy = WEAPONBOTTOM;

    P_SetPsprite(player, 0, newstate);
}

/// Returns true if there is enough ammo to shoot.
#[no_mangle]
pub unsafe extern "C" fn P_CheckAmmo(player: *mut PlayerT) -> c_int {
    let ammo = weaponinfo[(*player).readyweapon as usize].ammo;

    let count: c_int;
    if (*player).readyweapon == wp_bfg {
        count = DEH_DEFAULT_BFG_CELLS_PER_SHOT;
    } else if (*player).readyweapon == wp_supershotgun {
        count = 2;
    } else {
        count = 1;
    }

    if ammo == am_noammo || (*player).ammo[ammo as usize] >= count {
        return 1;
    }

    // Out of ammo, pick a weapon to change to.
    loop {
        if (*player).weaponowned[wp_plasma as usize] != 0
            && (*player).ammo[am_cell as usize] != 0
            && gamemode != shareware
        {
            (*player).pendingweapon = wp_plasma;
        } else if (*player).weaponowned[wp_supershotgun as usize] != 0
            && (*player).ammo[am_shell as usize] > 2
            && gamemode == commercial
        {
            (*player).pendingweapon = wp_supershotgun;
        } else if (*player).weaponowned[wp_chaingun as usize] != 0
            && (*player).ammo[am_clip as usize] != 0
        {
            (*player).pendingweapon = wp_chaingun;
        } else if (*player).weaponowned[wp_shotgun as usize] != 0
            && (*player).ammo[am_shell as usize] != 0
        {
            (*player).pendingweapon = wp_shotgun;
        } else if (*player).ammo[am_clip as usize] != 0 {
            (*player).pendingweapon = wp_pistol;
        } else if (*player).weaponowned[wp_chainsaw as usize] != 0 {
            (*player).pendingweapon = wp_chainsaw;
        } else if (*player).weaponowned[wp_missile as usize] != 0
            && (*player).ammo[am_misl as usize] != 0
        {
            (*player).pendingweapon = wp_missile;
        } else if (*player).weaponowned[wp_bfg as usize] != 0
            && (*player).ammo[am_cell as usize] > 40
            && gamemode != shareware
        {
            (*player).pendingweapon = wp_bfg;
        } else {
            (*player).pendingweapon = wp_fist;
        }

        if (*player).pendingweapon != wp_nochange {
            break;
        }
    }

    P_SetPsprite(
        player,
        0,
        weaponinfo[(*player).readyweapon as usize].downstate,
    );

    0
}

/// Fire the current weapon.
#[no_mangle]
pub unsafe extern "C" fn P_FireWeapon(player: *mut PlayerT) {
    if P_CheckAmmo(player) == 0 {
        return;
    }

    P_SetMobjState((*player).mo as *mut mobj_t, S_PLAY_ATK1);
    let newstate = weaponinfo[(*player).readyweapon as usize].atkstate;
    P_SetPsprite(player, 0, newstate);
    P_NoiseAlert((*player).mo as *mut mobj_t, (*player).mo as *mut mobj_t);
}

/// Player died, so put the weapon away.
#[no_mangle]
pub unsafe extern "C" fn P_DropWeapon(player: *mut PlayerT) {
    P_SetPsprite(
        player,
        0,
        weaponinfo[(*player).readyweapon as usize].downstate,
    );
}

/// The player can fire the weapon or change to another weapon at this time.
#[no_mangle]
pub unsafe extern "C" fn A_WeaponReady(player: *mut PlayerT, psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;

    // Get out of attack state.
    let state_ptr = (*mo).state as *mut State;
    if state_ptr == &mut info::states[S_PLAY_ATK1 as usize] as *mut State
        || state_ptr == &mut info::states[S_PLAY_ATK2 as usize] as *mut State
    {
        P_SetMobjState(mo, S_PLAY);
    }

    if (*player).readyweapon == wp_chainsaw
        && (*psp).state as *mut State == &mut info::states[S_SAW as usize] as *mut State
    {
        S_StartSound(mo as *mut c_void, sfx_sawidl);
    }

    // Check for change: if player is dead, put the weapon away.
    if (*player).pendingweapon != wp_nochange || (*player).health == 0 {
        let newstate = weaponinfo[(*player).readyweapon as usize].downstate;
        P_SetPsprite(player, 0, newstate);
        return;
    }

    // Check for fire: the missile launcher and bfg do not auto fire.
    if (*player).cmd.buttons & BT_ATTACK != 0 {
        if (*player).attackdown == 0
            || ((*player).readyweapon != wp_missile && (*player).readyweapon != wp_bfg)
        {
            (*player).attackdown = 1;
            P_FireWeapon(player);
            return;
        }
    } else {
        (*player).attackdown = 0;
    }

    // Bob the weapon based on movement speed.
    let mut angle = (128 * leveltime) as u32 & FINEMASK as u32;
    (*psp).sx = FRACUNIT + FixedMul((*player).bob, *finecosine.0.add(angle as usize));
    angle &= (FINEANGLES / 2 - 1) as u32;
    (*psp).sy = WEAPONTOP + FixedMul((*player).bob, finesine[angle as usize]);
}

/// The player can re-fire the weapon without lowering it entirely.
#[no_mangle]
pub unsafe extern "C" fn A_ReFire(player: *mut PlayerT, _psp: *mut PspdefT) {
    if (*player).cmd.buttons & BT_ATTACK != 0
        && (*player).pendingweapon == wp_nochange
        && (*player).health != 0
    {
        (*player).refire += 1;
        P_FireWeapon(player);
    } else {
        (*player).refire = 0;
        P_CheckAmmo(player);
    }
}

/// Check if ammo is sufficient; switch weapons if not.
#[no_mangle]
pub unsafe extern "C" fn A_CheckReload(player: *mut PlayerT, _psp: *mut PspdefT) {
    P_CheckAmmo(player);
}

/// Lowers current weapon and changes weapon at bottom.
#[no_mangle]
pub unsafe extern "C" fn A_Lower(player: *mut PlayerT, psp: *mut PspdefT) {
    (*psp).sy += LOWERSPEED;

    // Is already down.
    if (*psp).sy < WEAPONBOTTOM {
        return;
    }

    // Player is dead.
    if (*player).playerstate == PST_DEAD {
        (*psp).sy = WEAPONBOTTOM;
        return;
    }

    // The old weapon has been lowered off the screen, so change the weapon
    // and start raising it.
    if (*player).health == 0 {
        P_SetPsprite(player, 0, S_NULL);
        return;
    }

    (*player).readyweapon = (*player).pendingweapon;
    P_BringUpWeapon(player);
}

/// Raises current weapon.
#[no_mangle]
pub unsafe extern "C" fn A_Raise(player: *mut PlayerT, psp: *mut PspdefT) {
    (*psp).sy -= RAISESPEED;

    if (*psp).sy > WEAPONTOP {
        return;
    }

    (*psp).sy = WEAPONTOP;

    let newstate = weaponinfo[(*player).readyweapon as usize].readystate;
    P_SetPsprite(player, 0, newstate);
}

/// Show gun flash and set player to attack state 2.
#[no_mangle]
pub unsafe extern "C" fn A_GunFlash(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;
    P_SetMobjState(mo, S_PLAY_ATK2);
    P_SetPsprite(
        player,
        1,
        weaponinfo[(*player).readyweapon as usize].flashstate,
    );
}

/// Fire fist punch.
#[no_mangle]
pub unsafe extern "C" fn A_Punch(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;

    let mut damage = ((P_Random() % 10) + 1) << 1;
    if (*player).powers[pw_strength] != 0 {
        damage *= 10;
    }

    let mut angle = (*mo).angle;
    angle = angle.wrapping_add(((P_Random() - P_Random()) as u32) << 18);
    let slope = P_AimLineAttack(mo as *mut _ as *mut CffiMobj, angle, MELEERANGE);
    P_LineAttack(
        mo as *mut _ as *mut CffiMobj,
        angle,
        MELEERANGE,
        slope,
        damage,
    );

    // Turn to face target.
    if !linetarget.is_null() {
        S_StartSound(mo as *mut c_void, sfx_punch);
        (*mo).angle = R_PointToAngle2((*mo).x, (*mo).y, (*linetarget).x, (*linetarget).y);
    }
}

/// Fire chainsaw.
#[no_mangle]
pub unsafe extern "C" fn A_Saw(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;

    let damage = 2 * ((P_Random() % 10) + 1);
    let mut angle = (*mo).angle;
    angle = angle.wrapping_add(((P_Random() - P_Random()) as u32) << 18);

    let slope = P_AimLineAttack(mo as *mut _ as *mut CffiMobj, angle, MELEERANGE + 1);
    P_LineAttack(
        mo as *mut _ as *mut CffiMobj,
        angle,
        MELEERANGE + 1,
        slope,
        damage,
    );

    if linetarget.is_null() {
        S_StartSound(mo as *mut c_void, sfx_sawful);
        return;
    }
    S_StartSound(mo as *mut c_void, sfx_sawhit);

    // Turn to face target.
    let angle = R_PointToAngle2((*mo).x, (*mo).y, (*linetarget).x, (*linetarget).y);
    let delta = angle.wrapping_sub((*mo).angle);
    if delta > ANG180 {
        let signed_delta = delta as i32;
        if signed_delta < -(ANG90 as i32) / 20 {
            (*mo).angle = angle.wrapping_add(ANG90 / 21);
        } else {
            (*mo).angle = (*mo).angle.wrapping_sub(ANG90 / 20);
        }
    } else {
        if delta > ANG90 / 20 {
            (*mo).angle = angle.wrapping_sub(ANG90 / 21);
        } else {
            (*mo).angle = (*mo).angle.wrapping_add(ANG90 / 20);
        }
    }
    (*mo).flags |= MF_JUSTATTACKED;
}

/// Decrease ammo, with Doom's original array-overflow emulation for ammo > NUMAMMO.
unsafe fn DecreaseAmmo(player: *mut PlayerT, ammonum: c_int, amount: c_int) {
    if ammonum < NUMAMMO as c_int {
        (*player).ammo[ammonum as usize] -= amount;
    } else {
        (*player).maxammo[(ammonum - NUMAMMO as c_int) as usize] -= amount;
    }
}

/// Fire rocket.
#[no_mangle]
pub unsafe extern "C" fn A_FireMissile(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;
    DecreaseAmmo(player, weaponinfo[(*player).readyweapon as usize].ammo, 1);
    P_SpawnPlayerMissile(mo, MT_ROCKET);
}

/// Fire BFG.
#[no_mangle]
pub unsafe extern "C" fn A_FireBFG(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;
    DecreaseAmmo(
        player,
        weaponinfo[(*player).readyweapon as usize].ammo,
        DEH_DEFAULT_BFG_CELLS_PER_SHOT,
    );
    P_SpawnPlayerMissile(mo, MT_BFG);
}

/// Fire plasma.
#[no_mangle]
pub unsafe extern "C" fn A_FirePlasma(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;
    DecreaseAmmo(player, weaponinfo[(*player).readyweapon as usize].ammo, 1);

    P_SetPsprite(
        player,
        1,
        weaponinfo[(*player).readyweapon as usize].flashstate + (P_Random() & 1),
    );

    P_SpawnPlayerMissile(mo, MT_PLASMA);
}

/// Sets a slope so a near miss is at approximately the height of the intended target.
#[no_mangle]
pub unsafe extern "C" fn P_BulletSlope(mo: *mut mobj_t) {
    let mut an = (*mo).angle;
    bulletslope = P_AimLineAttack(mo as *mut _ as *mut CffiMobj, an, 16 * 64 * FRACUNIT);

    if linetarget.is_null() {
        an = an.wrapping_add(1 << 26);
        bulletslope = P_AimLineAttack(mo as *mut _ as *mut CffiMobj, an, 16 * 64 * FRACUNIT);
        if linetarget.is_null() {
            an = an.wrapping_sub(2 << 26);
            bulletslope = P_AimLineAttack(mo as *mut _ as *mut CffiMobj, an, 16 * 64 * FRACUNIT);
        }
    }
}

/// Fire a single bullet.
#[no_mangle]
pub unsafe extern "C" fn P_GunShot(mo: *mut mobj_t, accurate: c_int) {
    let damage = 5 * ((P_Random() % 3) + 1);
    let mut angle = (*mo).angle;

    if accurate == 0 {
        angle = angle.wrapping_add(((P_Random() - P_Random()) as u32) << 18);
    }

    P_LineAttack(
        mo as *mut _ as *mut CffiMobj,
        angle,
        MISSILERANGE,
        bulletslope,
        damage,
    );
}

/// Fire pistol.
#[no_mangle]
pub unsafe extern "C" fn A_FirePistol(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;
    S_StartSound(mo as *mut c_void, sfx_pistol);

    P_SetMobjState(mo, S_PLAY_ATK2);
    DecreaseAmmo(player, weaponinfo[(*player).readyweapon as usize].ammo, 1);

    P_SetPsprite(
        player,
        1,
        weaponinfo[(*player).readyweapon as usize].flashstate,
    );

    P_BulletSlope(mo);
    P_GunShot(mo, ((*player).refire == 0) as c_int);
}

/// Fire shotgun.
#[no_mangle]
pub unsafe extern "C" fn A_FireShotgun(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;
    S_StartSound(mo as *mut c_void, sfx_shotgn);
    P_SetMobjState(mo, S_PLAY_ATK2);

    DecreaseAmmo(player, weaponinfo[(*player).readyweapon as usize].ammo, 1);

    P_SetPsprite(
        player,
        1,
        weaponinfo[(*player).readyweapon as usize].flashstate,
    );

    P_BulletSlope(mo);

    for _ in 0..7 {
        P_GunShot(mo, 0);
    }
}

/// Fire super shotgun.
#[no_mangle]
pub unsafe extern "C" fn A_FireShotgun2(player: *mut PlayerT, _psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;
    S_StartSound(mo as *mut c_void, sfx_dshtgn);
    P_SetMobjState(mo, S_PLAY_ATK2);

    DecreaseAmmo(player, weaponinfo[(*player).readyweapon as usize].ammo, 2);

    P_SetPsprite(
        player,
        1,
        weaponinfo[(*player).readyweapon as usize].flashstate,
    );

    P_BulletSlope(mo);

    for _ in 0..20 {
        let damage = 5 * ((P_Random() % 3) + 1);
        let mut angle = (*mo).angle;
        angle = angle.wrapping_add(((P_Random() - P_Random()) as u32) << 19);
        P_LineAttack(
            mo as *mut _ as *mut CffiMobj,
            angle,
            MISSILERANGE,
            bulletslope + (((P_Random() - P_Random()) as c_int) << 5),
            damage,
        );
    }
}

/// Fire chaingun.
#[no_mangle]
pub unsafe extern "C" fn A_FireCGun(player: *mut PlayerT, psp: *mut PspdefT) {
    let mo = (*player).mo as *mut mobj_t;
    S_StartSound(mo as *mut c_void, sfx_pistol);

    if (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] == 0 {
        return;
    }

    P_SetMobjState(mo, S_PLAY_ATK2);
    DecreaseAmmo(player, weaponinfo[(*player).readyweapon as usize].ammo, 1);

    let flashstate = weaponinfo[(*player).readyweapon as usize].flashstate;
    let state_offset = (*psp)
        .state
        .cast::<State>()
        .offset_from(&info::states[S_CHAIN1 as usize]);
    P_SetPsprite(player, 1, flashstate + state_offset as c_int);

    P_BulletSlope(mo);
    P_GunShot(mo, ((*player).refire == 0) as c_int);
}

/// Set extralight to 0.
#[no_mangle]
pub unsafe extern "C" fn A_Light0(player: *mut PlayerT, _psp: *mut PspdefT) {
    (*player).extralight = 0;
}

/// Set extralight to 1.
#[no_mangle]
pub unsafe extern "C" fn A_Light1(player: *mut PlayerT, _psp: *mut PspdefT) {
    (*player).extralight = 1;
}

/// Set extralight to 2.
#[no_mangle]
pub unsafe extern "C" fn A_Light2(player: *mut PlayerT, _psp: *mut PspdefT) {
    (*player).extralight = 2;
}

/// Spawn a BFG explosion on every monster in view.
#[no_mangle]
pub unsafe extern "C" fn A_BFGSpray(mo: *mut mobj_t) {
    for i in 0..40 {
        let an = (*mo).angle - ANG90 / 2 + (ANG90 / 40) * i as u32;

        P_AimLineAttack(
            (*mo).target as *mut _ as *mut CffiMobj,
            an,
            16 * 64 * FRACUNIT,
        );

        if linetarget.is_null() {
            continue;
        }

        P_SpawnMobj(
            (*linetarget).x,
            (*linetarget).y,
            (*linetarget).z + ((*linetarget).height >> 2),
            MT_EXTRABFG,
        );

        let mut damage = 0;
        for _ in 0..15 {
            damage += (P_Random() & 7) + 1;
        }

        P_DamageMobj(
            linetarget as *mut mobj_t,
            (*mo).target,
            (*mo).target,
            damage,
        );
    }
}

/// Play BFG firing sound.
#[no_mangle]
pub unsafe extern "C" fn A_BFGsound(player: *mut PlayerT, _psp: *mut PspdefT) {
    S_StartSound((*player).mo as *mut c_void, sfx_bfg);
}

/// Called at start of level for each player.
#[no_mangle]
pub unsafe extern "C" fn P_SetupPsprites(player: *mut PlayerT) {
    for i in 0..NUMPSPRITES {
        (*player).psprites[i].state = std::ptr::null_mut();
    }

    (*player).pendingweapon = (*player).readyweapon;
    P_BringUpWeapon(player);
}

/// Called every tic by player thinking routine.
#[no_mangle]
pub unsafe extern "C" fn P_MovePsprites(player: *mut PlayerT) {
    let mut psp = (*player).psprites.as_mut_ptr();

    for i in 0..NUMPSPRITES {
        let state = (*psp).state as *mut State;
        if !state.is_null() {
            // Drop tic count and possibly change state.
            // A -1 tic count never changes.
            if (*psp).tics != -1 {
                (*psp).tics -= 1;
                if (*psp).tics == 0 {
                    P_SetPsprite(player, i as c_int, (*state).nextstate);
                }
            }
        }
        psp = psp.add(1);
    }

    (*player).psprites[1].sx = (*player).psprites[0].sx;
    (*player).psprites[1].sy = (*player).psprites[0].sy;
}

/// Anchor function referenced to ensure all exports survive link-time DCE.
#[no_mangle]
pub extern "C" fn P_Pspr_Link_Anchor() {
    unsafe {
        let _ = swingx as usize;
        let _ = swingy as usize;
        let _ = bulletslope as usize;
    }
    let _ = P_SetPsprite as *const () as usize;
    let _ = P_CalcSwing as *const () as usize;
    let _ = P_BringUpWeapon as *const () as usize;
    let _ = P_CheckAmmo as *const () as usize;
    let _ = P_FireWeapon as *const () as usize;
    let _ = P_DropWeapon as *const () as usize;
    let _ = A_WeaponReady as *const () as usize;
    let _ = A_ReFire as *const () as usize;
    let _ = A_CheckReload as *const () as usize;
    let _ = A_Lower as *const () as usize;
    let _ = A_Raise as *const () as usize;
    let _ = A_GunFlash as *const () as usize;
    let _ = A_Punch as *const () as usize;
    let _ = A_Saw as *const () as usize;
    let _ = A_FireMissile as *const () as usize;
    let _ = A_FireBFG as *const () as usize;
    let _ = A_FirePlasma as *const () as usize;
    let _ = P_BulletSlope as *const () as usize;
    let _ = P_GunShot as *const () as usize;
    let _ = A_FirePistol as *const () as usize;
    let _ = A_FireShotgun as *const () as usize;
    let _ = A_FireShotgun2 as *const () as usize;
    let _ = A_FireCGun as *const () as usize;
    let _ = A_Light0 as *const () as usize;
    let _ = A_Light1 as *const () as usize;
    let _ = A_Light2 as *const () as usize;
    let _ = A_BFGSpray as *const () as usize;
    let _ = A_BFGsound as *const () as usize;
    let _ = P_SetupPsprites as *const () as usize;
    let _ = P_MovePsprites as *const () as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn constants_match_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(LOWERSPEED, FRACUNIT * 6);
        assert_eq!(RAISESPEED, FRACUNIT * 6);
        assert_eq!(WEAPONBOTTOM, 128 * FRACUNIT);
        assert_eq!(WEAPONTOP, 32 * FRACUNIT);
        assert_eq!(MELEERANGE, 64 * FRACUNIT);
        assert_eq!(MISSILERANGE, 32 * 64 * FRACUNIT);
    }

    #[test]
    fn weapon_constants_match() {
        assert_eq!(wp_fist, 0);
        assert_eq!(wp_pistol, 1);
        assert_eq!(wp_shotgun, 2);
        assert_eq!(wp_chaingun, 3);
        assert_eq!(wp_missile, 4);
        assert_eq!(wp_plasma, 5);
        assert_eq!(wp_bfg, 6);
        assert_eq!(wp_chainsaw, 7);
        assert_eq!(wp_supershotgun, 8);
        assert_eq!(wp_nochange, NUMWEAPONS as c_int);
    }

    #[test]
    fn swing_defaults_to_zero() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(swingx, 0);
            assert_eq!(swingy, 0);
        }
    }

    #[test]
    fn bulletslope_defaults_to_zero() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(bulletslope, 0);
        }
    }

    /// Regression tests for mobjtype constants used by weapon fire functions.
    /// These caught an off-by-one bug where MT_ROCKET/MT_PLASMA/MT_BFG were
    /// all 1 too high, causing A_FireMissile to spawn plasma balls instead
    /// of rockets (which then crashed on the missing PLSS sprite in the
    /// shareware WAD).
    #[test]
    fn mobjtype_constants_match_info() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(MT_ROCKET, 33);
        assert_eq!(MT_PLASMA, 34);
        assert_eq!(MT_BFG, 35);
        assert_eq!(MT_PUFF, 37);
        assert_eq!(MT_EXTRABFG, 42);
    }

    /// Verify that the mobjinfo table entries at the projectile indices
    /// have the expected spawnstates.  This catches index-vs-table drift.
    #[test]
    fn projectile_mobjinfo_entries() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(
                info::mobjinfo[MT_ROCKET as usize].spawnstate,
                S_ROCKET,
                "mobjinfo[MT_ROCKET] should spawn S_ROCKET"
            );
            assert_eq!(
                info::mobjinfo[MT_PLASMA as usize].spawnstate,
                S_PLASBALL,
                "mobjinfo[MT_PLASMA] should spawn S_PLASBALL"
            );
            assert_eq!(
                info::mobjinfo[MT_BFG as usize].spawnstate,
                S_BFGSHOT,
                "mobjinfo[MT_BFG] should spawn S_BFGSHOT"
            );
            assert_eq!(
                info::mobjinfo[MT_EXTRABFG as usize].spawnstate,
                S_BFGEXP,
                "mobjinfo[MT_EXTRABFG] should spawn S_BFGEXP"
            );
        }
    }
}
