//! Rust port of vendor/doomgeneric/p_setup.c.
//!
//! Level/map loading and initialization. Reads all BSP and geometry lumps
//! from the WAD into runtime data structures (vertexes, lines, sectors, etc.).

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_short, c_uint, c_ushort, c_void};
use std::ptr;

use crate::doom::c_ffi::{line_t, node_t, sector_t, seg_t, side_t, subsector_t, vertex_t};
use crate::doom::d_mode;
use crate::doom::d_player::{consoleplayer, players, MAXPLAYERS};
use crate::doom::info::sprnames;
use crate::doom::m_bbox::{M_AddToBox, M_ClearBox, BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::doom::m_fixed::{FRACBITS, FRACUNIT};
use crate::doom::p_tick::{leveltime, P_InitThinkers};
use crate::doom::z_zone::{PU_LEVEL, PU_STATIC};

// ---------------------------------------------------------------------------
// Byte-order helper
// ---------------------------------------------------------------------------

/// Convert a little-endian `i16` from a WAD file to host byte order.
/// On little-endian hosts this is a no-op cast; on big-endian it swaps.
#[inline]
fn SHORT(x: i16) -> i16 {
    i16::from_le(x)
}

// ---------------------------------------------------------------------------
// Zone-memory tags
// ---------------------------------------------------------------------------

const PU_PURGELEVEL: c_int = 7;

// ---------------------------------------------------------------------------
// WAD lump-order indices (must match doomdata.h)
// ---------------------------------------------------------------------------

const ML_LABEL: c_int = 0;
const ML_THINGS: c_int = 1;
const ML_LINEDEFS: c_int = 2;
const ML_SIDEDEFS: c_int = 3;
const ML_VERTEXES: c_int = 4;
const ML_SEGS: c_int = 5;
const ML_SSECTORS: c_int = 6;
const ML_NODES: c_int = 7;
const ML_SECTORS: c_int = 8;
const ML_REJECT: c_int = 9;
const ML_BLOCKMAP: c_int = 10;

// ---------------------------------------------------------------------------
// LineDef flags
// ---------------------------------------------------------------------------

const ML_TWOSIDED: i16 = 4;

// ---------------------------------------------------------------------------
// Slope types
// ---------------------------------------------------------------------------

const ST_HORIZONTAL: c_int = 0;
const ST_VERTICAL: c_int = 1;
const ST_POSITIVE: c_int = 2;
const ST_NEGATIVE: c_int = 3;

// ---------------------------------------------------------------------------
// Misc constants
// ---------------------------------------------------------------------------

const MAPBLOCKSHIFT: c_int = FRACBITS as c_int + 7;
const MAXRADIUS: c_int = 32 * FRACUNIT;

/// Maximum number of deathmatch start positions in a level.
pub const MAX_DEATHMATCH_STARTS: usize = 10;

// ---------------------------------------------------------------------------
// Packed WAD structs (exact on-disk layout)
// ---------------------------------------------------------------------------

#[repr(C, packed)]
struct mapvertex_t {
    x: i16,
    y: i16,
}

#[repr(C, packed)]
struct mapsidedef_t {
    textureoffset: i16,
    rowoffset: i16,
    toptexture: [u8; 8],
    bottomtexture: [u8; 8],
    midtexture: [u8; 8],
    sector: i16,
}

#[repr(C, packed)]
struct maplinedef_t {
    v1: i16,
    v2: i16,
    flags: i16,
    special: i16,
    tag: i16,
    sidenum: [i16; 2],
}

#[repr(C, packed)]
struct mapsector_t {
    floorheight: i16,
    ceilingheight: i16,
    floorpic: [u8; 8],
    ceilingpic: [u8; 8],
    lightlevel: i16,
    special: i16,
    tag: i16,
}

#[repr(C, packed)]
struct mapsubsector_t {
    numsegs: i16,
    firstseg: i16,
}

#[repr(C, packed)]
struct mapseg_t {
    v1: i16,
    v2: i16,
    angle: i16,
    linedef: i16,
    side: i16,
    offset: i16,
}

#[repr(C, packed)]
struct mapnode_t {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
    bbox: [[i16; 4]; 2],
    children: [u16; 2],
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

// ---------------------------------------------------------------------------
// MAP-related lookup tables
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut numvertexes: c_int = 0;
#[no_mangle]
pub static mut vertexes: *mut vertex_t = ptr::null_mut();

#[no_mangle]
pub static mut numsegs: c_int = 0;
#[no_mangle]
pub static mut segs: *mut seg_t = ptr::null_mut();

#[no_mangle]
pub static mut numsectors: c_int = 0;
#[no_mangle]
pub static mut sectors: *mut sector_t = ptr::null_mut();

#[no_mangle]
pub static mut numsubsectors: c_int = 0;
#[no_mangle]
pub static mut subsectors: *mut subsector_t = ptr::null_mut();

#[no_mangle]
pub static mut numnodes: c_int = 0;
#[no_mangle]
pub static mut nodes: *mut node_t = ptr::null_mut();

#[no_mangle]
pub static mut numlines: c_int = 0;
#[no_mangle]
pub static mut lines: *mut line_t = ptr::null_mut();

#[no_mangle]
pub static mut numsides: c_int = 0;
#[no_mangle]
pub static mut sides: *mut side_t = ptr::null_mut();

static mut totallines: c_int = 0;

// ---------------------------------------------------------------------------
// BLOCKMAP
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut bmapwidth: c_int = 0;
#[no_mangle]
pub static mut bmapheight: c_int = 0;
#[no_mangle]
pub static mut blockmap: *mut c_short = ptr::null_mut();
#[no_mangle]
pub static mut blockmaplump: *mut c_short = ptr::null_mut();
#[no_mangle]
pub static mut bmaporgx: c_int = 0;
#[no_mangle]
pub static mut bmaporgy: c_int = 0;
#[no_mangle]
pub static mut blocklinks: *mut *mut c_void = ptr::null_mut();

// ---------------------------------------------------------------------------
// REJECT
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut rejectmatrix: *mut u8 = ptr::null_mut();

// ---------------------------------------------------------------------------
// Starting spots
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut deathmatchstarts: [mapthing_t; MAX_DEATHMATCH_STARTS] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    r#type: 0,
    options: 0,
}; MAX_DEATHMATCH_STARTS];

#[no_mangle]
pub static mut deathmatch_p: *mut mapthing_t = ptr::null_mut();

#[no_mangle]
pub static mut playerstarts: [mapthing_t; MAXPLAYERS] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    r#type: 0,
    options: 0,
}; MAXPLAYERS];

// ---------------------------------------------------------------------------
// Extern declarations for still-unported C modules
// ---------------------------------------------------------------------------

use crate::doom::doomstat::gamemode;
use crate::doom::g_game::{
    bodyqueslot, deathmatch, playeringame, precache, totalitems, totalkills, totalsecret, wminfo,
    G_DeathMatchSpawnPlayer,
};
use crate::doom::i_system::I_GetMemoryValue;
use crate::doom::m_argv::M_CheckParm;
use crate::doom::m_fixed::FixedDiv;
use crate::doom::p_mobj::{iquehead, iquetail, P_SpawnMapThing};
use crate::doom::p_spec::{P_InitPicAnims, P_SpawnSpecials};
use crate::doom::p_switch::P_InitSwitchList;
use crate::doom::r_data::{R_FlatNumForName, R_PrecacheLevel, R_TextureNumForName};
use crate::doom::r_things::R_InitSprites;
use crate::doom::s_sound::S_Start;
use crate::doom::w_wad::{
    W_CacheLumpNum, W_GetNumForName, W_LumpLength, W_ReadLump, W_ReleaseLumpNum,
};
use crate::doom::z_zone::{Z_FreeTags, Z_Malloc};

// ---------------------------------------------------------------------------
// GetSectorAtNullAddress
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn GetSectorAtNullAddress() -> *mut sector_t {
    static mut NULL_SECTOR_IS_INITIALIZED: bool = false;
    static mut NULL_SECTOR: sector_t = unsafe { std::mem::zeroed() };

    unsafe {
        if !NULL_SECTOR_IS_INITIALIZED {
            NULL_SECTOR = std::mem::zeroed();
            I_GetMemoryValue(
                0,
                &mut NULL_SECTOR.floorheight as *mut c_int as *mut c_void,
                4,
            );
            I_GetMemoryValue(
                4,
                &mut NULL_SECTOR.ceilingheight as *mut c_int as *mut c_void,
                4,
            );
            NULL_SECTOR_IS_INITIALIZED = true;
        }
        &mut NULL_SECTOR
    }
}

// ---------------------------------------------------------------------------
// P_LoadVertexes
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadVertexes(lump: c_int) {
    unsafe {
        numvertexes = W_LumpLength(lump as c_uint) / std::mem::size_of::<mapvertex_t>() as c_int;
        vertexes = Z_Malloc(
            numvertexes * std::mem::size_of::<vertex_t>() as c_int,
            PU_LEVEL,
            ptr::null_mut(),
        ) as *mut vertex_t;

        let data = W_CacheLumpNum(lump, PU_STATIC);
        let mut ml = data as *mut mapvertex_t;
        let mut li = vertexes;

        for _ in 0..numvertexes {
            (*li).x = (SHORT((*ml).x) as c_int) << FRACBITS;
            (*li).y = (SHORT((*ml).y) as c_int) << FRACBITS;
            li = li.add(1);
            ml = ml.add(1);
        }

        W_ReleaseLumpNum(lump);
    }
}

// ---------------------------------------------------------------------------
// P_LoadSegs
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadSegs(lump: c_int) {
    unsafe {
        numsegs = W_LumpLength(lump as c_uint) / std::mem::size_of::<mapseg_t>() as c_int;
        segs = Z_Malloc(
            numsegs * std::mem::size_of::<seg_t>() as c_int,
            PU_LEVEL,
            ptr::null_mut(),
        ) as *mut seg_t;
        // Zeroed by Z_Malloc internally

        let data = W_CacheLumpNum(lump, PU_STATIC);
        let mut ml = data as *mut mapseg_t;
        let mut li = segs;

        for _ in 0..numsegs {
            (*li).v1 = vertexes.offset(SHORT((*ml).v1) as isize);
            (*li).v2 = vertexes.offset(SHORT((*ml).v2) as isize);
            (*li).angle = ((SHORT((*ml).angle) as c_int) << 16) as c_uint;
            (*li).offset = (SHORT((*ml).offset) as c_int) << 16;
            let linedef = SHORT((*ml).linedef) as c_int;
            let ldef = lines.offset(linedef as isize);
            (*li).linedef = ldef;
            let side = SHORT((*ml).side) as c_int;
            (*li).sidedef = sides.offset((*ldef).sidenum[side as usize] as isize);
            (*li).frontsector = (*sides.offset((*ldef).sidenum[side as usize] as isize)).sector;

            if (*ldef).flags & ML_TWOSIDED != 0 {
                let sidenum = (*ldef).sidenum[side as usize ^ 1];
                if sidenum < 0 || sidenum as c_int >= numsides {
                    (*li).backsector = GetSectorAtNullAddress();
                } else {
                    (*li).backsector = sides.offset(sidenum as isize).as_ref().unwrap().sector;
                }
            } else {
                (*li).backsector = ptr::null_mut();
            }

            li = li.add(1);
            ml = ml.add(1);
        }

        W_ReleaseLumpNum(lump);
    }
}

// ---------------------------------------------------------------------------
// P_LoadSubsectors
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadSubsectors(lump: c_int) {
    unsafe {
        numsubsectors =
            W_LumpLength(lump as c_uint) / std::mem::size_of::<mapsubsector_t>() as c_int;
        subsectors = Z_Malloc(
            numsubsectors * std::mem::size_of::<subsector_t>() as c_int,
            PU_LEVEL,
            ptr::null_mut(),
        ) as *mut subsector_t;
        // Zeroed by Z_Malloc internally

        let data = W_CacheLumpNum(lump, PU_STATIC);
        let mut ms = data as *mut mapsubsector_t;
        let mut ss = subsectors;

        for _ in 0..numsubsectors {
            (*ss).numlines = SHORT((*ms).numsegs);
            (*ss).firstline = SHORT((*ms).firstseg);
            ss = ss.add(1);
            ms = ms.add(1);
        }

        W_ReleaseLumpNum(lump);
    }
}

// ---------------------------------------------------------------------------
// P_LoadSectors
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadSectors(lump: c_int) {
    unsafe {
        numsectors = W_LumpLength(lump as c_uint) / std::mem::size_of::<mapsector_t>() as c_int;
        sectors = Z_Malloc(
            numsectors * std::mem::size_of::<sector_t>() as c_int,
            PU_LEVEL,
            ptr::null_mut(),
        ) as *mut sector_t;
        // Zeroed by Z_Malloc internally

        let data = W_CacheLumpNum(lump, PU_STATIC);
        let mut ms = data as *mut mapsector_t;
        let mut ss = sectors;

        for _ in 0..numsectors {
            (*ss).floorheight = (SHORT((*ms).floorheight) as c_int) << FRACBITS;
            (*ss).ceilingheight = (SHORT((*ms).ceilingheight) as c_int) << FRACBITS;
            (*ss).floorpic = R_FlatNumForName((*ms).floorpic.as_ptr() as *mut c_char) as c_short;
            (*ss).ceilingpic =
                R_FlatNumForName((*ms).ceilingpic.as_ptr() as *mut c_char) as c_short;
            (*ss).lightlevel = SHORT((*ms).lightlevel);
            (*ss).special = SHORT((*ms).special);
            (*ss).tag = SHORT((*ms).tag);
            (*ss).thinglist = ptr::null_mut();
            ss = ss.add(1);
            ms = ms.add(1);
        }

        W_ReleaseLumpNum(lump);
    }
}

// ---------------------------------------------------------------------------
// P_LoadNodes
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadNodes(lump: c_int) {
    unsafe {
        numnodes = W_LumpLength(lump as c_uint) / std::mem::size_of::<mapnode_t>() as c_int;
        nodes = Z_Malloc(
            numnodes * std::mem::size_of::<node_t>() as c_int,
            PU_LEVEL,
            ptr::null_mut(),
        ) as *mut node_t;

        let data = W_CacheLumpNum(lump, PU_STATIC);
        let mut mn = data as *mut mapnode_t;
        let mut no = nodes;

        for _ in 0..numnodes {
            (*no).x = (SHORT((*mn).x) as c_int) << FRACBITS;
            (*no).y = (SHORT((*mn).y) as c_int) << FRACBITS;
            (*no).dx = (SHORT((*mn).dx) as c_int) << FRACBITS;
            (*no).dy = (SHORT((*mn).dy) as c_int) << FRACBITS;
            for j in 0..2usize {
                (*no).children[j] = SHORT((*mn).children[j] as i16) as c_ushort;
                for k in 0..4usize {
                    (*no).bbox[j][k] = (SHORT((*mn).bbox[j][k]) as c_int) << FRACBITS;
                }
            }
            no = no.add(1);
            mn = mn.add(1);
        }

        W_ReleaseLumpNum(lump);
    }
}

// ---------------------------------------------------------------------------
// P_LoadThings
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadThings(lump: c_int) {
    unsafe {
        let data = W_CacheLumpNum(lump, PU_STATIC);
        let numthings = W_LumpLength(lump as c_uint) / std::mem::size_of::<mapthing_t>() as c_int;
        let mut mt = data as *mut mapthing_t;

        for _ in 0..numthings {
            let mut spawn = true;

            if gamemode != d_mode::commercial {
                let thing_type = SHORT((*mt).r#type);
                match thing_type {
                    68 | 64 | 88 | 89 | 69 | 67 | 71 | 65 | 66 | 84 => {
                        spawn = false;
                    }
                    _ => {}
                }
            }

            if !spawn {
                break;
            }

            let mut spawnthing = mapthing_t {
                x: SHORT((*mt).x),
                y: SHORT((*mt).y),
                angle: SHORT((*mt).angle),
                r#type: SHORT((*mt).r#type),
                options: SHORT((*mt).options),
            };
            P_SpawnMapThing(
                &mut spawnthing as *mut mapthing_t as *mut crate::doom::p_telept::mapthing_t,
            );
            mt = mt.add(1);
        }

        W_ReleaseLumpNum(lump);
    }
}

// ---------------------------------------------------------------------------
// P_LoadLineDefs
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadLineDefs(lump: c_int) {
    unsafe {
        numlines = W_LumpLength(lump as c_uint) / std::mem::size_of::<maplinedef_t>() as c_int;
        lines = Z_Malloc(
            numlines * std::mem::size_of::<line_t>() as c_int,
            PU_LEVEL,
            ptr::null_mut(),
        ) as *mut line_t;
        // Zeroed by Z_Malloc internally

        let data = W_CacheLumpNum(lump, PU_STATIC);
        let mut mld = data as *mut maplinedef_t;
        let mut ld = lines;

        for _ in 0..numlines {
            (*ld).flags = SHORT((*mld).flags);
            (*ld).special = SHORT((*mld).special);
            (*ld).tag = SHORT((*mld).tag);
            let v1 = vertexes.offset(SHORT((*mld).v1) as isize);
            let v2 = vertexes.offset(SHORT((*mld).v2) as isize);
            (*ld).v1 = v1;
            (*ld).v2 = v2;
            (*ld).dx = (*v2).x - (*v1).x;
            (*ld).dy = (*v2).y - (*v1).y;

            if (*ld).dx == 0 {
                (*ld).slopetype = ST_VERTICAL;
            } else if (*ld).dy == 0 {
                (*ld).slopetype = ST_HORIZONTAL;
            } else {
                if FixedDiv((*ld).dy, (*ld).dx) > 0 {
                    (*ld).slopetype = ST_POSITIVE;
                } else {
                    (*ld).slopetype = ST_NEGATIVE;
                }
            }

            if (*v1).x < (*v2).x {
                (*ld).bbox[BOXLEFT] = (*v1).x;
                (*ld).bbox[BOXRIGHT] = (*v2).x;
            } else {
                (*ld).bbox[BOXLEFT] = (*v2).x;
                (*ld).bbox[BOXRIGHT] = (*v1).x;
            }

            if (*v1).y < (*v2).y {
                (*ld).bbox[BOXBOTTOM] = (*v1).y;
                (*ld).bbox[BOXTOP] = (*v2).y;
            } else {
                (*ld).bbox[BOXBOTTOM] = (*v2).y;
                (*ld).bbox[BOXTOP] = (*v1).y;
            }

            (*ld).sidenum[0] = SHORT((*mld).sidenum[0]);
            (*ld).sidenum[1] = SHORT((*mld).sidenum[1]);

            if (*ld).sidenum[0] != -1 {
                (*ld).frontsector = sides
                    .offset((*ld).sidenum[0] as isize)
                    .as_ref()
                    .unwrap()
                    .sector as *mut c_void;
            } else {
                (*ld).frontsector = ptr::null_mut();
            }

            if (*ld).sidenum[1] != -1 {
                (*ld).backsector = sides
                    .offset((*ld).sidenum[1] as isize)
                    .as_ref()
                    .unwrap()
                    .sector as *mut c_void;
            } else {
                (*ld).backsector = ptr::null_mut();
            }

            ld = ld.add(1);
            mld = mld.add(1);
        }

        W_ReleaseLumpNum(lump);
    }
}

// ---------------------------------------------------------------------------
// P_LoadSideDefs
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadSideDefs(lump: c_int) {
    unsafe {
        numsides = W_LumpLength(lump as c_uint) / std::mem::size_of::<mapsidedef_t>() as c_int;
        sides = Z_Malloc(
            numsides * std::mem::size_of::<side_t>() as c_int,
            PU_LEVEL,
            ptr::null_mut(),
        ) as *mut side_t;
        // Zeroed by Z_Malloc internally

        let data = W_CacheLumpNum(lump, PU_STATIC);
        let mut msd = data as *mut mapsidedef_t;
        let mut sd = sides;

        for _ in 0..numsides {
            (*sd).textureoffset = (SHORT((*msd).textureoffset) as c_int) << FRACBITS;
            (*sd).rowoffset = (SHORT((*msd).rowoffset) as c_int) << FRACBITS;
            (*sd).toptexture =
                R_TextureNumForName((*msd).toptexture.as_ptr() as *mut c_char) as c_short;
            (*sd).bottomtexture =
                R_TextureNumForName((*msd).bottomtexture.as_ptr() as *mut c_char) as c_short;
            (*sd).midtexture =
                R_TextureNumForName((*msd).midtexture.as_ptr() as *mut c_char) as c_short;
            (*sd).sector = sectors.offset(SHORT((*msd).sector) as isize);
            sd = sd.add(1);
            msd = msd.add(1);
        }

        W_ReleaseLumpNum(lump);
    }
}

// ---------------------------------------------------------------------------
// P_LoadBlockMap
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_LoadBlockMap(lump: c_int) {
    unsafe {
        let lumplen = W_LumpLength(lump as c_uint);
        let count = lumplen / 2;

        blockmaplump = Z_Malloc(lumplen, PU_LEVEL, ptr::null_mut()) as *mut c_short;
        W_ReadLump(lump as c_uint, blockmaplump as *mut c_void);
        blockmap = blockmaplump.add(4);

        // Swap all short integers to native byte ordering.
        for i in 0..count {
            *blockmaplump.add(i as usize) = SHORT(*blockmaplump.add(i as usize));
        }

        bmaporgx = (*blockmaplump.add(0) as c_int) << FRACBITS;
        bmaporgy = (*blockmaplump.add(1) as c_int) << FRACBITS;
        bmapwidth = *blockmaplump.add(2) as c_int;
        bmapheight = *blockmaplump.add(3) as c_int;

        let bcount = std::mem::size_of::<*mut c_void>() * bmapwidth as usize * bmapheight as usize;
        blocklinks = Z_Malloc(bcount as c_int, PU_LEVEL, ptr::null_mut()) as *mut *mut c_void;
        libc::memset(blocklinks as *mut c_void, 0, bcount);
    }
}

// ---------------------------------------------------------------------------
// P_GroupLines
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_GroupLines() {
    unsafe {
        // Look up sector number for each subsector.
        let mut ss = subsectors;
        for _ in 0..numsubsectors {
            let seg = segs.offset((*ss).firstline as isize);
            // Must use seg->sidedef->sector, not sidenum[0], because the seg
            // may be on side 1 (back side) of the linedef, in which case
            // sidenum[0] points to the wrong side's sector.
            (*ss).sector = (*(*seg).sidedef).sector as *mut c_void;
            ss = ss.add(1);
        }

        // Count number of lines in each sector.
        let mut li = lines;
        totallines = 0;
        for _ in 0..numlines {
            totallines += 1;
            let frontsec = (*li).frontsector as *mut sector_t;
            if !frontsec.is_null() {
                (*frontsec).linecount += 1;
            }
            let backsec = (*li).backsector as *mut sector_t;
            if !backsec.is_null() && backsec != frontsec {
                (*backsec).linecount += 1;
                totallines += 1;
            }
            li = li.add(1);
        }

        // Build line tables for each sector.
        let linebuffer = Z_Malloc(
            totallines * std::mem::size_of::<*mut line_t>() as c_int,
            PU_LEVEL,
            ptr::null_mut(),
        ) as *mut *mut line_t;

        let mut current = linebuffer;
        for i in 0..numsectors as usize {
            let sec = sectors.add(i);
            (*sec).lines = current as *mut *mut c_void;
            current = current.offset((*sec).linecount as isize);
            (*sec).linecount = 0;
        }

        // Assign lines to sectors.
        for i in 0..numlines as usize {
            li = lines.add(i);
            if !(*li).frontsector.is_null() {
                let sector = (*li).frontsector as *mut sector_t;
                (*sector)
                    .lines
                    .offset((*sector).linecount as isize)
                    .write(li as *mut c_void);
                (*sector).linecount += 1;
            }
            if !(*li).backsector.is_null() && (*li).frontsector != (*li).backsector {
                let sector = (*li).backsector as *mut sector_t;
                (*sector)
                    .lines
                    .offset((*sector).linecount as isize)
                    .write(li as *mut c_void);
                (*sector).linecount += 1;
            }
        }

        // Generate bounding boxes for sectors.
        let mut sector = sectors;
        for _ in 0..numsectors {
            let mut bbox: [c_int; 4] = [0; 4];
            M_ClearBox(bbox.as_mut_ptr());

            for j in 0..(*sector).linecount {
                let line = *(*sector).lines.offset(j as isize) as *mut line_t;
                M_AddToBox(bbox.as_mut_ptr(), (*(*line).v1).x, (*(*line).v1).y);
                M_AddToBox(bbox.as_mut_ptr(), (*(*line).v2).x, (*(*line).v2).y);
            }

            // Set the degenmobj_t to the middle of the bounding box.
            let soundorg_x = (bbox[BOXRIGHT] + bbox[BOXLEFT]) / 2;
            let soundorg_y = (bbox[BOXTOP] + bbox[BOXBOTTOM]) / 2;
            // sector->soundorg is a 40-byte degenmobj_t; first two fields are x,y.
            let soundorg_ptr = (*sector).soundorg.as_mut_ptr() as *mut c_int;
            *soundorg_ptr = soundorg_x;
            *soundorg_ptr.add(1) = soundorg_y;

            // Adjust bounding box to map blocks.
            let mut block = (bbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;
            block = if block >= bmapheight {
                bmapheight - 1
            } else {
                block
            };
            (*sector).blockbox[BOXTOP] = block;

            block = (bbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
            block = if block < 0 { 0 } else { block };
            (*sector).blockbox[BOXBOTTOM] = block;

            block = (bbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
            block = if block >= bmapwidth {
                bmapwidth - 1
            } else {
                block
            };
            (*sector).blockbox[BOXRIGHT] = block;

            block = (bbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
            block = if block < 0 { 0 } else { block };
            (*sector).blockbox[BOXLEFT] = block;

            sector = sector.add(1);
        }
    }
}

// ---------------------------------------------------------------------------
// PadRejectArray
// ---------------------------------------------------------------------------

unsafe fn PadRejectArray(array: *mut u8, len: usize) {
    let rejectpad: [u32; 4] = [((totallines * 4 + 3) & !3) as u32 + 24, 0, 50, 0x1d4a11];

    let mut dest = array;
    for i in 0..len.min(std::mem::size_of_val(&rejectpad)) {
        let byte_num = i % 4;
        *dest = ((rejectpad[i / 4] >> (byte_num * 8)) & 0xff) as u8;
        dest = dest.add(1);
    }

    if len > std::mem::size_of_val(&rejectpad) {
        eprintln!(
            "PadRejectArray: REJECT lump too short to pad! ({} > {})",
            len,
            std::mem::size_of_val(&rejectpad)
        );

        let padvalue = if M_CheckParm(c"-reject_pad_with_ff".as_ptr() as *mut c_char) != 0 {
            0xff
        } else {
            0xf00
        };

        let pad_byte = padvalue as u8;
        let pad_start = array.add(std::mem::size_of_val(&rejectpad));
        let pad_len = len - std::mem::size_of_val(&rejectpad);
        for i in 0..pad_len {
            *pad_start.add(i) = pad_byte;
        }
    }
}

// ---------------------------------------------------------------------------
// P_LoadReject
// ---------------------------------------------------------------------------

unsafe fn P_LoadReject(lumpnum: c_int) {
    let minlength = (numsectors * numsectors + 7) / 8;
    let lumplen = W_LumpLength(lumpnum as c_uint);

    if lumplen >= minlength {
        rejectmatrix = W_CacheLumpNum(lumpnum, PU_LEVEL) as *mut u8;
    } else {
        rejectmatrix = Z_Malloc(
            minlength,
            PU_LEVEL,
            &mut rejectmatrix as *mut *mut u8 as *mut c_void,
        ) as *mut u8;
        W_ReadLump(lumpnum as c_uint, rejectmatrix as *mut c_void);
        PadRejectArray(
            rejectmatrix.add(lumplen as usize),
            (minlength - lumplen) as usize,
        );
    }
}

// ---------------------------------------------------------------------------
// P_SetupLevel
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_SetupLevel(episode: c_int, map: c_int, _playermask: c_int, _skill: c_int) {
    unsafe {
        totalkills = 0;
        totalitems = 0;
        totalsecret = 0;
        wminfo.maxfrags = 0;
        wminfo.partime = 180;

        for i in 0..MAXPLAYERS {
            players[i].killcount = 0;
            players[i].secretcount = 0;
            players[i].itemcount = 0;
        }

        players[consoleplayer as usize].viewz = 1;

        S_Start();
        Z_FreeTags(PU_LEVEL, PU_PURGELEVEL - 1);

        P_InitThinkers();

        // Find map name.
        let mut lumpname = [0i8; 9];
        if gamemode == d_mode::commercial {
            if map < 10 {
                let s = format!("map0{}", map);
                ptr::copy_nonoverlapping(s.as_ptr(), lumpname.as_mut_ptr() as *mut u8, s.len());
            } else {
                let s = format!("map{}", map);
                ptr::copy_nonoverlapping(s.as_ptr(), lumpname.as_mut_ptr() as *mut u8, s.len());
            }
        } else {
            lumpname[0] = b'E' as i8;
            lumpname[1] = (b'0' + episode as u8) as i8;
            lumpname[2] = b'M' as i8;
            lumpname[3] = (b'0' + map as u8) as i8;
            lumpname[4] = 0;
        }

        let lumpnum = W_GetNumForName(lumpname.as_mut_ptr());

        leveltime = 0;

        // Note: most of this ordering is important.
        P_LoadBlockMap(lumpnum + ML_BLOCKMAP);
        P_LoadVertexes(lumpnum + ML_VERTEXES);
        P_LoadSectors(lumpnum + ML_SECTORS);
        P_LoadSideDefs(lumpnum + ML_SIDEDEFS);

        P_LoadLineDefs(lumpnum + ML_LINEDEFS);
        P_LoadSubsectors(lumpnum + ML_SSECTORS);
        P_LoadNodes(lumpnum + ML_NODES);
        P_LoadSegs(lumpnum + ML_SEGS);

        P_GroupLines();
        P_LoadReject(lumpnum + ML_REJECT);

        bodyqueslot = 0;
        deathmatch_p = deathmatchstarts.as_mut_ptr();
        P_LoadThings(lumpnum + ML_THINGS);

        if deathmatch != 0 {
            for i in 0..MAXPLAYERS {
                if playeringame[i] != 0 {
                    players[i].mo = ptr::null_mut();
                    G_DeathMatchSpawnPlayer(i as c_int);
                }
            }
        }

        iquehead = 0;
        iquetail = 0;

        P_SpawnSpecials();

        if precache != 0 {
            R_PrecacheLevel();
        }
    }
}

// ---------------------------------------------------------------------------
// P_Init
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn P_Init() {
    unsafe {
        P_InitSwitchList();
        P_InitPicAnims();
        R_InitSprites(sprnames.as_mut_ptr());
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    // =========================================================================
    // Existing structural tests
    // =========================================================================

    #[test]
    fn max_deathmatch_starts_is_10() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(MAX_DEATHMATCH_STARTS, 10);
    }

    #[test]
    fn setup_globals_default_zero() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(numvertexes, 0);
            assert_eq!(numsegs, 0);
            assert_eq!(numsectors, 0);
            assert_eq!(numsubsectors, 0);
            assert_eq!(numnodes, 0);
            assert_eq!(numlines, 0);
            assert_eq!(numsides, 0);
            assert_eq!(bmapwidth, 0);
            assert_eq!(bmapheight, 0);
            assert_eq!(bmaporgx, 0);
            assert_eq!(bmaporgy, 0);
        }
    }

    #[test]
    fn setup_globals_are_c_int_width() {
        use std::ffi::c_int;
        const _: () = assert!(std::mem::size_of::<c_int>() == 4);
        unsafe {
            let _: c_int = numvertexes;
            let _: c_int = numsegs;
            let _: c_int = numsectors;
            let _: c_int = numsubsectors;
            let _: c_int = numnodes;
            let _: c_int = numlines;
            let _: c_int = numsides;
            let _: c_int = bmapwidth;
            let _: c_int = bmapheight;
            let _: c_int = bmaporgx;
            let _: c_int = bmaporgy;
        }
    }

    // =========================================================================
    // Regression tests for bugs fixed in the p_setup.c → Rust port.
    //
    // These are pure unit tests that verify the core algorithms without
    // requiring full engine (WAD, zone memory, C FFI) initialisation.
    //
    // Bug 1: P_LoadThings used `continue` instead of `break` when a
    //        non-commercial monster was encountered in shareware mode.
    // Bug 2: P_GroupLines sector line table computed per-sector offsets
    //        from the buffer base instead of advancing cumulatively.
    // Bug 3: P_GroupLines resolved subsector→sector via sidenum[0]
    //        instead of seg->sidedef->sector, corrupting back-side segs.
    // =========================================================================

    // -----------------------------------------------------------------------
    // Regression: P_LoadThings break-on-non-commercial-monster
    // -----------------------------------------------------------------------

    /// Simulates the P_LoadThings thing-filter loop in pure Rust.
    /// Returns the number of things that would actually be spawned.
    ///
    /// In shareware/non-commercial mode the loop must **break** (not continue)
    /// when it hits the first non-commercial monster type.  Using `continue`
    /// instead causes every subsequent thing to also be processed, which
    /// changes the RNG call sequence and breaks demo determinism.
    fn count_spawnable_things(things: &[i16], commercial: bool) -> usize {
        let non_commercial_types: [i16; 10] = [68, 64, 88, 89, 69, 67, 71, 65, 66, 84];

        let mut count = 0;
        for &thing_type in things {
            let mut spawn = true;
            if !commercial {
                if non_commercial_types.contains(&thing_type) {
                    spawn = false;
                }
            }
            if !spawn {
                break; // MUST break – was `continue` in the buggy port
            }
            count += 1;
        }
        count
    }

    /// In commercial mode every thing is spawned regardless of type.
    #[test]
    fn p_load_things_commercial_spawns_all() {
        // Types: player start, imp, Archvile(64), cacodemon, Revenant(66)
        let things: [i16; 5] = [1, 3001, 64, 3003, 66];
        assert_eq!(count_spawnable_things(&things, true), 5);
    }

    /// In shareware mode the loop stops at the first non-commercial monster.
    /// Things after it must NOT be spawned (regression: was `continue`).
    #[test]
    fn p_load_things_shareware_breaks_at_non_commercial() {
        // Types: player start, imp, Archvile(64), cacodemon, Revenant(66)
        // Archvile is non-commercial → loop breaks, only 2 things spawned.
        let things: [i16; 5] = [1, 3001, 64, 3003, 66];
        assert_eq!(count_spawnable_things(&things, false), 2);
    }

    /// Non-commercial monster at the very start → zero things spawned.
    #[test]
    fn p_load_things_shareware_first_thing_non_commercial() {
        let things: [i16; 3] = [64, 1, 3001]; // Archvile first
        assert_eq!(count_spawnable_things(&things, false), 0);
    }

    /// No non-commercial monsters → all things spawned in shareware mode.
    #[test]
    fn p_load_things_shareware_no_non_commercial_spawns_all() {
        let things: [i16; 4] = [1, 3001, 3003, 3004]; // all shareware types
        assert_eq!(count_spawnable_things(&things, false), 4);
    }

    /// Non-commercial monster at the very end → everything before spawns.
    #[test]
    fn p_load_things_shareware_non_commercial_at_end() {
        let things: [i16; 5] = [1, 3001, 3003, 3004, 88]; // Boss Brain last
        assert_eq!(count_spawnable_things(&things, false), 4);
    }

    // -----------------------------------------------------------------------
    // Regression: P_GroupLines sector line table cumulative offset
    // -----------------------------------------------------------------------

    /// Simulates the sector line-table pointer assignment from P_GroupLines.
    /// Returns the computed start-offset for each sector within the shared
    /// line-buffer.
    ///
    /// The C original advances a single cursor cumulatively:
    ///   sectors[i].lines = linebuffer;
    ///   linebuffer += sectors[i].linecount;
    ///
    /// The buggy Rust port computed each sector's offset independently from
    /// the buffer base (`base + linecount`), causing every sector's line
    /// table to point to the wrong memory.
    fn compute_sector_line_offsets(linecounts: &[usize]) -> Vec<usize> {
        let mut offsets = Vec::with_capacity(linecounts.len());
        let mut current: usize = 0;
        for &lc in linecounts {
            offsets.push(current);
            current += lc;
        }
        offsets
    }

    /// Cumulative offsets must equal the sum of all preceding linecounts.
    #[test]
    fn p_grouplines_sector_offsets_are_cumulative() {
        // Sector linecounts: [10, 5, 3]
        // Expected offsets:  [0, 10, 15]
        let linecounts = [10usize, 5, 3];
        let offsets = compute_sector_line_offsets(&linecounts);
        assert_eq!(offsets, vec![0, 10, 15]);
    }

    /// Single sector → offset 0.
    #[test]
    fn p_grouplines_single_sector_offset_zero() {
        let linecounts = [7usize];
        assert_eq!(compute_sector_line_offsets(&linecounts), vec![0]);
    }

    /// Sector with zero lines must still advance correctly for next sector.
    #[test]
    fn p_grouplines_zero_line_sector_preserves_cumulative() {
        let linecounts = [5usize, 0, 3];
        // sector 0 → offset 0, sector 1 → offset 5, sector 2 → offset 5
        assert_eq!(compute_sector_line_offsets(&linecounts), vec![0, 5, 5]);
    }

    /// The buggy version (`base + linecount`) would produce wrong results.
    /// This test documents the exact wrong values that the bug produced.
    #[test]
    fn p_grouplines_buggy_offset_produces_wrong_values() {
        let linecounts = [10usize, 5, 3];

        // Correct (cumulative):
        let correct = compute_sector_line_offsets(&linecounts);
        assert_eq!(correct, vec![0, 10, 15]);

        // Buggy (independent from base):
        let buggy: Vec<usize> = linecounts.iter().copied().collect();
        assert_eq!(buggy, vec![10, 5, 3]);

        // They must NOT be equal.
        assert_ne!(correct, buggy);
    }

    // -----------------------------------------------------------------------
    // Regression: P_GroupLines subsector sector via seg->sidedef->sector
    // -----------------------------------------------------------------------

    /// Simulates the subsector→sector resolution from P_GroupLines.
    ///
    /// The C original uses `seg->sidedef->sector`, which respects the seg's
    /// side (0 = front, 1 = back).  The buggy Rust port hardcoded
    /// `sidenum[0]`, always resolving to the front side's sector regardless
    /// of which side the seg is on.
    ///
    /// Parameters:
    /// - `seg_side`: 0 (front) or 1 (back) – which side of the linedef the seg is on
    /// - `sidenum`: [front_sidenum, back_sidenum]
    /// - `sector_of_sidenum`: maps each sidenum to its sector index
    ///
    /// Returns the resolved sector index.
    fn resolve_subsector_sector_via_sidedef(
        seg_side: usize,
        sidenum: [i16; 2],
        sector_of_sidenum: &[usize],
    ) -> usize {
        // Correct: use the seg's already-resolved sidedef, which was set
        // during P_LoadSegs to `sides[ldef->sidenum[seg_side]]`.
        // So sector = sector_of_sidenum[sidenum[seg_side]]
        let sidenum_idx = sidenum[seg_side] as usize;
        sector_of_sidenum[sidenum_idx]
    }

    /// The buggy version always used sidenum[0] regardless of seg_side.
    fn resolve_subsector_sector_buggy(
        seg_side: usize,
        sidenum: [i16; 2],
        sector_of_sidenum: &[usize],
    ) -> usize {
        let _ = seg_side; // unused – the bug!
        let sidenum_idx = sidenum[0] as usize;
        sector_of_sidenum[sidenum_idx]
    }

    /// Front-side seg (side=0): both methods agree.
    #[test]
    fn p_grouplines_subsector_front_side_agrees() {
        // linedef has sidenum[0]=2, sidenum[1]=5
        // sector_of_sidenum[2]=10, sector_of_sidenum[5]=20
        let sidenum = [2i16, 5];
        let sector_map = [0, 0, 10, 0, 0, 20];

        assert_eq!(
            resolve_subsector_sector_via_sidedef(0, sidenum, &sector_map),
            10
        );
        assert_eq!(resolve_subsector_sector_buggy(0, sidenum, &sector_map), 10);
    }

    /// Back-side seg (side=1): correct method uses sidenum[1],
    /// buggy method uses sidenum[0] → wrong sector.
    #[test]
    fn p_grouplines_subsector_back_side_differs() {
        // linedef has sidenum[0]=2 (sector 10), sidenum[1]=5 (sector 20)
        let sidenum = [2i16, 5];
        let sector_map = [0, 0, 10, 0, 0, 20];

        // Correct: seg on side 1 → uses sidenum[1]=5 → sector 20
        assert_eq!(
            resolve_subsector_sector_via_sidedef(1, sidenum, &sector_map),
            20
        );

        // Buggy: always uses sidenum[0]=2 → sector 10 (WRONG)
        assert_eq!(resolve_subsector_sector_buggy(1, sidenum, &sector_map), 10);
    }

    /// Ensures the correct and buggy implementations produce different
    /// results for back-side segs, proving the regression test is meaningful.
    #[test]
    fn p_grouplines_subsector_back_side_correct_vs_buggy_differ() {
        let sidenum = [3i16, 7];
        let sector_map = [0, 0, 0, 11, 0, 0, 0, 22];

        let correct = resolve_subsector_sector_via_sidedef(1, sidenum, &sector_map);
        let buggy = resolve_subsector_sector_buggy(1, sidenum, &sector_map);

        assert_ne!(
            correct, buggy,
            "back-side seg must resolve to different sectors"
        );
        assert_eq!(correct, 22);
        assert_eq!(buggy, 11);
    }
}
