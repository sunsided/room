//! Rust port of vendor/doomgeneric/am_map.c.
//!
//! The fullscreen automap: player arrow, wall lines, grid, zoom/pan,
//! mark points, and crosshair.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int};
use std::os::raw::c_uint;
use std::ptr;

use crate::c_write;
use crate::doom::c_ffi::{mobj_t, sector_t, MAPBLOCKSHIFT, MAPBLOCKSIZE, MAPBLOCKUNITS};
use crate::doom::d_event::event_t;
use crate::doom::d_player::{PlayerT, MAXPLAYERS};
use crate::doom::i_video::I_VideoBuffer;
use crate::doom::i_video::{SCREENHEIGHT, SCREENWIDTH};
use crate::doom::m_cheat::{cheatseq_t, cht_CheckCheat};
use crate::doom::m_controls::{
    key_map_clearmark, key_map_east, key_map_follow, key_map_grid, key_map_mark, key_map_maxzoom,
    key_map_north, key_map_south, key_map_toggle, key_map_west, key_map_zoomin, key_map_zoomout,
};
use crate::doom::m_fixed::{fixed_t, FixedDiv, FixedMul};
use crate::doom::m_fixed::{FRACBITS, FRACUNIT};
use crate::doom::p_setup::{
    bmaporgx, bmaporgy, lines, numlines, numsectors, numvertexes, sectors, vertexes,
};
use crate::doom::tables::ANGLETOFINESHIFT;
use crate::doom::tables::{finecosine, finesine};
use crate::doom::v_video::{patch_t, V_DrawPatch, V_MarkRect};
use crate::doom::w_wad::{W_CacheLumpName, W_ReleaseLumpName};
use crate::doom::g_game::{
    consoleplayer, deathmatch, gameepisode, gamemap, netgame, playeringame, players, singledemo,
    viewactive,
};
use crate::doom::st_stuff::ST_Responder;

use crate::doom::z_zone::PU_STATIC;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const AM_NUMMARKPOINTS: usize = 10;
pub const INITSCALEMTOF: c_int = (0.2 * FRACUNIT as f64) as c_int;
pub const F_PANINC: c_int = 4;
pub const M_ZOOMIN: c_int = (1.02 * FRACUNIT as f64) as c_int;
pub const M_ZOOMOUT: c_int = (FRACUNIT as f64 / 1.02) as c_int;

const PLAYERRADIUS: c_int = 16 * FRACUNIT;

// Colour palette indices.
const REDS: c_int = 256 - 5 * 16;
const REDRANGE: c_int = 16;
const BLUES: c_int = 256 - 4 * 16 + 8;
const BLUERANGE: c_int = 8;
const GREENS: c_int = 7 * 16;
const GREENRANGE: c_int = 16;
const GRAYS: c_int = 6 * 16;
const GRAYSRANGE: c_int = 16;
const BROWNS: c_int = 4 * 16;
const BROWNRANGE: c_int = 16;
const YELLOWS: c_int = 256 - 32 + 7;
const YELLOWRANGE: c_int = 1;
const BLACK: c_int = 0;
const WHITE: c_int = 256 - 47;

// Automap colours.
const BACKGROUND: c_int = BLACK;
const WALLCOLORS: c_int = REDS;
const WALLRANGE: c_int = REDRANGE;
const TSWALLCOLORS: c_int = GRAYS;
const TSWALLRANGE: c_int = GRAYSRANGE;
const FDWALLCOLORS: c_int = BROWNS;
const FDWALLRANGE: c_int = BROWNRANGE;
const CDWALLCOLORS: c_int = YELLOWS;
const CDWALLRANGE: c_int = YELLOWRANGE;
const THINGCOLORS: c_int = GREENS;
const THINGRANGE: c_int = GREENRANGE;
const SECRETWALLCOLORS: c_int = WALLCOLORS;
const SECRETWALLRANGE: c_int = WALLRANGE;
const GRIDCOLORS: c_int = GRAYS + GRAYSRANGE / 2;
const XHAIRCOLORS: c_int = GRAYS;

// LineDef flag bits used by the automap.
const ML_MAPPED: i16 = 256;
const ML_SECRET: i16 = 32;
const ML_DONTDRAW: i16 = 128;

// Automap message constants (matches st_stuff.c expectations).
const AM_MSGHEADER: c_int = (('a' as c_int) << 24) + (('m' as c_int) << 16);
const AM_MSGENTERED: c_int = AM_MSGHEADER | (('e' as c_int) << 8);
const AM_MSGEXITED: c_int = AM_MSGHEADER | (('x' as c_int) << 8);

// DEH_String is identity when dehacked is disabled.
#[inline(always)]
unsafe fn DEH_String(s: *mut c_char) -> *mut c_char {
    s
}

// ---------------------------------------------------------------------------
// C globals still provided by unported C modules
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
struct fpoint_t {
    x: c_int,
    y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct fline_t {
    a: fpoint_t,
    b: fpoint_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mpoint_t {
    x: fixed_t,
    y: fixed_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct mline_t {
    a: mpoint_t,
    b: mpoint_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct islope_t {
    slp: fixed_t,
    islp: fixed_t,
}

// ---------------------------------------------------------------------------
// Player-arrow shape data
// ---------------------------------------------------------------------------

const R_ARROW: c_int = (8 * PLAYERRADIUS) / 7;

const PLAYER_ARROW: [mline_t; 7] = [
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t { x: R_ARROW, y: 0 },
    },
    mline_t {
        a: mpoint_t { x: R_ARROW, y: 0 },
        b: mpoint_t {
            x: R_ARROW - R_ARROW / 2,
            y: R_ARROW / 4,
        },
    },
    mline_t {
        a: mpoint_t { x: R_ARROW, y: 0 },
        b: mpoint_t {
            x: R_ARROW - R_ARROW / 2,
            y: -R_ARROW / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW - R_ARROW / 8,
            y: R_ARROW / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW - R_ARROW / 8,
            y: -R_ARROW / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + 3 * R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: R_ARROW / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + 3 * R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: -R_ARROW / 4,
        },
    },
];

const CHEAT_ARROW: [mline_t; 16] = [
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t { x: R_ARROW, y: 0 },
    },
    mline_t {
        a: mpoint_t { x: R_ARROW, y: 0 },
        b: mpoint_t {
            x: R_ARROW - R_ARROW / 2,
            y: R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t { x: R_ARROW, y: 0 },
        b: mpoint_t {
            x: R_ARROW - R_ARROW / 2,
            y: -R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW - R_ARROW / 8,
            y: R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW - R_ARROW / 8,
            y: -R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + 3 * R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW + 3 * R_ARROW / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW + R_ARROW / 8,
            y: -R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW / 2,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW / 2,
            y: -R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW / 2,
            y: -R_ARROW / 6,
        },
        b: mpoint_t {
            x: -R_ARROW / 2 + R_ARROW / 6,
            y: -R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW / 2 + R_ARROW / 6,
            y: -R_ARROW / 6,
        },
        b: mpoint_t {
            x: -R_ARROW / 2 + R_ARROW / 6,
            y: R_ARROW / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW / 6,
            y: 0,
        },
        b: mpoint_t {
            x: -R_ARROW / 6,
            y: -R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_ARROW / 6,
            y: -R_ARROW / 6,
        },
        b: mpoint_t {
            x: 0,
            y: -R_ARROW / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: 0,
            y: -R_ARROW / 6,
        },
        b: mpoint_t {
            x: 0,
            y: R_ARROW / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_ARROW / 6,
            y: R_ARROW / 4,
        },
        b: mpoint_t {
            x: R_ARROW / 6,
            y: -R_ARROW / 7,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_ARROW / 6,
            y: -R_ARROW / 7,
        },
        b: mpoint_t {
            x: R_ARROW / 6 + R_ARROW / 32,
            y: -R_ARROW / 7 - R_ARROW / 32,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_ARROW / 6 + R_ARROW / 32,
            y: -R_ARROW / 7 - R_ARROW / 32,
        },
        b: mpoint_t {
            x: R_ARROW / 6 + R_ARROW / 10,
            y: -R_ARROW / 7,
        },
    },
];

const TRIANGLE_GUY: [mline_t; 3] = [
    mline_t {
        a: mpoint_t {
            x: (-0.867f64 * FRACUNIT as f64) as fixed_t,
            y: (-0.5f64 * FRACUNIT as f64) as fixed_t,
        },
        b: mpoint_t {
            x: (0.867f64 * FRACUNIT as f64) as fixed_t,
            y: (-0.5f64 * FRACUNIT as f64) as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: (0.867f64 * FRACUNIT as f64) as fixed_t,
            y: (-0.5f64 * FRACUNIT as f64) as fixed_t,
        },
        b: mpoint_t { x: 0, y: FRACUNIT },
    },
    mline_t {
        a: mpoint_t { x: 0, y: FRACUNIT },
        b: mpoint_t {
            x: (-0.867f64 * FRACUNIT as f64) as fixed_t,
            y: (-0.5f64 * FRACUNIT as f64) as fixed_t,
        },
    },
];

const THINTRIANGLE_GUY: [mline_t; 3] = [
    mline_t {
        a: mpoint_t {
            x: (-0.5f64 * FRACUNIT as f64) as fixed_t,
            y: (-0.7f64 * FRACUNIT as f64) as fixed_t,
        },
        b: mpoint_t { x: FRACUNIT, y: 0 },
    },
    mline_t {
        a: mpoint_t { x: FRACUNIT, y: 0 },
        b: mpoint_t {
            x: (-0.5f64 * FRACUNIT as f64) as fixed_t,
            y: (0.7f64 * FRACUNIT as f64) as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: (-0.5f64 * FRACUNIT as f64) as fixed_t,
            y: (0.7f64 * FRACUNIT as f64) as fixed_t,
        },
        b: mpoint_t {
            x: (-0.5f64 * FRACUNIT as f64) as fixed_t,
            y: (-0.7f64 * FRACUNIT as f64) as fixed_t,
        },
    },
];

// ---------------------------------------------------------------------------
// Internal state
// ---------------------------------------------------------------------------

static mut cheating: c_int = 0;
static mut grid: c_int = 0;
static mut leveljuststarted: c_int = 1;

#[no_mangle]
pub static mut automapactive: c_int = 0;

static mut finit_width: c_int = SCREENWIDTH;
static mut finit_height: c_int = SCREENHEIGHT - 32;

static mut f_x: c_int = 0;
static mut f_y: c_int = 0;
static mut f_w: c_int = 0;
static mut f_h: c_int = 0;

static mut lightlev: c_int = 0;
static mut fb: *mut u8 = ptr::null_mut();
static mut amclock: c_int = 0;

static mut m_paninc: mpoint_t = mpoint_t { x: 0, y: 0 };
static mut mtof_zoommul: fixed_t = FRACUNIT;
static mut ftom_zoommul: fixed_t = FRACUNIT;

static mut m_x: fixed_t = 0;
static mut m_y: fixed_t = 0;
static mut m_x2: fixed_t = 0;
static mut m_y2: fixed_t = 0;
static mut m_w: fixed_t = 0;
static mut m_h: fixed_t = 0;

static mut min_x: fixed_t = 0;
static mut min_y: fixed_t = 0;
static mut max_x: fixed_t = 0;
static mut max_y: fixed_t = 0;
static mut max_w: fixed_t = 0;
static mut max_h: fixed_t = 0;

static mut min_w: fixed_t = 0;
static mut min_h: fixed_t = 0;

static mut min_scale_mtof: fixed_t = 0;
static mut max_scale_mtof: fixed_t = 0;

static mut old_m_w: fixed_t = 0;
static mut old_m_h: fixed_t = 0;
static mut old_m_x: fixed_t = 0;
static mut old_m_y: fixed_t = 0;

static mut f_oldloc: mpoint_t = mpoint_t { x: 0, y: 0 };

static mut scale_mtof: fixed_t = INITSCALEMTOF;
static mut scale_ftom: fixed_t = 0;

static mut plr: *mut PlayerT = ptr::null_mut();

static mut marknums: [*mut patch_t; AM_NUMMARKPOINTS] = [ptr::null_mut(); AM_NUMMARKPOINTS];
static mut markpoints: [mpoint_t; AM_NUMMARKPOINTS] = [mpoint_t { x: -1, y: -1 }; AM_NUMMARKPOINTS];
static mut markpointnum: c_int = 0;

static mut followplayer: c_int = 1;

#[no_mangle]
pub static mut cheat_amap: cheatseq_t = cheatseq_t {
    sequence: make_cheat_seq(b"iddt"),
    sequence_len: 4,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; 5],
};

const fn make_cheat_seq(seq: &[u8]) -> [c_char; 25] {
    let mut arr = [0i8; 25];
    let mut i = 0;
    while i < seq.len() {
        arr[i] = seq[i] as c_char;
        i += 1;
    }
    arr
}

static mut stopped: c_int = 1;

// ---------------------------------------------------------------------------
// Helper: FTOM / MTOF macros
// ---------------------------------------------------------------------------

#[inline(always)]
unsafe fn FTOM(x: c_int) -> fixed_t {
    FixedMul((x as fixed_t) << FRACBITS, scale_ftom)
}

#[inline(always)]
unsafe fn MTOF(x: fixed_t) -> c_int {
    (FixedMul(x, scale_mtof) >> FRACBITS) as c_int
}

#[inline(always)]
unsafe fn CXMTOF(x: fixed_t) -> c_int {
    f_x + MTOF(x - m_x)
}

#[inline(always)]
unsafe fn CYMTOF(y: fixed_t) -> c_int {
    f_y + (f_h - MTOF(y - m_y))
}

// ---------------------------------------------------------------------------
// Internal functions
// ---------------------------------------------------------------------------

unsafe fn AM_getIslope(ml: *mut mline_t, is: *mut islope_t) {
    let dy = (*ml).a.y - (*ml).b.y;
    let dx = (*ml).b.x - (*ml).a.x;
    if dy == 0 {
        (*is).islp = if dx < 0 { -c_int::MAX } else { c_int::MAX };
    } else {
        (*is).islp = FixedDiv(dx, dy);
    }
    if dx == 0 {
        (*is).slp = if dy < 0 { -c_int::MAX } else { c_int::MAX };
    } else {
        (*is).slp = FixedDiv(dy, dx);
    }
}

unsafe fn AM_activateNewScale() {
    m_x += m_w / 2;
    m_y += m_h / 2;
    m_w = FTOM(f_w);
    m_h = FTOM(f_h);
    m_x -= m_w / 2;
    m_y -= m_h / 2;
    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;
}

unsafe fn AM_saveScaleAndLoc() {
    old_m_x = m_x;
    old_m_y = m_y;
    old_m_w = m_w;
    old_m_h = m_h;
}

unsafe fn AM_restoreScaleAndLoc() {
    m_w = old_m_w;
    m_h = old_m_h;
    if followplayer == 0 {
        m_x = old_m_x;
        m_y = old_m_y;
    } else {
        m_x = (*((*plr).mo as *mut mobj_t)).x - m_w / 2;
        m_y = (*((*plr).mo as *mut mobj_t)).y - m_h / 2;
    }
    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;
    scale_mtof = FixedDiv((f_w as fixed_t) << FRACBITS, m_w);
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
}

unsafe fn AM_addMark() {
    markpoints[markpointnum as usize].x = m_x + m_w / 2;
    markpoints[markpointnum as usize].y = m_y + m_h / 2;
    markpointnum = (markpointnum + 1) % AM_NUMMARKPOINTS as c_int;
}

unsafe fn AM_findMinMaxBoundaries() {
    min_x = c_int::MAX;
    min_y = c_int::MAX;
    max_x = -c_int::MAX;
    max_y = -c_int::MAX;

    for i in 0..numvertexes {
        let v = &*vertexes.add(i as usize);
        if v.x < min_x {
            min_x = v.x;
        } else if v.x > max_x {
            max_x = v.x;
        }
        if v.y < min_y {
            min_y = v.y;
        } else if v.y > max_y {
            max_y = v.y;
        }
    }

    max_w = max_x - min_x;
    max_h = max_y - min_y;

    min_w = 2 * PLAYERRADIUS;
    min_h = 2 * PLAYERRADIUS;

    let a = FixedDiv((f_w as fixed_t) << FRACBITS, max_w);
    let b = FixedDiv((f_h as fixed_t) << FRACBITS, max_h);
    min_scale_mtof = if a < b { a } else { b };
    max_scale_mtof = FixedDiv((f_h as fixed_t) << FRACBITS, 2 * PLAYERRADIUS);
}

unsafe fn AM_changeWindowLoc() {
    if m_paninc.x != 0 || m_paninc.y != 0 {
        followplayer = 0;
        f_oldloc.x = c_int::MAX;
    }

    m_x += m_paninc.x;
    m_y += m_paninc.y;

    if m_x + m_w / 2 > max_x {
        m_x = max_x - m_w / 2;
    } else if m_x + m_w / 2 < min_x {
        m_x = min_x - m_w / 2;
    }

    if m_y + m_h / 2 > max_y {
        m_y = max_y - m_h / 2;
    } else if m_y + m_h / 2 < min_y {
        m_y = min_y - m_h / 2;
    }

    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;
}

unsafe fn AM_initVariables() {
    automapactive = 1;
    fb = I_VideoBuffer;

    f_oldloc.x = c_int::MAX;
    amclock = 0;
    lightlev = 0;

    m_paninc.x = 0;
    m_paninc.y = 0;
    ftom_zoommul = FRACUNIT;
    mtof_zoommul = FRACUNIT;

    m_w = FTOM(f_w);
    m_h = FTOM(f_h);

    if playeringame[consoleplayer as usize] != 0 {
        plr = players.as_mut_ptr().add(consoleplayer as usize);
    } else {
        plr = players.as_mut_ptr();
        for pnum in 0..MAXPLAYERS {
            if playeringame[pnum] != 0 {
                plr = players.as_mut_ptr().add(pnum);
                break;
            }
        }
    }

    m_x = (*((*plr).mo as *mut mobj_t)).x - m_w / 2;
    m_y = (*((*plr).mo as *mut mobj_t)).y - m_h / 2;
    AM_changeWindowLoc();

    old_m_x = m_x;
    old_m_y = m_y;
    old_m_w = m_w;
    old_m_h = m_h;

    let st_notify = event_t {
        type_: 1, // ev_keyup
        data1: AM_MSGENTERED,
        data2: 0,
        data3: 0,
        data4: 0,
    };
    ST_Responder(&st_notify as *const _ as *mut event_t);
}

unsafe fn AM_loadPics() {
    let mut namebuf: [c_char; 9] = [0; 9];
    for i in 0..10i32 {
        c_write!(namebuf, "AMMNUM{}", i);
        marknums[i as usize] = W_CacheLumpName(namebuf.as_mut_ptr(), PU_STATIC) as *mut patch_t;
    }
}

unsafe fn AM_unloadPics() {
    let mut namebuf: [c_char; 9] = [0; 9];
    for i in 0..10i32 {
        c_write!(namebuf, "AMMNUM{}", i);
        W_ReleaseLumpName(namebuf.as_mut_ptr());
    }
}

unsafe fn AM_clearMarks() {
    for i in 0..AM_NUMMARKPOINTS {
        markpoints[i].x = -1;
    }
    markpointnum = 0;
}

unsafe fn AM_LevelInit() {
    leveljuststarted = 0;

    f_x = 0;
    f_y = 0;
    f_w = finit_width;
    f_h = finit_height;

    AM_clearMarks();
    AM_findMinMaxBoundaries();

    scale_mtof = FixedDiv(min_scale_mtof, (0.7 * FRACUNIT as f64) as fixed_t);
    if scale_mtof > max_scale_mtof {
        scale_mtof = min_scale_mtof;
    }
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
}

// ---------------------------------------------------------------------------
// Public functions
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn AM_Stop() {
    let st_notify = event_t {
        type_: 0, // ev_keydown
        data1: 1, // ev_keyup
        data2: AM_MSGEXITED,
        data3: 0,
        data4: 0,
    };
    AM_unloadPics();
    automapactive = 0;
    ST_Responder(&st_notify as *const _ as *mut event_t);
    stopped = 1;
}

#[no_mangle]
pub unsafe extern "C" fn AM_Start() {
    static mut lastlevel: c_int = -1;
    static mut lastepisode: c_int = -1;

    if stopped == 0 {
        AM_Stop();
    }
    stopped = 0;
    if lastlevel != gamemap || lastepisode != gameepisode {
        AM_LevelInit();
        lastlevel = gamemap;
        lastepisode = gameepisode;
    }
    AM_initVariables();
    AM_loadPics();
}

unsafe fn AM_minOutWindowScale() {
    scale_mtof = min_scale_mtof;
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
    AM_activateNewScale();
}

unsafe fn AM_maxOutWindowScale() {
    scale_mtof = max_scale_mtof;
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
    AM_activateNewScale();
}

#[no_mangle]
pub unsafe extern "C" fn AM_Responder(ev: *mut event_t) -> c_int {
    let mut rc: c_int = 0;
    static mut bigstate: c_int = 0;

    if automapactive == 0 {
        if (*ev).type_ == 0 && (*ev).data1 == key_map_toggle {
            // ev_keydown
            AM_Start();
            viewactive = 0;
            rc = 1;
        }
    } else if (*ev).type_ == 0 {
        // ev_keydown
        rc = 1;
        let key = (*ev).data1;

        if key == key_map_east {
            if followplayer == 0 {
                m_paninc.x = FTOM(F_PANINC);
            } else {
                rc = 0;
            }
        } else if key == key_map_west {
            if followplayer == 0 {
                m_paninc.x = -FTOM(F_PANINC);
            } else {
                rc = 0;
            }
        } else if key == key_map_north {
            if followplayer == 0 {
                m_paninc.y = FTOM(F_PANINC);
            } else {
                rc = 0;
            }
        } else if key == key_map_south {
            if followplayer == 0 {
                m_paninc.y = -FTOM(F_PANINC);
            } else {
                rc = 0;
            }
        } else if key == key_map_zoomout {
            mtof_zoommul = M_ZOOMOUT;
            ftom_zoommul = M_ZOOMIN;
        } else if key == key_map_zoomin {
            mtof_zoommul = M_ZOOMIN;
            ftom_zoommul = M_ZOOMOUT;
        } else if key == key_map_toggle {
            bigstate = 0;
            viewactive = 1;
            AM_Stop();
        } else if key == key_map_maxzoom {
            bigstate = !bigstate;
            if bigstate != 0 {
                AM_saveScaleAndLoc();
                AM_minOutWindowScale();
            } else {
                AM_restoreScaleAndLoc();
            }
        } else if key == key_map_follow {
            followplayer = !followplayer;
            f_oldloc.x = c_int::MAX;
            if followplayer != 0 {
                (*plr).message = DEH_String(b"Follow Mode ON\0".as_ptr() as *mut c_char);
            } else {
                (*plr).message = DEH_String(b"Follow Mode OFF\0".as_ptr() as *mut c_char);
            }
        } else if key == key_map_grid {
            grid = !grid;
            if grid != 0 {
                (*plr).message = DEH_String(b"Grid ON\0".as_ptr() as *mut c_char);
            } else {
                (*plr).message = DEH_String(b"Grid OFF\0".as_ptr() as *mut c_char);
            }
        } else if key == key_map_mark {
            static mut AM_MARK_MSG: [c_char; 20] = [0; 20];
            c_write!(AM_MARK_MSG, "Marked Spot {}", markpointnum);
            (*plr).message = AM_MARK_MSG.as_mut_ptr();
            AM_addMark();
        } else if key == key_map_clearmark {
            AM_clearMarks();
            (*plr).message = DEH_String(b"All Marks Cleared\0".as_ptr() as *mut c_char);
        } else {
            rc = 0;
        }

        if deathmatch == 0 && cht_CheckCheat(&mut cheat_amap, (*ev).data2 as c_char) != 0 {
            rc = 0;
            cheating = (cheating + 1) % 3;
        }
    } else if (*ev).type_ == 1 {
        // ev_keyup
        rc = 0;
        let key = (*ev).data1;

        if key == key_map_east {
            if followplayer == 0 {
                m_paninc.x = 0;
            }
        } else if key == key_map_west {
            if followplayer == 0 {
                m_paninc.x = 0;
            }
        } else if key == key_map_north {
            if followplayer == 0 {
                m_paninc.y = 0;
            }
        } else if key == key_map_south {
            if followplayer == 0 {
                m_paninc.y = 0;
            }
        } else if key == key_map_zoomout || key == key_map_zoomin {
            mtof_zoommul = FRACUNIT;
            ftom_zoommul = FRACUNIT;
        }
    }

    rc
}

unsafe fn AM_changeWindowScale() {
    scale_mtof = FixedMul(scale_mtof, mtof_zoommul);
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);

    if scale_mtof < min_scale_mtof {
        AM_minOutWindowScale();
    } else if scale_mtof > max_scale_mtof {
        AM_maxOutWindowScale();
    } else {
        AM_activateNewScale();
    }
}

unsafe fn AM_doFollowPlayer() {
    if f_oldloc.x != (*((*plr).mo as *mut mobj_t)).x
        || f_oldloc.y != (*((*plr).mo as *mut mobj_t)).y
    {
        m_x = FTOM(MTOF((*((*plr).mo as *mut mobj_t)).x)) - m_w / 2;
        m_y = FTOM(MTOF((*((*plr).mo as *mut mobj_t)).y)) - m_h / 2;
        m_x2 = m_x + m_w;
        m_y2 = m_y + m_h;
        f_oldloc.x = (*((*plr).mo as *mut mobj_t)).x;
        f_oldloc.y = (*((*plr).mo as *mut mobj_t)).y;
    }
}

#[allow(dead_code)]
unsafe fn AM_updateLightLev() {
    static mut nexttic: c_int = 0;
    static LITELEVELS: [c_int; 8] = [0, 4, 7, 10, 12, 14, 15, 15];
    static mut litelevelscnt: c_int = 0;

    if amclock > nexttic {
        lightlev = LITELEVELS[litelevelscnt as usize];
        litelevelscnt += 1;
        if litelevelscnt == LITELEVELS.len() as c_int {
            litelevelscnt = 0;
        }
        nexttic = amclock + 6 - (amclock % 6);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AM_Ticker() {
    if automapactive == 0 {
        return;
    }

    amclock += 1;

    if followplayer != 0 {
        AM_doFollowPlayer();
    }

    if ftom_zoommul != FRACUNIT {
        AM_changeWindowScale();
    }

    if m_paninc.x != 0 || m_paninc.y != 0 {
        AM_changeWindowLoc();
    }

    // AM_updateLightLev();
}

unsafe fn AM_clearFB(color: c_int) {
    std::ptr::write_bytes(fb, color as u8, (f_w * f_h) as usize);
}

// Cohen-Sutherland outcode constants.
const OC_LEFT: c_int = 1;
const OC_RIGHT: c_int = 2;
const OC_BOTTOM: c_int = 4;
const OC_TOP: c_int = 8;

#[inline(always)]
unsafe fn dooutcode(mx: c_int, my: c_int) -> c_int {
    let mut oc = 0;
    if my < 0 {
        oc |= OC_TOP;
    } else if my >= f_h {
        oc |= OC_BOTTOM;
    }
    if mx < 0 {
        oc |= OC_LEFT;
    } else if mx >= f_w {
        oc |= OC_RIGHT;
    }
    oc
}

unsafe fn AM_clipMline(ml: *mut mline_t, fl: *mut fline_t) -> c_int {
    let mut outcode1: c_int = 0;
    let mut outcode2: c_int = 0;
    let mut outside: c_int;
    let mut tmp: fpoint_t = fpoint_t { x: 0, y: 0 };
    let mut dx: c_int;
    let mut dy: c_int;

    // Trivial rejects and outcodes in map coords.
    if (*ml).a.y > m_y2 {
        outcode1 = OC_TOP;
    } else if (*ml).a.y < m_y {
        outcode1 = OC_BOTTOM;
    }
    if (*ml).b.y > m_y2 {
        outcode2 = OC_TOP;
    } else if (*ml).b.y < m_y {
        outcode2 = OC_BOTTOM;
    }
    if outcode1 & outcode2 != 0 {
        return 0;
    }

    if (*ml).a.x < m_x {
        outcode1 |= OC_LEFT;
    } else if (*ml).a.x > m_x2 {
        outcode1 |= OC_RIGHT;
    }
    if (*ml).b.x < m_x {
        outcode2 |= OC_LEFT;
    } else if (*ml).b.x > m_x2 {
        outcode2 |= OC_RIGHT;
    }
    if outcode1 & outcode2 != 0 {
        return 0;
    }

    // Transform to frame-buffer coordinates.
    (*fl).a.x = CXMTOF((*ml).a.x);
    (*fl).a.y = CYMTOF((*ml).a.y);
    (*fl).b.x = CXMTOF((*ml).b.x);
    (*fl).b.y = CYMTOF((*ml).b.y);

    outcode1 = dooutcode((*fl).a.x, (*fl).a.y);
    outcode2 = dooutcode((*fl).b.x, (*fl).b.y);

    if outcode1 & outcode2 != 0 {
        return 0;
    }

    while outcode1 | outcode2 != 0 {
        if outcode1 != 0 {
            outside = outcode1;
        } else {
            outside = outcode2;
        }

        if outside & OC_TOP != 0 {
            dy = (*fl).a.y - (*fl).b.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.x = (*fl).a.x + (dx * (*fl).a.y) / dy;
            tmp.y = 0;
        } else if outside & OC_BOTTOM != 0 {
            dy = (*fl).a.y - (*fl).b.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.x = (*fl).a.x + (dx * ((*fl).a.y - f_h)) / dy;
            tmp.y = f_h - 1;
        } else if outside & OC_RIGHT != 0 {
            dy = (*fl).b.y - (*fl).a.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.y = (*fl).a.y + (dy * (f_w - 1 - (*fl).a.x)) / dx;
            tmp.x = f_w - 1;
        } else if outside & OC_LEFT != 0 {
            dy = (*fl).b.y - (*fl).a.y;
            dx = (*fl).b.x - (*fl).a.x;
            tmp.y = (*fl).a.y + (dy * (-(*fl).a.x)) / dx;
            tmp.x = 0;
        } else {
            tmp.x = 0;
            tmp.y = 0;
        }

        if outside == outcode1 {
            (*fl).a = tmp;
            outcode1 = dooutcode((*fl).a.x, (*fl).a.y);
        } else {
            (*fl).b = tmp;
            outcode2 = dooutcode((*fl).b.x, (*fl).b.y);
        }

        if outcode1 & outcode2 != 0 {
            return 0;
        }
    }

    1
}

unsafe fn AM_drawFline(fl: *mut fline_t, color: c_int) {
    static mut fuck: c_int = 0;

    if (*fl).a.x < 0
        || (*fl).a.x >= f_w
        || (*fl).a.y < 0
        || (*fl).a.y >= f_h
        || (*fl).b.x < 0
        || (*fl).b.x >= f_w
        || (*fl).b.y < 0
        || (*fl).b.y >= f_h
    {
        // For debugging only — matches C behaviour of returning without drawing.
        fuck += 1;
        return;
    }

    let dx = (*fl).b.x - (*fl).a.x;
    let ax = 2 * (if dx < 0 { -dx } else { dx });
    let sx = if dx < 0 { -1 } else { 1 };

    let dy = (*fl).b.y - (*fl).a.y;
    let ay = 2 * (if dy < 0 { -dy } else { dy });
    let sy = if dy < 0 { -1 } else { 1 };

    let mut x = (*fl).a.x;
    let mut y = (*fl).a.y;

    if ax > ay {
        let mut d = ay - ax / 2;
        loop {
            *fb.add((y * f_w + x) as usize) = color as u8;
            if x == (*fl).b.x {
                return;
            }
            if d >= 0 {
                y += sy;
                d -= ax;
            }
            x += sx;
            d += ay;
        }
    } else {
        let mut d = ax - ay / 2;
        loop {
            *fb.add((y * f_w + x) as usize) = color as u8;
            if y == (*fl).b.y {
                return;
            }
            if d >= 0 {
                x += sx;
                d -= ay;
            }
            y += sy;
            d += ax;
        }
    }
}

unsafe fn AM_drawMline(ml: *mut mline_t, color: c_int) {
    static mut fl: fline_t = fline_t {
        a: fpoint_t { x: 0, y: 0 },
        b: fpoint_t { x: 0, y: 0 },
    };
    if AM_clipMline(ml, &mut fl) != 0 {
        AM_drawFline(&mut fl, color);
    }
}

unsafe fn AM_drawGrid(color: c_int) {
    let mut start: fixed_t;
    let mut end: fixed_t;
    let mut ml: mline_t = mline_t {
        a: mpoint_t { x: 0, y: 0 },
        b: mpoint_t { x: 0, y: 0 },
    };

    // Vertical gridlines.
    start = m_x;
    if (start - bmaporgx) % (MAPBLOCKUNITS << FRACBITS) != 0 {
        start += (MAPBLOCKUNITS << FRACBITS) - ((start - bmaporgx) % (MAPBLOCKUNITS << FRACBITS));
    }
    end = m_x + m_w;

    ml.a.y = m_y;
    ml.b.y = m_y + m_h;
    let mut x = start;
    while x < end {
        ml.a.x = x;
        ml.b.x = x;
        AM_drawMline(&mut ml, color);
        x += MAPBLOCKUNITS << FRACBITS;
    }

    // Horizontal gridlines.
    start = m_y;
    if (start - bmaporgy) % (MAPBLOCKUNITS << FRACBITS) != 0 {
        start += (MAPBLOCKUNITS << FRACBITS) - ((start - bmaporgy) % (MAPBLOCKUNITS << FRACBITS));
    }
    end = m_y + m_h;

    ml.a.x = m_x;
    ml.b.x = m_x + m_w;
    let mut y = start;
    while y < end {
        ml.a.y = y;
        ml.b.y = y;
        AM_drawMline(&mut ml, color);
        y += MAPBLOCKUNITS << FRACBITS;
    }
}

unsafe fn AM_drawWalls() {
    static mut l: mline_t = mline_t {
        a: mpoint_t { x: 0, y: 0 },
        b: mpoint_t { x: 0, y: 0 },
    };

    for i in 0..numlines {
        let li = &*lines.add(i as usize);
        l.a.x = (*li.v1).x;
        l.a.y = (*li.v1).y;
        l.b.x = (*li.v2).x;
        l.b.y = (*li.v2).y;

        let flags = li.flags as c_int;

        if cheating != 0 || (flags & ML_MAPPED as c_int) != 0 {
            if (flags & ML_DONTDRAW as c_int) != 0 && cheating == 0 {
                continue;
            }
            if li.backsector.is_null() {
                AM_drawMline(&mut l, WALLCOLORS + lightlev);
            } else {
                let back = li.backsector as *mut sector_t;
                let front = li.frontsector as *mut sector_t;
                if li.special == 39 {
                    // teleporters
                    AM_drawMline(&mut l, WALLCOLORS + WALLRANGE / 2);
                } else if (flags & ML_SECRET as c_int) != 0 {
                    if cheating != 0 {
                        AM_drawMline(&mut l, SECRETWALLCOLORS + lightlev);
                    } else {
                        AM_drawMline(&mut l, WALLCOLORS + lightlev);
                    }
                } else if (*back).floorheight != (*front).floorheight {
                    AM_drawMline(&mut l, FDWALLCOLORS + lightlev);
                } else if (*back).ceilingheight != (*front).ceilingheight {
                    AM_drawMline(&mut l, CDWALLCOLORS + lightlev);
                } else if cheating != 0 {
                    AM_drawMline(&mut l, TSWALLCOLORS + lightlev);
                }
            }
        } else if (*plr).powers[4] != 0 {
            // pw_allmap
            if (flags & ML_DONTDRAW as c_int) == 0 {
                AM_drawMline(&mut l, GRAYS + 3);
            }
        }
    }
}

unsafe fn AM_rotate(x: *mut fixed_t, y: *mut fixed_t, a: c_uint) {
    let tmpx = FixedMul(*x, *finecosine.0.add((a >> ANGLETOFINESHIFT) as usize))
        - FixedMul(*y, finesine[(a >> ANGLETOFINESHIFT) as usize]);
    *y = FixedMul(*x, finesine[(a >> ANGLETOFINESHIFT) as usize])
        + FixedMul(*y, *finecosine.0.add((a >> ANGLETOFINESHIFT) as usize));
    *x = tmpx;
}

unsafe fn AM_drawLineCharacter(
    lineguy: *mut mline_t,
    lineguylines: c_int,
    scale: fixed_t,
    angle: c_uint,
    color: c_int,
    x: fixed_t,
    y: fixed_t,
) {
    let mut l: mline_t = mline_t {
        a: mpoint_t { x: 0, y: 0 },
        b: mpoint_t { x: 0, y: 0 },
    };

    for i in 0..lineguylines {
        l.a.x = (*lineguy.add(i as usize)).a.x;
        l.a.y = (*lineguy.add(i as usize)).a.y;

        if scale != 0 {
            l.a.x = FixedMul(scale, l.a.x);
            l.a.y = FixedMul(scale, l.a.y);
        }

        if angle != 0 {
            AM_rotate(&mut l.a.x, &mut l.a.y, angle);
        }

        l.a.x += x;
        l.a.y += y;

        l.b.x = (*lineguy.add(i as usize)).b.x;
        l.b.y = (*lineguy.add(i as usize)).b.y;

        if scale != 0 {
            l.b.x = FixedMul(scale, l.b.x);
            l.b.y = FixedMul(scale, l.b.y);
        }

        if angle != 0 {
            AM_rotate(&mut l.b.x, &mut l.b.y, angle);
        }

        l.b.x += x;
        l.b.y += y;

        AM_drawMline(&mut l, color);
    }
}

unsafe fn AM_drawPlayers() {
    let their_colors: [c_int; 4] = [GREENS, GRAYS, BROWNS, REDS];
    let mut their_color: c_int = -1;
    let mut color: c_int;

    if netgame == 0 {
        if cheating != 0 {
            AM_drawLineCharacter(
                CHEAT_ARROW.as_ptr() as *mut mline_t,
                CHEAT_ARROW.len() as c_int,
                0,
                (*((*plr).mo as *mut mobj_t)).angle,
                WHITE,
                (*((*plr).mo as *mut mobj_t)).x,
                (*((*plr).mo as *mut mobj_t)).y,
            );
        } else {
            AM_drawLineCharacter(
                PLAYER_ARROW.as_ptr() as *mut mline_t,
                PLAYER_ARROW.len() as c_int,
                0,
                (*((*plr).mo as *mut mobj_t)).angle,
                WHITE,
                (*((*plr).mo as *mut mobj_t)).x,
                (*((*plr).mo as *mut mobj_t)).y,
            );
        }
        return;
    }

    for i in 0..MAXPLAYERS {
        their_color += 1;
        let p = players.as_mut_ptr().add(i);

        if deathmatch != 0 && singledemo == 0 && p != plr {
            continue;
        }

        if playeringame[i] == 0 {
            continue;
        }

        if (*p).powers[2] != 0 {
            // pw_invisibility
            color = 246;
        } else {
            color = their_colors[their_color as usize];
        }

        AM_drawLineCharacter(
            PLAYER_ARROW.as_ptr() as *mut mline_t,
            PLAYER_ARROW.len() as c_int,
            0,
            (*((*p).mo as *mut mobj_t)).angle,
            color,
            (*((*p).mo as *mut mobj_t)).x,
            (*((*p).mo as *mut mobj_t)).y,
        );
    }
}

unsafe fn AM_drawThings(colors: c_int, _colorrange: c_int) {
    for i in 0..numsectors {
        let mut t = (*sectors.add(i as usize)).thinglist as *mut mobj_t;
        while !t.is_null() {
            AM_drawLineCharacter(
                THINTRIANGLE_GUY.as_ptr() as *mut mline_t,
                THINTRIANGLE_GUY.len() as c_int,
                16 << FRACBITS,
                (*t).angle,
                colors + lightlev,
                (*t).x,
                (*t).y,
            );
            t = (*t).snext as *mut mobj_t;
        }
    }
}

unsafe fn AM_drawMarks() {
    for i in 0..AM_NUMMARKPOINTS {
        if markpoints[i].x != -1 {
            let w = 5;
            let h = 6;
            let fx = CXMTOF(markpoints[i].x);
            let fy = CYMTOF(markpoints[i].y);
            if fx >= f_x && fx <= f_w - w && fy >= f_y && fy <= f_h - h {
                V_DrawPatch(fx, fy, marknums[i]);
            }
        }
    }
}

unsafe fn AM_drawCrosshair(color: c_int) {
    *fb.add(((f_w * (f_h + 1)) / 2) as usize) = color as u8;
}

#[no_mangle]
pub unsafe extern "C" fn AM_Drawer() {
    if automapactive == 0 {
        return;
    }

    AM_clearFB(BACKGROUND);
    if grid != 0 {
        AM_drawGrid(GRIDCOLORS);
    }
    AM_drawWalls();
    AM_drawPlayers();
    if cheating == 2 {
        AM_drawThings(THINGCOLORS, THINGRANGE);
    }
    AM_drawCrosshair(XHAIRCOLORS);
    AM_drawMarks();

    V_MarkRect(f_x, f_y, f_w, f_h);
}

// ---------------------------------------------------------------------------
// Link anchor — ensures symbols are not dropped by the linker.
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn AM_Map_Link_Anchor() {
    AM_Responder(ptr::null_mut());
    AM_Ticker();
    AM_Drawer();
    AM_Stop();
}
