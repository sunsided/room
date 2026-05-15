//! Rust port of vendor/doomgeneric/r_data.c.
//!
//! Texture / flat / colormap data loading, caching, and lookup.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use crate::i_error;
use std::ffi::{c_char, c_int, c_short, c_uint, c_ushort, c_void, CStr};

use crate::types::Boolean;
use std::ptr;

use crate::doom::c_ffi::{mobj_t, sector_t, side_t};
use crate::doom::m_fixed::FRACBITS;
use crate::doom::p_tick::thinker_t;
use crate::doom::z_zone::{PU_CACHE, PU_STATIC};

// ---------------------------------------------------------------------------
// Constants & helpers
// ---------------------------------------------------------------------------

#[inline]
fn SHORT(x: i16) -> i16 {
    x
}
#[inline]
fn LONG(x: c_int) -> c_int {
    x
}

unsafe fn DEH_String(s: *mut c_char) -> *mut c_char {
    s
}

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct mappatch_t {
    originx: i16,
    originy: i16,
    patch: i16,
    stepdir: i16,
    colormap: i16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct maptexture_t {
    name: [c_char; 8],
    masked: c_int,
    width: i16,
    height: i16,
    obsolete: c_int,
    patchcount: i16,
    patches: mappatch_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct texpatch_t {
    originx: i16,
    originy: i16,
    patch: c_int,
}

#[repr(C)]
struct texture_t {
    name: [c_char; 8],
    width: i16,
    height: i16,
    index: c_int,
    next: *mut texture_t,
    patchcount: i16,
    patches: texpatch_t,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct patch_t {
    pub width: i16,
    pub height: i16,
    pub leftoffset: i16,
    pub topoffset: i16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct post_t {
    pub topdelta: u8,
    pub length: u8,
}

pub type column_t = post_t;

#[repr(C)]
#[derive(Clone, Copy)]
struct spriteframe_t {
    pub rotate: c_int,
    pub lump: [c_short; 8],
    pub flip: [u8; 8],
}

#[repr(C)]
struct spritedef_t {
    pub numframes: c_int,
    pub spriteframes: *mut spriteframe_t,
}

// ---------------------------------------------------------------------------
// Externs from other modules
// ---------------------------------------------------------------------------

extern "C" {
    fn I_ConsoleStdout() -> c_int;

    fn M_StringCopy(dest: *mut c_char, src: *const c_char, dest_size: usize) -> Boolean;

    fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void;
    fn Z_Free(ptr: *mut c_void);
    fn Z_ChangeTag2(ptr: *mut c_void, tag: c_int, file: *const c_char, line: c_int);

    fn W_CacheLumpName(name: *mut c_char, tag: c_int) -> *mut c_void;
    fn W_CacheLumpNum(lumpnum: c_int, tag: c_int) -> *mut c_void;
    fn W_CheckNumForName(name: *mut c_char) -> c_int;
    fn W_GetNumForName(name: *mut c_char) -> c_int;
    fn W_LumpLength(lump: c_int) -> c_int;
    fn W_ReleaseLumpName(name: *mut c_char);
    fn W_LumpNameHash(s: *const c_char) -> c_uint;

    fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;

    fn P_MobjThinker(mobj: *mut c_void);

    static mut lumpinfo: *mut crate::doom::w_wad::lumpinfo_t;

    static mut numsectors: c_int;
    static mut sectors: *mut sector_t;
    static mut numsides: c_int;
    static mut sides: *mut side_t;
    static mut skytexture: c_int;
    static mut thinkercap: thinker_t;
    static mut numsprites: c_int;
    static mut sprites: *mut spritedef_t;

    static mut demoplayback: c_int;
}

// ---------------------------------------------------------------------------
// Globals defined by this module
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut firstflat: c_int = 0;
#[no_mangle]
pub static mut lastflat: c_int = 0;
#[no_mangle]
pub static mut numflats: c_int = 0;

#[no_mangle]
pub static mut firstpatch: c_int = 0;
#[no_mangle]
pub static mut lastpatch: c_int = 0;
#[no_mangle]
pub static mut numpatches: c_int = 0;

#[no_mangle]
pub static mut firstspritelump: c_int = 0;
#[no_mangle]
pub static mut lastspritelump: c_int = 0;
#[no_mangle]
pub static mut numspritelumps: c_int = 0;

#[no_mangle]
pub static mut numtextures: c_int = 0;

static mut textures: *mut *mut texture_t = ptr::null_mut();
static mut textures_hashtable: *mut *mut texture_t = ptr::null_mut();

static mut texturewidthmask: *mut c_int = ptr::null_mut();
#[no_mangle]
pub static mut textureheight: *mut c_int = ptr::null_mut();
static mut texturecompositesize: *mut c_int = ptr::null_mut();
static mut texturecolumnlump: *mut *mut c_short = ptr::null_mut();
static mut texturecolumnofs: *mut *mut c_ushort = ptr::null_mut();
static mut texturecomposite: *mut *mut u8 = ptr::null_mut();

#[no_mangle]
pub static mut flattranslation: *mut c_int = ptr::null_mut();
#[no_mangle]
pub static mut texturetranslation: *mut c_int = ptr::null_mut();

#[no_mangle]
pub static mut spritewidth: *mut c_int = ptr::null_mut();
#[no_mangle]
pub static mut spriteoffset: *mut c_int = ptr::null_mut();
#[no_mangle]
pub static mut spritetopoffset: *mut c_int = ptr::null_mut();

#[no_mangle]
pub static mut colormaps: *mut u8 = ptr::null_mut();

#[no_mangle]
pub static mut flatmemory: c_int = 0;
#[no_mangle]
pub static mut texturememory: c_int = 0;
#[no_mangle]
pub static mut spritememory: c_int = 0;

// ---------------------------------------------------------------------------
// R_DrawColumnInCache
// ---------------------------------------------------------------------------

unsafe fn R_DrawColumnInCache(
    patch: *mut column_t,
    cache: *mut u8,
    originy: c_int,
    cacheheight: c_int,
) {
    let mut patch = patch;
    while (*patch).topdelta != 0xff {
        let source = (patch as *mut u8).add(3);
        let mut count = (*patch).length as c_int;
        let mut position = originy + (*patch).topdelta as c_int;

        if position < 0 {
            count += position;
            position = 0;
        }
        if position + count > cacheheight {
            count = cacheheight - position;
        }
        if count > 0 {
            std::ptr::copy_nonoverlapping(source, cache.add(position as usize), count as usize);
        }
        patch = (patch as *mut u8).add((*patch).length as usize + 4) as *mut column_t;
    }
}

// ---------------------------------------------------------------------------
// R_GenerateComposite
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_GenerateComposite(texnum: c_int) {
    let texture = *textures.add(texnum as usize);

    let block = Z_Malloc(
        *texturecompositesize.add(texnum as usize),
        PU_STATIC,
        texturecomposite.add(texnum as usize) as *mut c_void,
    ) as *mut u8;

    let collump = *texturecolumnlump.add(texnum as usize);
    let colofs = *texturecolumnofs.add(texnum as usize);
    let patchcount = (*texture).patchcount as c_int;
    let patches_base = std::ptr::addr_of!((*texture).patches) as *mut texpatch_t;

    for i in 0..patchcount {
        let patch = patches_base.add(i as usize);
        let realpatch = W_CacheLumpNum((*patch).patch, PU_CACHE) as *mut patch_t;
        let x1 = (*patch).originx as c_int;
        let mut x2 = x1 + SHORT((*realpatch).width) as c_int;

        let mut x = if x1 < 0 { 0 } else { x1 };
        if x2 > (*texture).width as c_int {
            x2 = (*texture).width as c_int;
        }

        let columnofs = (realpatch as *mut u8).add(8) as *mut c_int;
        while x < x2 {
            if *collump.add(x as usize) >= 0 {
                x += 1;
                continue;
            }
            let patchcol = (realpatch as *mut u8)
                .add(LONG(*columnofs.add((x - x1) as usize)) as usize)
                as *mut column_t;
            R_DrawColumnInCache(
                patchcol,
                block.add(*colofs.add(x as usize) as usize),
                (*patch).originy as c_int,
                (*texture).height as c_int,
            );
            x += 1;
        }
    }

    Z_ChangeTag2(block as *mut c_void, PU_CACHE, ptr::null(), 0);
}

// ---------------------------------------------------------------------------
// R_GenerateLookup
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_GenerateLookup(texnum: c_int) {
    let texture = *textures.add(texnum as usize);
    let width = (*texture).width as c_int;

    *texturecomposite.add(texnum as usize) = ptr::null_mut();
    *texturecompositesize.add(texnum as usize) = 0;
    let collump = *texturecolumnlump.add(texnum as usize);
    let colofs = *texturecolumnofs.add(texnum as usize);

    let mut patchcount_ptr: *mut u8 = ptr::null_mut();
    let patchcount_arr = Z_Malloc(
        width,
        PU_STATIC,
        &mut patchcount_ptr as *mut *mut u8 as *mut c_void,
    ) as *mut u8;
    let patchcount = patchcount_ptr; // Z_Malloc wrote the allocated pointer here

    for x in 0..width {
        *patchcount.add(x as usize) = 0;
        *collump.add(x as usize) = 0;
    }

    let patches_base = std::ptr::addr_of!((*texture).patches) as *mut texpatch_t;
    for i in 0..(*texture).patchcount as c_int {
        let patch = patches_base.add(i as usize);
        let realpatch = W_CacheLumpNum((*patch).patch, PU_CACHE) as *mut patch_t;
        let x1 = (*patch).originx as c_int;
        let mut x2 = x1 + SHORT((*realpatch).width) as c_int;

        let mut x = if x1 < 0 { 0 } else { x1 };
        if x2 > width {
            x2 = width;
        }

        let columnofs = (realpatch as *mut u8).add(8) as *mut c_int;
        while x < x2 {
            *patchcount.add(x as usize) += 1;
            *collump.add(x as usize) = (*patch).patch as c_short;
            *colofs.add(x as usize) = (LONG(*columnofs.add((x - x1) as usize)) + 3) as c_ushort;
            x += 1;
        }
    }

    for x in 0..width {
        if *patchcount.add(x as usize) == 0 {
            libc::printf(
                b"R_GenerateLookup: column without a patch (%s)\n\0".as_ptr() as *const c_char,
                (*texture).name.as_ptr(),
            );
            Z_Free(patchcount as *mut c_void);
            return;
        }
        if *patchcount.add(x as usize) > 1 {
            *collump.add(x as usize) = -1;
            *colofs.add(x as usize) = *texturecompositesize.add(texnum as usize) as c_ushort;

            if *texturecompositesize.add(texnum as usize) > 0x10000 - (*texture).height as c_int {
                i_error!("R_GenerateLookup: texture {} is >64k", texnum);
            }
            *texturecompositesize.add(texnum as usize) += (*texture).height as c_int;
        }
    }

    Z_Free(patchcount as *mut c_void);
}

// ---------------------------------------------------------------------------
// R_GetColumn
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_GetColumn(tex: c_int, col: c_int) -> *mut u8 {
    let col = col & *texturewidthmask.add(tex as usize);
    let lump = *(*texturecolumnlump.add(tex as usize)).add(col as usize) as c_int;
    let ofs = *(*texturecolumnofs.add(tex as usize)).add(col as usize) as c_int;

    if lump > 0 {
        return (W_CacheLumpNum(lump, PU_CACHE) as *mut u8).add(ofs as usize);
    }

    if (*texturecomposite.add(tex as usize)).is_null() {
        R_GenerateComposite(tex);
    }

    (*texturecomposite.add(tex as usize)).add(ofs as usize)
}

// ---------------------------------------------------------------------------
// GenerateTextureHashTable
// ---------------------------------------------------------------------------

unsafe fn GenerateTextureHashTable() {
    textures_hashtable = Z_Malloc(
        ((std::mem::size_of::<*mut texture_t>() * numtextures as usize) as c_int) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut *mut texture_t;

    for i in 0..numtextures as usize {
        *textures_hashtable.add(i) = ptr::null_mut();
    }

    for i in 0..numtextures as usize {
        let tex = *textures.add(i);
        (*tex).index = i as c_int;

        let key = (W_LumpNameHash((*tex).name.as_ptr()) % numtextures as c_uint) as usize;
        let mut rover = textures_hashtable.add(key);

        while !(*rover).is_null() {
            rover = std::ptr::addr_of_mut!((**rover).next);
        }

        (*tex).next = ptr::null_mut();
        *rover = tex;
    }
}

// ---------------------------------------------------------------------------
// R_InitTextures
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_InitTextures() {
    let mut name: [c_char; 9] = [0; 9];

    let names =
        W_CacheLumpName(DEH_String(b"PNAMES\0".as_ptr() as *mut c_char), PU_STATIC) as *mut c_int;
    let nummappatches = LONG(*names);
    let name_p = names.add(1) as *mut c_char;

    let patchlookup = Z_Malloc(
        (nummappatches as usize * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;

    for i in 0..nummappatches as usize {
        M_StringCopy(name.as_mut_ptr(), name_p.add(i * 8), name.len());
        *patchlookup.add(i) = W_CheckNumForName(name.as_mut_ptr());
    }
    W_ReleaseLumpName(DEH_String(b"PNAMES\0".as_ptr() as *mut c_char));

    let maptex1 =
        W_CacheLumpName(DEH_String(b"TEXTURE1\0".as_ptr() as *mut c_char), PU_STATIC) as *mut c_int;
    let numtextures1 = LONG(*maptex1);
    let maxoff = W_LumpLength(W_GetNumForName(DEH_String(
        b"TEXTURE1\0".as_ptr() as *mut c_char
    )));
    let mut directory = maptex1.add(1);

    let mut maptex2: *mut c_int = ptr::null_mut();
    let mut numtextures2: c_int = 0;
    let mut maxoff2: c_int = 0;

    if W_CheckNumForName(DEH_String(b"TEXTURE2\0".as_ptr() as *mut c_char)) != -1 {
        maptex2 = W_CacheLumpName(DEH_String(b"TEXTURE2\0".as_ptr() as *mut c_char), PU_STATIC)
            as *mut c_int;
        numtextures2 = LONG(*maptex2);
        maxoff2 = W_LumpLength(W_GetNumForName(DEH_String(
            b"TEXTURE2\0".as_ptr() as *mut c_char
        )));
    }

    numtextures = numtextures1 + numtextures2;

    textures = Z_Malloc(
        (numtextures as usize * std::mem::size_of::<*mut texture_t>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut *mut texture_t;
    texturecolumnlump = Z_Malloc(
        (numtextures as usize * std::mem::size_of::<*mut c_short>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut *mut c_short;
    texturecolumnofs = Z_Malloc(
        (numtextures as usize * std::mem::size_of::<*mut c_ushort>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut *mut c_ushort;
    texturecomposite = Z_Malloc(
        (numtextures as usize * std::mem::size_of::<*mut u8>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut *mut u8;
    texturecompositesize = Z_Malloc(
        (numtextures as usize * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;
    texturewidthmask = Z_Malloc(
        (numtextures as usize * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;
    textureheight = Z_Malloc(
        (numtextures as usize * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;

    let temp1 = W_GetNumForName(DEH_String(b"S_START\0".as_ptr() as *mut c_char));
    let temp2 = W_GetNumForName(DEH_String(b"S_END\0".as_ptr() as *mut c_char)) - 1;
    let temp3 = ((temp2 - temp1 + 63) / 64) + ((numtextures + 63) / 64);

    if I_ConsoleStdout() != 0 {
        libc::printf(b"[\0".as_ptr() as *const c_char);
        for _ in 0..temp3 + 9 {
            libc::printf(b" \0".as_ptr() as *const c_char);
        }
        libc::printf(b"]\0".as_ptr() as *const c_char);
        for _ in 0..temp3 + 10 {
            libc::printf(b"\x08\0".as_ptr() as *const c_char);
        }
    }

    let mut maptex = maptex1;
    let mut maxoff = maxoff;
    let mut directory = directory;

    for i in 0..numtextures as usize {
        if (i & 63) == 0 {
            libc::printf(b".\0".as_ptr() as *const c_char);
        }

        if i == numtextures1 as usize {
            maptex = maptex2;
            maxoff = maxoff2;
            directory = maptex2.add(1);
        }

        let offset = LONG(*directory) as isize;
        directory = directory.add(1);

        if offset > maxoff as isize {
            i_error!("R_InitTextures: bad texture directory");
        }

        let mtexture = (maptex as *mut u8).offset(offset) as *mut maptexture_t;
        let patchcount = SHORT((*mtexture).patchcount) as c_int;
        let texsize = std::mem::size_of::<texture_t>()
            + std::mem::size_of::<texpatch_t>() * (patchcount as usize - 1);
        let texture = Z_Malloc(texsize as c_int, PU_STATIC, ptr::null_mut()) as *mut texture_t;
        *textures.add(i) = texture;

        (*texture).width = SHORT((*mtexture).width);
        (*texture).height = SHORT((*mtexture).height);
        (*texture).patchcount = patchcount as i16;

        std::ptr::copy_nonoverlapping((*mtexture).name.as_ptr(), (*texture).name.as_mut_ptr(), 8);

        let mpatches_base = std::ptr::addr_of!((*mtexture).patches) as *mut mappatch_t;
        let tpatches_base = std::ptr::addr_of!((*texture).patches) as *mut texpatch_t;

        for j in 0..patchcount as usize {
            let mpatch = mpatches_base.add(j);
            let patch = tpatches_base.add(j);
            (*patch).originx = SHORT((*mpatch).originx);
            (*patch).originy = SHORT((*mpatch).originy);
            (*patch).patch = *patchlookup.add(SHORT((*mpatch).patch) as usize);
            if (*patch).patch == -1 {
                i_error!(
                    "R_InitTextures: Missing patch in texture {}",
                    CStr::from_ptr((*texture).name.as_ptr()).to_string_lossy()
                );
            }
        }

        let twidth = (*texture).width as c_int;
        *texturecolumnlump.add(i) = Z_Malloc(
            (twidth as usize * std::mem::size_of::<c_short>()) as c_int,
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut c_short;
        *texturecolumnofs.add(i) = Z_Malloc(
            (twidth as usize * std::mem::size_of::<c_ushort>()) as c_int,
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut c_ushort;

        let mut j = 1;
        while j * 2 <= twidth {
            j <<= 1;
        }
        *texturewidthmask.add(i) = j - 1;
        *textureheight.add(i) = ((*texture).height as c_int) << FRACBITS;
    }

    Z_Free(patchlookup as *mut c_void);
    W_ReleaseLumpName(DEH_String(b"TEXTURE1\0".as_ptr() as *mut c_char));
    if !maptex2.is_null() {
        W_ReleaseLumpName(DEH_String(b"TEXTURE2\0".as_ptr() as *mut c_char));
    }

    for i in 0..numtextures as usize {
        R_GenerateLookup(i as c_int);
    }

    texturetranslation = Z_Malloc(
        ((numtextures as usize + 1) * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;
    for i in 0..numtextures as usize {
        *texturetranslation.add(i) = i as c_int;
    }

    GenerateTextureHashTable();
}

// ---------------------------------------------------------------------------
// R_InitFlats
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_InitFlats() {
    firstflat = W_GetNumForName(DEH_String(b"F_START\0".as_ptr() as *mut c_char)) + 1;
    lastflat = W_GetNumForName(DEH_String(b"F_END\0".as_ptr() as *mut c_char)) - 1;
    numflats = lastflat - firstflat + 1;

    flattranslation = Z_Malloc(
        ((numflats as usize + 1) * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;

    for i in 0..numflats as usize {
        *flattranslation.add(i) = i as c_int;
    }
}

// ---------------------------------------------------------------------------
// R_InitSpriteLumps
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_InitSpriteLumps() {
    firstspritelump = W_GetNumForName(DEH_String(b"S_START\0".as_ptr() as *mut c_char)) + 1;
    lastspritelump = W_GetNumForName(DEH_String(b"S_END\0".as_ptr() as *mut c_char)) - 1;
    numspritelumps = lastspritelump - firstspritelump + 1;

    spritewidth = Z_Malloc(
        (numspritelumps as usize * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;
    spriteoffset = Z_Malloc(
        (numspritelumps as usize * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;
    spritetopoffset = Z_Malloc(
        (numspritelumps as usize * std::mem::size_of::<c_int>()) as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut c_int;

    for i in 0..numspritelumps as usize {
        if (i & 63) == 0 {
            libc::printf(b".\0".as_ptr() as *const c_char);
        }

        let patch = W_CacheLumpNum(firstspritelump + i as c_int, PU_CACHE) as *mut patch_t;
        *spritewidth.add(i) = (SHORT((*patch).width) as c_int) << FRACBITS;
        *spriteoffset.add(i) = (SHORT((*patch).leftoffset) as c_int) << FRACBITS;
        *spritetopoffset.add(i) = (SHORT((*patch).topoffset) as c_int) << FRACBITS;
    }
}

// ---------------------------------------------------------------------------
// R_InitColormaps
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_InitColormaps() {
    let lump = W_GetNumForName(DEH_String(b"COLORMAP\0".as_ptr() as *mut c_char));
    colormaps = W_CacheLumpNum(lump, PU_STATIC) as *mut u8;
}

// ---------------------------------------------------------------------------
// R_InitData
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_InitData() {
    R_InitTextures();
    libc::printf(b".\0".as_ptr() as *const c_char);
    R_InitFlats();
    libc::printf(b".\0".as_ptr() as *const c_char);
    R_InitSpriteLumps();
    libc::printf(b".\0".as_ptr() as *const c_char);
    R_InitColormaps();
}

// ---------------------------------------------------------------------------
// R_FlatNumForName
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_FlatNumForName(name: *mut c_char) -> c_int {
    let i = W_CheckNumForName(name);
    if i == -1 {
        let mut namet: [c_char; 9] = [0; 9];
        std::ptr::copy_nonoverlapping(name, namet.as_mut_ptr(), 8);
        i_error!(
            "R_FlatNumForName: {} not found",
            CStr::from_ptr(namet.as_ptr()).to_string_lossy()
        );
    }
    i - firstflat
}

// ---------------------------------------------------------------------------
// R_CheckTextureNumForName
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_CheckTextureNumForName(name: *mut c_char) -> c_int {
    if *name == b'-' as c_char {
        return 0;
    }

    let key = (W_LumpNameHash(name) % numtextures as c_uint) as usize;
    let mut texture = *textures_hashtable.add(key);

    while !texture.is_null() {
        if strncasecmp((*texture).name.as_ptr(), name, 8) == 0 {
            return (*texture).index;
        }
        texture = (*texture).next;
    }

    -1
}

// ---------------------------------------------------------------------------
// R_TextureNumForName
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_TextureNumForName(name: *mut c_char) -> c_int {
    let i = R_CheckTextureNumForName(name);
    if i == -1 {
        i_error!(
            "R_TextureNumForName: {} not found",
            CStr::from_ptr(name).to_string_lossy()
        );
    }
    i
}

// ---------------------------------------------------------------------------
// R_PrecacheLevel
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_PrecacheLevel() {
    if demoplayback != 0 {
        return;
    }

    // Precache flats
    let flatpresent = Z_Malloc(numflats, PU_STATIC, ptr::null_mut()) as *mut u8;
    for i in 0..numflats as usize {
        *flatpresent.add(i) = 0;
    }

    for i in 0..numsectors as usize {
        *flatpresent.add((*sectors.add(i)).floorpic as usize) = 1;
        *flatpresent.add((*sectors.add(i)).ceilingpic as usize) = 1;
    }

    flatmemory = 0;
    for i in 0..numflats as usize {
        if *flatpresent.add(i) != 0 {
            let lump = firstflat + i as c_int;
            flatmemory += (*lumpinfo.add(lump as usize)).size;
            W_CacheLumpNum(lump, PU_CACHE);
        }
    }
    Z_Free(flatpresent as *mut c_void);

    // Precache textures
    let texturepresent = Z_Malloc(numtextures, PU_STATIC, ptr::null_mut()) as *mut u8;
    for i in 0..numtextures as usize {
        *texturepresent.add(i) = 0;
    }

    for i in 0..numsides as usize {
        *texturepresent.add((*sides.add(i)).toptexture as usize) = 1;
        *texturepresent.add((*sides.add(i)).midtexture as usize) = 1;
        *texturepresent.add((*sides.add(i)).bottomtexture as usize) = 1;
    }
    *texturepresent.add(skytexture as usize) = 1;

    texturememory = 0;
    for i in 0..numtextures as usize {
        if *texturepresent.add(i) == 0 {
            continue;
        }
        let texture = *textures.add(i);
        for j in 0..(*texture).patchcount as usize {
            let lump = (*std::ptr::addr_of!((*texture).patches).add(j)).patch;
            texturememory += (*lumpinfo.add(lump as usize)).size;
            W_CacheLumpNum(lump, PU_CACHE);
        }
    }
    Z_Free(texturepresent as *mut c_void);

    // Precache sprites
    let spritepresent = Z_Malloc(numsprites, PU_STATIC, ptr::null_mut()) as *mut u8;
    for i in 0..numsprites as usize {
        *spritepresent.add(i) = 0;
    }

    let mut th = thinkercap.next;
    while !std::ptr::eq(th, std::ptr::addr_of!(thinkercap)) {
        if (*th).function.acp1 == Some(P_MobjThinker) {
            let mobj = th as *mut mobj_t;
            *spritepresent.add((*mobj).sprite as usize) = 1;
        }
        th = (*th).next;
    }

    spritememory = 0;
    for i in 0..numsprites as usize {
        if *spritepresent.add(i) == 0 {
            continue;
        }
        let sprdef = sprites.add(i);
        for j in 0..(*sprdef).numframes as usize {
            let sf = (*sprdef).spriteframes.add(j);
            for k in 0..8usize {
                let lump = firstspritelump + (*sf).lump[k] as c_int;
                spritememory += (*lumpinfo.add(lump as usize)).size;
                W_CacheLumpNum(lump, PU_CACHE);
            }
        }
    }
    Z_Free(spritepresent as *mut c_void);
}

// ---------------------------------------------------------------------------
// Anchor
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn R_Data_Link_Anchor() {
    let _ = R_GenerateComposite as *const () as usize;
    let _ = R_GenerateLookup as *const () as usize;
    let _ = R_GetColumn as *const () as usize;
    let _ = R_InitTextures as *const () as usize;
    let _ = R_InitFlats as *const () as usize;
    let _ = R_InitSpriteLumps as *const () as usize;
    let _ = R_InitColormaps as *const () as usize;
    let _ = R_InitData as *const () as usize;
    let _ = R_FlatNumForName as *const () as usize;
    let _ = R_CheckTextureNumForName as *const () as usize;
    let _ = R_TextureNumForName as *const () as usize;
    let _ = R_PrecacheLevel as *const () as usize;
}
