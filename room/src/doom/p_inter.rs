//! Rust port of vendor/doomgeneric/p_inter.c.
//!
//! Player/item interactions: ammo, weapons, armor, cards, power-ups, damage,
//! kills, and the `P_TouchSpecialThing` pick-up routine.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_void};
use std::os::raw::c_int;

use crate::doom::d_items::weaponinfo;
use crate::doom::d_player::{consoleplayer, players, PlayerT, CF_GODMODE};
use crate::doom::doomstat::{gamemode, gameversion};
use crate::doom::info::{self, *};
use crate::doom::m_fixed::{fixed_t, FixedMul, FRACUNIT};
use crate::doom::m_random::P_Random;
use crate::doom::p_pspr::P_DropWeapon;
use crate::doom::p_telept::mobj_t;
use crate::doom::tables::{finecosine, finesine, ANG180, ANGLETOFINESHIFT};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const BONUSADD: c_int = 6;
const NUMAMMO: usize = 4;
const MAXHEALTH: c_int = 100;
const ONFLOORZ: c_int = i32::MIN;
const BASETHRESHOLD: c_int = 100;

// Skill levels
const sk_baby: c_int = 0;
const sk_nightmare: c_int = 4;

// Power-up durations (TICRATE = 35)
const INVULNTICS: c_int = 30 * 35;
const INVISTICS: c_int = 60 * 35;
const INFRATICS: c_int = 120 * 35;
const IRONTICS: c_int = 60 * 35;

// Weapon types
const wp_fist: c_int = 0;
const wp_pistol: c_int = 1;
const wp_shotgun: c_int = 2;
const wp_chaingun: c_int = 3;
const wp_missile: c_int = 4;
const wp_plasma: c_int = 5;
const wp_bfg: c_int = 6;
const wp_chainsaw: c_int = 7;
const wp_supershotgun: c_int = 8;

// Ammo types
const am_noammo: c_int = 5;
const am_clip: c_int = 0;
const am_shell: c_int = 1;
const am_cell: c_int = 2;
const am_misl: c_int = 3;

// Card types
const it_bluecard: c_int = 0;
const it_yellowcard: c_int = 1;
const it_redcard: c_int = 2;
const it_blueskull: c_int = 3;
const it_yellowskull: c_int = 4;
const it_redskull: c_int = 5;

// Power types
const pw_invulnerability: usize = 0;
const pw_strength: usize = 1;
const pw_invisibility: usize = 2;
const pw_ironfeet: usize = 3;
const pw_allmap: usize = 4;
const pw_infrared: usize = 5;

// Sound effects
const sfx_itemup: c_int = 32;
const sfx_wpnup: c_int = 33;
const sfx_getpow: c_int = 93;

// Game version
const exe_chex: c_int = 9;

// Game mode
const commercial: c_int = 2;

// Dehacked defaults (FEATURE_DEHACKED is not defined)
const DEH_DEFAULT_MAX_HEALTH: c_int = 200;
const DEH_DEFAULT_MAX_ARMOR: c_int = 200;
const DEH_DEFAULT_GREEN_ARMOR_CLASS: c_int = 1;
const DEH_DEFAULT_BLUE_ARMOR_CLASS: c_int = 2;
const DEH_DEFAULT_MAX_SOULSPHERE: c_int = 200;
const DEH_DEFAULT_SOULSPHERE_HEALTH: c_int = 100;
const DEH_DEFAULT_MEGASPHERE_HEALTH: c_int = 200;

// ---------------------------------------------------------------------------
// Pick-up message strings
// ---------------------------------------------------------------------------

const GOTARMOR: *mut c_char = b"Picked up the armor.\0".as_ptr() as *mut c_char;
const GOTMEGA: *mut c_char = b"Picked up the MegaArmor!\0".as_ptr() as *mut c_char;
const GOTHTHBONUS: *mut c_char = b"Picked up a health bonus.\0".as_ptr() as *mut c_char;
const GOTARMBONUS: *mut c_char = b"Picked up an armor bonus.\0".as_ptr() as *mut c_char;
const GOTSTIM: *mut c_char = b"Picked up a stimpack.\0".as_ptr() as *mut c_char;
const GOTMEDINEED: *mut c_char =
    b"Picked up a medikit that you REALLY need!\0".as_ptr() as *mut c_char;
const GOTMEDIKIT: *mut c_char = b"Picked up a medikit.\0".as_ptr() as *mut c_char;
const GOTSUPER: *mut c_char = b"Supercharge!\0".as_ptr() as *mut c_char;
const GOTMSPHERE: *mut c_char = b"MegaSphere!\0".as_ptr() as *mut c_char;
const GOTBLUECARD: *mut c_char = b"Picked up a blue keycard.\0".as_ptr() as *mut c_char;
const GOTYELWCARD: *mut c_char = b"Picked up a yellow keycard.\0".as_ptr() as *mut c_char;
const GOTREDCARD: *mut c_char = b"Picked up a red keycard.\0".as_ptr() as *mut c_char;
const GOTBLUESKUL: *mut c_char = b"Picked up a blue skull key.\0".as_ptr() as *mut c_char;
const GOTYELWSKUL: *mut c_char = b"Picked up a yellow skull key.\0".as_ptr() as *mut c_char;
const GOTREDSKULL: *mut c_char = b"Picked up a red skull key.\0".as_ptr() as *mut c_char;
const GOTINVUL: *mut c_char = b"Invulnerability!\0".as_ptr() as *mut c_char;
const GOTBERSERK: *mut c_char = b"Berserk!\0".as_ptr() as *mut c_char;
const GOTINVIS: *mut c_char = b"Partial Invisibility\0".as_ptr() as *mut c_char;
const GOTSUIT: *mut c_char = b"Radiation Shielding Suit\0".as_ptr() as *mut c_char;
const GOTMAP: *mut c_char = b"Computer Area Map\0".as_ptr() as *mut c_char;
const GOTVISOR: *mut c_char = b"Light Amplification Visor\0".as_ptr() as *mut c_char;
const GOTCLIP: *mut c_char = b"Picked up a clip.\0".as_ptr() as *mut c_char;
const GOTCLIPBOX: *mut c_char = b"Picked up a box of bullets.\0".as_ptr() as *mut c_char;
const GOTROCKET: *mut c_char = b"Picked up a rocket.\0".as_ptr() as *mut c_char;
const GOTROCKBOX: *mut c_char = b"Picked up a box of rockets.\0".as_ptr() as *mut c_char;
const GOTCELL: *mut c_char = b"Picked up an energy cell.\0".as_ptr() as *mut c_char;
const GOTCELLBOX: *mut c_char = b"Picked up an energy cell pack.\0".as_ptr() as *mut c_char;
const GOTSHELLS: *mut c_char = b"Picked up 4 shotgun shells.\0".as_ptr() as *mut c_char;
const GOTSHELLBOX: *mut c_char = b"Picked up a box of shotgun shells.\0".as_ptr() as *mut c_char;
const GOTBACKPACK: *mut c_char = b"Picked up a backpack full of ammo!\0".as_ptr() as *mut c_char;
const GOTBFG9000: *mut c_char = b"You got the BFG9000!  Oh, yes.\0".as_ptr() as *mut c_char;
const GOTCHAINGUN: *mut c_char = b"You got the chaingun!\0".as_ptr() as *mut c_char;
const GOTCHAINSAW: *mut c_char = b"A chainsaw!  Find some meat!\0".as_ptr() as *mut c_char;
const GOTLAUNCHER: *mut c_char = b"You got the rocket launcher!\0".as_ptr() as *mut c_char;
const GOTPLASMA: *mut c_char = b"You got the plasma gun!\0".as_ptr() as *mut c_char;
const GOTSHOTGUN: *mut c_char = b"You got the shotgun!\0".as_ptr() as *mut c_char;
const GOTSHOTGUN2: *mut c_char = b"You got the super shotgun!\0".as_ptr() as *mut c_char;

// ---------------------------------------------------------------------------
// DEH_String shim — identity when dehacked is disabled.
// ---------------------------------------------------------------------------

#[inline(always)]
unsafe fn DEH_String(s: *mut c_char) -> *mut c_char {
    s
}

// ---------------------------------------------------------------------------
// C globals still provided by unported C modules
// ---------------------------------------------------------------------------

extern "C" {
    static mut gameskill: c_int;
    static mut netgame: c_int;
    static mut deathmatch: c_int;
    static mut automapactive: c_int;

    fn I_Error(msg: *const c_char);
    fn I_Tactile(on: c_int, off: c_int, total: c_int);
    fn S_StartSound(origin: *mut c_void, sfx_id: c_int);
    fn P_SetMobjState(mobj: *mut mobj_t, state: c_int) -> c_int;
    fn P_RemoveMobj(mobj: *mut mobj_t);
    fn P_SpawnMobj(x: c_int, y: c_int, z: c_int, type_: c_int) -> *mut mobj_t;
    fn AM_Stop();
    fn R_PointToAngle2(x1: fixed_t, y1: fixed_t, x2: fixed_t, y2: fixed_t) -> u32;
}

// ---------------------------------------------------------------------------
// Ammo tables
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut maxammo: [c_int; NUMAMMO] = [200, 50, 300, 50];

#[no_mangle]
pub static mut clipammo: [c_int; NUMAMMO] = [10, 4, 20, 1];

// ---------------------------------------------------------------------------
// P_GiveAmmo
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_GiveAmmo(player: *mut PlayerT, ammo: c_int, mut num: c_int) -> c_int {
    if ammo == am_noammo {
        return 0;
    }
    if ammo > NUMAMMO as c_int {
        I_Error(b"P_GiveAmmo: bad type\0".as_ptr() as *const c_char);
    }
    if (*player).ammo[ammo as usize] == (*player).maxammo[ammo as usize] {
        return 0;
    }
    if num != 0 {
        num *= clipammo[ammo as usize];
    } else {
        num = clipammo[ammo as usize] / 2;
    }
    if gameskill == sk_baby || gameskill == sk_nightmare {
        num <<= 1;
    }
    let oldammo = (*player).ammo[ammo as usize];
    (*player).ammo[ammo as usize] += num;
    if (*player).ammo[ammo as usize] > (*player).maxammo[ammo as usize] {
        (*player).ammo[ammo as usize] = (*player).maxammo[ammo as usize];
    }
    if oldammo != 0 {
        return 1;
    }
    match ammo {
        am_clip => {
            if (*player).readyweapon == wp_fist {
                if (*player).weaponowned[wp_chaingun as usize] != 0 {
                    (*player).pendingweapon = wp_chaingun;
                } else {
                    (*player).pendingweapon = wp_pistol;
                }
            }
        }
        am_shell => {
            if (*player).readyweapon == wp_fist || (*player).readyweapon == wp_pistol {
                if (*player).weaponowned[wp_shotgun as usize] != 0 {
                    (*player).pendingweapon = wp_shotgun;
                }
            }
        }
        am_cell => {
            if (*player).readyweapon == wp_fist || (*player).readyweapon == wp_pistol {
                if (*player).weaponowned[wp_plasma as usize] != 0 {
                    (*player).pendingweapon = wp_plasma;
                }
            }
        }
        am_misl => {
            if (*player).readyweapon == wp_fist {
                if (*player).weaponowned[wp_missile as usize] != 0 {
                    (*player).pendingweapon = wp_missile;
                }
            }
        }
        _ => {}
    }
    1
}

// ---------------------------------------------------------------------------
// P_GiveWeapon
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_GiveWeapon(
    player: *mut PlayerT,
    weapon: c_int,
    dropped: c_int,
) -> c_int {
    if netgame != 0 && deathmatch != 2 && dropped == 0 {
        if (*player).weaponowned[weapon as usize] != 0 {
            return 0;
        }
        (*player).bonuscount += BONUSADD;
        (*player).weaponowned[weapon as usize] = 1;
        if deathmatch != 0 {
            P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 5);
        } else {
            P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 2);
        }
        (*player).pendingweapon = weapon;
        if std::ptr::eq(player, players.as_mut_ptr().add(consoleplayer as usize)) {
            S_StartSound(std::ptr::null_mut(), sfx_wpnup);
        }
        return 0;
    }
    let gaveammo: c_int;
    if weaponinfo[weapon as usize].ammo != am_noammo {
        if dropped != 0 {
            gaveammo = P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 1);
        } else {
            gaveammo = P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 2);
        }
    } else {
        gaveammo = 0;
    }
    let gaveweapon: c_int;
    if (*player).weaponowned[weapon as usize] != 0 {
        gaveweapon = 0;
    } else {
        gaveweapon = 1;
        (*player).weaponowned[weapon as usize] = 1;
        (*player).pendingweapon = weapon;
    }
    (gaveweapon != 0 || gaveammo != 0) as c_int
}

// ---------------------------------------------------------------------------
// P_GiveBody
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_GiveBody(player: *mut PlayerT, num: c_int) -> c_int {
    if (*player).health >= MAXHEALTH {
        return 0;
    }
    (*player).health += num;
    if (*player).health > MAXHEALTH {
        (*player).health = MAXHEALTH;
    }
    let mo = (*player).mo as *mut mobj_t;
    (*mo).health = (*player).health;
    1
}

// ---------------------------------------------------------------------------
// P_GiveArmor
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_GiveArmor(player: *mut PlayerT, armortype: c_int) -> c_int {
    let hits = armortype * 100;
    if (*player).armorpoints >= hits {
        return 0;
    }
    (*player).armortype = armortype;
    (*player).armorpoints = hits;
    1
}

// ---------------------------------------------------------------------------
// P_GiveCard
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_GiveCard(player: *mut PlayerT, card: c_int) {
    if (*player).cards[card as usize] != 0 {
        return;
    }
    (*player).bonuscount = BONUSADD;
    (*player).cards[card as usize] = 1;
}

// ---------------------------------------------------------------------------
// P_GivePower
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_GivePower(player: *mut PlayerT, power: c_int) -> c_int {
    let power = power as usize;
    if power == pw_invulnerability {
        (*player).powers[power] = INVULNTICS;
        return 1;
    }
    if power == pw_invisibility {
        (*player).powers[power] = INVISTICS;
        let mo = (*player).mo as *mut mobj_t;
        (*mo).flags |= MF_SHADOW;
        return 1;
    }
    if power == pw_infrared {
        (*player).powers[power] = INFRATICS;
        return 1;
    }
    if power == pw_ironfeet {
        (*player).powers[power] = IRONTICS;
        return 1;
    }
    if power == pw_strength {
        P_GiveBody(player, 100);
        (*player).powers[power] = 1;
        return 1;
    }
    if (*player).powers[power] != 0 {
        return 0;
    }
    (*player).powers[power] = 1;
    1
}

// ---------------------------------------------------------------------------
// P_TouchSpecialThing
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_TouchSpecialThing(special: *mut mobj_t, toucher: *mut mobj_t) {
    let _test_spr = SPR_ARM1;
    let delta = (*special).z - (*toucher).z;
    if delta > (*toucher).height || delta < -8 * FRACUNIT {
        return;
    }

    let mut sound: c_int = sfx_itemup;
    let player = (*toucher).player as *mut PlayerT;

    match (*special).sprite {
        // armor
        SPR_ARM1 => {
            if P_GiveArmor(player, 1) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTARMOR);
        }
        SPR_ARM2 => {
            if P_GiveArmor(player, DEH_DEFAULT_BLUE_ARMOR_CLASS) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTMEGA);
        }
        // bonus items
        SPR_BON1 => {
            (*player).health += 1;
            if (*player).health > DEH_DEFAULT_MAX_HEALTH {
                (*player).health = DEH_DEFAULT_MAX_HEALTH;
            }
            let mo = (*player).mo as *mut mobj_t;
            (*mo).health = (*player).health;
            (*player).message = DEH_String(GOTHTHBONUS);
        }
        SPR_BON2 => {
            (*player).armorpoints += 1;
            if (*player).armorpoints > DEH_DEFAULT_MAX_ARMOR {
                (*player).armorpoints = DEH_DEFAULT_MAX_ARMOR;
            }
            if (*player).armortype == 0 {
                (*player).armortype = 1;
            }
            (*player).message = DEH_String(GOTARMBONUS);
        }
        SPR_SOUL => {
            (*player).health += DEH_DEFAULT_SOULSPHERE_HEALTH;
            if (*player).health > DEH_DEFAULT_MAX_SOULSPHERE {
                (*player).health = DEH_DEFAULT_MAX_SOULSPHERE;
            }
            let mo = (*player).mo as *mut mobj_t;
            (*mo).health = (*player).health;
            (*player).message = DEH_String(GOTSUPER);
            sound = sfx_getpow;
        }
        SPR_MEGA => {
            if gamemode != commercial {
                return;
            }
            (*player).health = DEH_DEFAULT_MEGASPHERE_HEALTH;
            let mo = (*player).mo as *mut mobj_t;
            (*mo).health = (*player).health;
            P_GiveArmor(player, 2);
            (*player).message = DEH_String(GOTMSPHERE);
            sound = sfx_getpow;
        }
        // cards
        SPR_BKEY => {
            if (*player).cards[it_bluecard as usize] == 0 {
                (*player).message = DEH_String(GOTBLUECARD);
            }
            P_GiveCard(player, it_bluecard);
            if netgame == 0 {
                // fall through to common epilogue
            } else {
                return;
            }
        }
        SPR_YKEY => {
            if (*player).cards[it_yellowcard as usize] == 0 {
                (*player).message = DEH_String(GOTYELWCARD);
            }
            P_GiveCard(player, it_yellowcard);
            if netgame == 0 {
                // fall through
            } else {
                return;
            }
        }
        SPR_RKEY => {
            if (*player).cards[it_redcard as usize] == 0 {
                (*player).message = DEH_String(GOTREDCARD);
            }
            P_GiveCard(player, it_redcard);
            if netgame == 0 {
                // fall through
            } else {
                return;
            }
        }
        SPR_BSKU => {
            if (*player).cards[it_blueskull as usize] == 0 {
                (*player).message = DEH_String(GOTBLUESKUL);
            }
            P_GiveCard(player, it_blueskull);
            if netgame == 0 {
                // fall through
            } else {
                return;
            }
        }
        SPR_YSKU => {
            if (*player).cards[it_yellowskull as usize] == 0 {
                (*player).message = DEH_String(GOTYELWSKUL);
            }
            P_GiveCard(player, it_yellowskull);
            if netgame == 0 {
                // fall through
            } else {
                return;
            }
        }
        SPR_RSKU => {
            if (*player).cards[it_redskull as usize] == 0 {
                (*player).message = DEH_String(GOTREDSKULL);
            }
            P_GiveCard(player, it_redskull);
            if netgame == 0 {
                // fall through
            } else {
                return;
            }
        }
        // medikits, heals
        SPR_STIM => {
            if P_GiveBody(player, 10) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTSTIM);
        }
        SPR_MEDI => {
            if P_GiveBody(player, 25) == 0 {
                return;
            }
            if (*player).health < 25 {
                (*player).message = DEH_String(GOTMEDINEED);
            } else {
                (*player).message = DEH_String(GOTMEDIKIT);
            }
        }
        // power ups
        SPR_PINV => {
            if P_GivePower(player, pw_invulnerability as c_int) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTINVUL);
            sound = sfx_getpow;
        }
        SPR_PSTR => {
            if P_GivePower(player, pw_strength as c_int) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTBERSERK);
            if (*player).readyweapon != wp_fist {
                (*player).pendingweapon = wp_fist;
            }
            sound = sfx_getpow;
        }
        SPR_PINS => {
            if P_GivePower(player, pw_invisibility as c_int) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTINVIS);
            sound = sfx_getpow;
        }
        SPR_SUIT => {
            if P_GivePower(player, pw_ironfeet as c_int) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTSUIT);
            sound = sfx_getpow;
        }
        SPR_PMAP => {
            if P_GivePower(player, pw_allmap as c_int) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTMAP);
            sound = sfx_getpow;
        }
        SPR_PVIS => {
            if P_GivePower(player, pw_infrared as c_int) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTVISOR);
            sound = sfx_getpow;
        }
        // ammo
        SPR_CLIP => {
            if (*special).flags & MF_DROPPED != 0 {
                if P_GiveAmmo(player, am_clip, 0) == 0 {
                    return;
                }
            } else {
                if P_GiveAmmo(player, am_clip, 1) == 0 {
                    return;
                }
            }
            (*player).message = DEH_String(GOTCLIP);
        }
        SPR_AMMO => {
            if P_GiveAmmo(player, am_clip, 5) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTCLIPBOX);
        }
        SPR_ROCK => {
            if P_GiveAmmo(player, am_misl, 1) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTROCKET);
        }
        SPR_BROK => {
            if P_GiveAmmo(player, am_misl, 5) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTROCKBOX);
        }
        SPR_CELL => {
            if P_GiveAmmo(player, am_cell, 1) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTCELL);
        }
        SPR_CELP => {
            if P_GiveAmmo(player, am_cell, 5) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTCELLBOX);
        }
        SPR_SHEL => {
            if P_GiveAmmo(player, am_shell, 1) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTSHELLS);
        }
        SPR_SBOX => {
            if P_GiveAmmo(player, am_shell, 5) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTSHELLBOX);
        }
        SPR_BPAK => {
            if (*player).backpack == 0 {
                for i in 0..NUMAMMO {
                    (*player).maxammo[i] *= 2;
                }
                (*player).backpack = 1;
            }
            for i in 0..NUMAMMO {
                P_GiveAmmo(player, i as c_int, 1);
            }
            (*player).message = DEH_String(GOTBACKPACK);
        }
        // weapons
        SPR_BFUG => {
            if P_GiveWeapon(player, wp_bfg, 0) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTBFG9000);
            sound = sfx_wpnup;
        }
        SPR_MGUN => {
            if P_GiveWeapon(
                player,
                wp_chaingun,
                (((*special).flags & MF_DROPPED) != 0) as c_int,
            ) == 0
            {
                return;
            }
            (*player).message = DEH_String(GOTCHAINGUN);
            sound = sfx_wpnup;
        }
        SPR_CSAW => {
            if P_GiveWeapon(player, wp_chainsaw, 0) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTCHAINSAW);
            sound = sfx_wpnup;
        }
        SPR_LAUN => {
            if P_GiveWeapon(player, wp_missile, 0) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTLAUNCHER);
            sound = sfx_wpnup;
        }
        SPR_PLAS => {
            if P_GiveWeapon(player, wp_plasma, 0) == 0 {
                return;
            }
            (*player).message = DEH_String(GOTPLASMA);
            sound = sfx_wpnup;
        }
        SPR_SHOT => {
            if P_GiveWeapon(
                player,
                wp_shotgun,
                (((*special).flags & MF_DROPPED) != 0) as c_int,
            ) == 0
            {
                return;
            }
            (*player).message = DEH_String(GOTSHOTGUN);
            sound = sfx_wpnup;
        }
        SPR_SGN2 => {
            if P_GiveWeapon(
                player,
                wp_supershotgun,
                (((*special).flags & MF_DROPPED) != 0) as c_int,
            ) == 0
            {
                return;
            }
            (*player).message = DEH_String(GOTSHOTGUN2);
            sound = sfx_wpnup;
        }
        _ => {
            I_Error(b"P_SpecialThing: Unknown gettable thing\0".as_ptr() as *const c_char);
        }
    }

    if (*special).flags & MF_COUNTITEM != 0 {
        (*player).itemcount += 1;
    }
    P_RemoveMobj(special);
    (*player).bonuscount += BONUSADD;
    if std::ptr::eq(player, players.as_mut_ptr().add(consoleplayer as usize)) {
        S_StartSound(std::ptr::null_mut(), sound);
    }
}

// ---------------------------------------------------------------------------
// P_KillMobj
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_KillMobj(source: *mut mobj_t, target: *mut mobj_t) {
    let info = (*target).info as *mut MobjInfo;
    (*target).flags &= !(MF_SHOOTABLE | MF_FLOAT | MF_SKULLFLY);
    if (*target).mobjtype != MT_SKULL {
        (*target).flags &= !MF_NOGRAVITY;
    }
    (*target).flags |= MF_CORPSE | MF_DROPOFF;
    (*target).height >>= 2;

    if !source.is_null() && !(*source).player.is_null() {
        let source_player = (*source).player as *mut PlayerT;
        if (*target).flags & MF_COUNTKILL != 0 {
            (*source_player).killcount += 1;
        }
        if !(*target).player.is_null() {
            let target_player = (*target).player as *mut PlayerT;
            let idx = target_player.offset_from(players.as_mut_ptr()) as usize;
            (*source_player).frags[idx] += 1;
        }
    } else if netgame == 0 && (*target).flags & MF_COUNTKILL != 0 {
        players[0].killcount += 1;
    }

    if !(*target).player.is_null() {
        let target_player = (*target).player as *mut PlayerT;
        if source.is_null() {
            let idx = target_player.offset_from(players.as_mut_ptr()) as usize;
            (*target_player).frags[idx] += 1;
        }
        (*target).flags &= !MF_SOLID;
        (*target_player).playerstate = 1; // PST_DEAD
        P_DropWeapon(target_player);
        if std::ptr::eq(
            target_player,
            players.as_mut_ptr().add(consoleplayer as usize),
        ) && automapactive != 0
        {
            AM_Stop();
        }
    }

    if (*target).health < -(*info).spawnhealth && (*info).xdeathstate != 0 {
        P_SetMobjState(target, (*info).xdeathstate);
    } else {
        P_SetMobjState(target, (*info).deathstate);
    }
    (*target).tics -= P_Random() & 3;
    if (*target).tics < 1 {
        (*target).tics = 1;
    }

    if gameversion == exe_chex {
        return;
    }

    let item: c_int = match (*target).mobjtype {
        MT_WOLFSS | MT_POSSESSED => MT_CLIP,
        MT_SHOTGUY => MT_SHOTGUN,
        MT_CHAINGUY => MT_CHAINGUN,
        _ => return,
    };

    let mo = P_SpawnMobj((*target).x, (*target).y, ONFLOORZ, item);
    (*mo).flags |= MF_DROPPED;
}

// ---------------------------------------------------------------------------
// P_DamageMobj
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_DamageMobj(
    target: *mut mobj_t,
    inflictor: *mut mobj_t,
    source: *mut mobj_t,
    mut damage: c_int,
) {
    if ((*target).flags & MF_SHOOTABLE) == 0 {
        return;
    }
    if (*target).health <= 0 {
        return;
    }
    if ((*target).flags & MF_SKULLFLY) != 0 {
        (*target).momx = 0;
        (*target).momy = 0;
        (*target).momz = 0;
    }

    let player = (*target).player as *mut PlayerT;
    if !player.is_null() && gameskill == sk_baby {
        damage >>= 1;
    }

    if !inflictor.is_null()
        && ((*target).flags & MF_NOCLIP) == 0
        && (source.is_null()
            || (*source).player.is_null()
            || (*((*source).player as *mut PlayerT)).readyweapon != wp_chainsaw)
    {
        let mut ang = R_PointToAngle2((*inflictor).x, (*inflictor).y, (*target).x, (*target).y);
        let info = (*target).info as *mut MobjInfo;
        let mut thrust = damage * (FRACUNIT >> 3) * 100 / (*info).mass;

        if damage < 40
            && damage > (*target).health
            && (*target).z - (*inflictor).z > 64 * FRACUNIT
            && (P_Random() & 1) != 0
        {
            ang = ang.wrapping_add(ANG180);
            thrust *= 4;
        }

        ang >>= ANGLETOFINESHIFT;
        (*target).momx += FixedMul(thrust, *finecosine.0.add(ang as usize));
        (*target).momy += FixedMul(thrust, finesine[ang as usize]);
    }

    if !player.is_null() {
        // end of game hell hack
        if (*(*(*target).subsector).sector).special == 11 && damage >= (*target).health {
            damage = (*target).health - 1;
        }

        if damage < 1000
            && (((*player).cheats & CF_GODMODE) != 0 || (*player).powers[pw_invulnerability] != 0)
        {
            return;
        }

        if (*player).armortype != 0 {
            let saved = if (*player).armortype == 1 {
                damage / 3
            } else {
                damage / 2
            };
            let mut saved_actual = saved;
            if (*player).armorpoints <= saved_actual {
                saved_actual = (*player).armorpoints;
                (*player).armortype = 0;
            }
            (*player).armorpoints -= saved_actual;
            damage -= saved_actual;
        }
        (*player).health -= damage;
        if (*player).health < 0 {
            (*player).health = 0;
        }
        (*player).attacker = source as *mut crate::doom::d_player::mobj_t;
        (*player).damagecount += damage;
        if (*player).damagecount > 100 {
            (*player).damagecount = 100;
        }
        let temp = if damage < 100 { damage } else { 100 };
        if std::ptr::eq(player, players.as_mut_ptr().add(consoleplayer as usize)) {
            I_Tactile(40, 10, 40 + temp * 2);
        }
    }

    (*target).health -= damage;
    if (*target).health <= 0 {
        P_KillMobj(source, target);
        return;
    }

    let info = (*target).info as *mut MobjInfo;
    if P_Random() < (*info).painchance && ((*target).flags & MF_SKULLFLY) == 0 {
        (*target).flags |= MF_JUSTHIT;
        P_SetMobjState(target, (*info).painstate);
    }

    (*target).reactiontime = 0;

    if (((*target).threshold == 0 || (*target).mobjtype == MT_VILE)
        && !source.is_null()
        && source != target
        && (*source).mobjtype != MT_VILE)
    {
        (*target).target = source;
        (*target).threshold = BASETHRESHOLD;
        let state_ptr = (*target).state as *mut State;
        let spawnstate_ptr = &info::states[(*info).spawnstate as usize] as *const State;
        if state_ptr == spawnstate_ptr as *mut State && (*info).seestate != S_NULL {
            P_SetMobjState(target, (*info).seestate);
        }
    }
}

// ---------------------------------------------------------------------------
// Link anchor
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_Inter_Link_Anchor() {
    unsafe {
        let _ = maxammo.as_mut_ptr() as usize;
        let _ = clipammo.as_mut_ptr() as usize;
    }
    let _ = P_GiveAmmo as *const () as usize;
    let _ = P_GiveWeapon as *const () as usize;
    let _ = P_GiveBody as *const () as usize;
    let _ = P_GiveArmor as *const () as usize;
    let _ = P_GiveCard as *const () as usize;
    let _ = P_GivePower as *const () as usize;
    let _ = P_TouchSpecialThing as *const () as usize;
    let _ = P_KillMobj as *const () as usize;
    let _ = P_DamageMobj as *const () as usize;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn maxammo_defaults() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(maxammo, [200, 50, 300, 50]);
        }
    }

    #[test]
    fn clipammo_defaults() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(clipammo, [10, 4, 20, 1]);
        }
    }

    #[test]
    fn constants_match() {
        assert_eq!(BONUSADD, 6);
        assert_eq!(NUMAMMO, 4);
        assert_eq!(MAXHEALTH, 100);
        assert_eq!(ONFLOORZ, i32::MIN);
        assert_eq!(FRACUNIT, 65536);
        assert_eq!(ANG180, 0x80000000);
        assert_eq!(ANGLETOFINESHIFT, 19);
        assert_eq!(BASETHRESHOLD, 100);
        assert_eq!(CF_GODMODE, 2);
        assert_eq!(sk_baby, 0);
        assert_eq!(sk_nightmare, 4);
        assert_eq!(INVULNTICS, 1050);
        assert_eq!(INVISTICS, 2100);
        assert_eq!(INFRATICS, 4200);
        assert_eq!(IRONTICS, 2100);
        assert_eq!(am_noammo, 5);
        assert_eq!(MF_DROPPED, 0x00020000);
        assert_eq!(MF_COUNTITEM, 0x00800000);
        assert_eq!(MF_SHOOTABLE, 4);
        assert_eq!(MF_SKULLFLY, 0x01000000);
        assert_eq!(exe_chex, 9);
        assert_eq!(commercial, 2);
    }

    #[test]
    fn deh_defaults_match() {
        assert_eq!(DEH_DEFAULT_MAX_HEALTH, 200);
        assert_eq!(DEH_DEFAULT_MAX_ARMOR, 200);
        assert_eq!(DEH_DEFAULT_GREEN_ARMOR_CLASS, 1);
        assert_eq!(DEH_DEFAULT_BLUE_ARMOR_CLASS, 2);
        assert_eq!(DEH_DEFAULT_MAX_SOULSPHERE, 200);
        assert_eq!(DEH_DEFAULT_SOULSPHERE_HEALTH, 100);
        assert_eq!(DEH_DEFAULT_MEGASPHERE_HEALTH, 200);
    }
}
