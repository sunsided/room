//! Rust port of vendor/doomgeneric/p_user.c.
//!
//! Player-related stuff: bobbing POV/weapon, movement, pending weapon, death think.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_int;

use crate::doom::d_mode::{commercial, shareware};
use crate::doom::d_player::{PlayerT, CF_NOCLIP, CF_NOMOMENTUM};
use crate::doom::doomstat::gamemode;
use crate::doom::info::*;
use crate::doom::m_fixed::{fixed_t, FixedMul};
use crate::doom::p_telept::mobj_t;
use crate::doom::p_tick::leveltime;
use crate::doom::tables::{finecosine, finesine, FINEANGLES};

const MAXBOB: c_int = 0x100000;
const ANGLETOFINESHIFT: u32 = 19;
const VIEWHEIGHT: fixed_t = 41 * FRACUNIT;
const FRACUNIT: fixed_t = 1 << 16;
const ANG90: u32 = 0x40000000;
const ANG180: u32 = 0x80000000;
const ANG270: u32 = 0xc0000000;
const ANG5: u32 = ANG90 / 18;

// Button constants (from d_event.h)
const BT_SPECIAL: u8 = 128;
const BT_CHANGE: u8 = 4;
const BT_WEAPONMASK: u8 = 8 + 16 + 32;
const BT_WEAPONSHIFT: u8 = 3;
const BT_USE: u8 = 2;
const BT_ATTACK: u8 = 1;

// Weapon types (from doomdef.h)
const wp_fist: c_int = 0;
const wp_pistol: c_int = 1;
const wp_shotgun: c_int = 2;
const wp_chaingun: c_int = 3;
const wp_missile: c_int = 4;
const wp_plasma: c_int = 5;
const wp_bfg: c_int = 6;
const wp_chainsaw: c_int = 7;
const wp_supershotgun: c_int = 8;

// Power types (from doomdef.h)
const pw_invulnerability: usize = 0;
const pw_strength: usize = 1;
const pw_invisibility: usize = 2;
const pw_ironfeet: usize = 3;
const pw_allmap: usize = 4;
const pw_infrared: usize = 5;

// Player states (from d_player.h)
const PST_LIVE: c_int = 0;
const PST_DEAD: c_int = 1;
const PST_REBORN: c_int = 2;

// Colormap index
const INVERSECOLORMAP: c_int = 32;

/// Size of state_t on x86_64 Linux (sprite+frame+tics=12 + 4 pad + action=8
/// + nextstate+misc1+misc2=12 + 4 pad = 40 bytes).
const STATE_T_SIZEOF: usize = 40;

/// Whether the player is on ground (boolean → c_int for 4-byte ABI).
/// Read by not-yet-ported C modules (e.g. p_pspr.c).
#[no_mangle]
pub static mut onground: c_int = 0;

extern "C" {
    fn P_SetMobjState(thing: *mut mobj_t, state: c_int) -> c_int;
    fn P_MovePsprites(player: *mut PlayerT);
    fn P_BringUpWeapon(player: *mut PlayerT);
    fn P_SetPsprite(player: *mut PlayerT, psprite: c_int, state: c_int);
    fn P_PlayerInSpecialSector(player: *mut PlayerT);
    fn P_UseLines(player: *mut PlayerT);
    fn R_PointToAngle2(x1: fixed_t, y1: fixed_t, x2: fixed_t, y2: fixed_t) -> u32;
    // `states` is `state_t states[NUMSTATES]` in info.c. We declare it as
    // an opaque byte (size-0 would be invalid); only its ADDRESS is used,
    // via byte-offset arithmetic with STATE_T_SIZEOF.
    static states: u8;
    static mut demorecording: c_int;
    static mut viewangleoffset: c_int;
    static mut allowautoaim: c_int;
    fn P_CalcSwirl();
}

#[no_mangle]
pub extern "C" fn P_Thrust(player: *mut PlayerT, angle: u32, move_: fixed_t) {
    unsafe {
        let mo = (*player).mo as *mut mobj_t;
        let angle = (angle >> ANGLETOFINESHIFT) as usize;
        (*mo).momx += FixedMul(move_, *finecosine.0.add(angle));
        (*mo).momy += FixedMul(move_, finesine[angle]);
    }
}

#[no_mangle]
pub extern "C" fn P_CalcHeight(player: *mut PlayerT) {
    unsafe {
        let mo = (*player).mo as *mut mobj_t;
        let angle: c_int;
        let bob: fixed_t;

        // Regular movement bobbing
        (*player).bob = FixedMul((*mo).momx, (*mo).momx) + FixedMul((*mo).momy, (*mo).momy);

        (*player).bob >>= 2;

        if (*player).bob > MAXBOB {
            (*player).bob = MAXBOB;
        }

        if ((*player).cheats & CF_NOMOMENTUM) != 0 || onground == 0 {
            (*player).viewz = (*mo).z + VIEWHEIGHT;

            if (*player).viewz > (*mo).ceilingz - 4 * FRACUNIT {
                (*player).viewz = (*mo).ceilingz - 4 * FRACUNIT;
            }

            (*player).viewz = (*mo).z + (*player).viewheight;
            return;
        }

        angle = ((FINEANGLES as c_int / 20 * leveltime) & (FINEANGLES as c_int - 1)) as c_int;
        bob = FixedMul((*player).bob / 2, finesine[angle as usize]);

        // Move viewheight
        if (*player).playerstate == PST_LIVE {
            (*player).viewheight += (*player).deltaviewheight;

            if (*player).viewheight > VIEWHEIGHT {
                (*player).viewheight = VIEWHEIGHT;
                (*player).deltaviewheight = 0;
            }

            if (*player).viewheight < VIEWHEIGHT / 2 {
                (*player).viewheight = VIEWHEIGHT / 2;
                if (*player).deltaviewheight <= 0 {
                    (*player).deltaviewheight = 1;
                }
            }

            if (*player).deltaviewheight != 0 {
                (*player).deltaviewheight += FRACUNIT / 4;
                if (*player).deltaviewheight == 0 {
                    (*player).deltaviewheight = 1;
                }
            }
        }
        (*player).viewz = (*mo).z + (*player).viewheight + bob;

        if (*player).viewz > (*mo).ceilingz - 4 * FRACUNIT {
            (*player).viewz = (*mo).ceilingz - 4 * FRACUNIT;
        }
    }
}

#[no_mangle]
pub extern "C" fn P_MovePlayer(player: *mut PlayerT) {
    unsafe {
        let mo = (*player).mo as *mut mobj_t;
        let cmd = &(*player).cmd;

        (*mo).angle = (*mo)
            .angle
            .wrapping_add(((cmd.angleturn) as u32).wrapping_shl(16));

        // Do not let the player control movement if not onground.
        onground = ((*mo).z <= (*mo).floorz) as c_int;

        if cmd.forwardmove != 0 && onground != 0 {
            P_Thrust(player, (*mo).angle, cmd.forwardmove as fixed_t * 2048);
        }

        if cmd.sidemove != 0 && onground != 0 {
            P_Thrust(
                player,
                (*mo).angle.wrapping_sub(ANG90),
                cmd.sidemove as fixed_t * 2048,
            );
        }

        // C: player->mo->state == &states[S_PLAY]
        let p_play = (&states as *const u8).add(S_PLAY as usize * STATE_T_SIZEOF);
        if (cmd.forwardmove != 0 || cmd.sidemove != 0) && (*mo).state as *const u8 == p_play {
            P_SetMobjState(mo, S_PLAY_RUN1);
        }
    }
}

#[no_mangle]
pub extern "C" fn P_DeathThink(player: *mut PlayerT) {
    unsafe {
        let mo = (*player).mo as *mut mobj_t;
        let angle: u32;
        let delta: u32;

        P_MovePsprites(player);

        // Fall to the ground
        if (*player).viewheight > 6 * FRACUNIT {
            (*player).viewheight -= FRACUNIT;
        }

        if (*player).viewheight < 6 * FRACUNIT {
            (*player).viewheight = 6 * FRACUNIT;
        }

        (*player).deltaviewheight = 0;
        onground = ((*mo).z <= (*mo).floorz) as c_int;
        P_CalcHeight(player);

        if !(*player).attacker.is_null() && (*player).attacker != (*player).mo {
            let atk = (*player).attacker as *mut mobj_t;
            angle = R_PointToAngle2((*mo).x, (*mo).y, (*atk).x, (*atk).y);

            delta = angle.wrapping_sub((*mo).angle);

            if delta < ANG5 || delta > (!ANG5).wrapping_add(1) {
                // Looking at killer, so fade damage flash down.
                (*mo).angle = angle;

                if (*player).damagecount != 0 {
                    (*player).damagecount -= 1;
                }
            } else if delta < ANG180 {
                (*mo).angle = (*mo).angle.wrapping_add(ANG5);
            } else {
                (*mo).angle = (*mo).angle.wrapping_sub(ANG5);
            }
        } else if (*player).damagecount != 0 {
            (*player).damagecount -= 1;
        }

        if (*player).cmd.buttons & BT_USE != 0 {
            (*player).playerstate = PST_REBORN;
        }
    }
}

#[no_mangle]
pub extern "C" fn P_PlayerThink(player: *mut PlayerT) {
    unsafe {
        let mo = (*player).mo as *mut mobj_t;
        let mut newweapon: c_int;

        // Fixme: do this in the cheat code
        if (*player).cheats & CF_NOCLIP != 0 {
            (*mo).flags |= MF_NOCLIP;
        } else {
            (*mo).flags &= !MF_NOCLIP;
        }

        // Chain saw run forward
        let cmd = &mut (*player).cmd;
        if (*mo).flags & MF_JUSTATTACKED != 0 {
            cmd.angleturn = 0;
            cmd.forwardmove = (0xc800 / 512) as i8;
            cmd.sidemove = 0;
            (*mo).flags &= !MF_JUSTATTACKED;
        }

        if (*player).playerstate == PST_DEAD {
            P_DeathThink(player);
            return;
        }

        // Move around.
        // Reactiontime is used to prevent movement for a bit after a teleport.
        if (*mo).reactiontime != 0 {
            (*mo).reactiontime -= 1;
        } else {
            P_MovePlayer(player);
        }

        P_CalcHeight(player);
        if (*(*(*mo).subsector).sector).special != 0 {
            P_PlayerInSpecialSector(player);
        }
        // Check for weapon change.
        // A special event has no other buttons.
        if cmd.buttons & BT_SPECIAL != 0 {
            cmd.buttons = 0;
        }

        if cmd.buttons & BT_CHANGE != 0 {
            // The actual changing of the weapon is done
            // when the weapon psprite can do it
            // (read: not in the middle of an attack).
            newweapon = (cmd.buttons as c_int & BT_WEAPONMASK as c_int) >> BT_WEAPONSHIFT;

            if newweapon == wp_fist
                && (*player).weaponowned[wp_chainsaw as usize] != 0
                && !((*player).readyweapon == wp_chainsaw && (*player).powers[pw_strength] != 0)
            {
                newweapon = wp_chainsaw;
            }

            if gamemode == commercial
                && newweapon == wp_shotgun
                && (*player).weaponowned[wp_supershotgun as usize] != 0
                && (*player).readyweapon != wp_supershotgun
            {
                newweapon = wp_supershotgun;
            }

            if (*player).weaponowned[newweapon as usize] != 0 && newweapon != (*player).readyweapon
            {
                // Do not go to plasma or BFG in shareware, even if cheated.
                if (newweapon != wp_plasma && newweapon != wp_bfg) || gamemode != shareware {
                    (*player).pendingweapon = newweapon;
                }
            }
        }

        // Check for use
        if cmd.buttons & BT_USE != 0 {
            if (*player).usedown == 0 {
                P_UseLines(player);
                (*player).usedown = 1;
            }
        } else {
            (*player).usedown = 0;
        }

        // Cycle psprites
        P_MovePsprites(player);

        // Counters, time dependent power ups.
        // Strength counts up to diminish fade.
        if (*player).powers[pw_strength] != 0 {
            (*player).powers[pw_strength] += 1;
        }

        if (*player).powers[pw_invulnerability] != 0 {
            (*player).powers[pw_invulnerability] -= 1;
        }

        if (*player).powers[pw_invisibility] != 0 {
            (*player).powers[pw_invisibility] -= 1;
            if (*player).powers[pw_invisibility] == 0 {
                (*mo).flags &= !MF_SHADOW;
            }
        }

        if (*player).powers[pw_infrared] != 0 {
            (*player).powers[pw_infrared] -= 1;
        }

        if (*player).powers[pw_ironfeet] != 0 {
            (*player).powers[pw_ironfeet] -= 1;
        }

        if (*player).damagecount != 0 {
            (*player).damagecount -= 1;
        }

        if (*player).bonuscount != 0 {
            (*player).bonuscount -= 1;
        }

        // Handling colormaps.
        if (*player).powers[pw_invulnerability] != 0 {
            if (*player).powers[pw_invulnerability] > 4 * 32
                || ((*player).powers[pw_invulnerability] & 8) != 0
            {
                (*player).fixedcolormap = INVERSECOLORMAP;
            } else {
                (*player).fixedcolormap = 0;
            }
        } else if (*player).powers[pw_infrared] != 0 {
            if (*player).powers[pw_infrared] > 4 * 32 || ((*player).powers[pw_infrared] & 8) != 0 {
                // Almost full bright
                (*player).fixedcolormap = 1;
            } else {
                (*player).fixedcolormap = 0;
            }
        } else {
            (*player).fixedcolormap = 0;
        }
    }
}

/// Anchor function referenced from `doomgeneric_Create` to ensure all
/// `#[no_mangle]` exports survive link-time dead-code elimination.
#[no_mangle]
pub extern "C" fn P_User_Link_Anchor() {
    unsafe {
        let _ = onground as usize;
    }
    let _ = P_Thrust as *const () as usize;
    let _ = P_CalcHeight as *const () as usize;
    let _ = P_MovePlayer as *const () as usize;
    let _ = P_DeathThink as *const () as usize;
    let _ = P_PlayerThink as *const () as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doom::d_mode::{commercial, registered, retail, shareware};
    use crate::doom::d_player::{PspdefT, NUMPOWERS};
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const PSPDEF_T_SIZEOF: usize = 24;

    #[test]
    fn pspdef_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<PspdefT>(),
            PSPDEF_T_SIZEOF,
            "PspdefT size mismatch: Rust={}, expected={}",
            std::mem::size_of::<PspdefT>(),
            PSPDEF_T_SIZEOF,
        );
    }

    #[test]
    fn onground_defaults_to_zero() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(onground, 0);
        }
    }

    #[test]
    fn constants_match_c_header_values() {
        // Verify button constants match d_event.h
        assert_eq!(BT_SPECIAL, 128);
        assert_eq!(BT_CHANGE, 4);
        assert_eq!(BT_WEAPONMASK, 56); // 8+16+32
        assert_eq!(BT_WEAPONSHIFT, 3);
        assert_eq!(BT_USE, 2);
        assert_eq!(BT_ATTACK, 1);

        // Verify game mode enum values match d_mode.rs
        assert_eq!(shareware, 0);
        assert_eq!(registered, 1);
        assert_eq!(commercial, 2);
        assert_eq!(retail, 3);

        // Verify angle constants match tables.h
        assert_eq!(ANG90, 0x40000000);
        assert_eq!(ANG270, 0xc0000000);

        // Verify player state enum values match d_player.h
        assert_eq!(PST_LIVE, 0);
        assert_eq!(PST_DEAD, 1);
        assert_eq!(PST_REBORN, 2);

        // Verify mobj flags match p_mobj.h
        assert_eq!(MF_NOCLIP, 0x1000);
        assert_eq!(MF_JUSTATTACKED, 128);
        assert_eq!(MF_SHADOW, 0x40000);

        // Verify power indices match doomdef.h (pw_allmap sits between
        // pw_ironfeet and pw_infrared; NUMPOWERS is 6).
        assert_eq!(pw_invulnerability, 0);
        assert_eq!(pw_strength, 1);
        assert_eq!(pw_invisibility, 2);
        assert_eq!(pw_ironfeet, 3);
        assert_eq!(pw_allmap, 4);
        assert_eq!(pw_infrared, 5);
        assert_eq!(NUMPOWERS, 6);
    }
}
