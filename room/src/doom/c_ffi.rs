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
