//! Mirror of `d_player.h` types for FFI with remaining C code.
//!
//! Provides `PlayerT`, `TiccmdT`, `PspdefT` and the C globals `players` / `consoleplayer`.
//! Also implements `M_Menu_SetPlayerMessage` (previously in `m_menu_shim.c`).

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_char;
use std::os::raw::c_int;

pub const NUMPOWERS: usize = 6;
pub const NUMCARDS: usize = 6;
pub const NUMWEAPONS: usize = 9;
pub const NUMAMMO: usize = 4;
pub const NUMPSPRITES: usize = 2;
pub const MAXPLAYERS: usize = 4;

// Cheat flags (d_player.h)
pub const CF_NOCLIP: c_int = 1;
pub const CF_GODMODE: c_int = 2;
pub const CF_NOMOMENTUM: c_int = 4;

pub enum mobj_t {}
pub enum state_t {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TiccmdT {
    pub forwardmove: i8,
    pub sidemove: i8,
    pub angleturn: i16,
    pub chatchar: u8,
    pub buttons: u8,
    pub consistancy: u8,
    pub buttons2: u8,
    pub inventory: c_int,
    pub lookfly: u8,
    pub arti: u8,
    _pad: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PspdefT {
    pub state: *mut state_t,
    pub tics: c_int,
    pub sx: c_int,
    pub sy: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PlayerT {
    pub mo: *mut mobj_t,
    pub playerstate: c_int,
    pub cmd: TiccmdT,
    pub viewz: c_int,
    pub viewheight: c_int,
    pub deltaviewheight: c_int,
    pub bob: c_int,
    pub health: c_int,
    pub armorpoints: c_int,
    pub armortype: c_int,
    pub powers: [c_int; NUMPOWERS],
    pub cards: [c_int; NUMCARDS],
    pub backpack: c_int,
    pub frags: [c_int; MAXPLAYERS],
    pub readyweapon: c_int,
    pub pendingweapon: c_int,
    pub weaponowned: [c_int; NUMWEAPONS],
    pub ammo: [c_int; NUMAMMO],
    pub maxammo: [c_int; NUMAMMO],
    pub attackdown: c_int,
    pub usedown: c_int,
    pub cheats: c_int,
    pub refire: c_int,
    pub killcount: c_int,
    pub itemcount: c_int,
    pub secretcount: c_int,
    pub message: *mut c_char,
    pub damagecount: c_int,
    pub bonuscount: c_int,
    pub attacker: *mut mobj_t,
    pub extralight: c_int,
    pub fixedcolormap: c_int,
    pub colormap: c_int,
    pub psprites: [PspdefT; NUMPSPRITES],
    pub didsecret: c_int,
}

extern "C" {
    pub static mut players: [PlayerT; MAXPLAYERS];
    pub static mut consoleplayer: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_SetPlayerMessage(msg: *const c_char) {
    (*players.as_mut_ptr().offset(consoleplayer as isize)).message = msg as *mut c_char;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected layout of `player_t` as emitted by layout_probe.c
    /// on x86_64 Linux.  Kept in-source so tests don't depend on the
    /// linker pulling symbols out of a static archive.
    const PLAYER_T_SIZEOF: usize = 328;
    const PLAYER_T_MESSAGE_OFFSET: usize = 232;

    #[test]
    fn player_t_size_matches_c() {
        assert_eq!(
            std::mem::size_of::<PlayerT>(),
            PLAYER_T_SIZEOF,
            "PlayerT size mismatch: Rust={}, expected={}",
            std::mem::size_of::<PlayerT>(),
            PLAYER_T_SIZEOF
        );
    }

    #[test]
    fn player_t_message_offset_matches_c() {
        assert_eq!(
            std::mem::offset_of!(PlayerT, message),
            PLAYER_T_MESSAGE_OFFSET,
            "PlayerT.message offset mismatch: Rust={}, expected={}",
            std::mem::offset_of!(PlayerT, message),
            PLAYER_T_MESSAGE_OFFSET
        );
    }
}
