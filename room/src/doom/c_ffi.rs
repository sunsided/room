//! Centralized FFI declarations for unported C modules.
//!
//! Each `extern "C"` block is organized by the original `.c` source file.
//! These declarations allow Rust unit tests to call the original C functions
//! directly and read C global variables, establishing behavioral baselines
//! before porting each module.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::ffi::{c_char, c_int, c_short, c_uint, c_ushort, c_void};

// ---------------------------------------------------------------------------
// Opaque types — we only need pointers to these for many FFI signatures.
// ---------------------------------------------------------------------------

pub enum state_t {}
pub enum mobjinfo_t {}

// ---------------------------------------------------------------------------
// Minimal repr(C) types needed for p_maputl.c tests.
//
// These definitions match the layouts in p_telept.rs / p_sight.rs so that
// pointer casts between the two are safe.
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct vertex_t {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct divline_t {
    pub x: c_int,
    pub y: c_int,
    pub dx: c_int,
    pub dy: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct subsector_t {
    pub sector: *mut c_void,
    pub numlines: i16,
    pub firstline: i16,
    _pad: [u8; 4],
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sector_t {
    pub floorheight: c_int,
    pub ceilingheight: c_int,
    pub floorpic: c_short,
    pub ceilingpic: c_short,
    pub lightlevel: c_short,
    pub special: c_short,
    pub tag: c_short,
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct line_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub dx: c_int,
    pub dy: c_int,
    pub flags: c_short,
    pub special: c_short,
    pub tag: c_short,
    pub sidenum: [c_short; 2],
    pub bbox: [c_int; 4],
    pub slopetype: c_int,
    pub frontsector: *mut c_void,
    pub backsector: *mut c_void,
    pub validcount: c_int,
    pub specialdata: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct intercept_t {
    pub frac: c_int,
    pub isaline: c_int, // boolean
    pub d: intercept_t_d,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union intercept_t_d {
    pub thing: *mut c_void,
    pub line: *mut line_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mobj_t {
    pub thinker_prev: *mut c_void,
    pub thinker_next: *mut c_void,
    pub thinker_fn: *mut c_void,
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    _pad0: [u8; 4],
    pub snext: *mut c_void,
    pub sprev: *mut c_void,
    pub angle: c_uint,
    pub sprite: c_int,
    pub frame: c_int,
    _pad1: [u8; 4],
    pub bnext: *mut c_void,
    pub bprev: *mut c_void,
    pub subsector: *mut c_void,
    pub floorz: c_int,
    pub ceilingz: c_int,
    pub radius: c_int,
    pub height: c_int,
    pub momx: c_int,
    pub momy: c_int,
    pub momz: c_int,
    pub validcount: c_int,
    pub type_: c_int,
    _pad2: [u8; 4],
    pub info: *mut mobjinfo_t,
    pub tics: c_int,
    _pad3: [u8; 4],
    pub state: *mut state_t,
    pub flags: c_int,
    pub health: c_int,
    pub movedir: c_int,
    pub movecount: c_int,
    pub target: *mut c_void,
    pub reactiontime: c_int,
    pub threshold: c_int,
    pub player: *mut c_void,
    pub lastlook: c_int,
    pub spawnpoint: mapthing_t,
    _pad4: [u8; 2],
    pub tracer: *mut c_void,
}

// ---------------------------------------------------------------------------
// p_maputl.c
// ---------------------------------------------------------------------------

pub const MAXINTERCEPTS_ORIGINAL: usize = 128;
pub const MAXINTERCEPTS: usize = MAXINTERCEPTS_ORIGINAL + 61;

extern "C" {
    pub static mut opentop: c_int;
    pub static mut openbottom: c_int;
    pub static mut openrange: c_int;
    pub static mut lowfloor: c_int;

    pub static mut intercepts: [intercept_t; MAXINTERCEPTS];
    pub static mut intercept_p: *mut intercept_t;

    pub static mut trace: divline_t;

    pub fn P_AproxDistance(dx: c_int, dy: c_int) -> c_int;
    pub fn P_PointOnLineSide(x: c_int, y: c_int, line: *mut line_t) -> c_int;
    pub fn P_PointOnDivlineSide(x: c_int, y: c_int, line: *mut divline_t) -> c_int;
    pub fn P_MakeDivline(li: *mut line_t, dl: *mut divline_t);
    pub fn P_InterceptVector(v2: *mut divline_t, v1: *mut divline_t) -> c_int;
    pub fn P_BoxOnLineSide(tmbox: *mut c_int, ld: *mut line_t) -> c_int;
    pub fn P_LineOpening(linedef: *mut line_t);
    pub fn P_UnsetThingPosition(thing: *mut mobj_t);
    pub fn P_SetThingPosition(thing: *mut mobj_t);
    pub fn P_PathTraverse(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        flags: c_int,
        trav: Option<unsafe extern "C" fn(*mut intercept_t) -> c_uint>,
    ) -> c_uint;
    pub fn P_TraverseIntercepts(
        func: Option<unsafe extern "C" fn(*mut intercept_t) -> c_uint>,
        maxfrac: c_int,
    ) -> c_uint;
    pub fn PIT_AddLineIntercepts(ld: *mut line_t) -> c_uint;
    pub fn PIT_AddThingIntercepts(thing: *mut mobj_t) -> c_uint;
}

// ---------------------------------------------------------------------------
// p_inter.c
// ---------------------------------------------------------------------------

extern "C" {
    pub static mut maxammo: [c_int; 4];
    pub static mut clipammo: [c_int; 4];

    pub fn P_GiveAmmo(player: *mut c_void, ammo: c_int, num: c_int) -> c_uint;
    pub fn P_GiveWeapon(player: *mut c_void, weapon: c_int, dropped: c_uint) -> c_uint;
    pub fn P_GiveBody(player: *mut c_void, num: c_int) -> c_uint;
    pub fn P_GiveArmor(player: *mut c_void, armortype: c_int) -> c_uint;
    pub fn P_GivePower(player: *mut c_void, power: c_int) -> c_uint;
    pub fn P_TouchSpecialThing(special: *mut mobj_t, toucher: *mut mobj_t);
    pub fn P_KillMobj(source: *mut mobj_t, target: *mut mobj_t);
    pub fn P_DamageMobj(
        target: *mut mobj_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damage: c_int,
    );
}

// ---------------------------------------------------------------------------
// p_spec.c
// ---------------------------------------------------------------------------

extern "C" {
    pub static mut leveltime: c_int;
}

// ---------------------------------------------------------------------------
// p_mobj.c
// ---------------------------------------------------------------------------

extern "C" {
    pub static mut itemrespawnque: [u8; 128 * 10]; // mapthing_t[ITEMQUESIZE]
    pub static mut itemrespawntime: [c_int; 128];
    pub static mut iquehead: c_int;
    pub static mut iquetail: c_int;

    pub fn P_SetMobjState(mobj: *mut mobj_t, state: c_int) -> c_uint;
    pub fn P_ExplodeMissile(mo: *mut mobj_t);
    pub fn P_XYMovement(mo: *mut mobj_t);
    pub fn P_ZMovement(mo: *mut mobj_t);
    pub fn P_NightmareRespawn(mobj: *mut mobj_t);
    pub fn P_MobjThinker(mobj: *mut mobj_t);
    pub fn P_SpawnMobj(x: c_int, y: c_int, z: c_int, type_: c_int) -> *mut mobj_t;
    pub fn P_RemoveMobj(mobj: *mut mobj_t);
    pub fn P_RespawnSpecials();
    pub fn P_SpawnPuff(x: c_int, y: c_int, z: c_int);
    pub fn P_SpawnBlood(x: c_int, y: c_int, z: c_int, damage: c_int);
    pub fn P_CheckMissileSpawn(th: *mut mobj_t);
    pub fn P_SubstNullMobj(mobj: *mut mobj_t) -> *mut mobj_t;
    pub fn P_SpawnMissile(source: *mut mobj_t, dest: *mut mobj_t, type_: c_int) -> *mut mobj_t;
    pub fn P_SpawnPlayerMissile(source: *mut mobj_t, type_: c_int);
}

// ---------------------------------------------------------------------------
// r_draw.c
// ---------------------------------------------------------------------------

extern "C" {
    pub static mut viewwidth: c_int;
    pub static mut viewheight: c_int;
    pub static mut viewwindowx: c_int;
    pub static mut viewwindowy: c_int;
    pub static mut dc_x: c_int;
    pub static mut dc_yl: c_int;
    pub static mut dc_yh: c_int;
    pub static mut dc_iscale: c_int;
    pub static mut dc_texturemid: c_int;
    pub static mut fuzzpos: c_int;
    pub static mut ds_y: c_int;
    pub static mut ds_x1: c_int;
    pub static mut ds_x2: c_int;
    pub static mut ds_xfrac: c_int;
    pub static mut ds_yfrac: c_int;
    pub static mut ds_xstep: c_int;
    pub static mut ds_ystep: c_int;

    pub fn R_InitBuffer(width: c_int, height: c_int);
    pub fn R_InitTranslationTables();
    pub fn R_FillBackScreen();
    pub fn R_VideoErase(ofs: c_uint, count: c_int);
    pub fn R_DrawViewBorder();
}

// ---------------------------------------------------------------------------
// r_data.c
// ---------------------------------------------------------------------------

extern "C" {
    pub static mut firstflat: c_int;
    pub static mut lastflat: c_int;
    pub static mut numflats: c_int;
    pub static mut firstspritelump: c_int;
    pub static mut lastspritelump: c_int;
    pub static mut numspritelumps: c_int;
    pub static mut numtextures: c_int;

    pub fn R_GetColumn(tex: c_int, col: c_int) -> *mut u8;
    pub fn R_GenerateComposite(texnum: c_int);
    pub fn R_GenerateLookup(texnum: c_int);
    pub fn R_InitTextures();
    pub fn R_InitFlats();
    pub fn R_InitSpriteLumps();
    pub fn R_InitColormaps();
    pub fn R_InitData();
    pub fn R_FlatNumForName(name: *mut c_char) -> c_int;
    pub fn R_CheckTextureNumForName(name: *mut c_char) -> c_int;
    pub fn R_TextureNumForName(name: *mut c_char) -> c_int;
    pub fn R_PrecacheLevel();
}

// ---------------------------------------------------------------------------
// Constants from C headers
// ---------------------------------------------------------------------------

pub const FRACBITS: u32 = 16;
pub const FRACUNIT: c_int = 1 << FRACBITS;

pub const MAPBLOCKUNITS: c_int = 128;
pub const MAPBLOCKSIZE: c_int = MAPBLOCKUNITS * FRACUNIT;
pub const MAPBLOCKSHIFT: c_int = FRACBITS as c_int + 7;
pub const MAPBMASK: c_int = MAPBLOCKSIZE - 1;
pub const MAPBTOFRAC: c_int = MAPBLOCKSHIFT - FRACBITS as c_int;

pub const FINEANGLES: usize = 8192;
pub const FINEMASK: c_int = FINEANGLES as c_int - 1;
pub const ANGLETOFINESHIFT: c_int = 19;

pub const ANG45: c_uint = 1 << 29;
pub const ANG90: c_uint = 1 << 30;
pub const ANG180: c_uint = 1 << 31;

pub const ITEMQUESIZE: usize = 128;

pub const PT_ADDLINES: c_int = 1;
pub const PT_ADDTHINGS: c_int = 2;
pub const PT_EARLYOUT: c_int = 4;

pub const BOXTOP: usize = 0;
pub const BOXBOTTOM: usize = 1;
pub const BOXLEFT: usize = 2;
pub const BOXRIGHT: usize = 3;

pub const ST_HORIZONTAL: c_int = 0;
pub const ST_VERTICAL: c_int = 1;
pub const ST_POSITIVE: c_int = 2;
pub const ST_NEGATIVE: c_int = 3;

// ---------------------------------------------------------------------------
// WAD lump-order indices (doomdata.h anonymous enum).
// These are the fixed positions of each data lump within a map's group of
// WAD lumps.  p_setup.c indexes directly into the lump list using these.
// ---------------------------------------------------------------------------

pub const ML_LABEL: c_int = 0; // ExMx / MAPxx separator
pub const ML_THINGS: c_int = 1; // Monster/item placement
pub const ML_LINEDEFS: c_int = 2; // Line definitions
pub const ML_SIDEDEFS: c_int = 3; // Side (texture) definitions
pub const ML_VERTEXES: c_int = 4; // Vertex coordinates
pub const ML_SEGS: c_int = 5; // BSP line segments
pub const ML_SSECTORS: c_int = 6; // BSP sub-sectors
pub const ML_NODES: c_int = 7; // BSP nodes
pub const ML_SECTORS: c_int = 8; // Sector definitions
pub const ML_REJECT: c_int = 9; // Sector-to-sector visibility table
pub const ML_BLOCKMAP: c_int = 10; // Motion-clipping blockmap

// ---------------------------------------------------------------------------
// LineDef flag bits (ML_* defines from doomdata.h).
// Used by p_map.c, p_spec.c, and the renderer to determine line properties.
// ---------------------------------------------------------------------------

pub const ML_BLOCKING: u16 = 1; // Solid obstacle
pub const ML_BLOCKMONSTERS: u16 = 2; // Blocks monsters only
pub const ML_TWOSIDED: u16 = 4; // Has a back sector
pub const ML_DONTPEGTOP: u16 = 8; // Upper texture is unpegged
pub const ML_DONTPEGBOTTOM: u16 = 16; // Lower texture is unpegged
pub const ML_SECRET: u16 = 32; // Secret on automap
pub const ML_SOUNDBLOCK: u16 = 64; // Sound propagation barrier
pub const ML_DONTDRAW: u16 = 128; // Hidden on automap
pub const ML_MAPPED: u16 = 256; // Already revealed on automap

// ---------------------------------------------------------------------------
// Screen constants (from i_video.h / st_stuff.h)
// ---------------------------------------------------------------------------

pub const SCREENWIDTH: c_int = 320;
pub const SCREENHEIGHT: c_int = 200;
/// Status-bar height (ST_HEIGHT from st_stuff.h).
pub const SBARHEIGHT: c_int = 32;

// ---------------------------------------------------------------------------
// r_draw.c constants
// ---------------------------------------------------------------------------

/// Number of entries in the fuzz offset lookup table (FUZZTABLE in r_draw.c).
pub const FUZZTABLE: usize = 50;
/// Column offset used when rendering the fuzz/spectre effect (FUZZOFF = SCREENWIDTH).
pub const FUZZOFF: c_int = SCREENWIDTH;

// ---------------------------------------------------------------------------
// Runtime renderer types (r_defs.h) — used by unported r_segs.c / r_things.c
// ---------------------------------------------------------------------------

/// SideDef: visual appearance of a wall segment.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct side_t {
    pub textureoffset: c_int,
    pub rowoffset: c_int,
    pub toptexture: c_short,
    pub bottomtexture: c_short,
    pub midtexture: c_short,
    _pad: [u8; 2],
    pub sector: *mut sector_t,
}

/// LineSeg: a BSP-split segment of a line definition.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct seg_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub offset: c_int,
    pub angle: c_uint,
    pub sidedef: *mut side_t,
    pub linedef: *mut line_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
}

/// BSP node: partitions space into two sub-trees.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct node_t {
    pub x: c_int,
    pub y: c_int,
    pub dx: c_int,
    pub dy: c_int,
    pub bbox: [[c_int; 4]; 2],
    pub children: [c_ushort; 2],
}

/// Draw segment: one visible wall segment produced by the BSP traversal.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct drawseg_t {
    pub curline: *mut seg_t,
    pub x1: c_int,
    pub x2: c_int,
    pub scale1: c_int,
    pub scale2: c_int,
    pub scalestep: c_int,
    pub silhouette: c_int,
    pub bsilheight: c_int,
    pub tsilheight: c_int,
    pub sprtopclip: *mut c_short,
    pub sprbottomclip: *mut c_short,
    pub maskedtexturecol: *mut c_short,
}

/// Visible sprite: a thing that is (partly) visible in the current frame.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct vissprite_t {
    pub prev: *mut vissprite_t,
    pub next: *mut vissprite_t,
    pub x1: c_int,
    pub x2: c_int,
    pub gx: c_int,
    pub gy: c_int,
    pub gz: c_int,
    pub gzt: c_int,
    pub startfrac: c_int,
    pub scale: c_int,
    pub xiscale: c_int,
    pub texturemid: c_int,
    pub patch: c_int,
    _pad: [u8; 4],
    pub colormap: *mut u8,
    pub mobjflags: c_int,
    _pad2: [u8; 4],
}

// ---------------------------------------------------------------------------
// r_draw.c additional externs
// ---------------------------------------------------------------------------

extern "C" {
    /// The fuzz column-offset lookup table; length == FUZZTABLE.
    pub static mut fuzzoffset: [c_int; FUZZTABLE];
    /// Scaled (actual) view width; set alongside viewwidth in R_ExecuteSetViewSize.
    pub static mut scaledviewwidth: c_int;
}

// ---------------------------------------------------------------------------
// r_segs.c
// Segment rendering state – set each frame by R_StoreWallRange before any
// draw calls; all zero before the first frame is rendered.
// ---------------------------------------------------------------------------

extern "C" {
    /// True if any texture on the current seg might be visible.
    pub static mut segtextured: c_int;
    /// False when the back sector shares the same floor plane.
    pub static mut markfloor: c_int;
    /// False when the back sector shares the same ceiling plane.
    pub static mut markceiling: c_int;
    /// True when there is a masked (transparent) mid-texture on the seg.
    pub static mut maskedtexture: c_int;
    /// Texture number for the upper (top) wall texture.
    pub static mut toptexture: c_int;
    /// Texture number for the lower (bottom) wall texture.
    pub static mut bottomtexture: c_int;
    /// Texture number for the middle (solid) wall texture.
    pub static mut midtexture: c_int;
    /// Normal angle of the current segment (BAM units).
    pub static mut rw_normalangle: c_uint;
    /// Angle from player to line origin; used for texture offsetting.
    pub static mut rw_angle1: c_int;
    /// Left column (inclusive) of the wall strip being drawn.
    pub static mut rw_x: c_int;
    /// Right column (exclusive) of the wall strip.
    pub static mut rw_stopx: c_int;
    /// Angle used to compute per-column scale (BAM units).
    pub static mut rw_centerangle: c_uint;
    /// Horizontal texture offset along the seg.
    pub static mut rw_offset: c_int;
    /// Perpendicular distance from the player to the seg.
    pub static mut rw_distance: c_int;
    /// Scale factor at the left edge of the strip.
    pub static mut rw_scale: c_int;
    /// Per-column scale delta.
    pub static mut rw_scalestep: c_int;
    /// Texture-coordinate midpoint for the mid texture.
    pub static mut rw_midtexturemid: c_int;
    /// Texture-coordinate midpoint for the top texture.
    pub static mut rw_toptexturemid: c_int;
    /// Texture-coordinate midpoint for the bottom texture.
    pub static mut rw_bottomtexturemid: c_int;
    /// World-space top of the visible wall opening (ceiling).
    pub static mut worldtop: c_int;
    /// World-space bottom of the visible wall opening (floor).
    pub static mut worldbottom: c_int;
    /// World-space top of the back-sector ceiling (two-sided walls).
    pub static mut worldhigh: c_int;
    /// World-space bottom of the back-sector floor (two-sided walls).
    pub static mut worldlow: c_int;
    /// Current high-wall pixel position (fixed-point screen coords).
    pub static mut pixhigh: c_int;
    /// Current low-wall pixel position (fixed-point screen coords).
    pub static mut pixlow: c_int;
    /// Per-column step for pixhigh.
    pub static mut pixhighstep: c_int;
    /// Per-column step for pixlow.
    pub static mut pixlowstep: c_int;
    /// Current top texture fractional row (fixed-point).
    pub static mut topfrac: c_int;
    /// Per-column step for topfrac.
    pub static mut topstep: c_int;
    /// Current bottom texture fractional row (fixed-point).
    pub static mut bottomfrac: c_int;
    /// Per-column step for bottomfrac.
    pub static mut bottomstep: c_int;
    /// Pointer to the active light table array for this seg's light level.
    pub static mut walllights: *mut *mut u8; // lighttable_t**
    /// Column offset array for masked (transparent) mid-textures.
    pub static mut maskedtexturecol: *mut c_short;
}

// ---------------------------------------------------------------------------
// r_things.c
// Sprite rendering state – set during R_DrawMasked / R_ProjectSprite.
// pspritescale and pspriteiscale are set per frame; all others zero before
// R_InitSprites is called.
// ---------------------------------------------------------------------------

/// One animation-frame of a sprite, matching r_defs.h `spriteframe_t`.
///
/// Layout (28 bytes):
///   +0  rotate (boolean/int – 4 bytes)
///   +4  lump[8] (short[8] – 16 bytes)
///   +20 flip[8] (byte[8] – 8 bytes)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct spriteframe_t {
    /// 0 = use frame 0 for all rotations; 1 = use rotation-specific lumps.
    pub rotate: c_int,
    /// WAD lump index (relative to firstspritelump) for each of 8 rotations.
    pub lump: [c_short; 8],
    /// 1 = horizontally flip the lump for this rotation; 0 = no flip.
    pub flip: [u8; 8],
}

extern "C" {
    /// Scale applied to player weapon (psprite) columns this frame.
    pub static mut pspritescale: c_int;
    /// Inverse of pspritescale (= FRACUNIT / pspritescale).
    pub static mut pspriteiscale: c_int;
    /// Pointer to the active light-table array for sprites this frame.
    pub static mut spritelights: *mut *mut u8; // lighttable_t**
    /// Clipping array initialised to -1 for psprite bottom clipping.
    pub static mut negonearray: [c_short; 320]; // [SCREENWIDTH]
    /// Clipping array initialised to SCREENHEIGHT for psprite top clipping.
    pub static mut screenheightarray: [c_short; 320]; // [SCREENWIDTH]
    /// Pointer to the sprite definition table (set by R_InitSprites).
    pub static mut sprites: *mut c_void; // spritedef_t*
    /// Total number of sprite names found in the WAD (set by R_InitSprites).
    pub static mut numsprites: c_int;
    /// Temporary frame-building array used during R_InitSprites; length 29.
    pub static mut sprtemp: [spriteframe_t; 29];
    /// Highest frame index seen for the current sprite during R_InitSprites.
    pub static mut maxframe: c_int;
    /// Name of the sprite currently being processed by R_InitSprites.
    pub static mut spritename: *mut c_char;
}

// ---------------------------------------------------------------------------
// g_game.c
// Movement tables and game-state globals.
// forwardmove / sidemove / angleturn are static initialisers (non-zero).
// ---------------------------------------------------------------------------

extern "C" {
    /// Forward movement speed table: [slow, fast] (fixed_t, unit/tic).
    /// Values: {0x19, 0x32} = {25, 50}.
    pub static mut forwardmove: [c_int; 2];
    /// Lateral (strafe) movement speed table: [slow, fast] (fixed_t, unit/tic).
    /// Values: {0x18, 0x28} = {24, 40}.
    pub static mut sidemove: [c_int; 2];
    /// Turn-speed table: [normal, fast, slow] (BAM units/tic).
    /// Values: {640, 1280, 320}.  Index 2 is used for the first SLOWTURNTICS (6)
    /// tics; after that index 0 (or 1 with run) is used.
    pub static mut angleturn: [c_int; 3];
    /// Next slot in the circular body-queue ring buffer.
    pub static mut bodyqueslot: c_int;
    /// When true, savegame file size is capped at the vanilla limit (0x2c000).
    pub static mut vanilla_savegame_limit: c_int;
    /// When true, demo file size is capped at the vanilla limit.
    pub static mut vanilla_demo_limit: c_int;
    /// When true, all graphics are preloaded at level start.
    pub static mut precache: c_int;
    /// When true (set by -testcontrols), exit after the first tic.
    pub static mut testcontrols: c_int;
    /// Gametic at which the current level started.
    pub static mut levelstarttic: c_int;
    /// Total enemy count on the current level (for intermission).
    pub static mut totalkills: c_int;
    /// Total item count on the current level (for intermission).
    pub static mut totalitems: c_int;
    /// Total secret count on the current level (for intermission).
    pub static mut totalsecret: c_int;
}

// ---------------------------------------------------------------------------
// p_pspr.c
// Weapon-sprite (psprite) state globals.
// swingx/swingy are computed each tic by P_CalcSwing; zero before first tic.
// ---------------------------------------------------------------------------

extern "C" {
    /// Horizontal weapon-bob offset (fixed_t); updated each tic by P_CalcSwing.
    pub static mut swingx: c_int;
    /// Vertical weapon-bob offset (fixed_t); updated each tic by P_CalcSwing.
    pub static mut swingy: c_int;
}

// ---------------------------------------------------------------------------
// hu_stuff.c
// HUD globals – string tables and toggle flags.
// ---------------------------------------------------------------------------

extern "C" {
    /// The 10 pre-defined chat macro strings (Ctrl+1 … Ctrl+0).
    /// Pointers to string literals from d_englsh.h; never NULL.
    pub static mut chat_macros: [*mut c_char; 10];
    /// The 4 per-player name-prefix strings ("Green: ", "Indigo: ", …).
    pub static mut player_names: [*mut c_char; 4];
    /// The most-recently dequeued chat character (internal use).
    pub static mut chat_char: c_char;
    /// True while a chat message is being composed.
    pub static mut chat_on: c_int;
    /// When true, the "message_dontfuckwithme" flag suppresses the next msg.
    pub static mut message_dontfuckwithme: c_int;
    /// Level-name strings for DOOM shareware/registered/retail (36 real + 9 placeholder).
    pub static mut mapnames: [*mut c_char; 45];
    /// Level-name strings for commercial IWADs (32 DOOM2 + 32 Plutonia + 32 TNT).
    pub static mut mapnames_commercial: [*mut c_char; 96];
}

// ---------------------------------------------------------------------------
// am_map.c
// ---------------------------------------------------------------------------

extern "C" {
    /// True while the automap is open.
    pub static mut automapactive: c_int;
}

// ---------------------------------------------------------------------------
// i_scale.c
// Screen-scaling mode descriptors.
// ---------------------------------------------------------------------------

/// A screen scaling mode descriptor, matching `screen_mode_t` in `i_video.h`.
///
/// Layout on 64-bit (32 bytes):
///   +0  width          (int, 4 bytes)
///   +4  height         (int, 4 bytes)
///   +8  InitMode       (fn ptr, 8 bytes)
///   +16 DrawScreen     (fn ptr, 8 bytes)
///   +24 poor_quality   (int/boolean, 4 bytes)
///   +28 [4 bytes tail-padding to align struct to 8]
#[repr(C)]
pub struct screen_mode_t {
    /// Output buffer width in pixels.
    pub width: c_int,
    /// Output buffer height in pixels.
    pub height: c_int,
    /// Optional initialiser called once with the game palette.
    pub init_mode: Option<unsafe extern "C" fn(*mut u8)>,
    /// Draw function: copies src buffer → dest buffer for the given rectangle.
    pub draw_screen: Option<unsafe extern "C" fn(c_int, c_int, c_int, c_int) -> c_int>,
    /// True when this mode uses blended interpolation (lower visual quality).
    pub poor_quality: c_int,
}

extern "C" {
    // Direct pixel-double scale modes (320×200 → N×(320×200))
    pub static mut mode_scale_1x: screen_mode_t; // 320×200
    pub static mut mode_scale_2x: screen_mode_t; // 640×400
    pub static mut mode_scale_3x: screen_mode_t; // 960×600
    pub static mut mode_scale_4x: screen_mode_t; // 1280×800
    pub static mut mode_scale_5x: screen_mode_t; // 1600×1000

    // Vertically-stretched modes (320×200 → N×(320×240))
    pub static mut mode_stretch_1x: screen_mode_t; // 320×240  (poor)
    pub static mut mode_stretch_2x: screen_mode_t; // 640×480
    pub static mut mode_stretch_3x: screen_mode_t; // 960×720
    pub static mut mode_stretch_4x: screen_mode_t; // 1280×960
    pub static mut mode_stretch_5x: screen_mode_t; // 1600×1200

    // Horizontally-squashed modes (320×200 → N×(256×200))
    pub static mut mode_squash_1x: screen_mode_t; // 256×200  (poor)
    pub static mut mode_squash_2x: screen_mode_t; // 512×400
    pub static mut mode_squash_3x: screen_mode_t; // 800×600  (quirk: not 768×600)
    pub static mut mode_squash_4x: screen_mode_t; // 1024×800
    pub static mut mode_squash_5x: screen_mode_t; // 1280×1000
}

// ---------------------------------------------------------------------------
// Additional constants from C headers
// ---------------------------------------------------------------------------

/// Screen width used by the "squash" scale modes (SCREENWIDTH_4_3 in i_video.h).
pub const SCREENWIDTH_4_3: c_int = 256;
/// Screen height used by the "stretch" scale modes (SCREENHEIGHT_4_3 in i_video.h).
pub const SCREENHEIGHT_4_3: c_int = 240;

/// Ticks per second (TICRATE in i_timer.h).
pub const TICRATE: c_int = 35;

/// First printable character in the HUD font (HU_FONTSTART in hu_stuff.h).
pub const HU_FONTSTART: u8 = b'!'; // 33
/// Last printable character in the HUD font (HU_FONTEND in hu_stuff.h).
pub const HU_FONTEND: u8 = b'_'; // 95
/// Number of glyphs in the HUD font (HU_FONTSIZE = HU_FONTEND - HU_FONTSTART + 1).
pub const HU_FONTSIZE: usize = (HU_FONTEND - HU_FONTSTART + 1) as usize; // 63

/// Broadcast player index (HU_BROADCAST in hu_stuff.h).
pub const HU_BROADCAST: c_int = 5;
/// HUD message area width in characters (HU_MSGWIDTH in hu_stuff.h).
pub const HU_MSGWIDTH: c_int = 64;
/// HUD message area height in lines (HU_MSGHEIGHT in hu_stuff.h).
pub const HU_MSGHEIGHT: c_int = 1;

/// Size of the body-object circular queue (BODYQUESIZE in g_game.c).
pub const BODYQUESIZE: usize = 32;
/// Number of tics during which slow-turn speed is used before switching to
/// normal turn speed (SLOWTURNTICS in g_game.c).
pub const SLOWTURNTICS: c_int = 6;
/// Speed threshold above which the turbo-cheat detector fires
/// (TURBOTHRESHOLD in g_game.c = 0x32 = 50).
pub const TURBOTHRESHOLD: c_int = 0x32;

/// Weapon-sprite lower speed (LOWERSPEED in p_pspr.c = FRACUNIT × 6).
pub const LOWERSPEED: c_int = FRACUNIT * 6;
/// Weapon-sprite raise speed (RAISESPEED in p_pspr.c = FRACUNIT × 6).
pub const RAISESPEED: c_int = FRACUNIT * 6;
/// Y-position of the weapon at its lowest (off-screen) rest point
/// (WEAPONBOTTOM in p_pspr.c = 128 × FRACUNIT).
pub const WEAPONBOTTOM: c_int = 128 * FRACUNIT;
/// Y-position of the weapon at its highest (ready) position
/// (WEAPONTOP in p_pspr.c = 32 × FRACUNIT).
pub const WEAPONTOP: c_int = 32 * FRACUNIT;

/// Minimum sprite Z distance; sprites closer than this are not projected
/// (MINZ in r_things.c = FRACUNIT × 4).
pub const MINZ: c_int = FRACUNIT * 4;
/// Vertical centre of the screen in pixels, used for sprite projection
/// (BASEYCENTER in r_things.c = 100).
pub const BASEYCENTER: c_int = 100;

// ---------------------------------------------------------------------------
// p_inter.c — ammo and interaction constants
// ---------------------------------------------------------------------------

/// Number of ammo types (NUMAMMO in doomdef.h = 4: clip, shell, cell, misl).
pub const NUMAMMO: usize = 4;
/// Bonus count added per pick-up for the screen flash effect
/// (BONUSADD in p_inter.c = 6).
pub const BONUSADD: c_int = 6;

// ---------------------------------------------------------------------------
// p_mobj.c — map-object physics constants
// ---------------------------------------------------------------------------

/// Velocity below which horizontal momentum is zeroed (STOPSPEED = 0x1000).
pub const STOPSPEED: c_int = 0x1000;
/// Friction factor applied to horizontal velocity each tic (FRICTION = 0xe800).
pub const FRICTION: c_int = 0xe800_u32 as c_int;

// ---------------------------------------------------------------------------
// p_spec.c — sector-special and animation constants / globals
// ---------------------------------------------------------------------------

/// Maximum number of running floor/ceiling/door animations at once
/// (MAXANIMS in p_spec.c = 32).
pub const MAXANIMS: c_int = 32;
/// Maximum number of active line specials in a level
/// (MAXLINEANIMS in p_spec.c = 64).
pub const MAXLINEANIMS: c_int = 64;

/// Glow effect speed (GLOWSPEED in p_spec.h = 8).
pub const GLOWSPEED: c_int = 8;
/// Bright strobe level (STROBEBRIGHT in p_spec.h = 5).
pub const STROBEBRIGHT: c_int = 5;
/// Fast strobe dark duration in tics (FASTDARK in p_spec.h = 15).
pub const FASTDARK: c_int = 15;
/// Slow strobe dark duration in tics (SLOWDARK in p_spec.h = 35).
pub const SLOWDARK: c_int = 35;

/// Vertical door speed (VDOORSPEED = FRACUNIT × 2).
pub const VDOORSPEED: c_int = FRACUNIT * 2;
/// Vertical door wait time in tics (VDOORWAIT = 150).
pub const VDOORWAIT: c_int = 150;

/// Ceiling movement speed (CEILSPEED = FRACUNIT).
pub const CEILSPEED: c_int = FRACUNIT;
/// Maximum simultaneously active ceilings (MAXCEILINGS = 30).
pub const MAXCEILINGS: c_int = 30;
/// Ceiling movement wait time in tics (CEILWAIT = 150).
pub const CEILWAIT: c_int = 150;

/// Platform movement speed (PLATSPEED = FRACUNIT).
pub const PLATSPEED: c_int = FRACUNIT;
/// Maximum simultaneously active platforms (MAXPLATS = 30).
pub const MAXPLATS: c_int = 30;
/// Platform wait time in seconds (PLATWAIT = 3).
pub const PLATWAIT: c_int = 3;

/// Floor movement speed (FLOORSPEED = FRACUNIT).
pub const FLOORSPEED: c_int = FRACUNIT;

extern "C" {
    /// True when a level timer is active (set by map 96 line special).
    pub static mut levelTimer: c_int; // boolean
    /// Remaining tic count for the level timer.
    pub static mut levelTimeCount: c_int;
    /// Number of active line specials in the current level.
    pub static mut numlinespecials: c_short;
}

// ---------------------------------------------------------------------------
// p_switch.c — switch/button constants and globals
// ---------------------------------------------------------------------------

/// Maximum number of switch texture pairs in the alpha switch list
/// (MAXSWITCHES in p_spec.h = 50).
pub const MAXSWITCHES: usize = 50;
/// Maximum number of simultaneously active timed buttons
/// (MAXBUTTONS in p_spec.h = 16).
pub const MAXBUTTONS: usize = 16;
/// Duration a button stays pressed before reverting (BUTTONTIME = 35 = 1 sec).
pub const BUTTONTIME: c_int = 35;

extern "C" {
    /// Flat array of (texture1, texture2) pairs; length = numswitches × 2.
    pub static mut switchlist: [c_int; 100]; // MAXSWITCHES * 2
    /// Number of valid switch pairs initialised by P_InitSwitchList.
    pub static mut numswitches: c_int;
}

// ---------------------------------------------------------------------------
// p_map.c — collision detection globals and constants
// ---------------------------------------------------------------------------

/// Sentinel magic value used to detect the vanilla spechit overflow bug
/// (DEFAULT_SPECHIT_MAGIC in p_map.c = 0x01C09C98).
pub const DEFAULT_SPECHIT_MAGIC: c_uint = 0x01C09C98;

extern "C" {
    /// Bounding box of the thing being tested for movement.
    pub static mut tmbbox: [c_int; 4];
    /// MF_* flags of the thing being tested.
    pub static mut tmflags: c_int;
    /// X coordinate of the thing being tested.
    pub static mut tmx: c_int;
    /// Y coordinate of the thing being tested.
    pub static mut tmy: c_int;
    /// True when the move would be valid if within tmfloorz..tmceilingz.
    pub static mut floatok: c_int; // boolean
    /// Floor Z at the test position.
    pub static mut tmfloorz: c_int;
    /// Ceiling Z at the test position.
    pub static mut tmceilingz: c_int;
    /// Floor Z for drop-off testing.
    pub static mut tmdropoffz: c_int;
    /// Number of lines hit during the current P_CheckPosition call.
    pub static mut numspechit: c_int;
    /// Z height of the shoot ray origin.
    pub static mut shootz: c_int;
    /// Damage of the current ranged attack (0 = aim only).
    pub static mut la_damage: c_int;
    /// Range of the current attack in fixed-point map units.
    pub static mut attackrange: c_int;
    /// Vertical slope of the aiming trace.
    pub static mut aimslope: c_int;
    /// Fraction along the slide path to the nearest wall.
    pub static mut bestslidefrac: c_int;
    /// Fraction to the second-closest slide wall.
    pub static mut secondslidefrac: c_int;
    /// Damage radius for the current P_RadiusAttack call.
    pub static mut bombdamage: c_int;
}

// ---------------------------------------------------------------------------
// f_finale.c — end-of-episode cast/finale globals and constants
// ---------------------------------------------------------------------------

/// Speed at which text crawls during the text finale (TEXTSPEED = 3 tics/char).
pub const TEXTSPEED: c_int = 3;
/// Tics to wait after the text is fully displayed (TEXTWAIT = 250).
pub const TEXTWAIT: c_int = 250;

extern "C" {
    /// Pointer to the current finale text string (NULL before F_StartFinale).
    pub static mut finaletext: *mut c_char;
    /// Name of the flat used as the finale background.
    pub static mut finaleflat: *mut c_char;
    /// Index of the current cast-roll monster (0-based).
    pub static mut castnum: c_int;
    /// Tics remaining in the current cast frame.
    pub static mut casttics: c_int;
    /// True when the current cast monster is playing its death animation.
    pub static mut castdeath: c_int; // boolean
    /// Current frame number within the cast animation.
    pub static mut castframes: c_int;
    /// True when the cast monster is on a melee-attack frame.
    pub static mut castonmelee: c_int; // boolean
    /// True while the cast monster is playing an attack animation.
    pub static mut castattacking: c_int; // boolean
}

// ---------------------------------------------------------------------------
// d_loop.c — main game-loop state globals and constants
// ---------------------------------------------------------------------------

/// Number of tic-command slots in the ring buffer
/// (BACKUPTICS in net_defs.h = 128).
pub const BACKUPTICS: usize = 128;

extern "C" {
    /// Current game tic (incremented once per rendered frame).
    pub static mut gametic: c_int;
    /// Tic-duplication factor (1 = normal, 2 = send every 2nd tic, etc.).
    pub static mut ticdup: c_int;
    /// When true, run exactly one tic per frame (demo-timing mode).
    pub static mut singletics: c_int; // boolean
}

// ---------------------------------------------------------------------------
// p_saveg.c — save-game serialization constants and globals
// ---------------------------------------------------------------------------

/// End-of-file marker byte written at the end of every save game
/// (SAVEGAME_EOF in p_saveg.c = 0x1d).
pub const SAVEGAME_EOF: u8 = 0x1d;
/// Length of the version string embedded in save-game headers
/// (VERSIONSIZE in p_saveg.c = 16).
pub const VERSIONSIZE: usize = 16;

extern "C" {
    /// Total number of bytes written to the current save-game stream.
    pub static mut savegamelength: c_int;
    /// True when a write error has occurred during save serialization.
    pub static mut savegame_error: c_int; // boolean
}

// ---------------------------------------------------------------------------
// p_setup.c — level-loading globals and constants
// ---------------------------------------------------------------------------

/// Maximum number of deathmatch start positions in a level
/// (MAX_DEATHMATCH_STARTS in p_setup.c = 10).
pub const MAX_DEATHMATCH_STARTS: usize = 10;

extern "C" {
    /// Total number of vertexes in the loaded level.
    pub static mut numvertexes: c_int;
    /// Total number of BSP line segments.
    pub static mut numsegs: c_int;
    /// Total number of sectors in the loaded level.
    pub static mut numsectors: c_int;
    /// Total number of BSP subsectors.
    pub static mut numsubsectors: c_int;
    /// Total number of BSP nodes.
    pub static mut numnodes: c_int;
    /// Total number of linedefs in the loaded level.
    pub static mut numlines: c_int;
    /// Total number of sidedefs in the loaded level.
    pub static mut numsides: c_int;
    /// Width of the blockmap grid in map blocks.
    pub static mut bmapwidth: c_int;
    /// Height of the blockmap grid in map blocks.
    pub static mut bmapheight: c_int;
    /// X origin of the blockmap in fixed-point map units.
    pub static mut bmaporgx: c_int;
    /// Y origin of the blockmap in fixed-point map units.
    pub static mut bmaporgy: c_int;
}

// ---------------------------------------------------------------------------
// p_enemy.c — enemy AI globals and constants
// ---------------------------------------------------------------------------

/// Angle turned per tic when pursuing a lost target
/// (TRACEANGLE in p_enemy.c = 0xc000000).
pub const TRACEANGLE: c_uint = 0xc000000;
/// Arc spread angle for the Mancubus fireball pattern
/// (FATSPREAD = ANG90 / 8).
pub const FATSPREAD: c_uint = ANG90 / 8;
/// Speed of the Lost Soul charge attack in fixed-point units/tic
/// (SKULLSPEED = 20 × FRACUNIT).
pub const SKULLSPEED: c_int = 20 * FRACUNIT;

extern "C" {
    /// Per-direction X velocity multipliers for 8-directional monster movement.
    /// Values: {FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000, 0, 47000}.
    pub static mut xspeed: [c_int; 8];
    /// Per-direction Y velocity multipliers for 8-directional monster movement.
    /// Values: {0, 47000, FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000}.
    pub static mut yspeed: [c_int; 8];
    /// Array of potential brain-teleport destinations (Spider Mastermind).
    pub static mut braintargets: [*mut c_void; 32];
    /// Number of valid braintargets entries (set by P_SpawnBrainTargets).
    pub static mut numbraintargets: c_int;
    /// Index of the next braintarget to use in the round-robin sequence.
    pub static mut braintargeton: c_int;
}

// ---------------------------------------------------------------------------
// d_main.c — startup flag defaults and global state
// ---------------------------------------------------------------------------

extern "C" {
    /// True when the game was started with -devparm (developer mode).
    pub static mut devparm: c_int; // boolean
    /// True when -nomonsters was passed; all monsters are skipped.
    pub static mut nomonsters: c_int; // boolean
    /// True when -respawn was passed; item respawning is forced on.
    pub static mut respawnparm: c_int; // boolean
    /// True when -fast was passed; monster attack speed is doubled.
    pub static mut fastparm: c_int; // boolean
    /// Episode to start at (set by -episode).
    pub static mut startepisode: c_int;
    /// Map to start at (set by -warp / -episode).
    pub static mut startmap: c_int;
    /// True when -warp / -episode has provided an explicit start.
    pub static mut autostart: c_int; // boolean
    /// True while playing back a built-in demo sequence.
    pub static mut advancedemo: c_int; // boolean
    /// True when -record is combined with playing a demo (store-demo mode).
    pub static mut storedemo: c_int; // boolean
    /// True when the BFG edition of the IWAD is detected.
    pub static mut bfgedition: c_int; // boolean
    /// True once the main event loop has started.
    pub static mut main_loop_started: c_int; // boolean
    /// When non-zero, display the ENDOOM text on exit.
    pub static mut show_endoom: c_int;
}

// ---------------------------------------------------------------------------
// wi_stuff.c — intermission screen constants
// ---------------------------------------------------------------------------

/// Number of episodes (NUMEPISODES in wi_stuff.c = 4).
pub const WI_NUMEPISODES: usize = 4;
/// Number of maps per episode (NUMMAPS in wi_stuff.c = 9).
pub const WI_NUMMAPS: usize = 9;
/// Y-position of the level-name title on the intermission screen (WI_TITLEY = 2).
pub const WI_TITLEY: c_int = 2;
/// Vertical spacing between player stats rows (WI_SPACINGY = 33).
pub const WI_SPACINGY: c_int = 33;
/// X-position of single-player stats (SP_STATSX = 50).
pub const SP_STATSX: c_int = 50;
/// Y-position of single-player stats (SP_STATSY = 50).
pub const SP_STATSY: c_int = 50;
/// X-position of the time display (SP_TIMEX = 16).
pub const SP_TIMEX: c_int = 16;
/// Y-position of the time display (SP_TIMEY = SCREENHEIGHT − 32).
pub const SP_TIMEY: c_int = SCREENHEIGHT - 32;
/// Delay in tics before showing the "next level" location (SHOWNEXTLOCDELAY = 4).
pub const SHOWNEXTLOCDELAY: c_int = 4;
/// X spacing between deathmatch matrix columns (DM_SPACINGX = 40).
pub const DM_SPACINGX: c_int = 40;

// ---------------------------------------------------------------------------
// st_stuff.c — status bar constants
// ---------------------------------------------------------------------------

/// Palette index at which red (pain) palette rotations begin (STARTREDPALS = 1).
pub const STARTREDPALS: c_int = 1;
/// Palette index at which bonus (item pick-up) rotations begin (STARTBONUSPALS = 9).
pub const STARTBONUSPALS: c_int = 9;
/// Number of red pain palette entries (NUMREDPALS = 8).
pub const NUMREDPALS: c_int = 8;
/// Number of bonus pick-up palette entries (NUMBONUSPALS = 4).
pub const NUMBONUSPALS: c_int = 4;
/// Palette index used for the radiation suit overlay (RADIATIONPAL = 13).
pub const RADIATIONPAL: c_int = 13;
/// 1-in-N probability of a random face change per tic (ST_FACEPROBABILITY = 96).
pub const ST_FACEPROBABILITY: c_int = 96;
/// Number of pain-level face groups (ST_NUMPAINFACES = 5).
pub const ST_NUMPAINFACES: c_int = 5;
/// Number of straight (non-turning) faces per pain group (ST_NUMSTRAIGHTFACES = 3).
pub const ST_NUMSTRAIGHTFACES: c_int = 3;
/// Number of turning face variants per pain group (ST_NUMTURNFACES = 2).
pub const ST_NUMTURNFACES: c_int = 2;
/// Number of special face frames (ST_NUMSPECIALFACES = 3: ouch, evil-grin, rampage).
pub const ST_NUMSPECIALFACES: c_int = 3;
/// Number of extra face frames beyond the pain table (ST_NUMEXTRAFACES = 2: god, dead).
pub const ST_NUMEXTRAFACES: c_int = 2;
/// Total face sprite count = pain × stride + extra.
pub const ST_NUMFACES: c_int =
    ST_NUMPAINFACES * (ST_NUMSTRAIGHTFACES + ST_NUMTURNFACES + ST_NUMSPECIALFACES)
        + ST_NUMEXTRAFACES;
/// Tics of evil-grin display (ST_EVILGRINCOUNT = 2 × TICRATE).
pub const ST_EVILGRINCOUNT: c_int = 2 * TICRATE;
/// Tics of straight-face display (ST_STRAIGHTFACECOUNT = TICRATE / 2).
pub const ST_STRAIGHTFACECOUNT: c_int = TICRATE / 2;
/// Tics before turn-face times out (ST_TURNCOUNT = 1 × TICRATE).
pub const ST_TURNCOUNT: c_int = TICRATE;
/// Tics of ouch-face display (ST_OUCHCOUNT = 1 × TICRATE).
pub const ST_OUCHCOUNT: c_int = TICRATE;
/// Tics of rampage-face display (ST_RAMPAGEDELAY = 2 × TICRATE).
pub const ST_RAMPAGEDELAY: c_int = 2 * TICRATE;
/// Damage threshold for the rampage face (ST_MUCHPAIN = 20).
pub const ST_MUCHPAIN: c_int = 20;
/// X pixel offset of the status bar (ST_X = 0).
pub const ST_X: c_int = 0;
/// X pixel offset of the arms display (ST_X2 = 104).
pub const ST_X2: c_int = 104;

// ---------------------------------------------------------------------------
// am_map.c — automap constants
// ---------------------------------------------------------------------------

/// Number of mark points the player can drop on the automap
/// (AM_NUMMARKPOINTS = 10).
pub const AM_NUMMARKPOINTS: usize = 10;
/// Initial scale factor (map-to-frame) expressed as a fraction of FRACUNIT
/// (INITSCALEMTOF = 0.2 × FRACUNIT).
pub const INITSCALEMTOF: c_int = (0.2 * FRACUNIT as f64) as c_int;
/// Zoom-in factor per tic (M_ZOOMIN = 1.02 × FRACUNIT, truncated).
pub const M_ZOOMIN: c_int = (1.02 * FRACUNIT as f64) as c_int;
/// Zoom-out factor per tic (M_ZOOMOUT = FRACUNIT / 1.02, truncated).
pub const M_ZOOMOUT: c_int = (FRACUNIT as f64 / 1.02) as c_int;
/// Pan speed in map units per tic (F_PANINC = 4).
pub const F_PANINC: c_int = 4;
