//! Rust port of vendor/doomgeneric/w_wad.c.
//!
//! WAD file header / directory parsing, lump lookup, and caching.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::ptr;

use crate::doom::w_file::{wad_file_t, W_OpenFile, W_Read};

use crate::doom::z_zone::{PU_CACHE, PU_STATIC};

#[repr(C)]
pub struct lumpinfo_t {
    pub name: [c_char; 8],
    pub wad_file: *mut wad_file_t,
    pub position: c_int,
    pub size: c_int,
    pub cache: *mut c_void,
    pub next: *mut lumpinfo_t,
}

#[repr(C)]
struct wadinfo_t {
    identification: [c_char; 4],
    numlumps: c_int,
    infotableofs: c_int,
}

#[repr(C)]
struct filelump_t {
    filepos: c_int,
    size: c_int,
    name: [c_char; 8],
}

#[no_mangle]
pub static mut lumpinfo: *mut lumpinfo_t = ptr::null_mut();

#[no_mangle]
pub static mut numlumps: c_uint = 0;

static mut lumphash: *mut *mut lumpinfo_t = ptr::null_mut();

extern "C" {
    fn I_Error(format: *const c_char, ...);
    fn I_BeginRead();
    fn I_EndRead();

    fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void;
    fn Z_Free(ptr: *mut c_void);
    fn Z_ChangeUser(ptr: *mut c_void, user: *mut *mut c_void);
    fn Z_ChangeTag2(ptr: *mut c_void, tag: c_int, file: *const c_char, line: c_int);

    fn M_ExtractFileBase(path: *mut c_char, dest: *mut c_char);

    fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn toupper(c: c_int) -> c_int;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn D_SuggestGameName(mission: c_int, mode: c_int) -> *mut c_char;
    fn D_GameMissionString(mission: c_int) -> *mut c_char;
}

unsafe fn ExtendLumpInfo(newnumlumps: c_uint) {
    let newlumpinfo =
        calloc(newnumlumps as usize, std::mem::size_of::<lumpinfo_t>()) as *mut lumpinfo_t;
    if newlumpinfo.is_null() {
        I_Error(b"Couldn't realloc lumpinfo\0".as_ptr() as *const c_char);
    }

    for i in 0..numlumps.min(newnumlumps) {
        std::ptr::copy_nonoverlapping(lumpinfo.add(i as usize), newlumpinfo.add(i as usize), 1);

        if (*newlumpinfo.add(i as usize)).cache != ptr::null_mut() {
            Z_ChangeUser(
                (*newlumpinfo.add(i as usize)).cache,
                &mut (*newlumpinfo.add(i as usize)).cache as *mut *mut c_void,
            );
        }

        if (*lumpinfo.add(i as usize)).next != ptr::null_mut() {
            let nextlumpnum = ((*lumpinfo.add(i as usize)).next as usize - lumpinfo as usize)
                / std::mem::size_of::<lumpinfo_t>();
            (*newlumpinfo.add(i as usize)).next = newlumpinfo.add(nextlumpnum);
        }
    }

    free(lumpinfo as *mut c_void);
    lumpinfo = newlumpinfo;
    numlumps = newnumlumps;
}

#[no_mangle]
pub extern "C" fn W_LumpNameHash(s: *const c_char) -> c_uint {
    unsafe {
        let mut result: c_uint = 5381;
        for i in 0..8 {
            let ch = *s.add(i);
            if ch == 0 {
                break;
            }
            result = ((result << 5) ^ result) ^ (toupper(ch as c_int) as c_uint);
        }
        result
    }
}

#[no_mangle]
pub extern "C" fn W_AddFile(filename: *mut c_char) -> *mut wad_file_t {
    unsafe {
        let wad_file = W_OpenFile(filename);
        if wad_file.is_null() {
            libc::printf(b" couldn't open %s\n\0".as_ptr() as *const c_char, filename);
            return ptr::null_mut();
        }

        let mut newnumlumps = numlumps;
        let startlump = numlumps;

        let mut fileinfo: *mut filelump_t = ptr::null_mut();

        let fname_len = strlen(filename);
        if fname_len < 3
            || strcasecmp(
                filename.add(fname_len - 3),
                b"wad\0".as_ptr() as *const c_char,
            ) != 0
        {
            // Single lump file
            fileinfo = Z_Malloc(
                std::mem::size_of::<filelump_t>() as c_int,
                PU_STATIC,
                ptr::null_mut(),
            ) as *mut filelump_t;
            (*fileinfo).filepos = 0;
            (*fileinfo).size = (*wad_file).length as c_int;
            M_ExtractFileBase(filename, (*fileinfo).name.as_mut_ptr());
            newnumlumps += 1;
        } else {
            // WAD file
            let mut header: wadinfo_t = std::mem::zeroed();
            W_Read(
                wad_file,
                0,
                &mut header as *mut _ as *mut c_void,
                std::mem::size_of::<wadinfo_t>(),
            );

            if strncmp(
                header.identification.as_ptr(),
                b"IWAD\0".as_ptr() as *const c_char,
                4,
            ) != 0
            {
                if strncmp(
                    header.identification.as_ptr(),
                    b"PWAD\0".as_ptr() as *const c_char,
                    4,
                ) != 0
                {
                    I_Error(
                        b"Wad file %s doesn't have IWAD or PWAD id\n\0".as_ptr() as *const c_char,
                        filename,
                    );
                }
            }

            let header_numlumps = i32::from_le(header.numlumps);
            let header_infotableofs = i32::from_le(header.infotableofs);

            let length = (header_numlumps as usize) * std::mem::size_of::<filelump_t>();
            fileinfo = Z_Malloc(length as c_int, PU_STATIC, ptr::null_mut()) as *mut filelump_t;

            W_Read(
                wad_file,
                header_infotableofs as c_uint,
                fileinfo as *mut c_void,
                length,
            );
            newnumlumps += header_numlumps as c_uint;
        }

        ExtendLumpInfo(newnumlumps);

        let mut lump_p = lumpinfo.add(startlump as usize);
        let mut filerover = fileinfo;

        for _i in startlump..numlumps {
            (*lump_p).wad_file = wad_file;
            (*lump_p).position = i32::from_le((*filerover).filepos);
            (*lump_p).size = i32::from_le((*filerover).size);
            (*lump_p).cache = ptr::null_mut();
            strncpy((*lump_p).name.as_mut_ptr(), (*filerover).name.as_ptr(), 8);

            lump_p = lump_p.add(1);
            filerover = filerover.add(1);
        }

        Z_Free(fileinfo as *mut c_void);

        if !lumphash.is_null() {
            Z_Free(lumphash as *mut c_void);
            lumphash = ptr::null_mut();
        }

        wad_file
    }
}

#[no_mangle]
pub extern "C" fn W_NumLumps() -> c_int {
    unsafe { numlumps as c_int }
}

#[no_mangle]
pub extern "C" fn W_CheckNumForName(name: *mut c_char) -> c_int {
    unsafe {
        if !lumphash.is_null() {
            let hash = (W_LumpNameHash(name) % numlumps) as usize;
            let mut lump_p = *lumphash.add(hash);
            while !lump_p.is_null() {
                if strncasecmp((*lump_p).name.as_ptr(), name, 8) == 0 {
                    return (lump_p as usize - lumpinfo as usize) as c_int
                        / std::mem::size_of::<lumpinfo_t>() as c_int;
                }
                lump_p = (*lump_p).next;
            }
        } else {
            // Linear search, backwards so patch lumps take precedence
            let mut i = numlumps as i32 - 1;
            while i >= 0 {
                if strncasecmp((*lumpinfo.add(i as usize)).name.as_ptr(), name, 8) == 0 {
                    return i;
                }
                i -= 1;
            }
        }
        -1
    }
}

#[no_mangle]
pub extern "C" fn W_GetNumForName(name: *mut c_char) -> c_int {
    unsafe {
        let i = W_CheckNumForName(name);
        if i < 0 {
            I_Error(
                b"W_GetNumForName: %s not found!\0".as_ptr() as *const c_char,
                name,
            );
        }
        i
    }
}

#[no_mangle]
pub extern "C" fn W_LumpLength(lump: c_uint) -> c_int {
    unsafe {
        if lump >= numlumps {
            I_Error(
                b"W_LumpLength: %i >= numlumps\0".as_ptr() as *const c_char,
                lump as c_int,
            );
        }
        (*lumpinfo.add(lump as usize)).size
    }
}

#[no_mangle]
pub extern "C" fn W_ReadLump(lump: c_uint, dest: *mut c_void) {
    unsafe {
        if lump >= numlumps {
            I_Error(
                b"W_ReadLump: %i >= numlumps\0".as_ptr() as *const c_char,
                lump as c_int,
            );
        }
        let l = lumpinfo.add(lump as usize);
        I_BeginRead();
        let c = W_Read(
            (*l).wad_file,
            (*l).position as c_uint,
            dest,
            (*l).size as usize,
        );
        if c > (*l).size as usize {
            eprintln!(
                "[W_ReadLump] OVERFLOW: lump={}, requested={}, actually_read={}, lump.size={}",
                lump,
                (*l).size,
                c,
                (*l).size
            );
        }
        if c < (*l).size as usize {
            I_Error(
                b"W_ReadLump: only read %i of %i on lump %i\0".as_ptr() as *const c_char,
                c as c_int,
                (*l).size,
                lump as c_int,
            );
        }
        I_EndRead();
    }
}

#[no_mangle]
pub extern "C" fn W_CacheLumpNum(lumpnum: c_int, tag: c_int) -> *mut c_void {
    unsafe {
        if lumpnum < 0 || (lumpnum as c_uint) >= numlumps {
            I_Error(
                b"W_CacheLumpNum: %i >= numlumps\0".as_ptr() as *const c_char,
                lumpnum,
            );
        }
        let lump = lumpinfo.add(lumpnum as usize);

        if !(*(*lump).wad_file).mapped.is_null() {
            // Memory-mapped file
            (*(*lump).wad_file).mapped.add((*lump).position as usize) as *mut c_void
        } else if !(*lump).cache.is_null() {
            // Already cached
            let result = (*lump).cache;
            Z_ChangeTag2(result, tag, ptr::null(), 0);
            result
        } else {
            // Not yet loaded
            (*lump).cache = Z_Malloc(
                W_LumpLength(lumpnum as c_uint),
                tag,
                &mut (*lump).cache as *mut *mut c_void as *mut c_void,
            );
            W_ReadLump(lumpnum as c_uint, (*lump).cache);
            (*lump).cache
        }
    }
}

#[no_mangle]
pub extern "C" fn W_CacheLumpName(name: *mut c_char, tag: c_int) -> *mut c_void {
    unsafe { W_CacheLumpNum(W_GetNumForName(name), tag) }
}

#[no_mangle]
pub extern "C" fn W_ReleaseLumpNum(lumpnum: c_int) {
    unsafe {
        if lumpnum < 0 || (lumpnum as c_uint) >= numlumps {
            I_Error(
                b"W_ReleaseLumpNum: %i >= numlumps\0".as_ptr() as *const c_char,
                lumpnum,
            );
        }
        let lump = lumpinfo.add(lumpnum as usize);
        if !(*(*lump).wad_file).mapped.is_null() {
            // Memory-mapped: nothing to do
        } else {
            Z_ChangeTag2((*lump).cache, PU_CACHE, ptr::null(), 0);
        }
    }
}

#[no_mangle]
pub extern "C" fn W_ReleaseLumpName(name: *mut c_char) {
    unsafe { W_ReleaseLumpNum(W_GetNumForName(name)) }
}

#[no_mangle]
pub extern "C" fn W_GenerateHashTable() {
    unsafe {
        if !lumphash.is_null() {
            Z_Free(lumphash as *mut c_void);
        }

        if numlumps > 0 {
            lumphash = Z_Malloc(
                (std::mem::size_of::<*mut lumpinfo_t>() * numlumps as usize) as c_int,
                PU_STATIC,
                ptr::null_mut(),
            ) as *mut *mut lumpinfo_t;
            std::ptr::write_bytes(lumphash, 0, numlumps as usize);

            for i in 0..numlumps {
                let hash =
                    (W_LumpNameHash((*lumpinfo.add(i as usize)).name.as_ptr()) % numlumps) as usize;
                (*lumpinfo.add(i as usize)).next = *lumphash.add(hash);
                *lumphash.add(hash) = lumpinfo.add(i as usize);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn W_CheckCorrectIWAD(mission: c_int) {
    struct UniqueLump {
        mission: c_int,
        lumpname: &'static [u8],
    }

    const UNIQUE_LUMPS: [UniqueLump; 4] = [
        UniqueLump {
            mission: 0, // doom
            lumpname: b"POSSA1\0\0",
        },
        UniqueLump {
            mission: 6, // heretic
            lumpname: b"IMPXA1\0\0",
        },
        UniqueLump {
            mission: 7, // hexen
            lumpname: b"ETTNA1\0\0",
        },
        UniqueLump {
            mission: 8, // strife
            lumpname: b"AGRDA1\0\0",
        },
    ];

    unsafe {
        for ul in &UNIQUE_LUMPS {
            if mission != ul.mission {
                let lumpnum = W_CheckNumForName(ul.lumpname.as_ptr() as *mut c_char);
                if lumpnum >= 0 {
                    I_Error(
                        b"\nYou are trying to use a %s IWAD file with the %s%s binary.\nThis isn't going to work.\nYou probably want to use the %s%s binary.\0"
                            .as_ptr() as *const c_char,
                        D_SuggestGameName(ul.mission, 4), // indetermined
                        b"doomgeneric\0".as_ptr() as *const c_char,
                        D_GameMissionString(mission),
                        b"doomgeneric\0".as_ptr() as *const c_char,
                        D_GameMissionString(ul.mission),
                    );
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn W_Wad_Link_Anchor() {
    W_LumpNameHash(ptr::null());
    W_AddFile(ptr::null_mut());
    W_NumLumps();
    W_CheckNumForName(ptr::null_mut());
    W_GetNumForName(ptr::null_mut());
    W_LumpLength(0);
    W_ReadLump(0, ptr::null_mut());
    W_CacheLumpNum(0, 0);
    W_CacheLumpName(ptr::null_mut(), 0);
    W_ReleaseLumpNum(0);
    W_ReleaseLumpName(ptr::null_mut());
    W_GenerateHashTable();
    W_CheckCorrectIWAD(0);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lumpinfo_size_matches_c() {
        // C lumpinfo_t = name[8] + wad_file* + position + size + cache + next
        // On x86_64: 8 + 8 + 4 + 4 + 8 + 8 = 40 bytes
        assert_eq!(std::mem::size_of::<lumpinfo_t>(), 40);
    }

    #[test]
    fn wadinfo_size_matches_c() {
        // C wadinfo_t = ident[4] + numlumps + infotableofs
        // On x86_64: 4 + 4 + 4 = 12 bytes
        assert_eq!(std::mem::size_of::<wadinfo_t>(), 12);
    }

    #[test]
    fn filelump_size_matches_c() {
        // C filelump_t = filepos + size + name[8]
        // On x86_64: 4 + 4 + 8 = 16 bytes
        assert_eq!(std::mem::size_of::<filelump_t>(), 16);
    }
}
