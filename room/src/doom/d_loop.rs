//! Rust port of vendor/doomgeneric/d_loop.c.
//!
//! Main game loop timing and ticcmd management.
//! When `FEATURE_MULTIPLAYER` is not defined (our case), the networking
//! code is compiled out and this module only drives single-player timing.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_char;
use std::os::raw::c_int;

use crate::doom::d_net::{LoopInterfaceT, NetConnectDataT, NetGameSettingsT};
use crate::doom::d_player::TiccmdT;
use crate::doom::i_timer::TICRATE;
use crate::doom::m_fixed::FRACUNIT;

const NET_MAXPLAYERS: usize = 8;
pub const BACKUPTICS: usize = 128;

#[repr(C)]
struct TiccmdSetT {
    cmds: [TiccmdT; NET_MAXPLAYERS],
    ingame: [c_int; NET_MAXPLAYERS], // boolean
}

static mut TICDATA: [TiccmdSetT; BACKUPTICS] = unsafe { std::mem::zeroed() };

static mut MAKETIC: c_int = 0;
static mut RECVTIC: c_int = 0;

#[no_mangle]
pub static mut gametic: c_int = 0;

#[no_mangle]
pub static mut singletics: c_int = 0; // boolean

static mut LOCALPLAYER: c_int = 0;
static mut SKIPTICS: c_int = 0;

#[no_mangle]
pub static mut ticdup: c_int = 0;

#[no_mangle]
pub static mut offsetms: c_int = 0; // fixed_t

static mut NEW_SYNC: c_int = 0; // boolean

static mut LOOP_INTERFACE: *mut LoopInterfaceT = std::ptr::null_mut();

static mut LOCAL_PLAYERINGAME: [c_int; NET_MAXPLAYERS] = [0; NET_MAXPLAYERS];

static mut PLAYER_CLASS: c_int = 0;

static mut LASTTIME: c_int = 0;

static mut FRAMEON: c_int = 0;
static mut FRAMESKIP: [c_int; 4] = [0; 4];
static mut OLDNETTICS: c_int = 0;

extern "C" {
    static mut drone: c_int; // boolean
    static mut net_client_connected: c_int; // boolean

    fn I_GetTimeMS() -> c_int;
    fn I_GetTime() -> c_int;
    fn I_StartTic();
    fn I_Sleep(ms: c_int);
    fn I_Error(msg: *const c_char);
    fn I_AtExit(func: extern "C" fn(), run_on_error: c_int);

    fn D_ProcessEvents();
    fn G_BuildTiccmd(cmd: *mut TiccmdT, maketic: c_int);
    fn M_Ticker();
}

unsafe fn get_adjusted_time() -> c_int {
    let mut time_ms = I_GetTimeMS();
    if NEW_SYNC != 0 {
        time_ms += offsetms / FRACUNIT;
    }
    (time_ms * TICRATE) / 1000
}

unsafe fn build_new_tic() -> bool {
    let gameticdiv = gametic / ticdup;

    I_StartTic();

    let iface = &*LOOP_INTERFACE;
    iface.ProcessEvents.unwrap()();

    // Always run the menu
    iface.RunMenu.unwrap()();

    if drone != 0 {
        return false;
    }

    if NEW_SYNC != 0 {
        if net_client_connected == 0 && MAKETIC - gameticdiv > 2 {
            return false;
        }
        if MAKETIC - gameticdiv > 8 {
            return false;
        }
    } else {
        if MAKETIC - gameticdiv >= 5 {
            return false;
        }
    }

    let mut cmd = std::mem::zeroed::<TiccmdT>();
    iface.BuildTiccmd.unwrap()(&mut cmd, MAKETIC);

    let slot = (MAKETIC as usize) % BACKUPTICS;
    TICDATA[slot].cmds[LOCALPLAYER as usize] = cmd;
    TICDATA[slot].ingame[LOCALPLAYER as usize] = 1;

    MAKETIC += 1;

    true
}

#[no_mangle]
pub extern "C" fn NetUpdate() {
    unsafe {
        if singletics != 0 {
            return;
        }

        let nowtime = get_adjusted_time() / ticdup;
        let mut newtics = nowtime - LASTTIME;
        LASTTIME = nowtime;

        if SKIPTICS <= newtics {
            newtics -= SKIPTICS;
            SKIPTICS = 0;
        } else {
            SKIPTICS -= newtics;
            newtics = 0;
        }

        for _ in 0..newtics {
            if !build_new_tic() {
                break;
            }
        }
    }
}

unsafe fn d_disconnected() {
    if drone != 0 {
        I_Error(b"Disconnected from server in drone mode.\0".as_ptr() as *const c_char);
    }
}

#[no_mangle]
pub extern "C" fn D_ReceiveTic(ticcmds: *mut TiccmdT, players_mask: *mut c_int) {
    unsafe {
        if ticcmds.is_null() && players_mask.is_null() {
            d_disconnected();
            return;
        }

        for i in 0..NET_MAXPLAYERS {
            if drone == 0 && i == LOCALPLAYER as usize {
                // This is us.  Don't overwrite it.
            } else {
                TICDATA[(RECVTIC as usize) % BACKUPTICS].cmds[i] = *ticcmds.add(i);
                TICDATA[(RECVTIC as usize) % BACKUPTICS].ingame[i] = *players_mask.add(i);
            }
        }

        RECVTIC += 1;
    }
}

#[no_mangle]
pub extern "C" fn D_StartGameLoop() {
    unsafe {
        LASTTIME = get_adjusted_time() / ticdup;
    }
}

#[no_mangle]
pub extern "C" fn D_StartNetGame(settings: *mut NetGameSettingsT, _callback: *const ()) {
    unsafe {
        (*settings).consoleplayer = 0;
        (*settings).num_players = 1;
        (*settings).player_classes[0] = PLAYER_CLASS;
        (*settings).new_sync = 0;
        (*settings).extratics = 1;
        (*settings).ticdup = 1;

        ticdup = (*settings).ticdup;
        NEW_SYNC = (*settings).new_sync;
    }
}

#[no_mangle]
pub extern "C" fn D_InitNetGame(connect_data: *mut NetConnectDataT) -> c_int {
    unsafe {
        I_AtExit(D_QuitNetGame, 1);
        PLAYER_CLASS = (*connect_data).player_class;
    }
    0 // false
}

#[no_mangle]
pub extern "C" fn D_QuitNetGame() {
    // No-op when FEATURE_MULTIPLAYER is not defined.
}

unsafe fn get_low_tic() -> c_int {
    let mut lowtic = MAKETIC;
    if net_client_connected != 0 {
        if drone != 0 || RECVTIC < lowtic {
            lowtic = RECVTIC;
        }
    }
    lowtic
}

unsafe fn old_net_sync() {
    FRAMEON += 1;

    let mut keyplayer = -1;
    for i in 0..NET_MAXPLAYERS {
        if LOCAL_PLAYERINGAME[i] != 0 {
            keyplayer = i as c_int;
            break;
        }
    }

    if keyplayer < 0 {
        return;
    }

    if LOCALPLAYER == keyplayer {
        // the key player does not adapt
    } else {
        if MAKETIC <= RECVTIC {
            LASTTIME -= 1;
        }

        FRAMESKIP[(FRAMEON & 3) as usize] = (OLDNETTICS > RECVTIC) as c_int;
        OLDNETTICS = MAKETIC;

        if FRAMESKIP[0] != 0 && FRAMESKIP[1] != 0 && FRAMESKIP[2] != 0 && FRAMESKIP[3] != 0 {
            SKIPTICS = 1;
        }
    }
}

unsafe fn players_in_game() -> bool {
    let mut result = false;
    if net_client_connected != 0 {
        for i in 0..NET_MAXPLAYERS {
            result = result || LOCAL_PLAYERINGAME[i] != 0;
        }
    }
    if drone == 0 {
        result = true;
    }
    result
}

unsafe fn ticdup_squash(set: *mut TiccmdSetT) {
    const BT_SPECIAL: u8 = 128;
    for i in 0..NET_MAXPLAYERS {
        let cmd = &mut (*set).cmds[i];
        cmd.chatchar = 0;
        if cmd.buttons & BT_SPECIAL != 0 {
            cmd.buttons = 0;
        }
    }
}

unsafe fn single_player_clear(set: *mut TiccmdSetT) {
    for i in 0..NET_MAXPLAYERS {
        if i != LOCALPLAYER as usize {
            (*set).ingame[i] = 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn TryRunTics() {
    unsafe {
        let entertic = I_GetTime() / ticdup;
        static mut OLDENTERTICS: c_int = 0;
        let realtics = entertic - OLDENTERTICS;
        OLDENTERTICS = entertic;

        if singletics != 0 {
            build_new_tic();
        } else {
            NetUpdate();
        }

        let mut lowtic = get_low_tic();
        let availabletics = lowtic - gametic / ticdup;

        let counts: c_int;
        if NEW_SYNC != 0 {
            counts = availabletics;
        } else {
            let mut c: c_int;
            if realtics < availabletics - 1 {
                c = realtics + 1;
            } else if realtics < availabletics {
                c = realtics;
            } else {
                c = availabletics;
            }

            if c < 1 {
                c = 1;
            }

            if net_client_connected != 0 {
                old_net_sync();
            }

            counts = c;
        }

        let mut counts = if counts < 1 { 1 } else { counts };

        while !players_in_game() || lowtic < gametic / ticdup + counts {
            NetUpdate();
            lowtic = get_low_tic();

            if lowtic < gametic / ticdup {
                I_Error(b"TryRunTics: lowtic < gametic\0".as_ptr() as *const c_char);
            }

            if I_GetTime() / ticdup - entertic > 0 {
                return;
            }

            I_Sleep(1);
        }

        while counts > 0 {
            counts -= 1;

            if !players_in_game() {
                return;
            }

            let set = &mut TICDATA[((gametic / ticdup) as usize) % BACKUPTICS] as *mut TiccmdSetT;

            if net_client_connected == 0 {
                single_player_clear(set);
            }

            for _ in 0..ticdup {
                if gametic / ticdup > lowtic {
                    I_Error(b"gametic>lowtic\0".as_ptr() as *const c_char);
                }

                for i in 0..NET_MAXPLAYERS {
                    LOCAL_PLAYERINGAME[i] = (*set).ingame[i];
                }

                let iface = &*LOOP_INTERFACE;
                iface.RunTic.unwrap()((*set).cmds.as_mut_ptr(), (*set).ingame.as_mut_ptr());
                gametic += 1;

                ticdup_squash(set);
            }

            NetUpdate();
        }
    }
}

#[no_mangle]
pub extern "C" fn D_RegisterLoopCallbacks(i: *mut LoopInterfaceT) {
    unsafe {
        LOOP_INTERFACE = i;
    }
}
