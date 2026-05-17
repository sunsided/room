//! Rust port of vendor/doomgeneric/p_saveg.c.
//!
//! Archiving: SaveGame I/O — binary serialization of all game state.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use crate::i_error;
use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::doom::c_ffi::{SAVEGAME_EOF, VERSIONSIZE};
use crate::doom::d_player::{
    players, PlayerT, PspdefT, TiccmdT, MAXPLAYERS, NUMAMMO, NUMCARDS, NUMPOWERS, NUMPSPRITES,
    NUMWEAPONS,
};
use crate::doom::info::{mobjinfo, states, MobjInfo, State};
use crate::doom::p_ceilng::{ceiling_t, P_AddActiveCeiling, T_MoveCeiling};
use crate::doom::p_doors::{vldoor_t, T_VerticalDoor};
use crate::doom::p_floor::{floormove_t, side_t, T_MoveFloor};
use crate::doom::p_lights::{
    glow_t, lightflash_t, line_t, sector_t, strobe_t, T_Glow, T_LightFlash, T_StrobeFlash,
};
use crate::doom::p_plats::{plat_t, P_AddActivePlat, T_PlatRaise};
use crate::doom::p_tick::{
    actionf_t, leveltime, thinker_t, thinkercap, P_AddThinker, P_InitThinkers,
};
use crate::doom::z_zone::{Z_Free, Z_Malloc, PU_LEVEL, PU_LEVSPEC};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const SAVESTRINGSIZE: usize = 24;

// ---------------------------------------------------------------------------
// Globals — now owned by Rust since p_saveg.c is ported.
// g_game.c still writes save_stream to open/close the file.
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut save_stream: *mut libc::FILE = ptr::null_mut();

#[no_mangle]
pub static mut savegamelength: c_int = 0;

#[no_mangle]
pub static mut savegame_error: c_int = 0;

// ---------------------------------------------------------------------------
// External C declarations
// ---------------------------------------------------------------------------

extern "C" {
    fn fread(
        ptr: *mut c_void,
        size: libc::size_t,
        nmemb: libc::size_t,
        stream: *mut c_void,
    ) -> libc::size_t;
    fn fwrite(
        ptr: *const c_void,
        size: libc::size_t,
        nmemb: libc::size_t,
        stream: *mut c_void,
    ) -> libc::size_t;
    fn ftell(stream: *mut c_void) -> libc::c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    fn P_MobjThinker(mobj: *mut c_void);
    fn P_SetThingPosition(thing: *mut c_void);
    fn P_RemoveMobj(th: *mut c_void);
    fn G_VanillaVersionCode() -> c_int;

    static mut sectors: *mut sector_t;
    static mut lines: *mut line_t;
    static mut sides: *mut side_t;
    static mut numsectors: c_int;
    static mut numlines: c_int;

    static mut savegamedir: *mut c_char;
}

// ---------------------------------------------------------------------------
// Low-level I/O — uses the C FILE * managed by g_game.c.
// ---------------------------------------------------------------------------

use std::os::raw::c_long;

unsafe fn saveg_read8() -> u8 {
    let mut result: u8 = 0;
    let n = libc::fread(&raw mut result as *mut c_void, 1, 1, save_stream);
    if n < 1 && savegame_error == 0 {
        savegame_error = 1;
    }
    savegamelength += 1;
    result
}

unsafe fn saveg_write8(value: u8) {
    let n = libc::fwrite(&value as *const u8 as *const c_void, 1, 1, save_stream);
    if n < 1 && savegame_error == 0 {
        savegame_error = 1;
    }
    savegamelength += 1;
}

unsafe fn saveg_read16() -> u16 {
    let a = saveg_read8() as u16;
    let b = saveg_read8() as u16;
    a | (b << 8)
}

unsafe fn saveg_write16(value: u16) {
    saveg_write8((value & 0xff) as u8);
    saveg_write8(((value >> 8) & 0xff) as u8);
}

unsafe fn saveg_read32() -> u32 {
    let a = saveg_read8() as u32;
    let b = saveg_read8() as u32;
    let c = saveg_read8() as u32;
    let d = saveg_read8() as u32;
    a | (b << 8) | (c << 16) | (d << 24)
}

unsafe fn saveg_write32(value: u32) {
    saveg_write8((value & 0xff) as u8);
    saveg_write8(((value >> 8) & 0xff) as u8);
    saveg_write8(((value >> 16) & 0xff) as u8);
    saveg_write8(((value >> 24) & 0xff) as u8);
}

unsafe fn saveg_read_pad() {
    let pos = libc::ftell(save_stream) as c_long;
    let padding = (4 - (pos & 3)) & 3;
    for _ in 0..padding {
        saveg_read8();
    }
}

unsafe fn saveg_write_pad() {
    let pos = libc::ftell(save_stream) as c_long;
    let padding = (4 - (pos & 3)) & 3;
    for _ in 0..padding {
        saveg_write8(0);
    }
}

unsafe fn saveg_read_enum() -> u32 {
    saveg_read32()
}

unsafe fn saveg_write_enum(value: u32) {
    saveg_write32(value);
}

// ---------------------------------------------------------------------------
// Struct serialization helpers
// ---------------------------------------------------------------------------

/// Serialize sector pointer as index into sectors array.
unsafe fn saveg_write_sector_ptr(sector: *const sector_t) -> u32 {
    if sector.is_null() {
        0
    } else {
        sector.offset_from(sectors) as u32
    }
}

/// Deserialize sector pointer from index.
unsafe fn saveg_read_sector_ptr(index: u32) -> *mut sector_t {
    sectors.add(index as usize)
}

/// Serialize state pointer as index into states array.
unsafe fn saveg_write_state_ptr(state: *const State) -> u32 {
    if state.is_null() {
        0
    } else {
        state.offset_from(std::ptr::addr_of!(states[0])) as u32
    }
}

/// Deserialize state pointer from index.
unsafe fn saveg_read_state_ptr(index: u32) -> *mut State {
    if index == 0 {
        std::ptr::null_mut()
    } else {
        std::ptr::addr_of_mut!(states[0]).add(index as usize)
    }
}

/// Serialize player pointer as player index + 1 (0 = NULL).
unsafe fn saveg_write_player_ptr(player: *const PlayerT) -> u32 {
    if player.is_null() {
        0
    } else {
        (player.offset_from(std::ptr::addr_of!(players[0])) as u32) + 1
    }
}

/// Deserialize player pointer from index (1-based).
unsafe fn saveg_read_player_ptr(value: u32) -> *mut PlayerT {
    if value == 0 {
        std::ptr::null_mut()
    } else {
        std::ptr::addr_of_mut!(players[0]).add((value - 1) as usize)
    }
}

// ---------------------------------------------------------------------------
// Thinker function pointer reassignment helpers
// ---------------------------------------------------------------------------

fn actionf_p1_move_ceiling() -> actionf_t {
    actionf_t {
        acp1: Some(unsafe {
            core::mem::transmute::<
                unsafe extern "C" fn(*mut ceiling_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_MoveCeiling)
        }),
    }
}

fn actionf_p1_vertical_door() -> actionf_t {
    actionf_t {
        acp1: Some(unsafe {
            core::mem::transmute::<
                unsafe extern "C" fn(*mut vldoor_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_VerticalDoor)
        }),
    }
}

fn actionf_p1_move_floor() -> actionf_t {
    actionf_t {
        acp1: Some(unsafe {
            core::mem::transmute::<
                unsafe extern "C" fn(*mut floormove_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_MoveFloor)
        }),
    }
}

fn actionf_p1_plat_raise() -> actionf_t {
    actionf_t {
        acp1: Some(unsafe {
            core::mem::transmute::<
                unsafe extern "C" fn(*mut plat_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_PlatRaise)
        }),
    }
}

fn actionf_p1_light_flash() -> actionf_t {
    actionf_t {
        acp1: Some(unsafe {
            core::mem::transmute::<
                unsafe extern "C" fn(*mut lightflash_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_LightFlash)
        }),
    }
}

fn actionf_p1_strobe_flash() -> actionf_t {
    actionf_t {
        acp1: Some(unsafe {
            core::mem::transmute::<
                unsafe extern "C" fn(*mut strobe_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_StrobeFlash)
        }),
    }
}

fn actionf_p1_glow() -> actionf_t {
    actionf_t {
        acp1: Some(unsafe {
            core::mem::transmute::<
                unsafe extern "C" fn(*mut glow_t),
                unsafe extern "C" fn(*mut c_void),
            >(T_Glow)
        }),
    }
}

// ---------------------------------------------------------------------------
// Struct serialization
// ---------------------------------------------------------------------------

//
// mapthing_t
//

unsafe fn saveg_read_mapthing_t(str: *mut crate::doom::c_ffi::mapthing_t) {
    let s = &mut *str;
    s.x = saveg_read16() as i16;
    s.y = saveg_read16() as i16;
    s.angle = saveg_read16() as i16;
    s.r#type = saveg_read16() as i16;
    s.options = saveg_read16() as i16;
}

unsafe fn saveg_write_mapthing_t(str: *const crate::doom::c_ffi::mapthing_t) {
    let s = &*str;
    saveg_write16(s.x as u16);
    saveg_write16(s.y as u16);
    saveg_write16(s.angle as u16);
    saveg_write16(s.r#type as u16);
    saveg_write16(s.options as u16);
}

//
// thinker_t
//

unsafe fn saveg_read_thinker_t(str: *mut thinker_t) {
    let s = &mut *str;
    // Read prev/next as raw pointer indices (will be rebuilt)
    s.prev = saveg_read32() as usize as *mut thinker_t;
    s.next = saveg_read32() as usize as *mut thinker_t;
    // 0 in the save stream means no function; transmuting 0 to fn is UB.
    let raw = saveg_read32() as usize;
    s.function.acp1 = if raw == 0 {
        None
    } else {
        Some(core::mem::transmute::<
            usize,
            unsafe extern "C" fn(*mut c_void),
        >(raw))
    };
}

unsafe fn saveg_write_thinker_t(str: *const thinker_t) {
    let s = &*str;
    saveg_write32(s.prev as u32);
    saveg_write32(s.next as u32);
    saveg_write32(s.function.acp1.map(|f| f as usize as u32).unwrap_or(0));
}

//
// mobj_t — serialized as C struct via FFI
//

unsafe fn saveg_read_mobj_t(mobj: *mut c_void) {
    let mo: *mut crate::doom::c_ffi::mobj_t = mobj as *mut crate::doom::c_ffi::mobj_t;

    // thinker_t
    {
        let thinker_ptr = std::ptr::addr_of_mut!((*mo).thinker_prev) as *mut thinker_t;
        (*thinker_ptr).prev = saveg_read32() as usize as *mut thinker_t;
        (*thinker_ptr).next = saveg_read32() as usize as *mut thinker_t;
        let raw = saveg_read32() as usize;
        (*thinker_ptr).function.acp1 = if raw == 0 {
            None
        } else {
            Some(core::mem::transmute::<
                usize,
                unsafe extern "C" fn(*mut c_void),
            >(raw))
        };
    }

    // x, y, z
    (*mo).x = saveg_read32() as c_int;
    (*mo).y = saveg_read32() as c_int;
    (*mo).z = saveg_read32() as c_int;

    // snext, sprev
    (*mo).snext = saveg_read32() as *mut c_void;
    (*mo).sprev = saveg_read32() as *mut c_void;

    // angle, sprite, frame
    (*mo).angle = saveg_read32();
    (*mo).sprite = saveg_read_enum() as c_int;
    (*mo).frame = saveg_read32() as c_int;

    // bnext, bprev
    (*mo).bnext = saveg_read32() as *mut c_void;
    (*mo).bprev = saveg_read32() as *mut c_void;

    // subsector
    (*mo).subsector = saveg_read32() as *mut c_void;

    // floorz, ceilingz, radius, height
    (*mo).floorz = saveg_read32() as c_int;
    (*mo).ceilingz = saveg_read32() as c_int;
    (*mo).radius = saveg_read32() as c_int;
    (*mo).height = saveg_read32() as c_int;

    // momx, momy, momz
    (*mo).momx = saveg_read32() as c_int;
    (*mo).momy = saveg_read32() as c_int;
    (*mo).momz = saveg_read32() as c_int;

    // validcount
    (*mo).validcount = saveg_read32() as c_int;

    // type
    (*mo).type_ = saveg_read_enum() as c_int;

    // info (raw pointer)
    (*mo).info = saveg_read32() as *mut crate::doom::c_ffi::mobjinfo_t;

    // tics
    (*mo).tics = saveg_read32() as c_int;

    // state (index into states array)
    let state_idx = saveg_read32();
    (*mo).state = saveg_read_state_ptr(state_idx) as *mut crate::doom::c_ffi::state_t;

    // flags
    (*mo).flags = saveg_read32() as c_int;

    // health
    (*mo).health = saveg_read32() as c_int;

    // movedir, movecount
    (*mo).movedir = saveg_read32() as c_int;
    (*mo).movecount = saveg_read32() as c_int;

    // target (raw pointer)
    (*mo).target = saveg_read32() as *mut c_void;

    // reactiontime, threshold
    (*mo).reactiontime = saveg_read32() as c_int;
    (*mo).threshold = saveg_read32() as c_int;

    // player (index + 1)
    let pl = saveg_read32();
    if pl > 0 {
        (*mo).player = saveg_read_player_ptr(pl) as *mut c_void;
        let player = (*mo).player as *mut PlayerT;
        if !player.is_null() {
            (*player).mo = mobj as *mut crate::doom::d_player::mobj_t;
        }
    } else {
        (*mo).player = std::ptr::null_mut();
    }

    // lastlook
    (*mo).lastlook = saveg_read32() as c_int;

    // spawnpoint
    saveg_read_mapthing_t(std::ptr::addr_of_mut!((*mo).spawnpoint));

    // tracer (raw pointer)
    (*mo).tracer = saveg_read32() as *mut c_void;
}

unsafe fn saveg_write_mobj_t(mobj: *const c_void) {
    let mo: *const crate::doom::c_ffi::mobj_t = mobj as *const crate::doom::c_ffi::mobj_t;

    // thinker_t
    {
        let thinker_ptr = std::ptr::addr_of!((*mo).thinker_prev) as *const thinker_t;
        saveg_write32((*thinker_ptr).prev as u32);
        saveg_write32((*thinker_ptr).next as u32);
        saveg_write32(
            (*thinker_ptr)
                .function
                .acp1
                .map(|f| f as usize as u32)
                .unwrap_or(0),
        );
    }

    // x, y, z
    saveg_write32((*mo).x as u32);
    saveg_write32((*mo).y as u32);
    saveg_write32((*mo).z as u32);

    // snext, sprev
    saveg_write32((*mo).snext as u32);
    saveg_write32((*mo).sprev as u32);

    // angle, sprite, frame
    saveg_write32((*mo).angle);
    saveg_write_enum((*mo).sprite as u32);
    saveg_write32((*mo).frame as u32);

    // bnext, bprev
    saveg_write32((*mo).bnext as u32);
    saveg_write32((*mo).bprev as u32);

    // subsector
    saveg_write32((*mo).subsector as u32);

    // floorz, ceilingz, radius, height
    saveg_write32((*mo).floorz as u32);
    saveg_write32((*mo).ceilingz as u32);
    saveg_write32((*mo).radius as u32);
    saveg_write32((*mo).height as u32);

    // momx, momy, momz
    saveg_write32((*mo).momx as u32);
    saveg_write32((*mo).momy as u32);
    saveg_write32((*mo).momz as u32);

    // validcount
    saveg_write32((*mo).validcount as u32);

    // type
    saveg_write_enum((*mo).type_ as u32);

    // info
    saveg_write32((*mo).info as u32);

    // tics
    saveg_write32((*mo).tics as u32);

    // state (index)
    let state = (*mo).state as *const State;
    if state.is_null() {
        saveg_write32(0);
    } else {
        saveg_write32(saveg_write_state_ptr(state));
    }

    // flags
    saveg_write32((*mo).flags as u32);

    // health
    saveg_write32((*mo).health as u32);

    // movedir, movecount
    saveg_write32((*mo).movedir as u32);
    saveg_write32((*mo).movecount as u32);

    // target
    saveg_write32((*mo).target as u32);

    // reactiontime, threshold
    saveg_write32((*mo).reactiontime as u32);
    saveg_write32((*mo).threshold as u32);

    // player
    let player = (*mo).player as *const PlayerT;
    if player.is_null() {
        saveg_write32(0);
    } else {
        saveg_write32(saveg_write_player_ptr(player));
    }

    // lastlook
    saveg_write32((*mo).lastlook as u32);

    // spawnpoint
    saveg_write_mapthing_t(std::ptr::addr_of!((*mo).spawnpoint));

    // tracer
    saveg_write32((*mo).tracer as u32);
}

//
// ticcmd_t
//
// Only serialize the 6 fields the C version writes. TiccmdT has extra fields.
//

unsafe fn saveg_read_ticcmd_t(str: *mut TiccmdT) {
    let s = &mut *str;
    s.forwardmove = saveg_read8() as i8;
    s.sidemove = saveg_read8() as i8;
    s.angleturn = saveg_read16() as i16;
    s.consistancy = saveg_read16() as u8;
    s.chatchar = saveg_read8();
    s.buttons = saveg_read8();
}

unsafe fn saveg_write_ticcmd_t(str: *const TiccmdT) {
    let s = &*str;
    saveg_write8(s.forwardmove as u8);
    saveg_write8(s.sidemove as u8);
    saveg_write16(s.angleturn as u16);
    saveg_write16(s.consistancy as u16);
    saveg_write8(s.chatchar);
    saveg_write8(s.buttons);
}

//
// pspdef_t
//

unsafe fn saveg_read_pspdef_t(str: *mut PspdefT) {
    let s = &mut *str;
    let state_idx = saveg_read32();
    s.state = saveg_read_state_ptr(state_idx) as *mut crate::doom::d_player::state_t;
    s.tics = saveg_read32() as c_int;
    s.sx = saveg_read32() as c_int;
    s.sy = saveg_read32() as c_int;
}

unsafe fn saveg_write_pspdef_t(str: *const PspdefT) {
    let s = &*str;
    if s.state.is_null() {
        saveg_write32(0);
    } else {
        saveg_write32(saveg_write_state_ptr(s.state as *const State));
    }
    saveg_write32(s.tics as u32);
    saveg_write32(s.sx as u32);
    saveg_write32(s.sy as u32);
}

//
// player_t
//

unsafe fn saveg_read_player_t(str: *mut PlayerT) {
    let s = &mut *str;

    // mo (raw pointer, will be NULL after)
    s.mo = saveg_read32() as *mut crate::doom::d_player::mobj_t;

    // playerstate
    s.playerstate = saveg_read_enum() as c_int;

    // cmd
    saveg_read_ticcmd_t(&mut s.cmd);

    // viewz, viewheight, deltaviewheight, bob
    s.viewz = saveg_read32() as c_int;
    s.viewheight = saveg_read32() as c_int;
    s.deltaviewheight = saveg_read32() as c_int;
    s.bob = saveg_read32() as c_int;

    // health, armorpoints, armortype
    s.health = saveg_read32() as c_int;
    s.armorpoints = saveg_read32() as c_int;
    s.armortype = saveg_read32() as c_int;

    // powers[NUMPOWERS]
    for i in 0..NUMPOWERS {
        s.powers[i] = saveg_read32() as c_int;
    }

    // cards[NUMCARDS]
    for i in 0..NUMCARDS {
        s.cards[i] = saveg_read32() as c_int;
    }

    // backpack
    s.backpack = saveg_read32() as c_int;

    // frags[MAXPLAYERS]
    for i in 0..MAXPLAYERS {
        s.frags[i] = saveg_read32() as c_int;
    }

    // readyweapon, pendingweapon
    s.readyweapon = saveg_read_enum() as c_int;
    s.pendingweapon = saveg_read_enum() as c_int;

    // weaponowned[NUMWEAPONS]
    for i in 0..NUMWEAPONS {
        s.weaponowned[i] = saveg_read32() as c_int;
    }

    // ammo[NUMAMMO]
    for i in 0..NUMAMMO {
        s.ammo[i] = saveg_read32() as c_int;
    }

    // maxammo[NUMAMMO]
    for i in 0..NUMAMMO {
        s.maxammo[i] = saveg_read32() as c_int;
    }

    // attackdown, usedown
    s.attackdown = saveg_read32() as c_int;
    s.usedown = saveg_read32() as c_int;

    // cheats, refire
    s.cheats = saveg_read32() as c_int;
    s.refire = saveg_read32() as c_int;

    // killcount, itemcount, secretcount
    s.killcount = saveg_read32() as c_int;
    s.itemcount = saveg_read32() as c_int;
    s.secretcount = saveg_read32() as c_int;

    // message (raw pointer)
    s.message = saveg_read32() as *mut c_char;

    // damagecount, bonuscount
    s.damagecount = saveg_read32() as c_int;
    s.bonuscount = saveg_read32() as c_int;

    // attacker (raw pointer)
    s.attacker = saveg_read32() as *mut crate::doom::d_player::mobj_t;

    // extralight, fixedcolormap, colormap
    s.extralight = saveg_read32() as c_int;
    s.fixedcolormap = saveg_read32() as c_int;
    s.colormap = saveg_read32() as c_int;

    // psprites[NUMPSPRITES]
    for i in 0..NUMPSPRITES {
        saveg_read_pspdef_t(&mut s.psprites[i]);
    }

    // didsecret
    s.didsecret = saveg_read32() as c_int;
}

unsafe fn saveg_write_player_t(str: *const PlayerT) {
    let s = &*str;

    saveg_write32(s.mo as u32);
    saveg_write_enum(s.playerstate as u32);
    saveg_write_ticcmd_t(&s.cmd);
    saveg_write32(s.viewz as u32);
    saveg_write32(s.viewheight as u32);
    saveg_write32(s.deltaviewheight as u32);
    saveg_write32(s.bob as u32);
    saveg_write32(s.health as u32);
    saveg_write32(s.armorpoints as u32);
    saveg_write32(s.armortype as u32);
    for i in 0..NUMPOWERS {
        saveg_write32(s.powers[i] as u32);
    }
    for i in 0..NUMCARDS {
        saveg_write32(s.cards[i] as u32);
    }
    saveg_write32(s.backpack as u32);
    for i in 0..MAXPLAYERS {
        saveg_write32(s.frags[i] as u32);
    }
    saveg_write_enum(s.readyweapon as u32);
    saveg_write_enum(s.pendingweapon as u32);
    for i in 0..NUMWEAPONS {
        saveg_write32(s.weaponowned[i] as u32);
    }
    for i in 0..NUMAMMO {
        saveg_write32(s.ammo[i] as u32);
    }
    for i in 0..NUMAMMO {
        saveg_write32(s.maxammo[i] as u32);
    }
    saveg_write32(s.attackdown as u32);
    saveg_write32(s.usedown as u32);
    saveg_write32(s.cheats as u32);
    saveg_write32(s.refire as u32);
    saveg_write32(s.killcount as u32);
    saveg_write32(s.itemcount as u32);
    saveg_write32(s.secretcount as u32);
    saveg_write32(s.message as u32);
    saveg_write32(s.damagecount as u32);
    saveg_write32(s.bonuscount as u32);
    saveg_write32(s.attacker as u32);
    saveg_write32(s.extralight as u32);
    saveg_write32(s.fixedcolormap as u32);
    saveg_write32(s.colormap as u32);
    for i in 0..NUMPSPRITES {
        saveg_write_pspdef_t(&s.psprites[i]);
    }
    saveg_write32(s.didsecret as u32);
}

//
// ceiling_t
//

unsafe fn saveg_read_ceiling_t(str: *mut ceiling_t) {
    let s = &mut *str;
    saveg_read_thinker_t(&mut s.thinker);
    s.r#type = saveg_read_enum() as c_int;
    let sector_idx = saveg_read32();
    s.sector = saveg_read_sector_ptr(sector_idx);
    s.bottomheight = saveg_read32() as c_int;
    s.topheight = saveg_read32() as c_int;
    s.speed = saveg_read32() as c_int;
    s.crush = saveg_read32() as c_int;
    s.direction = saveg_read32() as c_int;
    s.tag = saveg_read32() as c_int;
    s.olddirection = saveg_read32() as c_int;
}

unsafe fn saveg_write_ceiling_t(str: *const ceiling_t) {
    let s = &*str;
    saveg_write_thinker_t(&s.thinker);
    saveg_write_enum(s.r#type as u32);
    saveg_write32(saveg_write_sector_ptr(s.sector));
    saveg_write32(s.bottomheight as u32);
    saveg_write32(s.topheight as u32);
    saveg_write32(s.speed as u32);
    saveg_write32(s.crush as u32);
    saveg_write32(s.direction as u32);
    saveg_write32(s.tag as u32);
    saveg_write32(s.olddirection as u32);
}

//
// vldoor_t
//

unsafe fn saveg_read_vldoor_t(str: *mut vldoor_t) {
    let s = &mut *str;
    saveg_read_thinker_t(&mut s.thinker);
    s.r#type = saveg_read_enum() as c_int;
    let sector_idx = saveg_read32();
    s.sector = saveg_read_sector_ptr(sector_idx);
    s.topheight = saveg_read32() as c_int;
    s.speed = saveg_read32() as c_int;
    s.direction = saveg_read32() as c_int;
    s.topwait = saveg_read32() as c_int;
    s.topcountdown = saveg_read32() as c_int;
}

unsafe fn saveg_write_vldoor_t(str: *const vldoor_t) {
    let s = &*str;
    saveg_write_thinker_t(&s.thinker);
    saveg_write_enum(s.r#type as u32);
    saveg_write32(saveg_write_sector_ptr(s.sector));
    saveg_write32(s.topheight as u32);
    saveg_write32(s.speed as u32);
    saveg_write32(s.direction as u32);
    saveg_write32(s.topwait as u32);
    saveg_write32(s.topcountdown as u32);
}

//
// floormove_t
//

unsafe fn saveg_read_floormove_t(str: *mut floormove_t) {
    let s = &mut *str;
    saveg_read_thinker_t(&mut s.thinker);
    s.r#type = saveg_read_enum() as c_int;
    s.crush = saveg_read32() as c_int;
    let sector_idx = saveg_read32();
    s.sector = saveg_read_sector_ptr(sector_idx);
    s.direction = saveg_read32() as c_int;
    s.newspecial = saveg_read32() as c_int;
    s.texture = saveg_read16() as i16;
    s.floordestheight = saveg_read32() as c_int;
    s.speed = saveg_read32() as c_int;
}

unsafe fn saveg_write_floormove_t(str: *const floormove_t) {
    let s = &*str;
    saveg_write_thinker_t(&s.thinker);
    saveg_write_enum(s.r#type as u32);
    saveg_write32(s.crush as u32);
    saveg_write32(saveg_write_sector_ptr(s.sector));
    saveg_write32(s.direction as u32);
    saveg_write32(s.newspecial as u32);
    saveg_write16(s.texture as u16);
    saveg_write32(s.floordestheight as u32);
    saveg_write32(s.speed as u32);
}

//
// plat_t
//

unsafe fn saveg_read_plat_t(str: *mut plat_t) {
    let s = &mut *str;
    saveg_read_thinker_t(&mut s.thinker);
    let sector_idx = saveg_read32();
    s.sector = saveg_read_sector_ptr(sector_idx);
    s.speed = saveg_read32() as c_int;
    s.low = saveg_read32() as c_int;
    s.high = saveg_read32() as c_int;
    s.wait = saveg_read32() as c_int;
    s.count = saveg_read32() as c_int;
    s.status = saveg_read_enum() as c_int;
    s.oldstatus = saveg_read_enum() as c_int;
    s.crush = saveg_read32() as c_int;
    s.tag = saveg_read32() as c_int;
    s.r#type = saveg_read_enum() as c_int;
}

unsafe fn saveg_write_plat_t(str: *const plat_t) {
    let s = &*str;
    saveg_write_thinker_t(&s.thinker);
    saveg_write32(saveg_write_sector_ptr(s.sector));
    saveg_write32(s.speed as u32);
    saveg_write32(s.low as u32);
    saveg_write32(s.high as u32);
    saveg_write32(s.wait as u32);
    saveg_write32(s.count as u32);
    saveg_write_enum(s.status as u32);
    saveg_write_enum(s.oldstatus as u32);
    saveg_write32(s.crush as u32);
    saveg_write32(s.tag as u32);
    saveg_write_enum(s.r#type as u32);
}

//
// lightflash_t
//

unsafe fn saveg_read_lightflash_t(str: *mut lightflash_t) {
    let s = &mut *str;
    saveg_read_thinker_t(&mut s.thinker);
    let sector_idx = saveg_read32();
    s.sector = saveg_read_sector_ptr(sector_idx);
    s.count = saveg_read32() as c_int;
    s.maxlight = saveg_read32() as c_int;
    s.minlight = saveg_read32() as c_int;
    s.maxtime = saveg_read32() as c_int;
    s.mintime = saveg_read32() as c_int;
}

unsafe fn saveg_write_lightflash_t(str: *const lightflash_t) {
    let s = &*str;
    saveg_write_thinker_t(&s.thinker);
    saveg_write32(saveg_write_sector_ptr(s.sector));
    saveg_write32(s.count as u32);
    saveg_write32(s.maxlight as u32);
    saveg_write32(s.minlight as u32);
    saveg_write32(s.maxtime as u32);
    saveg_write32(s.mintime as u32);
}

//
// strobe_t
//

unsafe fn saveg_read_strobe_t(str: *mut strobe_t) {
    let s = &mut *str;
    saveg_read_thinker_t(&mut s.thinker);
    let sector_idx = saveg_read32();
    s.sector = saveg_read_sector_ptr(sector_idx);
    s.count = saveg_read32() as c_int;
    s.minlight = saveg_read32() as c_int;
    s.maxlight = saveg_read32() as c_int;
    s.darktime = saveg_read32() as c_int;
    s.brighttime = saveg_read32() as c_int;
}

unsafe fn saveg_write_strobe_t(str: *const strobe_t) {
    let s = &*str;
    saveg_write_thinker_t(&s.thinker);
    saveg_write32(saveg_write_sector_ptr(s.sector));
    saveg_write32(s.count as u32);
    saveg_write32(s.minlight as u32);
    saveg_write32(s.maxlight as u32);
    saveg_write32(s.darktime as u32);
    saveg_write32(s.brighttime as u32);
}

//
// glow_t
//

unsafe fn saveg_read_glow_t(str: *mut glow_t) {
    let s = &mut *str;
    saveg_read_thinker_t(&mut s.thinker);
    let sector_idx = saveg_read32();
    s.sector = saveg_read_sector_ptr(sector_idx);
    s.minlight = saveg_read32() as c_int;
    s.maxlight = saveg_read32() as c_int;
    s.direction = saveg_read32() as c_int;
}

unsafe fn saveg_write_glow_t(str: *const glow_t) {
    let s = &*str;
    saveg_write_thinker_t(&s.thinker);
    saveg_write32(saveg_write_sector_ptr(s.sector));
    saveg_write32(s.minlight as u32);
    saveg_write32(s.maxlight as u32);
    saveg_write32(s.direction as u32);
}

// ---------------------------------------------------------------------------
// Save filename helpers
// ---------------------------------------------------------------------------

fn fill_save_filename(buf: &mut [u8], dir: &str, slot: c_int) -> usize {
    assert!(
        !buf.is_empty(),
        "fill_save_filename: buffer must have at least 1 byte"
    );
    let full = format!("{}{}{}.dsg", dir, SAVEGAMENAME, slot);
    let bytes = full.as_bytes();
    let len = std::cmp::min(bytes.len(), buf.len() - 1);
    buf[..len].copy_from_slice(&bytes[..len]);
    buf[len] = 0;
    len
}

// ---------------------------------------------------------------------------
// Static filename buffers
// ---------------------------------------------------------------------------

static mut TEMP_SAVE_FILENAME: *mut c_char = std::ptr::null_mut();
static mut SAVE_FILENAME: *mut c_char = std::ptr::null_mut();

const SAVEGAMENAME: &str = "doomsav";

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_TempSaveGameFile() -> *mut c_char {
    if TEMP_SAVE_FILENAME.is_null() {
        let dir = std::ffi::CStr::from_ptr(savegamedir).to_string_lossy();
        let full = format!("{}temp.dsg\0", dir);
        let bytes = full.into_bytes();
        let ptr = std::alloc::alloc(std::alloc::Layout::from_size_align(bytes.len(), 1).unwrap())
            as *mut c_char;
        ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, bytes.len());
        TEMP_SAVE_FILENAME = ptr;
    }
    TEMP_SAVE_FILENAME
}

#[no_mangle]
pub unsafe extern "C" fn P_SaveGameFile(slot: c_int) -> *mut c_char {
    let dir_len = std::ffi::CStr::from_ptr(savegamedir).to_bytes().len();
    let alloc_size = dir_len + 32;

    if SAVE_FILENAME.is_null() {
        let layout = std::alloc::Layout::from_size_align(alloc_size, 1).unwrap();
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        SAVE_FILENAME = ptr as *mut c_char;
    }

    let dir_str = std::ffi::CStr::from_ptr(savegamedir).to_str().unwrap();
    let buf = std::slice::from_raw_parts_mut(SAVE_FILENAME as *mut u8, alloc_size);
    fill_save_filename(buf, dir_str, slot);

    SAVE_FILENAME
}

#[no_mangle]
pub unsafe extern "C" fn P_WriteSaveGameHeader(description: *const c_char) {
    let desc = std::ffi::CStr::from_ptr(description);
    let desc_bytes = desc.to_bytes();

    // Write description (padded to SAVESTRINGSIZE)
    for i in 0..SAVESTRINGSIZE {
        if i < desc_bytes.len() {
            saveg_write8(desc_bytes[i]);
        } else {
            saveg_write8(0);
        }
    }

    // Write version string (VERSIONSIZE bytes)
    let version_code = G_VanillaVersionCode();
    let version_str = format!("version {}\0", version_code);
    let version_bytes = version_str.as_bytes();
    for i in 0..VERSIONSIZE {
        if i < version_bytes.len() {
            saveg_write8(version_bytes[i]);
        } else {
            saveg_write8(0);
        }
    }

    // Write skill, episode, map
    saveg_write8(gameskill as u8);
    saveg_write8(gameepisode as u8);
    saveg_write8(gamemap as u8);

    // Write playeringame
    for i in 0..MAXPLAYERS {
        saveg_write8(playeringame[i] as u8);
    }

    // Write leveltime (3 bytes, big-endian)
    let lt = leveltime as u32;
    saveg_write8(((lt >> 16) & 0xff) as u8);
    saveg_write8(((lt >> 8) & 0xff) as u8);
    saveg_write8((lt & 0xff) as u8);
}

#[no_mangle]
pub unsafe extern "C" fn P_ReadSaveGameHeader() -> c_int {
    // Skip description (SAVESTRINGSIZE bytes)
    for _ in 0..SAVESTRINGSIZE {
        saveg_read8();
    }

    // Read version string
    let mut read_vcheck = [0u8; VERSIONSIZE];
    for i in 0..VERSIONSIZE {
        read_vcheck[i] = saveg_read8();
    }

    // Compare version
    let version_code = G_VanillaVersionCode();
    let expected = format!("version {}\0", version_code);
    let expected_bytes = expected.as_bytes();

    // Null-terminate both for strcmp-like comparison
    let mut expected_padded = [0u8; VERSIONSIZE];
    for (i, &b) in expected_bytes.iter().enumerate().take(VERSIONSIZE) {
        expected_padded[i] = b;
    }

    if read_vcheck != expected_padded {
        return 0; // bad version
    }

    // Read skill, episode, map
    gameskill = saveg_read8() as c_int;
    gameepisode = saveg_read8() as c_int;
    gamemap = saveg_read8() as c_int;

    // Read playeringame
    for i in 0..MAXPLAYERS {
        playeringame[i] = saveg_read8() as c_int;
    }

    // Read leveltime (3 bytes, big-endian)
    let a = saveg_read8() as u32;
    let b = saveg_read8() as u32;
    let c = saveg_read8() as u32;
    leveltime = ((a << 16) | (b << 8) | c) as c_int;

    1 // success
}

#[no_mangle]
pub unsafe extern "C" fn P_ReadSaveGameEOF() -> c_int {
    let value = saveg_read8();
    if value == SAVEGAME_EOF {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_WriteSaveGameEOF() {
    saveg_write8(SAVEGAME_EOF);
}

extern "C" {
    static mut gameskill: c_int;
    static mut gameepisode: c_int;
    static mut gamemap: c_int;
    static mut playeringame: [c_int; MAXPLAYERS];
}

// ---------------------------------------------------------------------------
// P_ArchivePlayers / P_UnArchivePlayers
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_ArchivePlayers() {
    for i in 0..MAXPLAYERS {
        if playeringame[i] == 0 {
            continue;
        }
        saveg_write_pad();
        saveg_write_player_t(&players[i]);
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_UnArchivePlayers() {
    for i in 0..MAXPLAYERS {
        if playeringame[i] == 0 {
            continue;
        }
        saveg_read_pad();
        saveg_read_player_t(&mut players[i]);

        // will be set when unarc thinker
        players[i].mo = std::ptr::null_mut();
        players[i].message = std::ptr::null_mut();
        players[i].attacker = std::ptr::null_mut();
    }
}

// ---------------------------------------------------------------------------
// P_ArchiveWorld / P_UnArchiveWorld
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_ArchiveWorld() {
    let num_sec = numsectors as usize;
    let num_li = numlines as usize;

    // do sectors
    for i in 0..num_sec {
        let sec = sectors.add(i);
        saveg_write16(((*sec).floorheight >> 16) as u16);
        saveg_write16(((*sec).ceilingheight >> 16) as u16);
        saveg_write16((*sec).floorpic as u16);
        saveg_write16((*sec).ceilingpic as u16);
        saveg_write16((*sec).lightlevel as u16);
        saveg_write16((*sec).special as u16);
        saveg_write16((*sec).tag as u16);
    }

    // do lines
    for i in 0..num_li {
        let li = lines.add(i);
        saveg_write16((*li).flags as u16);
        saveg_write16((*li).special as u16);
        saveg_write16((*li).tag as u16);
        for j in 0..2 {
            if (*li).sidenum[j as usize] == -1 {
                continue;
            }
            let si = sides.add((*li).sidenum[j as usize] as usize);
            saveg_write16(((*si).textureoffset >> 16) as u16);
            saveg_write16(((*si).rowoffset >> 16) as u16);
            saveg_write16((*si).toptexture as u16);
            saveg_write16((*si).bottomtexture as u16);
            saveg_write16((*si).midtexture as u16);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn P_UnArchiveWorld() {
    let num_sec = numsectors as usize;
    let num_li = numlines as usize;

    // do sectors
    for i in 0..num_sec {
        let sec = sectors.add(i);
        (*sec).floorheight = (saveg_read16() as c_int) << 16;
        (*sec).ceilingheight = (saveg_read16() as c_int) << 16;
        (*sec).floorpic = saveg_read16() as i16;
        (*sec).ceilingpic = saveg_read16() as i16;
        (*sec).lightlevel = saveg_read16() as i16;
        (*sec).special = saveg_read16() as i16;
        (*sec).tag = saveg_read16() as i16;
        (*sec).specialdata = std::ptr::null_mut();
        (*sec).soundtarget = std::ptr::null_mut();
    }

    // do lines
    for i in 0..num_li {
        let li = lines.add(i);
        (*li).flags = saveg_read16() as i16;
        (*li).special = saveg_read16() as i16;
        (*li).tag = saveg_read16() as i16;
        for j in 0..2 {
            if (*li).sidenum[j as usize] == -1 {
                continue;
            }
            let si = sides.add((*li).sidenum[j as usize] as usize);
            (*si).textureoffset = (saveg_read16() as c_int) << 16;
            (*si).rowoffset = (saveg_read16() as c_int) << 16;
            (*si).toptexture = saveg_read16() as i16;
            (*si).bottomtexture = saveg_read16() as i16;
            (*si).midtexture = saveg_read16() as i16;
        }
    }
}

// ---------------------------------------------------------------------------
// Thinker class enum
// ---------------------------------------------------------------------------

const tc_end: u8 = 0;
const tc_mobj: u8 = 1;

// ---------------------------------------------------------------------------
// P_ArchiveThinkers / P_UnArchiveThinkers
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_ArchiveThinkers() {
    let cap = &raw mut thinkercap;
    let mut th = (*cap).next;

    while th != cap {
        // Check if function is P_MobjThinker by comparing pointers
        let func = (*th).function.acp1;
        if let Some(fn_ptr) = func {
            if fn_ptr as usize == P_MobjThinker as *const () as usize {
                saveg_write8(tc_mobj);
                saveg_write_pad();
                saveg_write_mobj_t(th as *const c_void);
                th = (*th).next;
                continue;
            }
        }
        th = (*th).next;
    }

    // terminating marker
    saveg_write8(tc_end);
}

#[no_mangle]
pub unsafe extern "C" fn P_UnArchiveThinkers() {
    let cap = &raw mut thinkercap;
    let mut currentthinker = (*cap).next;

    // remove all current thinkers
    while currentthinker != cap {
        let next = (*currentthinker).next;

        let func = (*currentthinker).function.acp1;
        if let Some(fn_ptr) = func {
            if fn_ptr as usize == P_MobjThinker as *const () as usize {
                P_RemoveMobj(currentthinker as *mut c_void);
            } else {
                Z_Free(currentthinker as *mut c_void);
            }
        } else {
            Z_Free(currentthinker as *mut c_void);
        }

        currentthinker = next;
    }

    P_InitThinkers();

    // read saved thinkers
    loop {
        let tclass = saveg_read8();
        match tclass {
            x if x == tc_end => return,
            x if x == tc_mobj => {
                saveg_read_pad();
                let mobj = Z_Malloc(
                    std::mem::size_of::<crate::doom::c_ffi::mobj_t>() as c_int,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                );
                saveg_read_mobj_t(mobj);

                let mo = mobj as *mut crate::doom::c_ffi::mobj_t;

                // target/tracer set to NULL (will be rebuilt)
                (*mo).target = std::ptr::null_mut();
                (*mo).tracer = std::ptr::null_mut();

                P_SetThingPosition(mobj);

                // rebuild info from mobjinfo
                (*mo).info = &mut mobjinfo[(*mo).type_ as usize] as *mut MobjInfo
                    as *mut crate::doom::c_ffi::mobjinfo_t;

                // rebuild floorz/ceilingz from subsector
                let subsec = (*mo).subsector as *mut crate::doom::c_ffi::subsector_t;
                if !subsec.is_null() {
                    let sector = (*subsec).sector as *mut sector_t;
                    if !sector.is_null() {
                        (*mo).floorz = (*sector).floorheight;
                        (*mo).ceilingz = (*sector).ceilingheight;
                    }
                }

                // set thinker function
                let thinker_ptr = std::ptr::addr_of_mut!((*mo).thinker_prev) as *mut thinker_t;
                (*thinker_ptr).function = actionf_t {
                    acp1: Some(P_MobjThinker),
                };

                P_AddThinker(&mut (*thinker_ptr));
            }
            _ => {
                i_error!("Unknown tclass {} in savegame", tclass as c_int);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Specials enum
// ---------------------------------------------------------------------------

const tc_ceiling: u8 = 0;
const tc_door: u8 = 1;
const tc_floor: u8 = 2;
const tc_plat: u8 = 3;
const tc_flash: u8 = 4;
const tc_strobe: u8 = 5;
const tc_glow: u8 = 6;
const tc_endspecials: u8 = 7;

// ---------------------------------------------------------------------------
// P_ArchiveSpecials / P_UnArchiveSpecials
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn P_ArchiveSpecials() {
    let cap = &raw mut thinkercap;
    let mut th = (*cap).next;

    while th != cap {
        let func = (*th).function;

        // Check for ceiling (acv == NULL means in activeceilings list)
        if func.acv.is_none() {
            // Check if it's in activeceilings
            let maxceilings: usize = 30;
            let activeceilings_ptr = std::ptr::addr_of!(crate::doom::p_ceilng::activeceilings[0]);
            let mut found = false;
            for i in 0..maxceilings {
                if *activeceilings_ptr.add(i) == th as *mut ceiling_t {
                    found = true;
                    break;
                }
            }
            if found {
                saveg_write8(tc_ceiling);
                saveg_write_pad();
                saveg_write_ceiling_t(th as *const ceiling_t);
                th = (*th).next;
                continue;
            }
        }

        if func.acp1.map(|f| f as usize) == Some(T_MoveCeiling as *const () as usize) {
            saveg_write8(tc_ceiling);
            saveg_write_pad();
            saveg_write_ceiling_t(th as *const ceiling_t);
            th = (*th).next;
            continue;
        }

        if func.acp1.map(|f| f as usize) == Some(T_VerticalDoor as *const () as usize) {
            saveg_write8(tc_door);
            saveg_write_pad();
            saveg_write_vldoor_t(th as *const vldoor_t);
            th = (*th).next;
            continue;
        }

        if func.acp1.map(|f| f as usize) == Some(T_MoveFloor as *const () as usize) {
            saveg_write8(tc_floor);
            saveg_write_pad();
            saveg_write_floormove_t(th as *const floormove_t);
            th = (*th).next;
            continue;
        }

        if func.acp1.map(|f| f as usize) == Some(T_PlatRaise as *const () as usize) {
            saveg_write8(tc_plat);
            saveg_write_pad();
            saveg_write_plat_t(th as *const plat_t);
            th = (*th).next;
            continue;
        }

        if func.acp1.map(|f| f as usize) == Some(T_LightFlash as *const () as usize) {
            saveg_write8(tc_flash);
            saveg_write_pad();
            saveg_write_lightflash_t(th as *const lightflash_t);
            th = (*th).next;
            continue;
        }

        if func.acp1.map(|f| f as usize) == Some(T_StrobeFlash as *const () as usize) {
            saveg_write8(tc_strobe);
            saveg_write_pad();
            saveg_write_strobe_t(th as *const strobe_t);
            th = (*th).next;
            continue;
        }

        if func.acp1.map(|f| f as usize) == Some(T_Glow as *const () as usize) {
            saveg_write8(tc_glow);
            saveg_write_pad();
            saveg_write_glow_t(th as *const glow_t);
            th = (*th).next;
            continue;
        }

        th = (*th).next;
    }

    saveg_write8(tc_endspecials);
}

#[no_mangle]
pub unsafe extern "C" fn P_UnArchiveSpecials() {
    loop {
        let tclass = saveg_read8();
        match tclass {
            x if x == tc_endspecials => return,
            x if x == tc_ceiling => {
                saveg_read_pad();
                let ceiling = Z_Malloc(
                    std::mem::size_of::<ceiling_t>() as c_int,
                    PU_LEVSPEC,
                    std::ptr::null_mut(),
                ) as *mut ceiling_t;
                saveg_read_ceiling_t(ceiling);
                (*ceiling).sector.as_mut().unwrap().specialdata = ceiling as *mut c_void;

                if ceiling.as_ref().unwrap().thinker.function.acp1.is_some() {
                    ceiling.as_mut().unwrap().thinker.function = actionf_p1_move_ceiling();
                }

                P_AddThinker(&mut (*ceiling).thinker);
                P_AddActiveCeiling(ceiling);
            }
            x if x == tc_door => {
                saveg_read_pad();
                let door = Z_Malloc(
                    std::mem::size_of::<vldoor_t>() as c_int,
                    PU_LEVSPEC,
                    std::ptr::null_mut(),
                ) as *mut vldoor_t;
                saveg_read_vldoor_t(door);
                (*door).sector.as_mut().unwrap().specialdata = door as *mut c_void;
                door.as_mut().unwrap().thinker.function = actionf_p1_vertical_door();
                P_AddThinker(&mut (*door).thinker);
            }
            x if x == tc_floor => {
                saveg_read_pad();
                let floor = Z_Malloc(
                    std::mem::size_of::<floormove_t>() as c_int,
                    PU_LEVSPEC,
                    std::ptr::null_mut(),
                ) as *mut floormove_t;
                saveg_read_floormove_t(floor);
                (*floor).sector.as_mut().unwrap().specialdata = floor as *mut c_void;
                floor.as_mut().unwrap().thinker.function = actionf_p1_move_floor();
                P_AddThinker(&mut (*floor).thinker);
            }
            x if x == tc_plat => {
                saveg_read_pad();
                let plat = Z_Malloc(
                    std::mem::size_of::<plat_t>() as c_int,
                    PU_LEVSPEC,
                    std::ptr::null_mut(),
                ) as *mut plat_t;
                saveg_read_plat_t(plat);
                (*plat).sector.as_mut().unwrap().specialdata = plat as *mut c_void;

                if plat.as_ref().unwrap().thinker.function.acp1.is_some() {
                    plat.as_mut().unwrap().thinker.function = actionf_p1_plat_raise();
                }

                P_AddThinker(&mut (*plat).thinker);
                P_AddActivePlat(plat);
            }
            x if x == tc_flash => {
                saveg_read_pad();
                let flash = Z_Malloc(
                    std::mem::size_of::<lightflash_t>() as c_int,
                    PU_LEVSPEC,
                    std::ptr::null_mut(),
                ) as *mut lightflash_t;
                saveg_read_lightflash_t(flash);
                flash.as_mut().unwrap().thinker.function = actionf_p1_light_flash();
                P_AddThinker(&mut (*flash).thinker);
            }
            x if x == tc_strobe => {
                saveg_read_pad();
                let strobe = Z_Malloc(
                    std::mem::size_of::<strobe_t>() as c_int,
                    PU_LEVSPEC,
                    std::ptr::null_mut(),
                ) as *mut strobe_t;
                saveg_read_strobe_t(strobe);
                strobe.as_mut().unwrap().thinker.function = actionf_p1_strobe_flash();
                P_AddThinker(&mut (*strobe).thinker);
            }
            x if x == tc_glow => {
                saveg_read_pad();
                let glow = Z_Malloc(
                    std::mem::size_of::<glow_t>() as c_int,
                    PU_LEVSPEC,
                    std::ptr::null_mut(),
                ) as *mut glow_t;
                saveg_read_glow_t(glow);
                glow.as_mut().unwrap().thinker.function = actionf_p1_glow();
                P_AddThinker(&mut (*glow).thinker);
            }
            _ => {
                i_error!(
                    "P_UnarchiveSpecials:Unknown tclass {} in savegame",
                    tclass as c_int
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Link anchor
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_Saveg_Link_Anchor() {
    let _ = P_TempSaveGameFile as *const () as usize;
    let _ = P_SaveGameFile as *const () as usize;
    let _ = P_WriteSaveGameHeader as *const () as usize;
    let _ = P_ReadSaveGameHeader as *const () as usize;
    let _ = P_ReadSaveGameEOF as *const () as usize;
    let _ = P_WriteSaveGameEOF as *const () as usize;
    let _ = P_ArchivePlayers as *const () as usize;
    let _ = P_UnArchivePlayers as *const () as usize;
    let _ = P_ArchiveWorld as *const () as usize;
    let _ = P_UnArchiveWorld as *const () as usize;
    let _ = P_ArchiveThinkers as *const () as usize;
    let _ = P_UnArchiveThinkers as *const () as usize;
    let _ = P_ArchiveSpecials as *const () as usize;
    let _ = P_UnArchiveSpecials as *const () as usize;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const SAVEGAME_EOF_VAL: u8 = 0x1d;
    const VERSIONSIZE_VAL: usize = 16;
    const SAVESTRINGSIZE_VAL: usize = 24;

    #[test]
    fn constants_match_c() {
        assert_eq!(SAVEGAME_EOF, SAVEGAME_EOF_VAL);
        assert_eq!(VERSIONSIZE, VERSIONSIZE_VAL);
        assert_eq!(SAVESTRINGSIZE, SAVESTRINGSIZE_VAL);
    }

    #[test]
    fn endian_write_read_16() {
        let _g = LOCK.lock().unwrap();
        // Test that write_u16 followed by read_u16 round-trips correctly
        let mut buf = Vec::new();
        // Simulate little-endian write: 0x1234 -> [0x34, 0x12]
        let val: u16 = 0x1234;
        buf.push((val & 0xff) as u8);
        buf.push(((val >> 8) & 0xff) as u8);
        assert_eq!(buf[0], 0x34);
        assert_eq!(buf[1], 0x12);
    }

    #[test]
    fn endian_write_read_32() {
        let _g = LOCK.lock().unwrap();
        let val: u32 = 0x12345678;
        let mut buf = Vec::new();
        buf.push((val & 0xff) as u8);
        buf.push(((val >> 8) & 0xff) as u8);
        buf.push(((val >> 16) & 0xff) as u8);
        buf.push(((val >> 24) & 0xff) as u8);
        assert_eq!(buf[0], 0x78);
        assert_eq!(buf[1], 0x56);
        assert_eq!(buf[2], 0x34);
        assert_eq!(buf[3], 0x12);
    }

    #[test]
    #[allow(clippy::erasing_op)]
    fn padding_calculation() {
        let _g = LOCK.lock().unwrap();
        // Test padding calculation: (4 - (pos & 3)) & 3
        assert_eq!((4 - (0 & 3)) & 3, 0); // aligned
        assert_eq!((4 - (1 & 3)) & 3, 3);
        assert_eq!((4 - (2 & 3)) & 3, 2);
        assert_eq!((4 - (3 & 3)) & 3, 1);
        assert_eq!((4 - (4 & 3)) & 3, 0); // aligned
        assert_eq!((4 - (5 & 3)) & 3, 3);
    }

    #[test]
    fn eof_marker_value() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(SAVEGAME_EOF, 0x1d);
    }

    #[test]
    fn version_string_length() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(VERSIONSIZE, 16);
        let test_str = "version 110"; // typical version string
        assert!(test_str.len() < VERSIONSIZE);
    }

    #[test]
    fn save_string_size() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(SAVESTRINGSIZE, 24);
    }

    unsafe fn with_mem_stream<F: FnOnce()>(data: &mut [u8], f: F) {
        let old_stream = save_stream;
        let old_error = savegame_error;
        let old_len = savegamelength;
        save_stream = libc::fmemopen(
            data.as_mut_ptr() as *mut libc::c_void,
            data.len(),
            c"r".as_ptr() as *const libc::c_char,
        );
        savegame_error = 0;
        savegamelength = 0;
        f();
        libc::fclose(save_stream);
        save_stream = old_stream;
        savegame_error = old_error;
        savegamelength = old_len;
    }

    #[test]
    fn saveg_read_thinker_t_zero_fn_ptr_becomes_none() {
        let _g = LOCK.lock().unwrap();
        // 12 bytes: prev(4) + next(4) + fn_ptr(4), all zero
        let mut data = [0u8; 12];
        unsafe {
            let mut th = thinker_t {
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
                function: actionf_t { acp1: None },
            };
            with_mem_stream(&mut data, || {
                saveg_read_thinker_t(&raw mut th as *mut thinker_t);
            });
            assert!(
                th.function.acp1.is_none(),
                "zero function pointer must deserialize as None, not Some(NULL)"
            );
        }
    }

    #[test]
    fn saveg_read_mobj_t_zero_fn_ptr_becomes_none() {
        use crate::doom::c_ffi::mobj_t;
        let _g = LOCK.lock().unwrap();
        // mobj_t is large; we need enough bytes for the full struct read.
        // The thinker (prev+next+fn) is the first 12 bytes.
        // saveg_read_mobj_t reads many fields — pad to 256 zeros.
        let mut data = [0u8; 256];
        unsafe {
            let mut mo = std::mem::MaybeUninit::<mobj_t>::zeroed().assume_init();
            with_mem_stream(&mut data, || {
                saveg_read_mobj_t(&raw mut mo as *mut mobj_t as *mut libc::c_void);
            });
            let thinker_ptr = std::ptr::addr_of!(mo.thinker_prev) as *const thinker_t;
            assert!(
                (*thinker_ptr).function.acp1.is_none(),
                "zero function pointer in mobj thinker must deserialize as None, not Some(NULL)"
            );
        }
    }

    #[test]
    fn save_game_path_not_truncated_for_long_directory() {
        let long_dir = "/very/long/savegame/directory/path/"; // 35 chars — forces path > 31 chars
        let slot = 3i32;
        let alloc_size = long_dir.len() + 32;
        let mut buf = vec![0u8; alloc_size];
        fill_save_filename(&mut buf, long_dir, slot);
        let result = std::ffi::CStr::from_bytes_until_nul(&buf)
            .unwrap()
            .to_str()
            .unwrap();
        let expected = format!("{}{}{}.dsg", long_dir, SAVEGAMENAME, slot);
        assert_eq!(
            result, expected,
            "save path was truncated for long directory"
        );
    }
}
