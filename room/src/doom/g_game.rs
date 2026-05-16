//! Rust port of vendor/doomgeneric/g_game.c.
//!
//! Core game loop, demo recording/playback, player management, save/load.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::types::Boolean;
use std::ptr;

use crate::doom::d_mode::{
    commercial, doom, exe_chex, exe_doom_1_2, exe_doom_1_666, exe_doom_1_7, exe_doom_1_8,
    exe_doom_1_9, exe_final2, exe_ultimate, shareware,
};
use crate::doom::d_player::{PlayerT, TiccmdT, MAXPLAYERS};
use crate::doom::doomstat::{gamemission, gamemode, gameversion};
use crate::doom::m_random::P_Random;
use crate::doom::p_inter::maxammo;
use crate::doom::p_setup::{
    deathmatch_p, deathmatchstarts, playerstarts,
    mapthing_t as SetupMapThing,
};
use crate::doom::p_telept::{mapthing_t, mobj_t, sector_t, subsector_t};
use crate::doom::tables::{finecosine, finesine, finetangent};
use crate::doom::wi_stuff::{wbplayerstruct_t, wbstartstruct_t};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type boolean = c_int;
type byte = u8;
type skill_t = c_int;
type fixed_t = c_int;
type c_long = libc::c_long;

// ---------------------------------------------------------------------------
// Game-state constants (from doomstat.h)
// ---------------------------------------------------------------------------

const GS_LEVEL: c_int = 0;
const GS_INTERMISSION: c_int = 1;
const GS_FINALE: c_int = 2;
const GS_DEMOSCREEN: c_int = 3;

// ---------------------------------------------------------------------------
// Game-action constants (from doomstat.h)
// ---------------------------------------------------------------------------

const ga_nothing: c_int = 0;
const ga_loadlevel: c_int = 1;
const ga_newgame: c_int = 2;
const ga_loadgame: c_int = 3;
const ga_savegame: c_int = 4;
const ga_playdemo: c_int = 5;
const ga_completed: c_int = 6;
const ga_victory: c_int = 7;
const ga_worlddone: c_int = 8;
const ga_screenshot: c_int = 9;

// ---------------------------------------------------------------------------
// Player state constants (from d_player.h)
// ---------------------------------------------------------------------------

const PST_LIVE: c_int = 0;
const PST_DEAD: c_int = 1;
const PST_REBORN: c_int = 2;

// ---------------------------------------------------------------------------
// Weapon constants (from doomdef.h)
// ---------------------------------------------------------------------------

const wp_fist: c_int = 0;
const wp_pistol: c_int = 1;
const wp_chainsaw: c_int = 7;
const wp_supershotgun: c_int = 8;
const wp_plasma: c_int = 5;
const wp_bfg: c_int = 6;
const wp_nochange: c_int = 9;

// ---------------------------------------------------------------------------
// Power types (from doomdef.h)
// ---------------------------------------------------------------------------

const pw_strength: usize = 1;

// ---------------------------------------------------------------------------
// Button constants (from d_event.h)
// ---------------------------------------------------------------------------

const BT_ATTACK: u8 = 1;
const BT_USE: u8 = 2;
const BT_CHANGE: u8 = 4;
const BT_WEAPONMASK: u8 = 8 + 16 + 32;
const BT_WEAPONSHIFT: u8 = 3;
const BT_SPECIAL: u8 = 128;
const BT_SPECIALMASK: u8 = 3;
const BTS_PAUSE: u8 = 1;
const BTS_SAVEGAME: u8 = 2;
const BTS_SAVEMASK: u8 = 4 + 8 + 16;
const BTS_SAVESHIFT: u8 = 2;

// ---------------------------------------------------------------------------
// Miscellaneous constants
// ---------------------------------------------------------------------------

const NUMKEYS: usize = 256;
const MAX_MOUSE_BUTTONS: usize = 8;
const MAX_JOY_BUTTONS: usize = 20;
const SAVEGAMESIZE: c_long = 0x2c000;
const DEMOMARKER: byte = 0x80;
const VERSIONSIZE: usize = 16;
const am_clip: usize = 0;
const MT_TFOG: c_int = 28;
const sfx_telept: c_int = 35;

// Initial player values when no DEH patch is applied.
const DEH_INITIAL_HEALTH: c_int = 100;
const DEH_INITIAL_BULLETS: c_int = 50;

// event_t from d_event.rs
use crate::doom::d_event::event_t;

// ---------------------------------------------------------------------------
// Weapon ordering table (for prev/next weapon cycling)
// ---------------------------------------------------------------------------

struct WeaponOrder {
    weapon: c_int,
    weapon_num: c_int,
}

static WEAPON_ORDER_TABLE: [WeaponOrder; 9] = [
    WeaponOrder {
        weapon: wp_fist,
        weapon_num: wp_fist,
    },
    WeaponOrder {
        weapon: wp_chainsaw,
        weapon_num: wp_fist,
    },
    WeaponOrder {
        weapon: wp_pistol,
        weapon_num: wp_pistol,
    },
    WeaponOrder {
        weapon: 3,
        /* shotgun */ weapon_num: 3,
    },
    WeaponOrder {
        weapon: 8,
        /* supershotgun */ weapon_num: 3,
    },
    WeaponOrder {
        weapon: 4,
        /* chaingun */ weapon_num: 4,
    },
    WeaponOrder {
        weapon: wp_plasma - 1,
        /* missile */ weapon_num: wp_plasma - 1,
    },
    WeaponOrder {
        weapon: wp_plasma,
        weapon_num: wp_plasma,
    },
    WeaponOrder {
        weapon: wp_bfg,
        weapon_num: wp_bfg,
    },
];

// ---------------------------------------------------------------------------
// Public globals — #[no_mangle] so other Rust modules can access them via
// `extern "C"` declarations (the same pattern used throughout this codebase).
// ---------------------------------------------------------------------------

/// Gamestate the last time G_Ticker was called.
#[no_mangle]
pub static mut oldgamestate: c_int = GS_DEMOSCREEN;

#[no_mangle]
pub static mut gameaction: c_int = ga_nothing;

#[no_mangle]
pub static mut gamestate: c_int = GS_DEMOSCREEN;

#[no_mangle]
pub static mut gameskill: c_int = 0;

#[no_mangle]
pub static mut respawnmonsters: boolean = 0;

#[no_mangle]
pub static mut gameepisode: c_int = 0;

#[no_mangle]
pub static mut gamemap: c_int = 0;

/// If non-zero, exit the level after this number of minutes.
#[no_mangle]
pub static mut timelimit: c_int = 0;

#[no_mangle]
pub static mut paused: boolean = 0;

#[no_mangle]
pub static mut sendpause: boolean = 0;

#[no_mangle]
pub static mut sendsave: boolean = 0;

#[no_mangle]
pub static mut usergame: boolean = 0;

#[no_mangle]
pub static mut timingdemo: boolean = 0;

#[no_mangle]
pub static mut nodrawers: boolean = 0;

#[no_mangle]
pub static mut starttime: c_int = 0;

#[no_mangle]
pub static mut viewactive: boolean = 0;

#[no_mangle]
pub static mut deathmatch: c_int = 0;

#[no_mangle]
pub static mut netgame: boolean = 0;

#[no_mangle]
pub static mut playeringame: [boolean; MAXPLAYERS] = [0; MAXPLAYERS];

#[no_mangle]
pub static mut players: [PlayerT; MAXPLAYERS] = unsafe { std::mem::zeroed() };

#[no_mangle]
pub static mut turbodetected: [boolean; MAXPLAYERS] = [0; MAXPLAYERS];

#[no_mangle]
pub static mut consoleplayer: c_int = 0;

#[no_mangle]
pub static mut displayplayer: c_int = 0;

#[no_mangle]
pub static mut levelstarttic: c_int = 0;

#[no_mangle]
pub static mut totalkills: c_int = 0;

#[no_mangle]
pub static mut totalitems: c_int = 0;

#[no_mangle]
pub static mut totalsecret: c_int = 0;

#[no_mangle]
pub static mut demoname: *mut c_char = ptr::null_mut();

#[no_mangle]
pub static mut demorecording: boolean = 0;

/// cph's doom 1.91 longtics hack
#[no_mangle]
pub static mut longtics: boolean = 0;

/// low resolution turning for longtics
#[no_mangle]
pub static mut lowres_turn: boolean = 0;

#[no_mangle]
pub static mut demoplayback: boolean = 0;

#[no_mangle]
pub static mut netdemo: boolean = 0;

#[no_mangle]
pub static mut demobuffer: *mut byte = ptr::null_mut();

#[no_mangle]
pub static mut demo_p: *mut byte = ptr::null_mut();

#[no_mangle]
pub static mut demoend: *mut byte = ptr::null_mut();

#[no_mangle]
pub static mut singledemo: boolean = 0;

#[no_mangle]
pub static mut precache: boolean = 1; // true by default

#[no_mangle]
pub static mut testcontrols: boolean = 0;

#[no_mangle]
pub static mut testcontrols_mousespeed: c_int = 0;

#[no_mangle]
pub static mut wminfo: wbstartstruct_t = wbstartstruct_t {
    epsd: 0,
    didsecret: 0,
    last: 0,
    next: 0,
    maxkills: 0,
    maxitems: 0,
    maxsecret: 0,
    maxfrags: 0,
    partime: 0,
    pnum: 0,
    plyr: [wbplayerstruct_t {
        in_: 0,
        skills: 0,
        sitems: 0,
        ssecret: 0,
        stime: 0,
        frags: [0; 4],
        score: 0,
    }; MAXPLAYERS],
};

#[no_mangle]
pub static mut consistancy: [[byte; 128]; MAXPLAYERS] = [[0; 128]; MAXPLAYERS];

#[no_mangle]
pub static mut bodyqueslot: c_int = 0;

#[no_mangle]
pub static mut vanilla_savegame_limit: c_int = 1;

#[no_mangle]
pub static mut vanilla_demo_limit: c_int = 1;

/// Forward-movement speed table: [slow=0x19, fast=0x32] (fixed_t per tic).
#[no_mangle]
pub static mut forwardmove: [fixed_t; 2] = [0x19, 0x32];

/// Lateral strafe speed table: [slow=0x18, fast=0x28] (fixed_t per tic).
#[no_mangle]
pub static mut sidemove: [fixed_t; 2] = [0x18, 0x28];

/// Turn-speed table: [normal=640, fast=1280, slow=320] (BAM units per tic).
#[no_mangle]
pub static mut angleturn: [fixed_t; 3] = [640, 1280, 320];

#[no_mangle]
pub static mut secretexit: boolean = 0;

#[no_mangle]
pub static mut defdemoname: *mut c_char = ptr::null_mut();

#[no_mangle]
pub static mut savename: [c_char; 256] = [0; 256];

/// Deferred G_DeferedInitNew parameters.
#[no_mangle]
pub static mut d_skill: skill_t = 0;
#[no_mangle]
pub static mut d_episode: c_int = 0;
#[no_mangle]
pub static mut d_map: c_int = 0;

/// Doom episode par times (episodes 1-3, maps 1-9).
#[no_mangle]
pub static mut pars: [[c_int; 10]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 30, 75, 120, 90, 165, 180, 180, 30, 165],
    [0, 90, 90, 90, 120, 90, 360, 240, 30, 170],
    [0, 90, 45, 90, 150, 90, 90, 165, 30, 135],
];

/// Doom II par times (maps 1-32).
#[no_mangle]
pub static mut cpars: [c_int; 32] = [
    30, 90, 120, 120, 90, 150, 120, 120, 270, 90, 210, 150, 150, 150, 210, 150, 420, 150, 210, 150,
    240, 150, 180, 150, 150, 300, 330, 420, 300, 180, 120, 30,
];

/// Circular queue of player corpse pointers.
#[no_mangle]
pub static mut bodyque: [*mut mobj_t; 32] = [ptr::null_mut(); 32];

// ---------------------------------------------------------------------------
// Module-private statics
// ---------------------------------------------------------------------------

static mut GAMEKEYDOWN: [boolean; NUMKEYS] = [0; NUMKEYS];
static mut TURNHELD: c_int = 0;

static mut MOUSEARRAY: [boolean; MAX_MOUSE_BUTTONS + 1] = [0; MAX_MOUSE_BUTTONS + 1];
// mousebuttons is &mousearray[1] in C — negative indexing; access via MOUSEARRAY[1+n]
static mut MOUSEX: c_int = 0;
static mut MOUSEY: c_int = 0;

static mut DCLICKTIME: c_int = 0;
static mut DCLICKSTATE: boolean = 0;
static mut DCLICKS: c_int = 0;
static mut DCLICKTIME2: c_int = 0;
static mut DCLICKSTATE2: boolean = 0;
static mut DCLICKS2: c_int = 0;

static mut JOYXMOVE: c_int = 0;
static mut JOYYMOVE: c_int = 0;
static mut JOYSTRAFEMOVE: c_int = 0;
static mut JOYARRAY: [boolean; MAX_JOY_BUTTONS + 1] = [0; MAX_JOY_BUTTONS + 1];
// joybuttons is &joyarray[1] in C — negative indexing; access via JOYARRAY[1+n]

static mut SAVEGAMESLOT: c_int = 0;
static mut SAVEDESCRIPTION: [c_char; 32] = [0; 32];

static mut NEXT_WEAPON: c_int = 0;

/// Carry for low-resolution turn rounding (static local in G_BuildTiccmd).
static mut LOWRES_TURN_CARRY: i16 = 0;

/// Buffer for turbo-cheat message (static local in G_Ticker).
static mut TURBOMESSAGE: [c_char; 80] = [0; 80];

/// Buffer for DemoVersionDescription (static local in G_DoPlayDemo).
static mut DEMOVERSIONBUF: [c_char; 16] = [0; 16];

// ---------------------------------------------------------------------------
// External function and variable declarations
// ---------------------------------------------------------------------------

extern "C" {
    // libc
    fn snprintf(buf: *mut c_char, len: usize, fmt: *const c_char, ...) -> c_int;

    // i_system.rs — keep as variadic extern because call sites pass format args
    fn I_Error(format: *const c_char, ...) -> !;
    fn I_Quit() -> !;
}

// d_loop.rs
use crate::doom::d_loop::{gametic, ticdup};

// d_main.rs
use crate::doom::d_main::{D_AdvanceDemo, D_PageTicker, fastparm, nomonsters, respawnparm, wipegamestate};

// am_map.rs
use crate::doom::am_map::{automapactive, AM_Responder, AM_Stop, AM_Ticker};

// d_net.rs
use crate::doom::d_net::netcmds;

// p_saveg.rs
use crate::doom::p_saveg::{
    save_stream, savegame_error, P_ArchivePlayers, P_ArchiveSpecials, P_ArchiveThinkers,
    P_ArchiveWorld, P_ReadSaveGameEOF, P_ReadSaveGameHeader, P_SaveGameFile, P_TempSaveGameFile,
    P_UnArchivePlayers, P_UnArchiveSpecials, P_UnArchiveThinkers, P_UnArchiveWorld,
    P_WriteSaveGameEOF, P_WriteSaveGameHeader,
};

// p_setup.rs
use crate::doom::p_setup::P_SetupLevel;

// p_mobj.rs
use crate::doom::p_mobj::{P_RemoveMobj, P_SpawnMobj, P_SpawnPlayer};

// p_map.rs
use crate::doom::p_map::P_CheckPosition;

// r_main.rs
use crate::doom::r_main::{setsizeneeded, R_ExecuteSetViewSize, R_PointInSubsector};

// r_data.rs
use crate::doom::r_data::{R_FlatNumForName, R_TextureNumForName};

// r_sky.rs
use crate::doom::r_sky::{skyflatnum, skytexture};

// r_draw.rs
use crate::doom::r_draw::R_FillBackScreen;

// z_zone.rs
use crate::doom::z_zone::{Z_CheckHeap, Z_Free, Z_Malloc};

// p_tick.rs
use crate::doom::p_tick::{leveltime, P_Ticker};

// st_stuff.rs
use crate::doom::st_stuff::{ST_Responder, ST_Ticker};

// hu_stuff.rs
use crate::doom::hu_stuff::{player_names, HU_dequeueChatChar, HU_Responder, HU_Ticker};

// wi_stuff.rs
use crate::doom::wi_stuff::{WI_End, WI_Start, WI_Ticker};

// f_finale.rs
use crate::doom::f_finale::{F_Responder, F_StartFinale, F_Ticker};

// s_sound.rs
use crate::doom::s_sound::{S_PauseSound, S_ResumeSound, S_StartSound};

// m_menu.rs
use crate::doom::m_menu::{mouseSensitivity, M_StartControlPanel};

// m_misc.rs
use crate::doom::m_misc::{M_StringCopy, M_TempFile, M_WriteFile, M_snprintf_clamp};

// m_argv.rs
use crate::doom::m_argv::{myargv, M_CheckParm, M_CheckParmWithArgs};

// m_random.rs
use crate::doom::m_random::{rndindex, M_ClearRandom};

// statdump.rs
use crate::doom::statdump::StatCopy;

// v_video.rs
use crate::doom::v_video::V_ScreenShot;

// i_timer.rs
use crate::doom::i_timer::I_GetTime;

// w_wad.rs
use crate::doom::w_wad::{W_CacheLumpName, W_CheckNumForName, W_ReleaseLumpName};

// m_controls.rs
use crate::doom::m_controls::{
    dclick_use, joybfire, joybprevweapon, joybspeed, joybstrafe, joybstrafeleft, joybstraferight,
    joybuse, joybnextweapon, key_demo_quit, key_down, key_fire, key_left, key_nextweapon,
    key_pause, key_prevweapon, key_right, key_speed, key_spy, key_strafe, key_strafeleft,
    key_straferight, key_up, key_use, key_weapon1, key_weapon2, key_weapon3, key_weapon4,
    key_weapon5, key_weapon6, key_weapon7, key_weapon8, mousebbackward, mousebfire,
    mousebforward, mousebnextweapon, mousebprevweapon, mousebstrafe, mousebstrafeleft,
    mousebstraferight, mousebuse,
};

use crate::doom::z_zone::{PU_CACHE, PU_STATIC};

// ---------------------------------------------------------------------------
// DEH_String identity (no dehacked support)
// ---------------------------------------------------------------------------

#[inline]
unsafe fn DEH_String(s: *const c_char) -> *const c_char {
    s
}

// ---------------------------------------------------------------------------
// logical_gamemission helper (mirrors doomstat.h macro)
// ---------------------------------------------------------------------------

#[inline]
unsafe fn logical_gamemission() -> c_int {
    use crate::doom::d_mode::{doom2, pack_plut, pack_tnt};
    if gamemission == pack_tnt || gamemission == pack_plut {
        doom2
    } else {
        gamemission
    }
}

// ---------------------------------------------------------------------------
// Mouse/joystick button accessors
// (In C: mousebuttons = &mousearray[1], joybuttons = &joyarray[1],
//  allowing negative index -1 meaning "no button".)
// ---------------------------------------------------------------------------

#[inline]
unsafe fn mousebutton(n: c_int) -> boolean {
    if n < 0 || n >= MAX_MOUSE_BUTTONS as c_int {
        return 0;
    }
    MOUSEARRAY[(n + 1) as usize]
}

#[inline]
unsafe fn joybutton(n: c_int) -> boolean {
    if n < 0 || n >= MAX_JOY_BUTTONS as c_int {
        return 0;
    }
    JOYARRAY[(n + 1) as usize]
}

// ---------------------------------------------------------------------------
// G_CmdChecksum
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_CmdChecksum(cmd: *const TiccmdT) -> c_int {
    let n = std::mem::size_of::<TiccmdT>() / 4 - 1;
    let words = cmd as *const c_int;
    let mut sum: c_int = 0;
    for i in 0..n {
        sum = sum.wrapping_add(*words.add(i));
    }
    sum
}

// ---------------------------------------------------------------------------
// WeaponSelectable (static)
// ---------------------------------------------------------------------------

unsafe fn weapon_selectable(weapon: c_int) -> bool {
    // Can't select supershotgun in Doom 1.
    if weapon == wp_supershotgun && logical_gamemission() == doom {
        return false;
    }
    // Plasma and BFG unavailable in shareware.
    if (weapon == wp_plasma || weapon == wp_bfg) && gamemission == doom && gamemode == shareware {
        return false;
    }
    let cp = consoleplayer as usize;
    if players[cp].weaponowned[weapon as usize] == 0 {
        return false;
    }
    // Can't select fist if we have chainsaw, unless we also have berserk.
    if weapon == wp_fist
        && players[cp].weaponowned[wp_chainsaw as usize] != 0
        && players[cp].powers[pw_strength] == 0
    {
        return false;
    }
    true
}

// ---------------------------------------------------------------------------
// G_NextWeapon (static)
// ---------------------------------------------------------------------------

unsafe fn g_next_weapon(direction: c_int) -> c_int {
    let cp = consoleplayer as usize;
    let weapon = if players[cp].pendingweapon == wp_nochange {
        players[cp].readyweapon
    } else {
        players[cp].pendingweapon
    };

    let n = WEAPON_ORDER_TABLE.len() as c_int;
    let mut i = 0;
    while i < n {
        if WEAPON_ORDER_TABLE[i as usize].weapon == weapon {
            break;
        }
        i += 1;
    }

    let start_i = i;
    loop {
        i += direction;
        i = (i + n) % n;
        if i == start_i || weapon_selectable(WEAPON_ORDER_TABLE[i as usize].weapon) {
            break;
        }
    }
    WEAPON_ORDER_TABLE[i as usize].weapon_num
}

// ---------------------------------------------------------------------------
// G_BuildTiccmd
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_BuildTiccmd(cmd: *mut TiccmdT, maketic: c_int) {
    use crate::doom::c_ffi::BACKUPTICS;

    let cmd = &mut *cmd;
    *cmd = std::mem::zeroed();

    cmd.consistancy = consistancy[consoleplayer as usize][(maketic as usize) % BACKUPTICS];

    let strafe = (GAMEKEYDOWN[key_strafe as usize] != 0)
        || (mousebutton(mousebstrafe) != 0)
        || (joybutton(joybstrafe) != 0);

    // "joyb_speed = 31" autorun hack: key_speed >= NUMKEYS means always running
    let speed = (key_speed >= NUMKEYS as c_int)
        || (joybspeed >= MAX_JOY_BUTTONS as c_int)
        || (GAMEKEYDOWN[key_speed as usize] != 0)
        || (joybutton(joybspeed) != 0);
    let speed = speed as usize;

    let mut forward: c_int = 0;
    let mut side: c_int = 0;

    // Two-stage accelerative turning
    if JOYXMOVE < 0
        || JOYXMOVE > 0
        || GAMEKEYDOWN[key_right as usize] != 0
        || GAMEKEYDOWN[key_left as usize] != 0
    {
        TURNHELD += ticdup;
    } else {
        TURNHELD = 0;
    }

    let tspeed = if TURNHELD < 6 { 2usize } else { speed };

    if strafe {
        if GAMEKEYDOWN[key_right as usize] != 0 {
            side += sidemove[speed];
        }
        if GAMEKEYDOWN[key_left as usize] != 0 {
            side -= sidemove[speed];
        }
        if JOYXMOVE > 0 {
            side += sidemove[speed];
        }
        if JOYXMOVE < 0 {
            side -= sidemove[speed];
        }
    } else {
        if GAMEKEYDOWN[key_right as usize] != 0 {
            cmd.angleturn -= angleturn[tspeed] as i16;
        }
        if GAMEKEYDOWN[key_left as usize] != 0 {
            cmd.angleturn += angleturn[tspeed] as i16;
        }
        if JOYXMOVE > 0 {
            cmd.angleturn -= angleturn[tspeed] as i16;
        }
        if JOYXMOVE < 0 {
            cmd.angleturn += angleturn[tspeed] as i16;
        }
    }

    if GAMEKEYDOWN[key_up as usize] != 0 {
        forward += forwardmove[speed];
    }
    if GAMEKEYDOWN[key_down as usize] != 0 {
        forward -= forwardmove[speed];
    }
    if JOYYMOVE < 0 {
        forward += forwardmove[speed];
    }
    if JOYYMOVE > 0 {
        forward -= forwardmove[speed];
    }

    if GAMEKEYDOWN[key_strafeleft as usize] != 0
        || joybutton(joybstrafeleft) != 0
        || mousebutton(mousebstrafeleft) != 0
        || JOYSTRAFEMOVE < 0
    {
        side -= sidemove[speed];
    }
    if GAMEKEYDOWN[key_straferight as usize] != 0
        || joybutton(joybstraferight) != 0
        || mousebutton(mousebstraferight) != 0
        || JOYSTRAFEMOVE > 0
    {
        side += sidemove[speed];
    }

    // Buttons
    cmd.chatchar = HU_dequeueChatChar() as u8;

    if GAMEKEYDOWN[key_fire as usize] != 0
        || mousebutton(mousebfire) != 0
        || joybutton(joybfire) != 0
    {
        cmd.buttons |= BT_ATTACK;
    }

    if GAMEKEYDOWN[key_use as usize] != 0 || joybutton(joybuse) != 0 || mousebutton(mousebuse) != 0
    {
        cmd.buttons |= BT_USE;
        DCLICKS = 0;
    }

    // Weapon cycling
    if gamestate == GS_LEVEL && NEXT_WEAPON != 0 {
        let i = g_next_weapon(NEXT_WEAPON);
        cmd.buttons |= BT_CHANGE;
        cmd.buttons |= (i as u8) << BT_WEAPONSHIFT;
    } else {
        let weapon_keys_vals = [
            key_weapon1,
            key_weapon2,
            key_weapon3,
            key_weapon4,
            key_weapon5,
            key_weapon6,
            key_weapon7,
            key_weapon8,
        ];
        for (i, &key) in weapon_keys_vals.iter().enumerate() {
            if GAMEKEYDOWN[key as usize] != 0 {
                cmd.buttons |= BT_CHANGE;
                cmd.buttons |= (i as u8) << BT_WEAPONSHIFT;
                break;
            }
        }
    }
    NEXT_WEAPON = 0;

    // Mouse forward/backward
    if mousebutton(mousebforward) != 0 {
        forward += forwardmove[speed];
    }
    if mousebutton(mousebbackward) != 0 {
        forward -= forwardmove[speed];
    }

    // Double-click use
    if dclick_use != 0 {
        if mousebutton(mousebforward) != DCLICKSTATE && DCLICKTIME > 1 {
            DCLICKSTATE = mousebutton(mousebforward);
            if DCLICKSTATE != 0 {
                DCLICKS += 1;
            }
            if DCLICKS == 2 {
                cmd.buttons |= BT_USE;
                DCLICKS = 0;
            } else {
                DCLICKTIME = 0;
            }
        } else {
            DCLICKTIME += ticdup;
            if DCLICKTIME > 20 {
                DCLICKS = 0;
                DCLICKSTATE = 0;
            }
        }

        let bstrafe = (mousebutton(mousebstrafe) != 0 || joybutton(joybstrafe) != 0) as boolean;
        if bstrafe != DCLICKSTATE2 && DCLICKTIME2 > 1 {
            DCLICKSTATE2 = bstrafe;
            if DCLICKSTATE2 != 0 {
                DCLICKS2 += 1;
            }
            if DCLICKS2 == 2 {
                cmd.buttons |= BT_USE;
                DCLICKS2 = 0;
            } else {
                DCLICKTIME2 = 0;
            }
        } else {
            DCLICKTIME2 += ticdup;
            if DCLICKTIME2 > 20 {
                DCLICKS2 = 0;
                DCLICKSTATE2 = 0;
            }
        }
    }

    forward += MOUSEY;
    if strafe {
        side += MOUSEX * 2;
    } else {
        cmd.angleturn -= (MOUSEX * 0x8) as i16;
    }

    if MOUSEX == 0 {
        testcontrols_mousespeed = 0;
    }
    MOUSEX = 0;
    MOUSEY = 0;

    // Clamp movement
    let maxplmove = forwardmove[1];
    if forward > maxplmove {
        forward = maxplmove;
    } else if forward < -maxplmove {
        forward = -maxplmove;
    }
    if side > maxplmove {
        side = maxplmove;
    } else if side < -maxplmove {
        side = -maxplmove;
    }

    cmd.forwardmove = (cmd.forwardmove as c_int + forward) as i8;
    cmd.sidemove = (cmd.sidemove as c_int + side) as i8;

    // Special buttons
    if sendpause != 0 {
        sendpause = 0;
        cmd.buttons = BT_SPECIAL | BTS_PAUSE;
    }
    if sendsave != 0 {
        sendsave = 0;
        cmd.buttons = BT_SPECIAL | BTS_SAVEGAME | (SAVEGAMESLOT as u8) << BTS_SAVESHIFT;
    }

    // Low-resolution turning
    if lowres_turn != 0 {
        let desired: i16 = (cmd.angleturn as i16).wrapping_add(LOWRES_TURN_CARRY);
        cmd.angleturn = ((desired as i32 + 128) & 0xff00) as i16;
        LOWRES_TURN_CARRY = desired.wrapping_sub(cmd.angleturn);
    }
}

// ---------------------------------------------------------------------------
// G_DoLoadLevel
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_DoLoadLevel() {
    skyflatnum = R_FlatNumForName(DEH_String(b"F_SKY1\0".as_ptr() as *const c_char) as *mut c_char);

    // Fix sky texture for Final Doom / Chex
    if gamemode == commercial && (gameversion == exe_final2 || gameversion == exe_chex) {
        let skytexturename: *const c_char = if gamemap < 12 {
            b"SKY1\0".as_ptr() as *const c_char
        } else if gamemap < 21 {
            b"SKY2\0".as_ptr() as *const c_char
        } else {
            b"SKY3\0".as_ptr() as *const c_char
        };
        skytexture = R_TextureNumForName(DEH_String(skytexturename) as *mut c_char);
    }

    levelstarttic = gametic;

    if wipegamestate == GS_LEVEL {
        wipegamestate = -1; // force a wipe
    }

    gamestate = GS_LEVEL;

    for i in 0..MAXPLAYERS {
        turbodetected[i] = 0;
        if playeringame[i] != 0 && players[i].playerstate == PST_DEAD {
            players[i].playerstate = PST_REBORN;
        }
        players[i].frags = [0; MAXPLAYERS];
    }

    P_SetupLevel(gameepisode, gamemap, 0, gameskill);
    displayplayer = consoleplayer;
    gameaction = ga_nothing;
    Z_CheckHeap();

    // Clear input state
    GAMEKEYDOWN = [0; NUMKEYS];
    JOYXMOVE = 0;
    JOYYMOVE = 0;
    JOYSTRAFEMOVE = 0;
    MOUSEX = 0;
    MOUSEY = 0;
    sendpause = 0;
    sendsave = 0;
    paused = 0;
    MOUSEARRAY = [0; MAX_MOUSE_BUTTONS + 1];
    JOYARRAY = [0; MAX_JOY_BUTTONS + 1];

    if testcontrols != 0 {
        players[consoleplayer as usize].message =
            b"Press escape to quit.\0".as_ptr() as *mut c_char;
    }
}

// ---------------------------------------------------------------------------
// SetJoyButtons / SetMouseButtons (static helpers)
// ---------------------------------------------------------------------------

unsafe fn set_joy_buttons(buttons_mask: c_uint) {
    for i in 0..MAX_JOY_BUTTONS {
        let button_on = ((buttons_mask >> i) & 1) != 0;
        if JOYARRAY[i + 1] == 0 && button_on {
            if i as c_int == joybprevweapon {
                NEXT_WEAPON = -1;
            } else if i as c_int == joybnextweapon {
                NEXT_WEAPON = 1;
            }
        }
        JOYARRAY[i + 1] = button_on as boolean;
    }
}

unsafe fn set_mouse_buttons(buttons_mask: c_uint) {
    for i in 0..MAX_MOUSE_BUTTONS {
        let button_on = ((buttons_mask >> i) & 1) != 0;
        if MOUSEARRAY[i + 1] == 0 && button_on {
            if i as c_int == mousebprevweapon {
                NEXT_WEAPON = -1;
            } else if i as c_int == mousebnextweapon {
                NEXT_WEAPON = 1;
            }
        }
        MOUSEARRAY[i + 1] = button_on as boolean;
    }
}

// ---------------------------------------------------------------------------
// G_Responder
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_Responder(ev: *mut event_t) -> boolean {
    let ev = &*ev;

    // Spy mode changes even during demo
    if gamestate == GS_LEVEL
        && ev.type_ == 0 // ev_keydown
        && ev.data1 == key_spy
        && (singledemo != 0 || deathmatch == 0)
    {
        loop {
            displayplayer += 1;
            if displayplayer == MAXPLAYERS as c_int {
                displayplayer = 0;
            }
            if playeringame[displayplayer as usize] != 0 || displayplayer == consoleplayer {
                break;
            }
        }
        return 1;
    }

    // Any key pops up menu if in demos
    if gameaction == ga_nothing
        && singledemo == 0
        && (demoplayback != 0 || gamestate == GS_DEMOSCREEN)
    {
        if ev.type_ == 0 // ev_keydown
            || (ev.type_ == 2 && ev.data1 != 0) // ev_mouse with buttons
            || (ev.type_ == 3 && ev.data1 != 0)
        // ev_joystick with buttons
        {
            M_StartControlPanel();
            return 1;
        }
        return 0;
    }

    if gamestate == GS_LEVEL {
        if HU_Responder(ev as *const event_t as *mut event_t) != 0 {
            return 1;
        }
        if ST_Responder(ev as *const event_t as *mut event_t) != 0 {
            return 1;
        }
        if AM_Responder(ev as *const event_t as *mut event_t) != 0 {
            return 1;
        }
    }

    if gamestate == GS_FINALE {
        if F_Responder(ev as *const event_t as *mut event_t) != 0 {
            return 1;
        }
    }

    if testcontrols != 0 && ev.type_ == 2 {
        // ev_mouse
        testcontrols_mousespeed = ev.data2.abs();
    }

    // Prev/next weapon keys
    if ev.type_ == 0 && ev.data1 == key_prevweapon {
        NEXT_WEAPON = -1;
    } else if ev.type_ == 0 && ev.data1 == key_nextweapon {
        NEXT_WEAPON = 1;
    }

    match ev.type_ {
        0 => {
            // ev_keydown
            if ev.data1 == key_pause {
                sendpause = 1;
            } else if (ev.data1 as usize) < NUMKEYS {
                GAMEKEYDOWN[ev.data1 as usize] = 1;
            }
            return 1;
        }
        1 => {
            // ev_keyup
            if (ev.data1 as usize) < NUMKEYS {
                GAMEKEYDOWN[ev.data1 as usize] = 0;
            }
            return 0;
        }
        2 => {
            // ev_mouse
            set_mouse_buttons(ev.data1 as c_uint);
            MOUSEX = ev.data2 * (mouseSensitivity + 5) / 10;
            MOUSEY = ev.data3 * (mouseSensitivity + 5) / 10;
            return 1;
        }
        3 => {
            // ev_joystick
            set_joy_buttons(ev.data1 as c_uint);
            JOYXMOVE = ev.data2;
            JOYYMOVE = ev.data3;
            JOYSTRAFEMOVE = ev.data4;
            return 1;
        }
        _ => {}
    }
    0
}

// ---------------------------------------------------------------------------
// G_Ticker
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_Ticker() {
    use crate::doom::c_ffi::BACKUPTICS;

    // Player reborns
    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 && players[i].playerstate == PST_REBORN {
            G_DoReborn(i as c_int);
        }
    }

    // Process pending game actions
    while gameaction != ga_nothing {
        match gameaction {
            ga_loadlevel => G_DoLoadLevel(),
            ga_newgame => G_DoNewGame(),
            ga_loadgame => G_DoLoadGame(),
            ga_savegame => G_DoSaveGame(),
            ga_playdemo => G_DoPlayDemo(),
            ga_completed => G_DoCompleted(),
            ga_victory => F_StartFinale(),
            ga_worlddone => G_DoWorldDone(),
            ga_screenshot => {
                V_ScreenShot(b"DOOM%02i.%s\0".as_ptr() as *mut c_char);
                players[consoleplayer as usize].message =
                    DEH_String(b"screen shot\0".as_ptr() as *const c_char) as *mut c_char;
                gameaction = ga_nothing;
            }
            _ => {}
        }
    }

    // Get commands, check consistency, build new consistency check
    let buf = ((gametic / ticdup) as usize) % BACKUPTICS;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            let cmd = &mut players[i].cmd as *mut TiccmdT;

            // Copy net command into player command
            std::ptr::copy_nonoverlapping(netcmds.add(i), cmd, 1);

            if demoplayback != 0 {
                G_ReadDemoTiccmd(cmd);
            }
            if demorecording != 0 {
                G_WriteDemoTiccmd(cmd);
            }

            // Turbo detection
            if (*cmd).forwardmove > 0x32 {
                turbodetected[i] = 1;
            }

            if (gametic & 31) == 0
                && ((gametic >> 5) % MAXPLAYERS as c_int) == i as c_int
                && turbodetected[i] != 0
            {
                M_snprintf_clamp(
                    TURBOMESSAGE.as_mut_ptr(),
                    TURBOMESSAGE.len(),
                    snprintf(
                        TURBOMESSAGE.as_mut_ptr(),
                        TURBOMESSAGE.len(),
                        b"%s is turbo!\0".as_ptr() as *const c_char,
                        player_names[i],
                    ),
                );
                players[consoleplayer as usize].message = TURBOMESSAGE.as_mut_ptr();
                turbodetected[i] = 0;
            }

            if netgame != 0 && netdemo == 0 && (gametic % ticdup) == 0 {
                if gametic > BACKUPTICS as c_int && consistancy[i][buf] != (*cmd).consistancy {
                    I_Error(
                        b"consistency failure (%i should be %i)\0".as_ptr() as *const c_char,
                        (*cmd).consistancy as c_int,
                        consistancy[i][buf] as c_int,
                    );
                }
                let mo = players[i].mo as *mut mobj_t;
                if !mo.is_null() {
                    consistancy[i][buf] = (*mo).x as u8;
                } else {
                    consistancy[i][buf] = rndindex as u8;
                }
            }
        }
    }

    // Check for special buttons
    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            let buttons = players[i].cmd.buttons;
            if buttons & BT_SPECIAL != 0 {
                match buttons & BT_SPECIALMASK {
                    BTS_PAUSE => {
                        paused ^= 1;
                        if paused != 0 {
                            S_PauseSound();
                        } else {
                            S_ResumeSound();
                        }
                    }
                    BTS_SAVEGAME => {
                        if SAVEDESCRIPTION[0] == 0 {
                            M_StringCopy(
                                SAVEDESCRIPTION.as_mut_ptr(),
                                b"NET GAME\0".as_ptr() as *const c_char,
                                SAVEDESCRIPTION.len(),
                            );
                        }
                        SAVEGAMESLOT = ((buttons & BTS_SAVEMASK) >> BTS_SAVESHIFT) as c_int;
                        gameaction = ga_savegame;
                    }
                    _ => {}
                }
            }
        }
    }

    // Check if intermission screen just ended
    if oldgamestate == GS_INTERMISSION && gamestate != GS_INTERMISSION {
        WI_End();
    }
    oldgamestate = gamestate;

    // Main game-state dispatch
    match gamestate {
        GS_LEVEL => {
            P_Ticker();
            ST_Ticker();
            AM_Ticker();
            HU_Ticker();
        }
        GS_INTERMISSION => {
            WI_Ticker();
        }
        GS_FINALE => {
            F_Ticker();
        }
        GS_DEMOSCREEN => {
            D_PageTicker();
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// G_InitPlayer
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_InitPlayer(player: c_int) {
    G_PlayerReborn(player);
}

// ---------------------------------------------------------------------------
// G_PlayerFinishLevel
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_PlayerFinishLevel(player: c_int) {
    let p = &mut players[player as usize];
    p.powers = [0; 6];
    p.cards = [0; 6];
    (*(p.mo as *mut mobj_t)).flags &= !crate::doom::info::MF_SHADOW;
    p.extralight = 0;
    p.fixedcolormap = 0;
    p.damagecount = 0;
    p.bonuscount = 0;
}

// ---------------------------------------------------------------------------
// G_PlayerReborn
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_PlayerReborn(player: c_int) {
    let idx = player as usize;
    let frags = players[idx].frags;
    let killcount = players[idx].killcount;
    let itemcount = players[idx].itemcount;
    let secretcount = players[idx].secretcount;

    players[idx] = std::mem::zeroed();

    players[idx].frags = frags;
    players[idx].killcount = killcount;
    players[idx].itemcount = itemcount;
    players[idx].secretcount = secretcount;

    players[idx].usedown = 1;
    players[idx].attackdown = 1;
    players[idx].playerstate = PST_LIVE;
    players[idx].health = DEH_INITIAL_HEALTH;
    players[idx].readyweapon = wp_pistol;
    players[idx].pendingweapon = wp_pistol;
    players[idx].weaponowned[wp_fist as usize] = 1;
    players[idx].weaponowned[wp_pistol as usize] = 1;
    players[idx].ammo[am_clip] = DEH_INITIAL_BULLETS;

    for i in 0..4 {
        // NUMAMMO = 4
        players[idx].maxammo[i] = maxammo[i];
    }
}

// ---------------------------------------------------------------------------
// G_CheckSpot
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_CheckSpot(playernum: c_int, mthing: *mut mapthing_t) -> boolean {
    if players[playernum as usize].mo.is_null() {
        // First spawn of level, before corpses
        for i in 0..playernum as usize {
            let mo = players[i].mo as *mut mobj_t;
            if (*mo).x == ((*mthing).x as fixed_t) << 16
                && (*mo).y == ((*mthing).y as fixed_t) << 16
            {
                return 0;
            }
        }
        return 1;
    }

    let x = ((*mthing).x as fixed_t) << 16;
    let y = ((*mthing).y as fixed_t) << 16;

    if P_CheckPosition(players[playernum as usize].mo as *mut crate::doom::c_ffi::mobj_t, x, y) == 0 {
        return 0;
    }

    // Flush old corpse
    if bodyqueslot >= 32 {
        P_RemoveMobj(bodyque[(bodyqueslot % 32) as usize]);
    }
    bodyque[(bodyqueslot % 32) as usize] = players[playernum as usize].mo as *mut mobj_t;
    bodyqueslot += 1;

    // Spawn teleport fog
    let ss = R_PointInSubsector(x, y);

    // Replicate vanilla signed-angle overflow (from PrBoom+)
    let an_raw = (0x10000000i32).wrapping_mul((*mthing).angle as i32 / 45);
    let xa: fixed_t;
    let ya: fixed_t;
    match an_raw {
        4096 => {
            xa = finetangent[2048];
            ya = finetangent[0];
        }
        5120 => {
            xa = finetangent[3072];
            ya = finetangent[1024];
        }
        6144 => {
            xa = finesine[0];
            ya = finetangent[2048];
        }
        7168 => {
            xa = finesine[1024];
            ya = finetangent[3072];
        }
        0 | 1024 | 2048 | 3072 => {
            xa = unsafe { *finecosine.0.add(an_raw as usize) };
            ya = finesine[an_raw as usize];
        }
        _ => {
            I_Error(
                b"G_CheckSpot: unexpected angle %d\n\0".as_ptr() as *const c_char,
                an_raw,
            );
            xa = 0;
            ya = 0;
        }
    }

    let floorheight = (*(*ss).sector).floorheight;
    let mo = P_SpawnMobj(x + 20 * xa, y + 20 * ya, floorheight, MT_TFOG);

    if players[consoleplayer as usize].viewz != 1 {
        S_StartSound(mo as *mut c_void, sfx_telept);
    }
    1
}

// ---------------------------------------------------------------------------
// G_DeathMatchSpawnPlayer
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_DeathMatchSpawnPlayer(playernum: c_int) {
    let selections = deathmatch_p.offset_from(deathmatchstarts.as_ptr()) as c_int;
    if selections < 4 {
        I_Error(
            b"Only %i deathmatch spots, 4 required\0".as_ptr() as *const c_char,
            selections,
        );
    }

    for _ in 0..20 {
        let i = (P_Random() % selections as c_int) as usize;
        if G_CheckSpot(
            playernum,
            &mut deathmatchstarts[i] as *mut SetupMapThing as *mut mapthing_t,
        ) != 0
        {
            deathmatchstarts[i].r#type = (playernum + 1) as i16;
            P_SpawnPlayer(&mut deathmatchstarts[i] as *mut SetupMapThing as *mut mapthing_t);
            return;
        }
    }
    P_SpawnPlayer(&mut playerstarts[playernum as usize] as *mut SetupMapThing as *mut mapthing_t);
}

// ---------------------------------------------------------------------------
// G_DoReborn
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_DoReborn(playernum: c_int) {
    if netgame == 0 {
        gameaction = ga_loadlevel;
    } else {
        let mo = players[playernum as usize].mo as *mut mobj_t;
        if !mo.is_null() {
            (*mo).player = ptr::null_mut();
        }

        if deathmatch != 0 {
            G_DeathMatchSpawnPlayer(playernum);
            return;
        }

        if G_CheckSpot(
            playernum,
            &mut playerstarts[playernum as usize] as *mut SetupMapThing as *mut mapthing_t,
        ) != 0
        {
            P_SpawnPlayer(
                &mut playerstarts[playernum as usize] as *mut SetupMapThing as *mut mapthing_t,
            );
            return;
        }

        for i in 0..MAXPLAYERS as c_int {
            if G_CheckSpot(
                playernum,
                &mut playerstarts[i as usize] as *mut SetupMapThing as *mut mapthing_t,
            ) != 0
            {
                playerstarts[i as usize].r#type = (playernum + 1) as i16;
                P_SpawnPlayer(
                    &mut playerstarts[i as usize] as *mut SetupMapThing as *mut mapthing_t,
                );
                playerstarts[i as usize].r#type = (i + 1) as i16;
                return;
            }
        }
        P_SpawnPlayer(
            &mut playerstarts[playernum as usize] as *mut SetupMapThing as *mut mapthing_t,
        );
    }
}

// ---------------------------------------------------------------------------
// G_ScreenShot
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_ScreenShot() {
    gameaction = ga_screenshot;
}

// ---------------------------------------------------------------------------
// G_ExitLevel / G_SecretExitLevel
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_ExitLevel() {
    secretexit = 0;
    gameaction = ga_completed;
}

#[no_mangle]
pub unsafe extern "C" fn G_SecretExitLevel() {
    if gamemode == commercial && W_CheckNumForName(b"map31\0".as_ptr() as *const c_char) < 0 {
        secretexit = 0;
    } else {
        secretexit = 1;
    }
    gameaction = ga_completed;
}

// ---------------------------------------------------------------------------
// G_DoCompleted
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_DoCompleted() {
    gameaction = ga_nothing;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            G_PlayerFinishLevel(i as c_int);
        }
    }

    if automapactive != 0 {
        AM_Stop();
    }

    if gamemode != commercial {
        if gameversion == exe_chex {
            if gamemap == 5 {
                gameaction = ga_victory;
                return;
            }
        } else {
            match gamemap {
                8 => {
                    gameaction = ga_victory;
                    return;
                }
                9 => {
                    for i in 0..MAXPLAYERS {
                        players[i].didsecret = 1;
                    }
                }
                _ => {}
            }
        }
    }

    if gamemap == 8 && gamemode != commercial {
        gameaction = ga_victory;
        return;
    }
    if gamemap == 9 && gamemode != commercial {
        for i in 0..MAXPLAYERS {
            players[i].didsecret = 1;
        }
    }

    wminfo.didsecret = players[consoleplayer as usize].didsecret;
    wminfo.epsd = gameepisode - 1;
    wminfo.last = gamemap - 1;

    if gamemode == commercial {
        wminfo.next = if secretexit != 0 {
            match gamemap {
                15 => 30,
                31 => 31,
                _ => gamemap,
            }
        } else {
            match gamemap {
                31 | 32 => 15,
                _ => gamemap,
            }
        };
    } else {
        wminfo.next = if secretexit != 0 {
            8
        } else if gamemap == 9 {
            match gameepisode {
                1 => 3,
                2 => 5,
                3 => 6,
                4 => 2,
                _ => gamemap,
            }
        } else {
            gamemap
        };
    }

    wminfo.maxkills = totalkills;
    wminfo.maxitems = totalitems;
    wminfo.maxsecret = totalsecret;
    wminfo.maxfrags = 0;

    wminfo.partime = if gamemode == commercial {
        35 * cpars[(gamemap - 1) as usize]
    } else if gameepisode < 4 {
        35 * pars[gameepisode as usize][gamemap as usize]
    } else {
        35 * cpars[gamemap as usize]
    };

    wminfo.pnum = consoleplayer;

    for i in 0..MAXPLAYERS {
        wminfo.plyr[i].in_ = playeringame[i];
        wminfo.plyr[i].skills = players[i].killcount;
        wminfo.plyr[i].sitems = players[i].itemcount;
        wminfo.plyr[i].ssecret = players[i].secretcount;
        wminfo.plyr[i].stime = leveltime;
        wminfo.plyr[i].frags = players[i].frags;
    }

    gamestate = GS_INTERMISSION;
    viewactive = 0;
    automapactive = 0;

    StatCopy(&mut wminfo as *mut _ as *mut crate::doom::statdump::wbstartstruct_t);
    WI_Start(&mut wminfo);
}

// ---------------------------------------------------------------------------
// G_WorldDone / G_DoWorldDone
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_WorldDone() {
    gameaction = ga_worlddone;

    if secretexit != 0 {
        players[consoleplayer as usize].didsecret = 1;
    }

    if gamemode == commercial {
        match gamemap {
            15 | 31 => {
                if secretexit == 0 {
                    return;
                }
                F_StartFinale();
            }
            6 | 11 | 20 | 30 => {
                F_StartFinale();
            }
            _ => {}
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn G_DoWorldDone() {
    gamestate = GS_LEVEL;
    gamemap = wminfo.next + 1;
    G_DoLoadLevel();
    gameaction = ga_nothing;
    viewactive = 1;
}

// ---------------------------------------------------------------------------
// G_LoadGame / G_DoLoadGame
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_LoadGame(name: *mut c_char) {
    M_StringCopy(savename.as_mut_ptr(), name, savename.len());
    gameaction = ga_loadgame;
}

#[no_mangle]
pub unsafe extern "C" fn G_DoLoadGame() {
    gameaction = ga_nothing;

    save_stream = libc::fopen(savename.as_ptr(), b"rb\0".as_ptr() as *const libc::c_char);
    if save_stream.is_null() {
        return;
    }

    savegame_error = 0;

    if P_ReadSaveGameHeader() == 0 {
        libc::fclose(save_stream);
        return;
    }

    let savedleveltime = leveltime;

    G_InitNew(gameskill, gameepisode, gamemap);

    leveltime = savedleveltime;

    P_UnArchivePlayers();
    P_UnArchiveWorld();
    P_UnArchiveThinkers();
    P_UnArchiveSpecials();

    if P_ReadSaveGameEOF() == 0 {
        I_Error(b"Bad savegame\0".as_ptr() as *const c_char);
    }

    libc::fclose(save_stream);

    if setsizeneeded.is_truthy() {
        R_ExecuteSetViewSize();
    }

    R_FillBackScreen();
}

// ---------------------------------------------------------------------------
// G_SaveGame / G_DoSaveGame
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_SaveGame(slot: c_int, description: *const c_char) {
    SAVEGAMESLOT = slot;
    M_StringCopy(
        SAVEDESCRIPTION.as_mut_ptr(),
        description,
        SAVEDESCRIPTION.len(),
    );
    sendsave = 1;
}

#[no_mangle]
pub unsafe extern "C" fn G_DoSaveGame() {
    let recovery_savegame_file: *mut c_char;
    let temp_savegame_file = P_TempSaveGameFile();
    let savegame_file = P_SaveGameFile(SAVEGAMESLOT);

    save_stream = libc::fopen(temp_savegame_file, b"wb\0".as_ptr() as *const libc::c_char);

    if save_stream.is_null() {
        let recovery = M_TempFile(b"recovery.dsg\0".as_ptr() as *mut c_char);
        recovery_savegame_file = recovery;
        save_stream = libc::fopen(recovery, b"wb\0".as_ptr() as *const libc::c_char);
        if save_stream.is_null() {
            I_Error(
                b"Failed to open either '%s' or '%s' to write savegame.\0".as_ptr()
                    as *const c_char,
                temp_savegame_file,
                recovery_savegame_file,
            );
        }
    } else {
        recovery_savegame_file = ptr::null_mut();
    }

    savegame_error = 0;

    P_WriteSaveGameHeader(SAVEDESCRIPTION.as_mut_ptr());
    P_ArchivePlayers();
    P_ArchiveWorld();
    P_ArchiveThinkers();
    P_ArchiveSpecials();
    P_WriteSaveGameEOF();

    if vanilla_savegame_limit != 0 && libc::ftell(save_stream as *mut libc::FILE) > SAVEGAMESIZE {
        I_Error(b"Savegame buffer overrun\0".as_ptr() as *const c_char);
    }

    libc::fclose(save_stream);

    if !recovery_savegame_file.is_null() {
        I_Error(
            b"Failed to open savegame file '%s' for writing.\nBut your game has been saved to '%s' for recovery.\0"
                .as_ptr() as *const c_char,
            temp_savegame_file,
            recovery_savegame_file,
        );
    }

    libc::remove(savegame_file);
    libc::rename(temp_savegame_file, savegame_file);

    gameaction = ga_nothing;
    M_StringCopy(
        SAVEDESCRIPTION.as_mut_ptr(),
        b"\0".as_ptr() as *const c_char,
        SAVEDESCRIPTION.len(),
    );

    players[consoleplayer as usize].message =
        DEH_String(b"game saved.\0".as_ptr() as *const c_char) as *mut c_char;

    R_FillBackScreen();
}

// ---------------------------------------------------------------------------
// G_DeferedInitNew / G_DoNewGame / G_InitNew
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_DeferedInitNew(skill: skill_t, episode: c_int, map: c_int) {
    d_skill = skill;
    d_episode = episode;
    d_map = map;
    gameaction = ga_newgame;
}

#[no_mangle]
pub unsafe extern "C" fn G_DoNewGame() {
    demoplayback = 0;
    netdemo = 0;
    netgame = 0;
    deathmatch = 0;
    playeringame[1] = 0;
    playeringame[2] = 0;
    playeringame[3] = 0;
    respawnparm = 0;
    fastparm = 0;
    nomonsters = 0;
    consoleplayer = 0;
    G_InitNew(d_skill, d_episode, d_map);
    gameaction = ga_nothing;
}

#[no_mangle]
pub unsafe extern "C" fn G_InitNew(skill: skill_t, episode: c_int, map: c_int) {
    let mut skill = skill;
    let mut episode = episode;
    let mut map = map;

    if paused != 0 {
        paused = 0;
        S_ResumeSound();
    }

    if skill > 4 {
        // sk_nightmare = 4
        skill = 4;
    }

    if gameversion >= exe_ultimate {
        if episode == 0 {
            episode = 4;
        }
    } else {
        if episode < 1 {
            episode = 1;
        }
        if episode > 3 {
            episode = 3;
        }
    }

    if episode > 1 && gamemode == shareware {
        episode = 1;
    }

    if map < 1 {
        map = 1;
    }
    if map > 9 && gamemode != commercial {
        map = 9;
    }

    M_ClearRandom();

    if skill == 4 || respawnparm != 0 {
        // sk_nightmare = 4
        respawnmonsters = 1;
    } else {
        respawnmonsters = 0;
    }

    // Fast monsters for nightmare or fastparm
    if fastparm != 0 || (skill == 4 && gameskill != 4) {
        set_fast_monsters(true);
    } else if skill != 4 && gameskill == 4 {
        set_fast_monsters(false);
    }

    // Force players to be reborn on first level load
    for i in 0..MAXPLAYERS {
        players[i].playerstate = PST_REBORN;
    }

    usergame = 1;
    paused = 0;
    demoplayback = 0;
    automapactive = 0;
    viewactive = 1;
    gameepisode = episode;
    gamemap = map;
    gameskill = skill;

    // Set sky texture (vanilla Doom broken behaviour for Doom II: set once at
    // game start, not per level — deliberately preserved for compatibility).
    let skytexturename: *const c_char = if gamemode == commercial {
        if gamemap < 12 {
            b"SKY1\0".as_ptr() as *const c_char
        } else if gamemap < 21 {
            b"SKY2\0".as_ptr() as *const c_char
        } else {
            b"SKY3\0".as_ptr() as *const c_char
        }
    } else {
        match gameepisode {
            1 => b"SKY1\0".as_ptr() as *const c_char,
            2 => b"SKY2\0".as_ptr() as *const c_char,
            3 => b"SKY3\0".as_ptr() as *const c_char,
            4 => b"SKY4\0".as_ptr() as *const c_char,
            _ => b"SKY1\0".as_ptr() as *const c_char,
        }
    };

    skytexture = R_TextureNumForName(DEH_String(skytexturename) as *mut c_char);

    G_DoLoadLevel();
}

// ---------------------------------------------------------------------------
// Fast-monster helper for G_InitNew
// ---------------------------------------------------------------------------

unsafe fn set_fast_monsters(fast: bool) {
    // S_SARG_RUN1 .. S_SARG_PAIN2 state range (states 134..160 in vanilla Doom)
    // mobjinfo adjustments for MT_BRUISERSHOT, MT_HEADSHOT, MT_TROOPSHOT
    // These are byte-offset operations into the info tables.
    // Delegate to a safe extern to avoid duplicating the state-offset arithmetic.
    if fast {
        G_SetFastMonsters(1);
    } else {
        G_SetFastMonsters(0);
    }
}


// ---------------------------------------------------------------------------
// G_SetFastMonsters — adjusts state tics and monster shot speeds
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_SetFastMonsters(fast: c_int) {
    use crate::doom::info::{
        mobjinfo, states, MT_BRUISERSHOT, MT_HEADSHOT, MT_TROOPSHOT, S_SARG_PAIN2, S_SARG_RUN1,
    };

    for i in S_SARG_RUN1 as usize..=S_SARG_PAIN2 as usize {
        if fast != 0 {
            states[i].tics >>= 1;
        } else {
            states[i].tics <<= 1;
        }
    }

    let (bruiser_spd, head_spd, troop_spd): (fixed_t, fixed_t, fixed_t) = if fast != 0 {
        (20 * 65536, 20 * 65536, 20 * 65536)
    } else {
        (15 * 65536, 10 * 65536, 10 * 65536)
    };

    mobjinfo[MT_BRUISERSHOT as usize].speed = bruiser_spd;
    mobjinfo[MT_HEADSHOT as usize].speed = head_spd;
    mobjinfo[MT_TROOPSHOT as usize].speed = troop_spd;
}

// ---------------------------------------------------------------------------
// G_PlayerMo accessor (reads .player field of mobj_t)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// G_ReadDemoTiccmd  ← CRITICAL for demo accuracy
// ---------------------------------------------------------------------------

/// Read one tic command from the demo buffer into `cmd`.
///
/// Non-longtics: 4 bytes (forwardmove, sidemove, angleturn-hi, buttons).
/// Longtics:     5 bytes (forwardmove, sidemove, angleturn-lo, angleturn-hi, buttons).
///
/// The angleturn encoding matches vanilla exactly:
///   non-longtics: byte is read as UNSIGNED, then shifted left 8 → fits [-32768, 32512]
///   longtics:     two bytes little-endian, no sign extension at individual bytes
#[no_mangle]
pub unsafe extern "C" fn G_ReadDemoTiccmd(cmd: *mut TiccmdT) {
    if *demo_p == DEMOMARKER {
        G_CheckDemoStatus();
        return;
    }

    let cmd = &mut *cmd;

    // forwardmove: byte reinterpreted as signed char
    cmd.forwardmove = (*demo_p) as i8;
    demo_p = demo_p.add(1);

    // sidemove: byte reinterpreted as signed char
    cmd.sidemove = (*demo_p) as i8;
    demo_p = demo_p.add(1);

    if longtics != 0 {
        // Little-endian 16-bit value; each byte read as unsigned
        let lo = *demo_p as u8;
        demo_p = demo_p.add(1);
        let hi = *demo_p as u8;
        demo_p = demo_p.add(1);
        // Matches C: angleturn = lo; angleturn |= hi << 8;
        cmd.angleturn = (lo as i16) | ((hi as i16) << 8);
    } else {
        // Non-longtics: one byte read as UNSIGNED, shifted left 8
        // C: cmd->angleturn = ((unsigned char)*demo_p++) << 8
        let byte = *demo_p as u8;
        demo_p = demo_p.add(1);
        cmd.angleturn = ((byte as u32) << 8) as i16;
    }

    // buttons: unsigned byte
    cmd.buttons = (*demo_p) as u8;
    demo_p = demo_p.add(1);
}

// ---------------------------------------------------------------------------
// IncreaseDemoBuffer (static)
// ---------------------------------------------------------------------------

unsafe fn increase_demo_buffer() {
    let current_length = demoend.offset_from(demobuffer) as c_int;
    let new_length = current_length * 2;
    let new_demobuffer = Z_Malloc(new_length, PU_STATIC, ptr::null_mut()) as *mut byte;
    let new_demop = new_demobuffer.add(demo_p.offset_from(demobuffer) as usize);
    std::ptr::copy_nonoverlapping(demobuffer, new_demobuffer, current_length as usize);
    Z_Free(demobuffer as *mut c_void);
    demobuffer = new_demobuffer;
    demo_p = new_demop;
    demoend = demobuffer.add(new_length as usize);
}

// ---------------------------------------------------------------------------
// G_WriteDemoTiccmd
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_WriteDemoTiccmd(cmd: *mut TiccmdT) {
    if GAMEKEYDOWN[key_demo_quit as usize] != 0 {
        G_CheckDemoStatus();
    }

    let cmd = &*cmd;
    let demo_start = demo_p;

    *demo_p = cmd.forwardmove as byte;
    demo_p = demo_p.add(1);
    *demo_p = cmd.sidemove as byte;
    demo_p = demo_p.add(1);

    if longtics != 0 {
        *demo_p = (cmd.angleturn & 0xff) as byte;
        demo_p = demo_p.add(1);
        *demo_p = ((cmd.angleturn >> 8) & 0xff) as byte;
        demo_p = demo_p.add(1);
    } else {
        *demo_p = (cmd.angleturn >> 8) as byte;
        demo_p = demo_p.add(1);
    }

    *demo_p = cmd.buttons;
    demo_p = demo_p.add(1);

    // Reset demo pointer — write then re-read to validate consistency
    demo_p = demo_start;

    if demo_p > demoend.sub(16) {
        if vanilla_demo_limit != 0 {
            G_CheckDemoStatus();
            return;
        } else {
            increase_demo_buffer();
        }
    }

    G_ReadDemoTiccmd(cmd as *const TiccmdT as *mut TiccmdT);
}

// ---------------------------------------------------------------------------
// G_RecordDemo / G_VanillaVersionCode / G_BeginRecording
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_RecordDemo(name: *mut c_char) {
    usergame = 0;
    let name_len = libc::strlen(name);
    let demoname_size = name_len + 5;
    demoname = Z_Malloc(demoname_size as c_int, PU_STATIC, ptr::null_mut()) as *mut c_char;
    M_snprintf_clamp(
        demoname,
        demoname_size,
        snprintf(
            demoname,
            demoname_size,
            b"%s.lmp\0".as_ptr() as *const c_char,
            name,
        ),
    );
    let mut maxsize: c_int = 0x20000;
    let i = M_CheckParmWithArgs(b"-maxdemo\0".as_ptr() as *mut c_char, 1);
    if i != 0 {
        maxsize = libc::atoi(*myargv.add(i as usize + 1) as *const libc::c_char);
        maxsize *= 1024;
    }
    demobuffer = Z_Malloc(maxsize, PU_STATIC, ptr::null_mut()) as *mut byte;
    demoend = demobuffer.add(maxsize as usize);
    demorecording = 1;
}

/// Returns the version code byte for the current gameversion.
#[no_mangle]
pub unsafe extern "C" fn G_VanillaVersionCode() -> c_int {
    g_vanilla_version_code_for(gameversion)
}

/// Pure helper for G_VanillaVersionCode, testable without globals.
fn g_vanilla_version_code_for(gv: c_int) -> c_int {
    match gv {
        v if v == exe_doom_1_2 => unsafe {
            I_Error(b"Doom 1.2 does not have a version code!\0".as_ptr() as *const c_char)
        },
        v if v == exe_doom_1_666 => 106,
        v if v == exe_doom_1_7 => 107,
        v if v == exe_doom_1_8 => 108,
        _ => 109, // exe_doom_1_9 and all later variants
    }
}

#[no_mangle]
pub unsafe extern "C" fn G_BeginRecording() {
    use crate::doom::c_ffi::DOOM_191_VERSION;

    longtics = (M_CheckParm(b"-longtics\0".as_ptr() as *mut c_char) != 0) as boolean;
    lowres_turn = (longtics == 0) as boolean;

    demo_p = demobuffer;

    if longtics != 0 {
        *demo_p = DOOM_191_VERSION as byte;
    } else {
        *demo_p = G_VanillaVersionCode() as byte;
    }
    demo_p = demo_p.add(1);

    *demo_p = gameskill as byte;
    demo_p = demo_p.add(1);
    *demo_p = gameepisode as byte;
    demo_p = demo_p.add(1);
    *demo_p = gamemap as byte;
    demo_p = demo_p.add(1);
    *demo_p = deathmatch as byte;
    demo_p = demo_p.add(1);
    *demo_p = respawnparm as byte;
    demo_p = demo_p.add(1);
    *demo_p = fastparm as byte;
    demo_p = demo_p.add(1);
    *demo_p = nomonsters as byte;
    demo_p = demo_p.add(1);
    *demo_p = consoleplayer as byte;
    demo_p = demo_p.add(1);

    for i in 0..MAXPLAYERS {
        *demo_p = playeringame[i] as byte;
        demo_p = demo_p.add(1);
    }
}

// ---------------------------------------------------------------------------
// G_DeferedPlayDemo / DemoVersionDescription / G_DoPlayDemo
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_DeferedPlayDemo(name: *const c_char) {
    defdemoname = name as *mut c_char;
    gameaction = ga_playdemo;
}

unsafe fn demo_version_description(version: c_int) -> *const c_char {
    match version {
        104 => b"v1.4\0".as_ptr() as *const c_char,
        105 => b"v1.5\0".as_ptr() as *const c_char,
        106 => b"v1.6/v1.666\0".as_ptr() as *const c_char,
        107 => b"v1.7/v1.7a\0".as_ptr() as *const c_char,
        108 => b"v1.8\0".as_ptr() as *const c_char,
        109 => b"v1.9\0".as_ptr() as *const c_char,
        _ => {
            if version >= 0 && version <= 4 {
                b"v1.0/v1.1/v1.2\0".as_ptr() as *const c_char
            } else {
                M_snprintf_clamp(
                    DEMOVERSIONBUF.as_mut_ptr(),
                    DEMOVERSIONBUF.len(),
                    snprintf(
                        DEMOVERSIONBUF.as_mut_ptr(),
                        DEMOVERSIONBUF.len(),
                        b"%i.%i (unknown)\0".as_ptr() as *const c_char,
                        version / 100,
                        version % 100,
                    ),
                );
                DEMOVERSIONBUF.as_ptr()
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn G_DoPlayDemo() {
    use crate::doom::c_ffi::DOOM_191_VERSION;

    gameaction = ga_nothing;
    demobuffer = W_CacheLumpName(defdemoname, PU_STATIC) as *mut byte;
    demo_p = demobuffer;

    let demoversion = *demo_p as c_int;
    demo_p = demo_p.add(1);

    if demoversion == G_VanillaVersionCode() {
        longtics = 0;
    } else if demoversion == DOOM_191_VERSION {
        longtics = 1;
    } else {
        let message = b"Demo is from a different game version!\n\
            (read %i, should be %i)\n\n\
            *** You may need to upgrade your version of Doom to v1.9. ***\n\
            See: https://www.doomworld.com/classicdoom/info/patches.php\n\
            This appears to be %s.\0";
        // C code uses printf (not I_Error) here so demo playback continues
        libc::printf(
            message.as_ptr() as *const libc::c_char,
            demoversion,
            G_VanillaVersionCode(),
            demo_version_description(demoversion),
        );
    }

    let skill = *demo_p as skill_t;
    demo_p = demo_p.add(1);
    let episode = *demo_p as c_int;
    demo_p = demo_p.add(1);
    let map = *demo_p as c_int;
    demo_p = demo_p.add(1);
    deathmatch = *demo_p as c_int;
    demo_p = demo_p.add(1);
    respawnparm = *demo_p as c_int;
    demo_p = demo_p.add(1);
    fastparm = *demo_p as c_int;
    demo_p = demo_p.add(1);
    nomonsters = *demo_p as c_int;
    demo_p = demo_p.add(1);
    consoleplayer = *demo_p as c_int;
    demo_p = demo_p.add(1);
    if consoleplayer < 0 || consoleplayer >= MAXPLAYERS as c_int {
        I_Error(
            b"G_DoPlayDemo: consoleplayer %d out of range\n\0".as_ptr() as *const c_char,
            consoleplayer,
        );
    }

    for i in 0..MAXPLAYERS {
        playeringame[i] = *demo_p as boolean;
        demo_p = demo_p.add(1);
    }

    if playeringame[1] != 0
        || M_CheckParm(b"-solo-net\0".as_ptr() as *mut c_char) > 0
        || M_CheckParm(b"-netdemo\0".as_ptr() as *mut c_char) > 0
    {
        netgame = 1;
        netdemo = 1;
    }

    precache = 0;
    G_InitNew(skill, episode, map);
    precache = 1;
    starttime = I_GetTime();

    usergame = 0;
    demoplayback = 1;
}

// ---------------------------------------------------------------------------
// G_TimeDemo
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_TimeDemo(name: *mut c_char) {
    nodrawers = M_CheckParm(b"-nodraw\0".as_ptr() as *mut c_char);
    timingdemo = 1;
    use crate::doom::d_loop::singletics;
    singletics = 1;
    defdemoname = name;
    gameaction = ga_playdemo;
}

// ---------------------------------------------------------------------------
// G_CheckDemoStatus
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn G_CheckDemoStatus() -> boolean {
    if timingdemo != 0 {
        let endtime = I_GetTime();
        let realtics = endtime - starttime;
        let fps = (gametic as f32 * 35.0) / realtics as f32;
        timingdemo = 0;
        demoplayback = 0;
        I_Error(
            b"timed %i gametics in %i realtics (%f fps)\0".as_ptr() as *const c_char,
            gametic,
            realtics,
            fps as f64,
        );
    }

    if demoplayback != 0 {
        W_ReleaseLumpName(defdemoname);
        demoplayback = 0;
        netdemo = 0;
        netgame = 0;
        deathmatch = 0;
        playeringame[1] = 0;
        playeringame[2] = 0;
        playeringame[3] = 0;
        respawnparm = 0;
        fastparm = 0;
        nomonsters = 0;
        consoleplayer = 0;

        if singledemo != 0 {
            I_Quit();
        } else {
            D_AdvanceDemo();
        }
        return 1;
    }

    if demorecording != 0 {
        *demo_p = DEMOMARKER;
        demo_p = demo_p.add(1);
        M_WriteFile(
            demoname,
            demobuffer as *mut c_void,
            demo_p.offset_from(demobuffer) as c_int,
        );
        Z_Free(demobuffer as *mut c_void);
        demorecording = 0;
        I_Error(b"Demo %s recorded\0".as_ptr() as *const c_char, demoname);
    }

    0
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doom::d_player::TiccmdT;

    fn zeroed_cmd() -> TiccmdT {
        unsafe { std::mem::zeroed() }
    }

    // --- G_CmdChecksum ---

    #[test]
    fn cmdchecksum_all_zeros_is_zero() {
        let cmd = zeroed_cmd();
        unsafe {
            assert_eq!(G_CmdChecksum(&cmd as *const TiccmdT), 0);
        }
    }

    #[test]
    fn cmdchecksum_forwardmove_one() {
        // sizeof(TiccmdT) = 16 bytes = 4 ints; loop runs for 3 ints.
        // First int = bytes [forwardmove(i8), sidemove(i8), angleturn_lo(u8), angleturn_hi(u8)]
        // With forwardmove=1, rest zero: first int = 0x00_00_00_01 = 1 (little-endian).
        let mut cmd = zeroed_cmd();
        cmd.forwardmove = 1;
        unsafe {
            assert_eq!(G_CmdChecksum(&cmd as *const TiccmdT), 1);
        }
    }

    #[test]
    fn cmdchecksum_excludes_last_int() {
        // G_CmdChecksum sums sizeof(ticcmd_t)/4 - 1 = 3 words (bytes 0-11).
        // The last word (bytes 12-15: lookfly + arti + _pad) is NOT summed.
        let mut cmd = zeroed_cmd();
        cmd.lookfly = 0x12;
        cmd.arti = 0x34;
        unsafe {
            assert_eq!(G_CmdChecksum(&cmd as *const TiccmdT), 0);
        }
    }

    #[test]
    fn cmdchecksum_includes_inventory() {
        // inventory is at bytes 8-11 (word 2), which IS included in the sum.
        let mut cmd = zeroed_cmd();
        cmd.inventory = 1;
        unsafe {
            assert_eq!(G_CmdChecksum(&cmd as *const TiccmdT), 1);
        }
    }

    // --- G_ReadDemoTiccmd (non-longtics) ---

    /// Test that G_ReadDemoTiccmd reads the non-longtics format correctly.
    /// Byte layout: [forwardmove, sidemove, angleturn_byte, buttons]
    #[test]
    fn read_demo_ticcmd_non_longtics_basic() {
        unsafe {
            let buf: [u8; 4] = [0x10, 0x20, 0x30, 0x01];
            demo_p = buf.as_ptr() as *mut u8;
            longtics = 0;
            let mut cmd = zeroed_cmd();
            // Don't call G_ReadDemoTiccmd (it calls G_CheckDemoStatus on DEMOMARKER),
            // instead test the inner read logic directly.
            read_demo_ticcmd_inner(&mut demo_p, &mut cmd, false);
            assert_eq!(cmd.forwardmove, 0x10i8);
            assert_eq!(cmd.sidemove, 0x20i8);
            // angleturn: (0x30 as u8 as u32) << 8 = 0x3000 = 12288 as i16
            assert_eq!(cmd.angleturn, 0x3000u16 as i16);
            assert_eq!(cmd.buttons, 0x01);
        }
    }

    /// Negative forwardmove/sidemove: byte 0xFF → -1 as i8.
    #[test]
    fn read_demo_ticcmd_negative_moves() {
        unsafe {
            let buf: [u8; 4] = [0xFF, 0x80, 0x00, 0x00];
            let mut p = buf.as_ptr() as *mut u8;
            let mut cmd = zeroed_cmd();
            read_demo_ticcmd_inner(&mut p, &mut cmd, false);
            assert_eq!(cmd.forwardmove, -1i8);
            assert_eq!(cmd.sidemove, -128i8);
        }
    }

    /// High angleturn byte 0xFF: must produce (0xFF as u32) << 8 = 0xFF00 = -256 as i16.
    /// This is the key signed-unsigned handling that must match vanilla C.
    #[test]
    fn read_demo_ticcmd_high_angleturn_byte() {
        unsafe {
            let buf: [u8; 4] = [0x00, 0x00, 0xFF, 0x00];
            let mut p = buf.as_ptr() as *mut u8;
            let mut cmd = zeroed_cmd();
            read_demo_ticcmd_inner(&mut p, &mut cmd, false);
            // (0xFF as u32) << 8 = 0xFF00; as i16 = -256
            assert_eq!(cmd.angleturn, -256i16);
        }
    }

    /// Angleturn byte 0x80 → 0x8000 = -32768 as i16 (maximum left turn).
    #[test]
    fn read_demo_ticcmd_angleturn_0x80() {
        unsafe {
            let buf: [u8; 4] = [0x00, 0x00, 0x80, 0x00];
            let mut p = buf.as_ptr() as *mut u8;
            let mut cmd = zeroed_cmd();
            read_demo_ticcmd_inner(&mut p, &mut cmd, false);
            assert_eq!(cmd.angleturn, i16::MIN); // 0x8000 = -32768
        }
    }

    // --- G_ReadDemoTiccmd (longtics) ---

    /// Longtics: two-byte little-endian angleturn, each byte treated as unsigned.
    #[test]
    fn read_demo_ticcmd_longtics_basic() {
        unsafe {
            let buf: [u8; 5] = [0x10, 0x20, 0xAB, 0xCD, 0x01];
            let mut p = buf.as_ptr() as *mut u8;
            let mut cmd = zeroed_cmd();
            read_demo_ticcmd_inner(&mut p, &mut cmd, true);
            assert_eq!(cmd.forwardmove, 0x10i8);
            assert_eq!(cmd.sidemove, 0x20i8);
            // angleturn = 0xAB | (0xCD << 8) = 0xCDAB as i16
            assert_eq!(cmd.angleturn, 0xCDABu16 as i16);
            assert_eq!(cmd.buttons, 0x01);
        }
    }

    /// Longtics with bytes [0x00, 0x80]: angleturn = 0 | (0x80 << 8) = 0x8000 = -32768.
    #[test]
    fn read_demo_ticcmd_longtics_high_hi_byte() {
        unsafe {
            let buf: [u8; 5] = [0x00, 0x00, 0x00, 0x80, 0x00];
            let mut p = buf.as_ptr() as *mut u8;
            let mut cmd = zeroed_cmd();
            read_demo_ticcmd_inner(&mut p, &mut cmd, true);
            assert_eq!(cmd.angleturn, i16::MIN);
        }
    }

    /// Longtics zero angleturn.
    #[test]
    fn read_demo_ticcmd_longtics_zero_angleturn() {
        unsafe {
            let buf: [u8; 5] = [0x00, 0x00, 0x00, 0x00, 0x00];
            let mut p = buf.as_ptr() as *mut u8;
            let mut cmd = zeroed_cmd();
            read_demo_ticcmd_inner(&mut p, &mut cmd, true);
            assert_eq!(cmd.angleturn, 0);
        }
    }

    // --- G_VanillaVersionCode ---

    #[test]
    fn vanilla_version_exe_doom_1_666_is_106() {
        assert_eq!(g_vanilla_version_code_for(exe_doom_1_666), 106);
    }

    #[test]
    fn vanilla_version_exe_doom_1_7_is_107() {
        assert_eq!(g_vanilla_version_code_for(exe_doom_1_7), 107);
    }

    #[test]
    fn vanilla_version_exe_doom_1_8_is_108() {
        assert_eq!(g_vanilla_version_code_for(exe_doom_1_8), 108);
    }

    #[test]
    fn vanilla_version_exe_doom_1_9_is_109() {
        assert_eq!(g_vanilla_version_code_for(exe_doom_1_9), 109);
    }

    #[test]
    fn vanilla_version_ultimate_is_109() {
        // exe_ultimate and all later variants map to 109
        assert_eq!(g_vanilla_version_code_for(exe_ultimate), 109);
    }

    #[test]
    fn vanilla_version_exe_final2_is_109() {
        assert_eq!(g_vanilla_version_code_for(exe_final2), 109);
    }

    // --- Movement table values (regression baseline) ---

    #[test]
    fn forwardmove_slow_is_0x19() {
        unsafe {
            assert_eq!(forwardmove[0], 0x19);
        }
    }

    #[test]
    fn forwardmove_fast_is_0x32() {
        unsafe {
            assert_eq!(forwardmove[1], 0x32);
        }
    }

    #[test]
    fn sidemove_slow_is_0x18() {
        unsafe {
            assert_eq!(sidemove[0], 0x18);
        }
    }

    #[test]
    fn sidemove_fast_is_0x28() {
        unsafe {
            assert_eq!(sidemove[1], 0x28);
        }
    }

    #[test]
    fn angleturn_values() {
        unsafe {
            assert_eq!(angleturn[0], 640);
            assert_eq!(angleturn[1], 1280);
            assert_eq!(angleturn[2], 320);
        }
    }

    #[test]
    fn button_constants_match_d_event_h() {
        assert_eq!(BT_ATTACK, 1);
        assert_eq!(BT_USE, 2);
        assert_eq!(BT_CHANGE, 4);
        assert_eq!(BT_SPECIAL, 128);
        assert_eq!(BT_SPECIALMASK, 3);
        assert_eq!(BTS_PAUSE, 1);
        assert_eq!(BTS_SAVEGAME, 2);
        assert_eq!(BTS_SAVEMASK, 28);
        assert_eq!(BTS_SAVESHIFT, 2);
    }
}

// ---------------------------------------------------------------------------
// Inner read helper — used by tests and by G_ReadDemoTiccmd
// ---------------------------------------------------------------------------

/// Pure implementation of the demo ticcmd read, parameterised over the
/// demo-pointer and longtics flag so unit tests can call it without globals.
#[inline]
unsafe fn read_demo_ticcmd_inner(p: &mut *mut u8, cmd: &mut TiccmdT, is_longtics: bool) {
    cmd.forwardmove = (**p) as i8;
    *p = p.add(1);

    cmd.sidemove = (**p) as i8;
    *p = p.add(1);

    if is_longtics {
        let lo = **p as u8;
        *p = p.add(1);
        let hi = **p as u8;
        *p = p.add(1);
        // C: angleturn = lo; angleturn |= (hi << 8);  — both lo and hi unsigned
        cmd.angleturn = (lo as i16) | ((hi as i16) << 8);
    } else {
        let byte = **p as u8;
        *p = p.add(1);
        // C: cmd->angleturn = ((unsigned char)*demo_p++) << 8
        cmd.angleturn = ((byte as u32) << 8) as i16;
    }

    cmd.buttons = **p;
    *p = p.add(1);
}
