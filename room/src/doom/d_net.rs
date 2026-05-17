//! Rust port of vendor/doomgeneric/d_net.c.
//!
//! Network game communication stubs.  When `FEATURE_MULTIPLAYER` is not
//! defined (our case), this module provides the single-player game loop
//! integration and netgame initialisation entry points.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_char;
use std::os::raw::c_int;

use crate::types::Boolean;

use crate::doom::d_player::{consoleplayer, players, TiccmdT, MAXPLAYERS};

const NET_MAXPLAYERS: usize = 8;
const SHA1_DIGEST_SIZE: usize = 20;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetConnectDataT {
    pub gamemode: c_int,
    pub gamemission: c_int,
    pub lowres_turn: c_int,
    pub drone: c_int,
    pub max_players: c_int,
    pub is_freedoom: c_int,
    pub wad_sha1sum: [u8; SHA1_DIGEST_SIZE],
    pub deh_sha1sum: [u8; SHA1_DIGEST_SIZE],
    pub player_class: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetGameSettingsT {
    pub ticdup: c_int,
    pub extratics: c_int,
    pub deathmatch: c_int,
    pub episode: c_int,
    pub nomonsters: c_int,
    pub fast_monsters: c_int,
    pub respawn_monsters: c_int,
    pub map: c_int,
    pub skill: c_int,
    pub gameversion: c_int,
    pub lowres_turn: c_int,
    pub new_sync: c_int,
    pub timelimit: c_int,
    pub loadgame: c_int,
    pub random: c_int,
    pub num_players: c_int,
    pub consoleplayer: c_int,
    pub player_classes: [c_int; NET_MAXPLAYERS],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LoopInterfaceT {
    pub ProcessEvents: Option<unsafe extern "C" fn()>,
    pub BuildTiccmd: Option<unsafe extern "C" fn(*mut TiccmdT, c_int)>,
    pub RunTic: Option<unsafe extern "C" fn(*mut TiccmdT, *mut c_int)>,
    pub RunMenu: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static mut netcmds: *mut TiccmdT = std::ptr::null_mut();

extern "C" {
    fn M_StringCopy(dst: *mut c_char, src: *const c_char, dst_size: usize) -> Boolean;
    fn M_CheckParm(parm: *const c_char) -> c_int;
    fn G_CheckDemoStatus() -> c_int;
    fn G_Ticker();
    fn D_DoAdvanceDemo();
    fn D_ProcessEvents();
    fn G_BuildTiccmd(cmd: *mut TiccmdT, maketic: c_int);
    fn M_Ticker();
    fn D_RegisterLoopCallbacks(i: *mut LoopInterfaceT);
    fn D_InitNetGame(connect_data: *mut NetConnectDataT) -> c_int;
    fn D_StartNetGame(settings: *mut NetGameSettingsT, callback: *const ());
    fn W_Checksum(digest: *mut u8);
    fn W_CheckNumForName(name: *const c_char) -> c_int;

    static mut deathmatch: c_int;
    static mut startepisode: c_int;
    static mut startmap: c_int;
    static mut startskill: c_int;
    static mut startloadgame: c_int;
    static mut lowres_turn: c_int;
    static mut nomonsters: c_int;
    static mut fastparm: c_int;
    static mut respawnparm: c_int;
    static mut timelimit: c_int;
    static mut gamemode: c_int;
    static mut gamemission: c_int;
    static mut gameversion: c_int;
    static mut viewangleoffset: c_int;
    static mut autostart: c_int;
    static mut netgame: c_int;
    static mut demorecording: c_int;
    static mut playeringame: [c_int; MAXPLAYERS];
}

static EXIT_MSG: [u8; 23] = *b"Player 1 left the game\0";

unsafe fn PlayerQuitGame(player_idx: usize) {
    static mut EXITMSG: [c_char; 80] = [0; 80];

    core::ptr::copy_nonoverlapping(
        EXIT_MSG.as_ptr() as *const c_char,
        std::ptr::addr_of_mut!(EXITMSG[0]),
        EXIT_MSG.len(),
    );

    EXITMSG[7] += player_idx as c_char;

    playeringame[player_idx] = 0;
    (*std::ptr::addr_of_mut!(players[0]).offset(consoleplayer as isize)).message =
        std::ptr::addr_of_mut!(EXITMSG[0]);

    if demorecording != 0 {
        G_CheckDemoStatus();
    }
}

unsafe extern "C" fn RunTic(cmds: *mut TiccmdT, ingame: *mut c_int) {
    for i in 0..MAXPLAYERS {
        if demorecording == 0 && playeringame[i] != 0 && *ingame.add(i) == 0 {
            PlayerQuitGame(i);
        }
    }

    netcmds = cmds;

    extern "C" {
        static mut advancedemo: c_int;
    }
    if advancedemo != 0 {
        D_DoAdvanceDemo();
    }

    G_Ticker();
}

unsafe fn LoadGameSettings(settings: *mut NetGameSettingsT) {
    deathmatch = (*settings).deathmatch;
    startepisode = (*settings).episode;
    startmap = (*settings).map;
    startskill = (*settings).skill;
    startloadgame = (*settings).loadgame;
    lowres_turn = (*settings).lowres_turn;
    nomonsters = (*settings).nomonsters;
    fastparm = (*settings).fast_monsters;
    respawnparm = (*settings).respawn_monsters;
    timelimit = (*settings).timelimit;
    consoleplayer = (*settings).consoleplayer;

    for i in 0..MAXPLAYERS {
        playeringame[i] = if i < (*settings).num_players as usize {
            1
        } else {
            0
        };
    }
}

unsafe fn SaveGameSettings(settings: *mut NetGameSettingsT) {
    (*settings).deathmatch = deathmatch;
    (*settings).episode = startepisode;
    (*settings).map = startmap;
    (*settings).skill = startskill;
    (*settings).loadgame = startloadgame;
    (*settings).gameversion = gameversion;
    (*settings).nomonsters = nomonsters;
    (*settings).fast_monsters = fastparm;
    (*settings).respawn_monsters = respawnparm;
    (*settings).timelimit = timelimit;
    (*settings).lowres_turn =
        if M_CheckParm(c"-record".as_ptr()) > 0 && M_CheckParm(c"-longtics".as_ptr()) == 0 {
            1
        } else {
            0
        };
}

unsafe fn InitConnectData(connect_data: *mut NetConnectDataT) {
    (*connect_data).max_players = MAXPLAYERS as c_int;
    (*connect_data).drone = 0;

    if M_CheckParm(c"-left".as_ptr()) > 0 {
        viewangleoffset = 0x40000000u32 as c_int;
        (*connect_data).drone = 1;
    }

    if M_CheckParm(c"-right".as_ptr()) > 0 {
        viewangleoffset = 0xC0000000u32 as c_int;
        (*connect_data).drone = 1;
    }

    (*connect_data).gamemode = gamemode;
    (*connect_data).gamemission = gamemission;

    (*connect_data).lowres_turn =
        if M_CheckParm(c"-record".as_ptr()) > 0 && M_CheckParm(c"-longtics".as_ptr()) == 0 {
            1
        } else {
            0
        };

    W_Checksum((*connect_data).wad_sha1sum.as_mut_ptr());

    let name = b"FREEDOOM\0";
    (*connect_data).is_freedoom = if W_CheckNumForName(name.as_ptr() as *const c_char) >= 0 {
        1
    } else {
        0
    };
}

static mut DOOM_LOOP_INTERFACE: LoopInterfaceT = LoopInterfaceT {
    ProcessEvents: Some(D_ProcessEvents),
    BuildTiccmd: Some(G_BuildTiccmd),
    RunTic: Some(RunTic),
    RunMenu: Some(M_Ticker),
};

#[no_mangle]
pub extern "C" fn D_ConnectNetGame() {
    unsafe {
        let mut connect_data: NetConnectDataT = std::mem::zeroed();
        InitConnectData(&mut connect_data);
        netgame = D_InitNetGame(&mut connect_data);

        if M_CheckParm(c"-solo-net".as_ptr()) > 0 {
            netgame = 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn D_CheckNetGame() {
    unsafe {
        if netgame != 0 {
            autostart = 1;
        }

        D_RegisterLoopCallbacks(&raw mut DOOM_LOOP_INTERFACE);

        let mut settings: NetGameSettingsT = std::mem::zeroed();
        SaveGameSettings(&mut settings);
        D_StartNetGame(&mut settings, std::ptr::null());
        LoadGameSettings(&mut settings);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const NET_CONNECT_DATA_T_SIZEOF: usize = 68;
    const NET_GAMESETTINGS_T_SIZEOF: usize = 100;
    const LOOP_INTERFACE_T_SIZEOF: usize = 32;

    #[test]
    fn net_connect_data_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<NetConnectDataT>(),
            NET_CONNECT_DATA_T_SIZEOF,
            "NetConnectDataT size mismatch: Rust={}, expected={}",
            std::mem::size_of::<NetConnectDataT>(),
            NET_CONNECT_DATA_T_SIZEOF,
        );
    }

    #[test]
    fn net_gamesettings_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<NetGameSettingsT>(),
            NET_GAMESETTINGS_T_SIZEOF,
            "NetGameSettingsT size mismatch: Rust={}, expected={}",
            std::mem::size_of::<NetGameSettingsT>(),
            NET_GAMESETTINGS_T_SIZEOF,
        );
    }

    #[test]
    fn loop_interface_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<LoopInterfaceT>(),
            LOOP_INTERFACE_T_SIZEOF,
            "LoopInterfaceT size mismatch: Rust={}, expected={}",
            std::mem::size_of::<LoopInterfaceT>(),
            LOOP_INTERFACE_T_SIZEOF,
        );
    }

    #[test]
    fn netcmds_defaults_to_null() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert!(netcmds.is_null());
        }
    }
}
