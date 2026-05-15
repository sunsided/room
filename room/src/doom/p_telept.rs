//! Rust port of vendor/doomgeneric/p_telept.c.
//!
//! Teleportation action special.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::c_void;
use std::os::raw::c_int;

use crate::doom::d_player::PlayerT;
use crate::doom::info::*;
use crate::doom::p_tick::thinker_t;
use crate::doom::tables::ANGLETOFINESHIFT;
const SFX_TELEPT: c_int = 35; // sfx_telept enum value

#[repr(C)]
#[derive(Clone, Copy)]
pub struct subsector_t {
    pub sector: *mut sector_t,
    pub numlines: i16,
    pub firstline: i16,
}
// Re-export the same layout as p_lights.rs so pointer casts work.

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sector_t {
    pub floorheight: c_int,
    pub ceilingheight: c_int,
    pub floorpic: i16,
    pub ceilingpic: i16,
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
    _pad0: [u8; 2],
    pub soundtraversed: c_int,
    pub soundtarget: *mut c_void,
    pub blockbox: [c_int; 4],
    pub soundorg: [u8; 40],
    pub validcount: c_int,
    pub thinglist: *mut c_void,
    pub specialdata: *mut c_void,
    pub linecount: c_int,
    pub lines: *mut *mut c_void,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct mapthing_t {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub r#type: i16,
    pub options: i16,
}

pub enum player_s {}
pub enum state_t {}
pub enum mobjinfo_t {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mobj_t {
    pub thinker: thinker_t,
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    _pad0: [u8; 4],
    pub snext: *mut mobj_t,
    pub sprev: *mut mobj_t,
    pub angle: u32,
    pub sprite: c_int,
    pub frame: c_int,
    _pad1: [u8; 4],
    pub bnext: *mut mobj_t,
    pub bprev: *mut mobj_t,
    pub subsector: *mut subsector_t,
    pub floorz: c_int,
    pub ceilingz: c_int,
    pub radius: c_int,
    pub height: c_int,
    pub momx: c_int,
    pub momy: c_int,
    pub momz: c_int,
    pub validcount: c_int,
    pub mobjtype: c_int,
    _pad2: [u8; 4],
    pub info: *mut mobjinfo_t,
    pub tics: c_int,
    _pad3: [u8; 4],
    pub state: *mut state_t,
    pub flags: c_int,
    pub health: c_int,
    pub movedir: c_int,
    pub movecount: c_int,
    pub target: *mut mobj_t,
    pub reactiontime: c_int,
    pub threshold: c_int,
    pub player: *mut player_s,
    pub lastlook: c_int,
    pub spawnpoint: mapthing_t,
    _pad4: [u8; 2],
    pub tracer: *mut mobj_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct line_t {
    pub v1: *mut c_void,
    pub v2: *mut c_void,
    pub dx: c_int,
    pub dy: c_int,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
    pub bbox: [c_int; 4],
    pub slopetype: c_int,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub validcount: c_int,
    pub specialdata: *mut c_void,
}

const EXE_FINAL: c_int = 7;

extern "C" {
    fn P_TeleportMove(thing: *mut mobj_t, x: c_int, y: c_int) -> c_int;
    fn P_SpawnMobj(x: c_int, y: c_int, z: c_int, mobjtype: c_int) -> *mut mobj_t;
    fn S_StartSound(mobj: *mut mobj_t, sfx_id: c_int);
    static mut thinkercap: thinker_t;
    static mut numsectors: c_int;
    static mut sectors: *mut sector_t;
    static mut gameversion: c_int;
    static mut finecosine: [c_int; 4096];
    static mut finesine: [c_int; 4096];
}
// We already have PlayerT in d_player.rs but can't use it here
// because the C player_s is different. Use pointer casts.

#[no_mangle]
pub extern "C" fn EV_Teleport(line: *mut line_t, side: c_int, thing: *mut mobj_t) -> c_int {
    unsafe {
        // Don't teleport missiles
        if (*thing).flags & MF_MISSILE != 0 {
            return 0;
        }

        // Don't teleport if hit back of line
        if side == 1 {
            return 0;
        }

        let tag = (*line).tag;

        for i in 0..numsectors as usize {
            if (*sectors.add(i)).tag != tag {
                continue;
            }

            let mut thinker = thinkercap.next;
            while !std::ptr::eq(thinker, &mut thinkercap) {
                // Not a mobj
                if (*thinker).function.acp1
                    != Some(core::mem::transmute::<
                        unsafe extern "C" fn(*mut mobj_t),
                        unsafe extern "C" fn(*mut c_void),
                    >(P_MobjThinker))
                {
                    thinker = (*thinker).next;
                    continue;
                }

                let m = thinker as *mut mobj_t;

                // Not a teleportman
                if (*m).mobjtype != MT_TELEPORTMAN {
                    thinker = (*thinker).next;
                    continue;
                }

                let sector = (*(*m).subsector).sector;
                // Wrong sector
                if sector.offset_from(sectors) != i as isize {
                    thinker = (*thinker).next;
                    continue;
                }

                let oldx = (*thing).x;
                let oldy = (*thing).y;
                let oldz = (*thing).z;

                if P_TeleportMove(thing, (*m).x, (*m).y) == 0 {
                    return 0;
                }

                // Final Doom quirk: don't set z
                if gameversion != EXE_FINAL {
                    (*thing).z = (*thing).floorz;
                }

                if !(*thing).player.is_null() {
                    let player = (*thing).player as *mut PlayerT;
                    (*player).viewz = (*thing).z + (*player).viewheight;
                }

                // Spawn teleport fog at source
                let fog = P_SpawnMobj(oldx, oldy, oldz, MT_TFOG);
                S_StartSound(fog, SFX_TELEPT);

                // Spawn teleport fog at destination
                let an = ((*m).angle >> ANGLETOFINESHIFT) as usize;
                let fog = P_SpawnMobj(
                    (*m).x + 20 * finecosine[an],
                    (*m).y + 20 * finesine[an],
                    (*thing).z,
                    MT_TFOG,
                );
                S_StartSound(fog, SFX_TELEPT);

                // Don't move for a bit
                if !(*thing).player.is_null() {
                    (*thing).reactiontime = 18;
                }

                (*thing).angle = (*m).angle;
                (*thing).momx = 0;
                (*thing).momy = 0;
                (*thing).momz = 0;
                return 1;
            }
        }
    }
    0
}

/// Stub declaration so we can compare function pointer values.
/// The real P_MobjThinker lives in C code (p_mobj.c).
extern "C" {
    fn P_MobjThinker(mobj: *mut mobj_t);
}

/// Anchor function referenced from `doomgeneric_Create` to ensure
/// `EV_Teleport` survives link-time dead-code elimination.
#[no_mangle]
pub extern "C" fn P_Telept_Link_Anchor() {
    let _ = EV_Teleport as *const () as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobj_t_size_is_224() {
        assert_eq!(std::mem::size_of::<mobj_t>(), 224);
    }

    #[test]
    fn mobj_t_field_offsets() {
        assert_eq!(std::mem::offset_of!(mobj_t, thinker), 0);
        assert_eq!(std::mem::offset_of!(mobj_t, x), 24);
        assert_eq!(std::mem::offset_of!(mobj_t, subsector), 88);
        assert_eq!(std::mem::offset_of!(mobj_t, floorz), 96);
        assert_eq!(std::mem::offset_of!(mobj_t, mobjtype), 128);
        assert_eq!(std::mem::offset_of!(mobj_t, flags), 160);
        assert_eq!(std::mem::offset_of!(mobj_t, player), 192);
        assert_eq!(std::mem::offset_of!(mobj_t, reactiontime), 184);
        assert_eq!(std::mem::offset_of!(mobj_t, tracer), 216);
    }

    #[test]
    fn subsector_t_size() {
        assert_eq!(std::mem::size_of::<subsector_t>(), 16);
    }

    #[test]
    fn sector_t_layout() {
        assert_eq!(std::mem::size_of::<sector_t>(), 128);
        assert_eq!(std::mem::offset_of!(sector_t, floorheight), 0);
        assert_eq!(std::mem::offset_of!(sector_t, ceilingheight), 4);
        assert_eq!(std::mem::offset_of!(sector_t, floorpic), 8);
        assert_eq!(std::mem::offset_of!(sector_t, ceilingpic), 10);
        assert_eq!(std::mem::offset_of!(sector_t, lightlevel), 12);
        assert_eq!(std::mem::offset_of!(sector_t, special), 14);
        assert_eq!(std::mem::offset_of!(sector_t, tag), 16);
        assert_eq!(std::mem::offset_of!(sector_t, soundtraversed), 20);
        assert_eq!(std::mem::offset_of!(sector_t, soundtarget), 24);
        assert_eq!(std::mem::offset_of!(sector_t, blockbox), 32);
        assert_eq!(std::mem::offset_of!(sector_t, soundorg), 48);
        assert_eq!(std::mem::offset_of!(sector_t, validcount), 88);
        assert_eq!(std::mem::offset_of!(sector_t, thinglist), 96);
        assert_eq!(std::mem::offset_of!(sector_t, specialdata), 104);
        assert_eq!(std::mem::offset_of!(sector_t, linecount), 112);
        assert_eq!(std::mem::offset_of!(sector_t, lines), 120);
    }
}
