//! Rust port of vendor/doomgeneric/wi_stuff.c.
//!
//! Intermission / victory screen logic: stats counting, animated backgrounds,
//! level-name display, and state-machine transitions.

#![allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    static_mut_refs,
    clippy::missing_safety_doc,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::needless_range_loop,
    clippy::ptr_offset_with_cast,
    clippy::manual_clamp,
    clippy::manual_c_str_literals
)]

use std::ffi::{c_char, c_int, c_void};
use std::ptr::{self, addr_of_mut};

use crate::types::Boolean;

use crate::doom::d_event::event_t;
use crate::doom::d_mode;
use crate::doom::d_player::{PlayerT, MAXPLAYERS};
use crate::doom::doomstat::gamemode;
use crate::doom::g_game::{deathmatch, netgame, playeringame, players, G_WorldDone};
use crate::doom::i_timer::TICRATE;
use crate::doom::i_video::{SCREENHEIGHT, SCREENWIDTH};
use crate::doom::m_misc::M_StringCopy;
use crate::doom::m_random::M_Random;
use crate::doom::s_sound::{S_ChangeMusic, S_StartSound};
use crate::doom::v_video::patch_t;
use crate::doom::v_video::V_DrawPatch;
use crate::doom::w_wad::{W_CacheLumpName, W_CheckNumForName, W_ReleaseLumpName};
use crate::doom::z_zone::{Z_Malloc, PU_STATIC};
use crate::{c_write, DEH_snprintf};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const NUMEPISODES: usize = 4;
const NUMMAPS: usize = 9;

const WI_TITLEY: c_int = 2;
const WI_SPACINGY: c_int = 33;

const SP_STATSX: c_int = 50;
const SP_STATSY: c_int = 50;
const SP_TIMEX: c_int = 16;
const SP_TIMEY: c_int = SCREENHEIGHT - 32;

const NG_STATSY: c_int = 50;
const NG_SPACINGX: c_int = 64;

const DM_MATRIXX: c_int = 42;
const DM_MATRIXY: c_int = 68;
const DM_SPACINGX: c_int = 40;
const DM_TOTALSX: c_int = 269;
const DM_KILLERSX: c_int = 10;
const DM_KILLERSY: c_int = 100;
const DM_VICTIMSX: c_int = 5;
const DM_VICTIMSY: c_int = 50;

const SHOWNEXTLOCDELAY: c_int = 4;

// Sound / music IDs
const sfx_pistol: c_int = 1;
const sfx_barexp: c_int = 82;
const sfx_slop: c_int = 31;
const sfx_sgcock: c_int = 3;
const sfx_pldeth: c_int = 57;
const mus_inter: c_int = 28;
const mus_dm2int: c_int = 67;

// ---------------------------------------------------------------------------
// Types that must match C layout (g_game.c is still C)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wbplayerstruct_t {
    pub in_: c_int,
    pub skills: c_int,
    pub sitems: c_int,
    pub ssecret: c_int,
    pub stime: c_int,
    pub frags: [c_int; 4],
    pub score: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wbstartstruct_t {
    pub epsd: c_int,
    pub didsecret: c_int,
    pub last: c_int,
    pub next: c_int,
    pub maxkills: c_int,
    pub maxitems: c_int,
    pub maxsecret: c_int,
    pub maxfrags: c_int,
    pub partime: c_int,
    pub pnum: c_int,
    pub plyr: [wbplayerstruct_t; MAXPLAYERS],
}

#[derive(Clone, Copy, PartialEq)]
enum stateenum_t {
    NoState = -1,
    StatCount,
    ShowNextLoc,
}

#[derive(Clone, Copy)]
struct point_t {
    x: c_int,
    y: c_int,
}

#[derive(Clone, Copy, PartialEq)]
enum animenum_t {
    ANIM_ALWAYS,
    ANIM_RANDOM,
    ANIM_LEVEL,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct anim_t {
    type_: animenum_t,
    period: c_int,
    nanims: c_int,
    loc: point_t,
    data1: c_int,
    data2: c_int,
    p: [*mut patch_t; 3],
    nexttic: c_int,
    lastdrawn: c_int,
    ctr: c_int,
    state: c_int,
}

// ---------------------------------------------------------------------------
// Static data tables
// ---------------------------------------------------------------------------

static LNODESEPSD0: [point_t; NUMMAPS] = [
    point_t { x: 185, y: 164 },
    point_t { x: 148, y: 143 },
    point_t { x: 69, y: 122 },
    point_t { x: 209, y: 102 },
    point_t { x: 116, y: 89 },
    point_t { x: 166, y: 55 },
    point_t { x: 71, y: 56 },
    point_t { x: 135, y: 29 },
    point_t { x: 71, y: 24 },
];

static LNODESEPSD1: [point_t; NUMMAPS] = [
    point_t { x: 254, y: 25 },
    point_t { x: 97, y: 50 },
    point_t { x: 188, y: 64 },
    point_t { x: 128, y: 78 },
    point_t { x: 214, y: 92 },
    point_t { x: 133, y: 130 },
    point_t { x: 208, y: 136 },
    point_t { x: 148, y: 140 },
    point_t { x: 235, y: 158 },
];

static LNODESEPSD2: [point_t; NUMMAPS] = [
    point_t { x: 156, y: 168 },
    point_t { x: 48, y: 154 },
    point_t { x: 174, y: 95 },
    point_t { x: 265, y: 75 },
    point_t { x: 130, y: 48 },
    point_t { x: 279, y: 23 },
    point_t { x: 198, y: 48 },
    point_t { x: 140, y: 25 },
    point_t { x: 281, y: 136 },
];

static LNODESEPSD3: [point_t; NUMMAPS] = [point_t { x: 0, y: 0 }; NUMMAPS];

static LNODES: [[point_t; NUMMAPS]; NUMEPISODES] =
    [LNODESEPSD0, LNODESEPSD1, LNODESEPSD2, LNODESEPSD3];

pub(crate) const EPSD0_NANIM: usize = 10;
pub(crate) const EPSD1_NANIM: usize = 9;
pub(crate) const EPSD2_NANIM: usize = 6;

// TODO: Consider splitting into immutable config fields (type_, period, nanims,
// loc, data1, data2) and mutable runtime state (ctr, nexttic, lastdrawn, state,
// p) using two separate arrays, or wrapping in UnsafeCell for explicit interior
// mutability without static mut.
static mut EPSD0ANIMINFO: [anim_t; EPSD0_NANIM] = [
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 224, y: 104 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 184, y: 160 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 112, y: 136 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 72, y: 112 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 88, y: 96 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 64, y: 48 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 192, y: 40 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 136, y: 16 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 80, y: 16 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 64, y: 24 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
];

// TODO: Same as EPSD0ANIMINFO — candidate for config/state split or UnsafeCell.
static mut EPSD1ANIMINFO: [anim_t; EPSD1_NANIM] = [
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 1,
        loc: point_t { x: 128, y: 136 },
        data1: 1,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 1,
        loc: point_t { x: 128, y: 136 },
        data1: 2,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 1,
        loc: point_t { x: 128, y: 136 },
        data1: 3,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 1,
        loc: point_t { x: 128, y: 136 },
        data1: 4,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 1,
        loc: point_t { x: 128, y: 136 },
        data1: 5,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 1,
        loc: point_t { x: 128, y: 136 },
        data1: 6,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 1,
        loc: point_t { x: 128, y: 136 },
        data1: 7,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 192, y: 144 },
        data1: 8,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_LEVEL,
        period: TICRATE / 3,
        nanims: 1,
        loc: point_t { x: 128, y: 136 },
        data1: 8,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
];

// TODO: Same as EPSD0ANIMINFO — candidate for config/state split or UnsafeCell.
static mut EPSD2ANIMINFO: [anim_t; EPSD2_NANIM] = [
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 104, y: 168 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 40, y: 136 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 160, y: 96 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 104, y: 80 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 3,
        nanims: 3,
        loc: point_t { x: 120, y: 32 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
    anim_t {
        type_: animenum_t::ANIM_ALWAYS,
        period: TICRATE / 4,
        nanims: 3,
        loc: point_t { x: 40, y: 0 },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    },
];

static NUMANIMS: [c_int; NUMEPISODES] = [
    EPSD0_NANIM as c_int,
    EPSD1_NANIM as c_int,
    EPSD2_NANIM as c_int,
    0,
];

static mut ANIMS: [*mut anim_t; NUMEPISODES] = [
    addr_of_mut!(EPSD0ANIMINFO) as *mut anim_t,
    addr_of_mut!(EPSD1ANIMINFO) as *mut anim_t,
    addr_of_mut!(EPSD2ANIMINFO) as *mut anim_t,
    ptr::null_mut(),
];

// ---------------------------------------------------------------------------
// Internal globals
// ---------------------------------------------------------------------------

static mut acceleratestage: c_int = 0;
static mut me: c_int = 0;
static mut state: stateenum_t = stateenum_t::NoState;
static mut wbs: *mut wbstartstruct_t = ptr::null_mut();
static mut plrs: *mut wbplayerstruct_t = ptr::null_mut();
static mut cnt: c_int = 0;
static mut bcnt: c_int = 0;
static mut firstrefresh: c_int = 0;

static mut cnt_kills: [c_int; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_items: [c_int; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_secret: [c_int; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_time: c_int = 0;
static mut cnt_par: c_int = 0;
static mut cnt_pause: c_int = 0;

static mut NUMCMAPS: c_int = 0;

// Graphics
static mut yah: [*mut patch_t; 3] = [ptr::null_mut(); 3];
static mut splat: [*mut patch_t; 2] = [ptr::null_mut(); 2];
static mut percent: *mut patch_t = ptr::null_mut();
static mut colon: *mut patch_t = ptr::null_mut();
static mut num: [*mut patch_t; 10] = [ptr::null_mut(); 10];
static mut wiminus: *mut patch_t = ptr::null_mut();
static mut finished: *mut patch_t = ptr::null_mut();
static mut entering: *mut patch_t = ptr::null_mut();
static mut sp_secret: *mut patch_t = ptr::null_mut();
static mut kills: *mut patch_t = ptr::null_mut();
static mut secret: *mut patch_t = ptr::null_mut();
static mut items: *mut patch_t = ptr::null_mut();
static mut frags: *mut patch_t = ptr::null_mut();
static mut timepatch: *mut patch_t = ptr::null_mut();
static mut par: *mut patch_t = ptr::null_mut();
static mut sucks: *mut patch_t = ptr::null_mut();
static mut killers: *mut patch_t = ptr::null_mut();
static mut victims: *mut patch_t = ptr::null_mut();
static mut total: *mut patch_t = ptr::null_mut();
static mut star: *mut patch_t = ptr::null_mut();
static mut bstar: *mut patch_t = ptr::null_mut();
static mut p: [*mut patch_t; MAXPLAYERS] = [ptr::null_mut(); MAXPLAYERS];
static mut bp: [*mut patch_t; MAXPLAYERS] = [ptr::null_mut(); MAXPLAYERS];
static mut lnames: *mut *mut patch_t = ptr::null_mut();
static mut background: *mut patch_t = ptr::null_mut();

// Deathmatch / netgame / single-player state
static mut dm_state: c_int = 0;
static mut dm_frags: [[c_int; MAXPLAYERS]; MAXPLAYERS] = [[0; MAXPLAYERS]; MAXPLAYERS];
static mut dm_totals: [c_int; MAXPLAYERS] = [0; MAXPLAYERS];

static mut cnt_frags: [c_int; MAXPLAYERS] = [0; MAXPLAYERS];
static mut dofrags: c_int = 0;
static mut ng_state: c_int = 0;

static mut sp_state: c_int = 0;
static mut snl_pointeron: bool = false;

// ---------------------------------------------------------------------------
// Externs
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

#[inline(always)]
unsafe fn DEH_String(s: *mut c_char) -> *mut c_char {
    s
}

#[inline(always)]
fn SHORT(x: i16) -> i16 {
    x
}

// ---------------------------------------------------------------------------
// WI_slamBackground
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn WI_slamBackground() {
    V_DrawPatch(0, 0, background);
}

// ---------------------------------------------------------------------------
// WI_Responder
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn WI_Responder(_ev: *mut event_t) -> c_int {
    0
}

// ---------------------------------------------------------------------------
// WI_drawLF
// ---------------------------------------------------------------------------

unsafe fn WI_drawLF() {
    let mut y = WI_TITLEY;

    if gamemode != d_mode::commercial || (*wbs).last < NUMCMAPS {
        V_DrawPatch(
            (SCREENWIDTH - SHORT((*(*lnames.offset((*wbs).last as isize))).width) as c_int) / 2,
            y,
            *lnames.offset((*wbs).last as isize),
        );
        y += (5 * SHORT((*(*lnames.offset((*wbs).last as isize))).height) as c_int) / 4;
        V_DrawPatch(
            (SCREENWIDTH - SHORT((*finished).width) as c_int) / 2,
            y,
            finished,
        );
    } else if (*wbs).last == NUMCMAPS {
        // MAP33 - nothing is displayed!
    } else if (*wbs).last > NUMCMAPS {
        // Deliberately trigger a V_DrawPatch error
        let tmp: patch_t = patch_t {
            width: SCREENWIDTH as i16,
            height: SCREENHEIGHT as i16,
            leftoffset: 1,
            topoffset: 1,
        };
        V_DrawPatch(0, y, &tmp as *const patch_t as *mut patch_t);
    }
}

// ---------------------------------------------------------------------------
// WI_drawEL
// ---------------------------------------------------------------------------

unsafe fn WI_drawEL() {
    let mut y = WI_TITLEY;

    V_DrawPatch(
        (SCREENWIDTH - SHORT((*entering).width) as c_int) / 2,
        y,
        entering,
    );

    y += (5 * SHORT((*(*lnames.offset((*wbs).next as isize))).height) as c_int) / 4;

    V_DrawPatch(
        (SCREENWIDTH - SHORT((*(*lnames.offset((*wbs).next as isize))).width) as c_int) / 2,
        y,
        *lnames.offset((*wbs).next as isize),
    );
}

// ---------------------------------------------------------------------------
// WI_drawOnLnode
// ---------------------------------------------------------------------------

unsafe fn WI_drawOnLnode(n: c_int, c: *mut *mut patch_t) {
    let mut i = 0;
    let mut fits = false;

    loop {
        let left = LNODES[(*wbs).epsd as usize][n as usize].x
            - SHORT((**c.offset(i as isize)).leftoffset) as c_int;
        let top = LNODES[(*wbs).epsd as usize][n as usize].y
            - SHORT((**c.offset(i as isize)).topoffset) as c_int;
        let right = left + SHORT((**c.offset(i as isize)).width) as c_int;
        let bottom = top + SHORT((**c.offset(i as isize)).height) as c_int;

        if left >= 0 && right < SCREENWIDTH && top >= 0 && bottom < SCREENHEIGHT {
            fits = true;
            break;
        }

        i += 1;
        if i == 2 || (*c.offset(i as isize)).is_null() {
            break;
        }
    }

    if fits && i < 2 {
        V_DrawPatch(
            LNODES[(*wbs).epsd as usize][n as usize].x,
            LNODES[(*wbs).epsd as usize][n as usize].y,
            *c.offset(i as isize),
        );
    } else {
        libc::printf(
            b"Could not place patch on level %d\0".as_ptr() as *const c_char,
            n + 1,
        );
    }
}

// ---------------------------------------------------------------------------
// Animated background
// ---------------------------------------------------------------------------

unsafe fn WI_initAnimatedBack() {
    if gamemode == d_mode::commercial {
        return;
    }
    if (*wbs).epsd > 2 {
        return;
    }

    let epsd = (*wbs).epsd as usize;
    let count = NUMANIMS[epsd] as usize;
    let base = ANIMS[epsd];

    for i in 0..count {
        let a = base.add(i);
        (*a).ctr = -1;
        if (*a).type_ == animenum_t::ANIM_ALWAYS {
            (*a).nexttic = bcnt + 1 + (M_Random() % (*a).period);
        } else if (*a).type_ == animenum_t::ANIM_RANDOM {
            (*a).nexttic = bcnt + 1 + (*a).data2 + (M_Random() % (*a).data1);
        } else if (*a).type_ == animenum_t::ANIM_LEVEL {
            (*a).nexttic = bcnt + 1;
        }
    }
}

unsafe fn WI_updateAnimatedBack() {
    if gamemode == d_mode::commercial {
        return;
    }
    if (*wbs).epsd > 2 {
        return;
    }

    let epsd = (*wbs).epsd as usize;
    let count = NUMANIMS[epsd] as usize;
    let base = ANIMS[epsd];

    for i in 0..count {
        let a = base.add(i);
        if bcnt == (*a).nexttic {
            match (*a).type_ {
                animenum_t::ANIM_ALWAYS => {
                    (*a).ctr += 1;
                    if (*a).ctr >= (*a).nanims {
                        (*a).ctr = 0;
                    }
                    (*a).nexttic = bcnt + (*a).period;
                }
                animenum_t::ANIM_RANDOM => {
                    (*a).ctr += 1;
                    if (*a).ctr == (*a).nanims {
                        (*a).ctr = -1;
                        (*a).nexttic = bcnt + (*a).data2 + (M_Random() % (*a).data1);
                    } else {
                        (*a).nexttic = bcnt + (*a).period;
                    }
                }
                animenum_t::ANIM_LEVEL => {
                    // gawd-awful hack for level anims
                    if !(state == stateenum_t::StatCount && i == 7) && (*wbs).next == (*a).data1 {
                        (*a).ctr += 1;
                        if (*a).ctr == (*a).nanims {
                            (*a).ctr -= 1;
                        }
                        (*a).nexttic = bcnt + (*a).period;
                    }
                }
            }
        }
    }
}

unsafe fn WI_drawAnimatedBack() {
    if gamemode == d_mode::commercial {
        return;
    }
    if (*wbs).epsd > 2 {
        return;
    }

    let epsd = (*wbs).epsd as usize;
    let count = NUMANIMS[epsd] as usize;
    let base = ANIMS[epsd] as *const anim_t;

    for i in 0..count {
        let a = base.add(i);
        if (*a).ctr >= 0 {
            V_DrawPatch((*a).loc.x, (*a).loc.y, (*a).p[(*a).ctr as usize]);
        }
    }
}

// ---------------------------------------------------------------------------
// Number drawing
// ---------------------------------------------------------------------------

unsafe fn WI_drawNum(mut x: c_int, y: c_int, mut n: c_int, mut digits: c_int) -> c_int {
    let fontwidth = SHORT((*num[0]).width) as c_int;
    let neg = n < 0;
    if neg {
        n = -n;
    }

    if digits < 0 {
        if n == 0 {
            digits = 1;
        } else {
            digits = 0;
            let mut temp = n;
            while temp != 0 {
                temp /= 10;
                digits += 1;
            }
        }
    }

    // if non-number, do not draw it
    if n == 1994 {
        return 0;
    }

    while digits > 0 {
        digits -= 1;
        x -= fontwidth;
        V_DrawPatch(x, y, num[(n % 10) as usize]);
        n /= 10;
    }

    if neg {
        x -= 8;
        V_DrawPatch(x, y, wiminus);
    }

    x
}

unsafe fn WI_drawPercent(x: c_int, y: c_int, pct: c_int) {
    if pct < 0 {
        return;
    }
    V_DrawPatch(x, y, percent);
    WI_drawNum(x, y, pct, -1);
}

unsafe fn WI_drawTime(mut x: c_int, y: c_int, t: c_int) {
    if t < 0 {
        return;
    }

    if t <= 61 * 59 {
        let mut div = 1;
        loop {
            let n = (t / div) % 60;
            x = WI_drawNum(x, y, n, 2) - SHORT((*colon).width) as c_int;
            div *= 60;
            if div == 60 || t / div != 0 {
                V_DrawPatch(x, y, colon);
            }
            if t / div == 0 {
                break;
            }
        }
    } else {
        V_DrawPatch(x - SHORT((*sucks).width) as c_int, y, sucks);
    }
}

// ---------------------------------------------------------------------------
// State: NoState
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn WI_End() {
    WI_unloadData();
}

unsafe fn WI_initNoState() {
    state = stateenum_t::NoState;
    acceleratestage = 0;
    cnt = 10;
}

unsafe fn WI_updateNoState() {
    WI_updateAnimatedBack();
    cnt -= 1;
    if cnt == 0 {
        G_WorldDone();
    }
}

// ---------------------------------------------------------------------------
// State: ShowNextLoc
// ---------------------------------------------------------------------------

unsafe fn WI_initShowNextLoc() {
    state = stateenum_t::ShowNextLoc;
    acceleratestage = 0;
    cnt = SHOWNEXTLOCDELAY * TICRATE;
    WI_initAnimatedBack();
}

unsafe fn WI_updateShowNextLoc() {
    WI_updateAnimatedBack();
    cnt -= 1;
    if cnt == 0 || acceleratestage != 0 {
        WI_initNoState();
    } else {
        snl_pointeron = (cnt & 31) < 20;
    }
}

unsafe fn WI_drawShowNextLoc() {
    WI_slamBackground();
    WI_drawAnimatedBack();

    if gamemode != d_mode::commercial {
        if (*wbs).epsd > 2 {
            WI_drawEL();
            return;
        }

        let last = if (*wbs).last == 8 {
            (*wbs).next - 1
        } else {
            (*wbs).last
        };

        for i in 0..=last {
            WI_drawOnLnode(i, std::ptr::addr_of_mut!(splat) as *mut *mut patch_t);
        }

        if (*wbs).didsecret != 0 {
            WI_drawOnLnode(8, std::ptr::addr_of_mut!(splat) as *mut *mut patch_t);
        }

        if snl_pointeron {
            WI_drawOnLnode(
                (*wbs).next,
                std::ptr::addr_of_mut!(yah) as *mut *mut patch_t,
            );
        }
    }

    if gamemode != d_mode::commercial || (*wbs).next != 30 {
        WI_drawEL();
    }
}

unsafe fn WI_drawNoState() {
    snl_pointeron = true;
    WI_drawShowNextLoc();
}

// ---------------------------------------------------------------------------
// Frag helpers
// ---------------------------------------------------------------------------

unsafe fn WI_fragSum(playernum: c_int) -> c_int {
    let mut sum = 0;
    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 && i as c_int != playernum {
            sum += (*plrs.offset(playernum as isize)).frags[i];
        }
    }
    sum -= (*plrs.offset(playernum as isize)).frags[playernum as usize];
    sum
}

// ---------------------------------------------------------------------------
// State: Deathmatch stats
// ---------------------------------------------------------------------------

unsafe fn WI_initDeathmatchStats() {
    state = stateenum_t::StatCount;
    acceleratestage = 0;
    dm_state = 1;
    cnt_pause = TICRATE;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            for j in 0..MAXPLAYERS {
                if playeringame[j] != 0 {
                    dm_frags[i][j] = 0;
                }
            }
            dm_totals[i] = 0;
        }
    }

    WI_initAnimatedBack();
}

unsafe fn WI_updateDeathmatchStats() {
    WI_updateAnimatedBack();

    if acceleratestage != 0 && dm_state != 4 {
        acceleratestage = 0;
        for i in 0..MAXPLAYERS {
            if playeringame[i] != 0 {
                for j in 0..MAXPLAYERS {
                    if playeringame[j] != 0 {
                        dm_frags[i][j] = (*plrs.offset(i as isize)).frags[j];
                    }
                }
                dm_totals[i] = WI_fragSum(i as c_int);
            }
        }
        S_StartSound(ptr::null_mut(), sfx_barexp);
        dm_state = 4;
    }

    if dm_state == 2 {
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] != 0 {
                for j in 0..MAXPLAYERS {
                    if playeringame[j] != 0 && dm_frags[i][j] != (*plrs.offset(i as isize)).frags[j]
                    {
                        if (*plrs.offset(i as isize)).frags[j] < 0 {
                            dm_frags[i][j] -= 1;
                        } else {
                            dm_frags[i][j] += 1;
                        }
                        if dm_frags[i][j] > 99 {
                            dm_frags[i][j] = 99;
                        }
                        if dm_frags[i][j] < -99 {
                            dm_frags[i][j] = -99;
                        }
                        stillticking = true;
                    }
                }
                dm_totals[i] = WI_fragSum(i as c_int);
                if dm_totals[i] > 99 {
                    dm_totals[i] = 99;
                }
                if dm_totals[i] < -99 {
                    dm_totals[i] = -99;
                }
            }
        }
        if !stillticking {
            S_StartSound(ptr::null_mut(), sfx_barexp);
            dm_state += 1;
        }
    } else if dm_state == 4 {
        if acceleratestage != 0 {
            S_StartSound(ptr::null_mut(), sfx_slop);
            if gamemode == d_mode::commercial {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if dm_state & 1 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            dm_state += 1;
            cnt_pause = TICRATE;
        }
    }
}

unsafe fn WI_drawDeathmatchStats() {
    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();

    V_DrawPatch(
        DM_TOTALSX - SHORT((*total).width) as c_int / 2,
        DM_MATRIXY - WI_SPACINGY + 10,
        total,
    );
    V_DrawPatch(DM_KILLERSX, DM_KILLERSY, killers);
    V_DrawPatch(DM_VICTIMSX, DM_VICTIMSY, victims);

    let mut x = DM_MATRIXX + DM_SPACINGX;
    let mut y = DM_MATRIXY;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            V_DrawPatch(
                x - SHORT((*p[i]).width) as c_int / 2,
                DM_MATRIXY - WI_SPACINGY,
                p[i],
            );
            V_DrawPatch(DM_MATRIXX - SHORT((*p[i]).width) as c_int / 2, y, p[i]);
            if i as c_int == me {
                V_DrawPatch(
                    x - SHORT((*p[i]).width) as c_int / 2,
                    DM_MATRIXY - WI_SPACINGY,
                    bstar,
                );
                V_DrawPatch(DM_MATRIXX - SHORT((*p[i]).width) as c_int / 2, y, star);
            }
        }
        x += DM_SPACINGX;
        y += WI_SPACINGY;
    }

    let mut y = DM_MATRIXY + 10;
    let w = SHORT((*num[0]).width) as c_int;

    for i in 0..MAXPLAYERS {
        let mut x = DM_MATRIXX + DM_SPACINGX;
        if playeringame[i] != 0 {
            for j in 0..MAXPLAYERS {
                if playeringame[j] != 0 {
                    WI_drawNum(x + w, y, dm_frags[i][j], 2);
                }
                x += DM_SPACINGX;
            }
            WI_drawNum(DM_TOTALSX + w, y, dm_totals[i], 2);
        }
        y += WI_SPACINGY;
    }
}

// ---------------------------------------------------------------------------
// State: Netgame stats
// ---------------------------------------------------------------------------

unsafe fn WI_initNetgameStats() {
    state = stateenum_t::StatCount;
    acceleratestage = 0;
    ng_state = 1;
    cnt_pause = TICRATE;

    for i in 0..MAXPLAYERS {
        if playeringame[i] == 0 {
            continue;
        }
        cnt_kills[i] = 0;
        cnt_items[i] = 0;
        cnt_secret[i] = 0;
        cnt_frags[i] = 0;
        dofrags += WI_fragSum(i as c_int);
    }

    dofrags = if dofrags != 0 { 1 } else { 0 };

    WI_initAnimatedBack();
}

unsafe fn WI_updateNetgameStats() {
    WI_updateAnimatedBack();

    if acceleratestage != 0 && ng_state != 10 {
        acceleratestage = 0;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_kills[i] = ((*plrs.offset(i as isize)).skills * 100) / (*wbs).maxkills;
            cnt_items[i] = ((*plrs.offset(i as isize)).sitems * 100) / (*wbs).maxitems;
            cnt_secret[i] = ((*plrs.offset(i as isize)).ssecret * 100) / (*wbs).maxsecret;
            if dofrags != 0 {
                cnt_frags[i] = WI_fragSum(i as c_int);
            }
        }
        S_StartSound(ptr::null_mut(), sfx_barexp);
        ng_state = 10;
    }

    if ng_state == 2 {
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_kills[i] += 2;
            let target = ((*plrs.offset(i as isize)).skills * 100) / (*wbs).maxkills;
            if cnt_kills[i] >= target {
                cnt_kills[i] = target;
            } else {
                stillticking = true;
            }
        }
        if !stillticking {
            S_StartSound(ptr::null_mut(), sfx_barexp);
            ng_state += 1;
        }
    } else if ng_state == 4 {
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_items[i] += 2;
            let target = ((*plrs.offset(i as isize)).sitems * 100) / (*wbs).maxitems;
            if cnt_items[i] >= target {
                cnt_items[i] = target;
            } else {
                stillticking = true;
            }
        }
        if !stillticking {
            S_StartSound(ptr::null_mut(), sfx_barexp);
            ng_state += 1;
        }
    } else if ng_state == 6 {
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_secret[i] += 2;
            let target = ((*plrs.offset(i as isize)).ssecret * 100) / (*wbs).maxsecret;
            if cnt_secret[i] >= target {
                cnt_secret[i] = target;
            } else {
                stillticking = true;
            }
        }
        if !stillticking {
            S_StartSound(ptr::null_mut(), sfx_barexp);
            ng_state += 1 + 2 * if dofrags == 0 { 1 } else { 0 };
        }
    } else if ng_state == 8 {
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_frags[i] += 1;
            let fsum = WI_fragSum(i as c_int);
            if cnt_frags[i] >= fsum {
                cnt_frags[i] = fsum;
            } else {
                stillticking = true;
            }
        }
        if !stillticking {
            S_StartSound(ptr::null_mut(), sfx_pldeth);
            ng_state += 1;
        }
    } else if ng_state == 10 {
        if acceleratestage != 0 {
            S_StartSound(ptr::null_mut(), sfx_sgcock);
            if gamemode == d_mode::commercial {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if ng_state & 1 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            ng_state += 1;
            cnt_pause = TICRATE;
        }
    }
}

unsafe fn WI_drawNetgameStats() {
    let pwidth = SHORT((*percent).width) as c_int;

    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();

    let ng_statsx = 32 + SHORT((*star).width) as c_int / 2 + 32 * if dofrags == 0 { 1 } else { 0 };

    V_DrawPatch(
        ng_statsx + NG_SPACINGX - SHORT((*kills).width) as c_int,
        NG_STATSY,
        kills,
    );
    V_DrawPatch(
        ng_statsx + 2 * NG_SPACINGX - SHORT((*items).width) as c_int,
        NG_STATSY,
        items,
    );
    V_DrawPatch(
        ng_statsx + 3 * NG_SPACINGX - SHORT((*secret).width) as c_int,
        NG_STATSY,
        secret,
    );
    if dofrags != 0 {
        V_DrawPatch(
            ng_statsx + 4 * NG_SPACINGX - SHORT((*frags).width) as c_int,
            NG_STATSY,
            frags,
        );
    }

    let mut y = NG_STATSY + SHORT((*kills).height) as c_int;

    for i in 0..MAXPLAYERS {
        if playeringame[i] == 0 {
            continue;
        }
        let mut x = ng_statsx;
        V_DrawPatch(x - SHORT((*p[i]).width) as c_int, y, p[i]);
        if i as c_int == me {
            V_DrawPatch(x - SHORT((*p[i]).width) as c_int, y, star);
        }
        x += NG_SPACINGX;
        WI_drawPercent(x - pwidth, y + 10, cnt_kills[i]);
        x += NG_SPACINGX;
        WI_drawPercent(x - pwidth, y + 10, cnt_items[i]);
        x += NG_SPACINGX;
        WI_drawPercent(x - pwidth, y + 10, cnt_secret[i]);
        x += NG_SPACINGX;
        if dofrags != 0 {
            WI_drawNum(x, y + 10, cnt_frags[i], -1);
        }
        y += WI_SPACINGY;
    }
}

// ---------------------------------------------------------------------------
// State: Single-player stats
// ---------------------------------------------------------------------------

unsafe fn WI_initStats() {
    state = stateenum_t::StatCount;
    acceleratestage = 0;
    sp_state = 1;
    cnt_kills[0] = -1;
    cnt_items[0] = -1;
    cnt_secret[0] = -1;
    cnt_time = -1;
    cnt_par = -1;
    cnt_pause = TICRATE;

    WI_initAnimatedBack();
}

unsafe fn WI_updateStats() {
    WI_updateAnimatedBack();

    if acceleratestage != 0 && sp_state != 10 {
        acceleratestage = 0;
        cnt_kills[0] = ((*plrs.offset(me as isize)).skills * 100) / (*wbs).maxkills;
        cnt_items[0] = ((*plrs.offset(me as isize)).sitems * 100) / (*wbs).maxitems;
        cnt_secret[0] = ((*plrs.offset(me as isize)).ssecret * 100) / (*wbs).maxsecret;
        cnt_time = (*plrs.offset(me as isize)).stime / TICRATE;
        cnt_par = (*wbs).partime / TICRATE;
        S_StartSound(ptr::null_mut(), sfx_barexp);
        sp_state = 10;
    }

    if sp_state == 2 {
        cnt_kills[0] += 2;
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        let target = ((*plrs.offset(me as isize)).skills * 100) / (*wbs).maxkills;
        if cnt_kills[0] >= target {
            cnt_kills[0] = target;
            S_StartSound(ptr::null_mut(), sfx_barexp);
            sp_state += 1;
        }
    } else if sp_state == 4 {
        cnt_items[0] += 2;
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        let target = ((*plrs.offset(me as isize)).sitems * 100) / (*wbs).maxitems;
        if cnt_items[0] >= target {
            cnt_items[0] = target;
            S_StartSound(ptr::null_mut(), sfx_barexp);
            sp_state += 1;
        }
    } else if sp_state == 6 {
        cnt_secret[0] += 2;
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        let target = ((*plrs.offset(me as isize)).ssecret * 100) / (*wbs).maxsecret;
        if cnt_secret[0] >= target {
            cnt_secret[0] = target;
            S_StartSound(ptr::null_mut(), sfx_barexp);
            sp_state += 1;
        }
    } else if sp_state == 8 {
        if bcnt & 3 == 0 {
            S_StartSound(ptr::null_mut(), sfx_pistol);
        }
        cnt_time += 3;
        let target_time = (*plrs.offset(me as isize)).stime / TICRATE;
        if cnt_time >= target_time {
            cnt_time = target_time;
        }
        cnt_par += 3;
        let target_par = (*wbs).partime / TICRATE;
        if cnt_par >= target_par {
            cnt_par = target_par;
            if cnt_time >= target_time {
                S_StartSound(ptr::null_mut(), sfx_barexp);
                sp_state += 1;
            }
        }
    } else if sp_state == 10 {
        if acceleratestage != 0 {
            S_StartSound(ptr::null_mut(), sfx_sgcock);
            if gamemode == d_mode::commercial {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if sp_state & 1 != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            sp_state += 1;
            cnt_pause = TICRATE;
        }
    }
}

unsafe fn WI_drawStats() {
    let lh = (3 * SHORT((*num[0]).height) as c_int) / 2;

    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();

    V_DrawPatch(SP_STATSX, SP_STATSY, kills);
    WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY, cnt_kills[0]);

    V_DrawPatch(SP_STATSX, SP_STATSY + lh, items);
    WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY + lh, cnt_items[0]);

    V_DrawPatch(SP_STATSX, SP_STATSY + 2 * lh, sp_secret);
    WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY + 2 * lh, cnt_secret[0]);

    V_DrawPatch(SP_TIMEX, SP_TIMEY, timepatch);
    WI_drawTime(SCREENWIDTH / 2 - SP_TIMEX, SP_TIMEY, cnt_time);

    if (*wbs).epsd < 3 {
        V_DrawPatch(SCREENWIDTH / 2 + SP_TIMEX, SP_TIMEY, par);
        WI_drawTime(SCREENWIDTH - SP_TIMEX, SP_TIMEY, cnt_par);
    }
}

// ---------------------------------------------------------------------------
// Accelerate check
// ---------------------------------------------------------------------------

unsafe fn WI_checkForAccelerate() {
    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            let player = players.as_mut_ptr().add(i);
            if (*player).cmd.buttons & 1 != 0 {
                // BT_ATTACK
                if (*player).attackdown == 0 {
                    acceleratestage = 1;
                }
                (*player).attackdown = 1;
            } else {
                (*player).attackdown = 0;
            }
            if (*player).cmd.buttons & 2 != 0 {
                // BT_USE
                if (*player).usedown == 0 {
                    acceleratestage = 1;
                }
                (*player).usedown = 1;
            } else {
                (*player).usedown = 0;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// WI_Ticker
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn WI_Ticker() {
    bcnt += 1;

    if bcnt == 1 {
        if gamemode == d_mode::commercial {
            S_ChangeMusic(mus_dm2int, 1);
        } else {
            S_ChangeMusic(mus_inter, 1);
        }
    }

    WI_checkForAccelerate();

    match state {
        stateenum_t::StatCount => {
            if deathmatch != 0 {
                WI_updateDeathmatchStats();
            } else if netgame != 0 {
                WI_updateNetgameStats();
            } else {
                WI_updateStats();
            }
        }
        stateenum_t::ShowNextLoc => {
            WI_updateShowNextLoc();
        }
        stateenum_t::NoState => {
            WI_updateNoState();
        }
    }
}

// ---------------------------------------------------------------------------
// Data loading / unloading
// ---------------------------------------------------------------------------

type LoadCallback = unsafe extern "C" fn(*mut c_char, *mut *mut patch_t);

unsafe fn WI_loadUnloadData(callback: LoadCallback) {
    let mut name: [c_char; 9] = [0; 9];

    if gamemode == d_mode::commercial {
        for i in 0..NUMCMAPS {
            DEH_snprintf!(name, "CWILV{:02}", i);
            callback(name.as_mut_ptr(), lnames.offset(i as isize));
        }
    } else {
        for i in 0..NUMMAPS as c_int {
            c_write!(name, "WILV{}{}", (*wbs).epsd, i);
            callback(name.as_mut_ptr(), lnames.offset(i as isize));
        }

        callback(DEH_String(b"WIURH0\0".as_ptr() as *mut c_char), &mut yah[0]);
        callback(DEH_String(b"WIURH1\0".as_ptr() as *mut c_char), &mut yah[1]);
        callback(
            DEH_String(b"WISPLAT\0".as_ptr() as *mut c_char),
            &mut splat[0],
        );

        if (*wbs).epsd < 3 {
            for j in 0..NUMANIMS[(*wbs).epsd as usize] {
                let a = ANIMS[(*wbs).epsd as usize].add(j as usize);
                for i in 0..(*a).nanims {
                    if (*wbs).epsd != 1 || j != 8 {
                        c_write!(name, "WIA{}{:02}{:02}", (*wbs).epsd, j, i);
                        callback(name.as_mut_ptr(), &mut (*a).p[i as usize]);
                    } else {
                        (*a).p[i as usize] = (*ANIMS[1].add(4)).p[i as usize];
                    }
                }
            }
        }
    }

    callback(
        DEH_String(b"WIMINUS\0".as_ptr() as *mut c_char),
        &mut wiminus,
    );

    for i in 0..10i32 {
        DEH_snprintf!(name, "WINUM{}", i);
        callback(name.as_mut_ptr(), &mut num[i as usize]);
    }

    callback(
        DEH_String(b"WIPCNT\0".as_ptr() as *mut c_char),
        &mut percent,
    );
    callback(DEH_String(b"WIF\0".as_ptr() as *mut c_char), &mut finished);
    callback(
        DEH_String(b"WIENTER\0".as_ptr() as *mut c_char),
        &mut entering,
    );
    callback(DEH_String(b"WIOSTK\0".as_ptr() as *mut c_char), &mut kills);
    callback(DEH_String(b"WIOSTS\0".as_ptr() as *mut c_char), &mut secret);
    callback(
        DEH_String(b"WISCRT2\0".as_ptr() as *mut c_char),
        &mut sp_secret,
    );

    if W_CheckNumForName(DEH_String(b"WIOBJ\0".as_ptr() as *mut c_char)) >= 0 {
        if netgame != 0 && deathmatch == 0 {
            callback(DEH_String(b"WIOBJ\0".as_ptr() as *mut c_char), &mut items);
        } else {
            callback(DEH_String(b"WIOSTI\0".as_ptr() as *mut c_char), &mut items);
        }
    } else {
        callback(DEH_String(b"WIOSTI\0".as_ptr() as *mut c_char), &mut items);
    }

    callback(DEH_String(b"WIFRGS\0".as_ptr() as *mut c_char), &mut frags);
    callback(DEH_String(b"WICOLON\0".as_ptr() as *mut c_char), &mut colon);
    callback(
        DEH_String(b"WITIME\0".as_ptr() as *mut c_char),
        &mut timepatch,
    );
    callback(DEH_String(b"WISUCKS\0".as_ptr() as *mut c_char), &mut sucks);
    callback(DEH_String(b"WIPAR\0".as_ptr() as *mut c_char), &mut par);
    callback(
        DEH_String(b"WIKILRS\0".as_ptr() as *mut c_char),
        &mut killers,
    );
    callback(
        DEH_String(b"WIVCTMS\0".as_ptr() as *mut c_char),
        &mut victims,
    );
    callback(DEH_String(b"WIMSTT\0".as_ptr() as *mut c_char), &mut total);

    for i in 0..MAXPLAYERS {
        DEH_snprintf!(name, "STPB{}", i);
        callback(name.as_mut_ptr(), &mut p[i]);
        DEH_snprintf!(name, "WIBP{}", i + 1);
        callback(name.as_mut_ptr(), &mut bp[i]);
    }

    if gamemode == d_mode::commercial {
        M_StringCopy(
            name.as_mut_ptr(),
            DEH_String(b"INTERPIC\0".as_ptr() as *mut c_char),
            name.len(),
        );
    } else if gamemode == d_mode::retail && (*wbs).epsd == 3 {
        M_StringCopy(
            name.as_mut_ptr(),
            DEH_String(b"INTERPIC\0".as_ptr() as *mut c_char),
            name.len(),
        );
    } else {
        DEH_snprintf!(name, "WIMAP{}", (*wbs).epsd);
    }

    callback(name.as_mut_ptr(), &mut background);
}

unsafe extern "C" fn WI_loadCallback(name: *mut c_char, variable: *mut *mut patch_t) {
    *variable = W_CacheLumpName(name, PU_STATIC) as *mut patch_t;
}

#[no_mangle]
pub unsafe extern "C" fn WI_loadData() {
    if gamemode == d_mode::commercial {
        NUMCMAPS = 32;
        lnames = Z_Malloc(
            (std::mem::size_of::<*mut patch_t>() * NUMCMAPS as usize) as c_int,
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut patch_t;
    } else {
        lnames = Z_Malloc(
            (std::mem::size_of::<*mut patch_t>() * NUMMAPS) as c_int,
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut patch_t;
    }

    WI_loadUnloadData(WI_loadCallback);

    star = W_CacheLumpName(DEH_String(b"STFST01\0".as_ptr() as *mut c_char), PU_STATIC)
        as *mut patch_t;
    bstar = W_CacheLumpName(DEH_String(b"STFDEAD0\0".as_ptr() as *mut c_char), PU_STATIC)
        as *mut patch_t;
}

unsafe extern "C" fn WI_unloadCallback(name: *mut c_char, variable: *mut *mut patch_t) {
    W_ReleaseLumpName(name);
    *variable = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn WI_unloadData() {
    WI_loadUnloadData(WI_unloadCallback);
}

// ---------------------------------------------------------------------------
// WI_Drawer
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn WI_Drawer() {
    match state {
        stateenum_t::StatCount => {
            if deathmatch != 0 {
                WI_drawDeathmatchStats();
            } else if netgame != 0 {
                WI_drawNetgameStats();
            } else {
                WI_drawStats();
            }
        }
        stateenum_t::ShowNextLoc => {
            WI_drawShowNextLoc();
        }
        stateenum_t::NoState => {
            WI_drawNoState();
        }
    }
}

// ---------------------------------------------------------------------------
// WI_initVariables / WI_Start
// ---------------------------------------------------------------------------

unsafe fn WI_initVariables(wbstartstruct: *mut wbstartstruct_t) {
    wbs = wbstartstruct;
    plrs = (*wbs).plyr.as_mut_ptr();

    acceleratestage = 0;
    cnt = 0;
    bcnt = 0;
    firstrefresh = 1;
    me = (*wbs).pnum;

    if (*wbs).maxkills == 0 {
        (*wbs).maxkills = 1;
    }
    if (*wbs).maxitems == 0 {
        (*wbs).maxitems = 1;
    }
    if (*wbs).maxsecret == 0 {
        (*wbs).maxsecret = 1;
    }

    if gamemode != d_mode::retail && (*wbs).epsd > 2 {
        (*wbs).epsd -= 3;
    }
}

#[no_mangle]
pub unsafe extern "C" fn WI_Start(wbstartstruct: *mut wbstartstruct_t) {
    WI_initVariables(wbstartstruct);
    WI_loadData();

    if deathmatch != 0 {
        WI_initDeathmatchStats();
    } else if netgame != 0 {
        WI_initNetgameStats();
    } else {
        WI_initStats();
    }
}
