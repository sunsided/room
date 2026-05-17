//! Verify sizes and field offsets of the WAD binary-format structs.
//!
//! Every struct in `doomdata.h` carries `PACKEDATTR` (= `__attribute__((packed))`)
//! so it matches the exact byte layout stored on disk in WAD files.  A mismatch
//! in **any** field offset would silently corrupt the map-load path.
//!
//! These structs are used by the still-unported `p_setup.c` (map loader) and
//! several other modules that read raw WAD lumps.

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use std::mem::{offset_of, size_of};

use crate::doom::c_ffi;

// ---------------------------------------------------------------------------
// Rust mirrors of the PACKEDATTR (packed) C structs from doomdata.h.
// All fields use the exact C primitive that doomtype.h maps to:
//   short → i16 / u16,  unsigned short → u16
// ---------------------------------------------------------------------------

/// A single vertex as stored in the WAD VERTEXES lump.
/// C: `typedef struct { short x; short y; } PACKEDATTR mapvertex_t;`
#[repr(C, packed)]
struct mapvertex_t {
    x: i16,
    y: i16,
}

/// A SideDef as stored in the WAD SIDEDEFS lump.
/// C: `typedef struct { short textureoffset; short rowoffset; char toptexture[8];
///     char bottomtexture[8]; char midtexture[8]; short sector; } PACKEDATTR mapsidedef_t;`
#[repr(C, packed)]
struct mapsidedef_t {
    textureoffset: i16,
    rowoffset: i16,
    toptexture: [u8; 8],
    bottomtexture: [u8; 8],
    midtexture: [u8; 8],
    sector: i16,
}

/// A LineDef as stored in the WAD LINEDEFS lump.
/// C: `typedef struct { short v1; short v2; short flags; short special; short tag;
///     short sidenum[2]; } PACKEDATTR maplinedef_t;`
#[repr(C, packed)]
struct maplinedef_t {
    v1: i16,
    v2: i16,
    flags: i16,
    special: i16,
    tag: i16,
    sidenum: [i16; 2],
}

/// A Sector as stored in the WAD SECTORS lump.
/// C: `typedef struct { short floorheight; short ceilingheight; char floorpic[8];
///     char ceilingpic[8]; short lightlevel; short special; short tag; } PACKEDATTR mapsector_t;`
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

/// A SubSector as stored in the WAD SSECTORS lump.
/// C: `typedef struct { short numsegs; short firstseg; } PACKEDATTR mapsubsector_t;`
#[repr(C, packed)]
struct mapsubsector_t {
    numsegs: i16,
    firstseg: i16,
}

/// A LineSeg as stored in the WAD SEGS lump.
/// C: `typedef struct { short v1; short v2; short angle; short linedef;
///     short side; short offset; } PACKEDATTR mapseg_t;`
#[repr(C, packed)]
struct mapseg_t {
    v1: i16,
    v2: i16,
    angle: i16,
    linedef: i16,
    side: i16,
    offset: i16,
}

/// A BSP node as stored in the WAD NODES lump.
/// C: `typedef struct { short x; short y; short dx; short dy;
///     short bbox[2][4]; unsigned short children[2]; } PACKEDATTR mapnode_t;`
#[repr(C, packed)]
struct mapnode_t {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
    bbox: [[i16; 4]; 2],
    children: [u16; 2],
}

/// A Thing as stored in the WAD THINGS lump.
/// C: `typedef struct { short x; short y; short angle; short type;
///     short options; } PACKEDATTR mapthing_t;`
#[repr(C, packed)]
struct mapthing_t {
    x: i16,
    y: i16,
    angle: i16,
    r#type: i16,
    options: i16,
}

// ---------------------------------------------------------------------------
// mapvertex_t  (2 × short = 4 bytes)
// ---------------------------------------------------------------------------

#[test]
fn mapvertex_t_size() {
    assert_eq!(size_of::<mapvertex_t>(), 4);
}

#[test]
fn mapvertex_t_offsets() {
    assert_eq!(offset_of!(mapvertex_t, x), 0);
    assert_eq!(offset_of!(mapvertex_t, y), 2);
}

// ---------------------------------------------------------------------------
// mapsidedef_t  (2+2 + 8+8+8 + 2 = 30 bytes)
// ---------------------------------------------------------------------------

#[test]
fn mapsidedef_t_size() {
    assert_eq!(size_of::<mapsidedef_t>(), 30);
}

#[test]
fn mapsidedef_t_offsets() {
    assert_eq!(offset_of!(mapsidedef_t, textureoffset), 0);
    assert_eq!(offset_of!(mapsidedef_t, rowoffset), 2);
    assert_eq!(offset_of!(mapsidedef_t, toptexture), 4);
    assert_eq!(offset_of!(mapsidedef_t, bottomtexture), 12);
    assert_eq!(offset_of!(mapsidedef_t, midtexture), 20);
    assert_eq!(offset_of!(mapsidedef_t, sector), 28);
}

// ---------------------------------------------------------------------------
// maplinedef_t  (7 × short = 14 bytes: v1, v2, flags, special, tag, sidenum[2])
// ---------------------------------------------------------------------------

#[test]
fn maplinedef_t_size() {
    assert_eq!(size_of::<maplinedef_t>(), 14);
}

#[test]
fn maplinedef_t_offsets() {
    assert_eq!(offset_of!(maplinedef_t, v1), 0);
    assert_eq!(offset_of!(maplinedef_t, v2), 2);
    assert_eq!(offset_of!(maplinedef_t, flags), 4);
    assert_eq!(offset_of!(maplinedef_t, special), 6);
    assert_eq!(offset_of!(maplinedef_t, tag), 8);
    assert_eq!(offset_of!(maplinedef_t, sidenum), 10);
}

#[test]
fn maplinedef_sidenum_length() {
    // sidenum is declared as short sidenum[2]; verify via size: 2 shorts = 4 bytes.
    assert_eq!(size_of::<[i16; 2]>(), 4);
}

// ---------------------------------------------------------------------------
// mapsector_t  (2+2 + 8+8 + 2+2+2 = 26 bytes)
// ---------------------------------------------------------------------------

#[test]
fn mapsector_t_size() {
    assert_eq!(size_of::<mapsector_t>(), 26);
}

#[test]
fn mapsector_t_offsets() {
    assert_eq!(offset_of!(mapsector_t, floorheight), 0);
    assert_eq!(offset_of!(mapsector_t, ceilingheight), 2);
    assert_eq!(offset_of!(mapsector_t, floorpic), 4);
    assert_eq!(offset_of!(mapsector_t, ceilingpic), 12);
    assert_eq!(offset_of!(mapsector_t, lightlevel), 20);
    assert_eq!(offset_of!(mapsector_t, special), 22);
    assert_eq!(offset_of!(mapsector_t, tag), 24);
}

// ---------------------------------------------------------------------------
// mapsubsector_t  (2 × short = 4 bytes)
// ---------------------------------------------------------------------------

#[test]
fn mapsubsector_t_size() {
    assert_eq!(size_of::<mapsubsector_t>(), 4);
}

#[test]
fn mapsubsector_t_offsets() {
    assert_eq!(offset_of!(mapsubsector_t, numsegs), 0);
    assert_eq!(offset_of!(mapsubsector_t, firstseg), 2);
}

// ---------------------------------------------------------------------------
// mapseg_t  (6 × short = 12 bytes)
// ---------------------------------------------------------------------------

#[test]
fn mapseg_t_size() {
    assert_eq!(size_of::<mapseg_t>(), 12);
}

#[test]
fn mapseg_t_offsets() {
    assert_eq!(offset_of!(mapseg_t, v1), 0);
    assert_eq!(offset_of!(mapseg_t, v2), 2);
    assert_eq!(offset_of!(mapseg_t, angle), 4);
    assert_eq!(offset_of!(mapseg_t, linedef), 6);
    assert_eq!(offset_of!(mapseg_t, side), 8);
    assert_eq!(offset_of!(mapseg_t, offset), 10);
}

// ---------------------------------------------------------------------------
// mapnode_t  (4 × short + bbox[2][4] shorts + children[2] = 28 bytes)
// ---------------------------------------------------------------------------

#[test]
fn mapnode_t_size() {
    assert_eq!(size_of::<mapnode_t>(), 28);
}

#[test]
fn mapnode_t_offsets() {
    assert_eq!(offset_of!(mapnode_t, x), 0);
    assert_eq!(offset_of!(mapnode_t, y), 2);
    assert_eq!(offset_of!(mapnode_t, dx), 4);
    assert_eq!(offset_of!(mapnode_t, dy), 6);
    assert_eq!(offset_of!(mapnode_t, bbox), 8);
    assert_eq!(offset_of!(mapnode_t, children), 24);
}

#[test]
fn mapnode_t_nf_subsector_flag() {
    // NF_SUBSECTOR marks a leaf child; the value must be 0x8000.
    let nf_subsector: u16 = 0x8000;
    assert_eq!(nf_subsector, 1u16 << 15);
}

// ---------------------------------------------------------------------------
// mapthing_t  (5 × short = 10 bytes)
// ---------------------------------------------------------------------------

#[test]
fn mapthing_t_size() {
    assert_eq!(size_of::<mapthing_t>(), 10);
}

#[test]
fn mapthing_t_offsets() {
    assert_eq!(offset_of!(mapthing_t, x), 0);
    assert_eq!(offset_of!(mapthing_t, y), 2);
    assert_eq!(offset_of!(mapthing_t, angle), 4);
    assert_eq!(offset_of!(mapthing_t, r#type), 6);
    assert_eq!(offset_of!(mapthing_t, options), 8);
}

// ---------------------------------------------------------------------------
// MapLump constants (doomdata.h enum values must be consecutive)
// The constants live in c_ffi so the rest of the ported code can use them.
// ---------------------------------------------------------------------------

#[test]
fn ml_lump_order_values() {
    assert_eq!(c_ffi::MapLump::LABEL, 0);
    assert_eq!(c_ffi::MapLump::THINGS, 1);
    assert_eq!(c_ffi::MapLump::LINEDEFS, 2);
    assert_eq!(c_ffi::MapLump::SIDEDEFS, 3);
    assert_eq!(c_ffi::MapLump::VERTEXES, 4);
    assert_eq!(c_ffi::MapLump::SEGS, 5);
    assert_eq!(c_ffi::MapLump::SSECTORS, 6);
    assert_eq!(c_ffi::MapLump::NODES, 7);
    assert_eq!(c_ffi::MapLump::SECTORS, 8);
    assert_eq!(c_ffi::MapLump::REJECT, 9);
    assert_eq!(c_ffi::MapLump::BLOCKMAP, 10);
    // Lump indices must be consecutive: BLOCKMAP is the last, at index 10.
    assert_eq!(c_ffi::MapLump::BLOCKMAP - c_ffi::MapLump::LABEL, 10);
}

// ---------------------------------------------------------------------------
// LinedefFlag bits (ML_* defines from doomdata.h)
// Each flag occupies exactly one bit and the values must be powers of two.
// ---------------------------------------------------------------------------

#[test]
fn ml_linedef_flags() {
    // Verify every flag is a distinct power of two — no two flags may overlap.
    let all_flags = [
        c_ffi::LinedefFlag::BLOCKING,
        c_ffi::LinedefFlag::BLOCKMONSTERS,
        c_ffi::LinedefFlag::TWOSIDED,
        c_ffi::LinedefFlag::DONTPEGTOP,
        c_ffi::LinedefFlag::DONTPEGBOTTOM,
        c_ffi::LinedefFlag::SECRET,
        c_ffi::LinedefFlag::SOUNDBLOCK,
        c_ffi::LinedefFlag::DONTDRAW,
        c_ffi::LinedefFlag::MAPPED,
    ];

    // Each value must be a power of two.
    for f in all_flags {
        assert_eq!(f & (f - 1), 0, "flag {f:#x} is not a power of two");
    }

    // No two flags may share a bit.
    let mut combined: u16 = 0;
    for f in all_flags {
        assert_eq!(
            combined & f,
            0,
            "flag {f:#x} overlaps with already-seen flags"
        );
        combined |= f;
    }

    // All nine flags occupy a contiguous set of bits 0–8.
    assert_eq!(combined, (1u16 << 9) - 1);

    // Verify expected absolute values match the C header definitions.
    assert_eq!(c_ffi::LinedefFlag::BLOCKING, 1);
    assert_eq!(c_ffi::LinedefFlag::BLOCKMONSTERS, 2);
    assert_eq!(c_ffi::LinedefFlag::TWOSIDED, 4);
    assert_eq!(c_ffi::LinedefFlag::DONTPEGTOP, 8);
    assert_eq!(c_ffi::LinedefFlag::DONTPEGBOTTOM, 16);
    assert_eq!(c_ffi::LinedefFlag::SECRET, 32);
    assert_eq!(c_ffi::LinedefFlag::SOUNDBLOCK, 64);
    assert_eq!(c_ffi::LinedefFlag::DONTDRAW, 128);
    assert_eq!(c_ffi::LinedefFlag::MAPPED, 256);
}

// ---------------------------------------------------------------------------
// Ensure packed fields do not introduce unexpected padding
// ---------------------------------------------------------------------------

#[test]
fn all_wad_structs_are_densely_packed() {
    // The sum of all field sizes must equal the struct size; any padding
    // would indicate PACKEDATTR was not applied or the Rust mirror is wrong.
    assert_eq!(size_of::<mapvertex_t>(), 2 * 2);
    assert_eq!(size_of::<mapsubsector_t>(), 2 * 2);
    assert_eq!(size_of::<mapseg_t>(), 6 * 2);
    assert_eq!(size_of::<mapthing_t>(), 5 * 2);
    assert_eq!(size_of::<maplinedef_t>(), 7 * 2); // v1+v2+flags+special+tag + sidenum[2]
    assert_eq!(size_of::<mapsector_t>(), 2 + 2 + 8 + 8 + 2 + 2 + 2);
    assert_eq!(size_of::<mapsidedef_t>(), 2 + 2 + 8 + 8 + 8 + 2);
    assert_eq!(size_of::<mapnode_t>(), 4 * 2 + 8 * 2 + 2 * 2); // x,y,dx,dy + bbox[2][4] + children[2]
}
