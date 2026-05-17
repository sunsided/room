//! Rust port of vendor/doomgeneric/p_tick.c.
//!
//! Thinker management and game tick loop.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_void;
use std::os::raw::c_int;

use crate::doom::d_player::{consoleplayer, players, MAXPLAYERS};
use crate::doom::g_game::{demoplayback, netgame, paused, playeringame};
use crate::doom::m_menu::menuactive;
use crate::doom::p_mobj::P_RespawnSpecials;
use crate::doom::p_spec::P_UpdateSpecials;
use crate::doom::p_user::P_PlayerThink;
use crate::doom::z_zone::Z_Free;

#[repr(C)]
#[derive(Clone, Copy)]
pub union actionf_t {
    pub acv: Option<unsafe extern "C" fn()>,
    pub acp1: Option<unsafe extern "C" fn(*mut c_void)>,
    pub acp2: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct thinker_t {
    pub prev: *mut thinker_t,
    pub next: *mut thinker_t,
    pub function: actionf_t,
}

/// Produce the sentinel value `(actionf_v)(-1)` that C uses to mark
/// thinkers for removal.  We build it at runtime so the compiler doesn't
/// try to validate the bit pattern as a real function pointer.
fn sentinel_ac() -> Option<unsafe extern "C" fn()> {
    // -1 as pointer-sized integer, reinterpreted as a function pointer.
    Some(unsafe { core::mem::transmute::<usize, unsafe extern "C" fn()>(usize::MAX) })
}

fn is_sentinel(f: actionf_t) -> bool {
    unsafe { f.acv == sentinel_ac() }
}

#[no_mangle]
pub static mut leveltime: c_int = 0;

#[no_mangle]
pub static mut thinkercap: thinker_t = thinker_t {
    prev: std::ptr::null_mut::<thinker_t>(),
    next: std::ptr::null_mut::<thinker_t>(),
    function: actionf_t { acv: None },
};

#[no_mangle]
pub extern "C" fn P_InitThinkers() {
    unsafe {
        thinkercap.prev = &raw mut thinkercap;
        thinkercap.next = &raw mut thinkercap;
    }
}

#[no_mangle]
pub extern "C" fn P_AddThinker(thinker: *mut thinker_t) {
    unsafe {
        let cap = &raw mut thinkercap;
        (*(*cap).prev).next = thinker;
        (*thinker).next = cap;
        (*thinker).prev = (*cap).prev;
        (*cap).prev = thinker;
    }
}

#[no_mangle]
pub extern "C" fn P_RemoveThinker(thinker: *mut thinker_t) {
    unsafe {
        (*thinker).function.acv = sentinel_ac();
    }
}

#[no_mangle]
pub extern "C" fn P_AllocateThinker(_thinker: *mut thinker_t) {}

#[no_mangle]
pub extern "C" fn P_RunThinkers() {
    unsafe {
        let cap = &raw mut thinkercap;
        let mut current = (*cap).next;

        while current != cap {
            if is_sentinel((*current).function) {
                let prev = (*current).prev;
                let next = (*current).next;
                (*prev).next = next;
                (*next).prev = prev;
                Z_Free(current as *mut c_void);
                current = next;
            } else {
                if let Some(fn_ptr) = (*current).function.acp1 {
                    fn_ptr(current as *mut c_void);
                }
                current = (*current).next;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn P_Ticker() {
    unsafe {
        if paused != 0 {
            return;
        }

        if netgame == 0
            && menuactive != 0
            && demoplayback == 0
            && (*std::ptr::addr_of!(players[0]).offset(consoleplayer as isize)).viewz != 1
        {
            return;
        }

        for i in 0..MAXPLAYERS {
            if playeringame[i] != 0 {
                P_PlayerThink(&mut players[i]);
            }
        }

        P_RunThinkers();
        P_UpdateSpecials();
        P_RespawnSpecials();

        leveltime += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const THINKER_T_SIZEOF: usize = 24;

    #[test]
    fn thinker_t_size_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(
            std::mem::size_of::<thinker_t>(),
            THINKER_T_SIZEOF,
            "thinker_t size mismatch: Rust={}, expected={}",
            std::mem::size_of::<thinker_t>(),
            THINKER_T_SIZEOF,
        );
    }

    #[test]
    fn globals_default() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(leveltime, 0);
        }
    }

    #[test]
    fn sentinel_is_all_ones() {
        let s = sentinel_ac();
        assert_eq!(usize::MAX, s.map(|f| f as usize).unwrap_or(0));
    }

    #[test]
    fn init_sets_self_pointers() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            thinkercap.prev = 0 as *mut thinker_t;
            thinkercap.next = 0 as *mut thinker_t;
            P_InitThinkers();
            assert_eq!(thinkercap.prev, &raw mut thinkercap as *mut thinker_t);
            assert_eq!(thinkercap.next, &raw mut thinkercap as *mut thinker_t);
        }
    }
}
