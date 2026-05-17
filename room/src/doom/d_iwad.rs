//! Rust port of vendor/doomgeneric/d_iwad.c.
//!
//! IWAD discovery and validation.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::doom::d_mode;
use crate::doom::m_misc::M_StringJoinA;
use crate::i_error;

const MAX_IWAD_DIRS: usize = 128;
const DIR_SEPARATOR: c_char = b'/' as c_char;
const DIR_SEPARATOR_S: &[u8] = b"/\0";

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn M_FileExists(filename: *mut c_char) -> c_int;

    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn malloc(size: usize) -> *mut c_void;

    static mut myargc: c_int;
    static mut myargv: *mut *mut c_char;
    fn M_CheckParmWithArgs(check: *const c_char, num_args: c_int) -> c_int;
}

#[repr(C)]
pub struct iwad_t {
    pub name: *mut c_char,
    pub mission: c_int,
    pub mode: c_int,
    pub description: *mut c_char,
}

unsafe impl Sync for iwad_t {}

static IWADS: [iwad_t; 14] = [
    iwad_t {
        name: cstr!("doom2.wad"),
        mission: d_mode::doom2,
        mode: d_mode::commercial,
        description: cstr!("Doom II"),
    },
    iwad_t {
        name: cstr!("plutonia.wad"),
        mission: d_mode::pack_plut,
        mode: d_mode::commercial,
        description: cstr!("Final Doom: Plutonia Experiment"),
    },
    iwad_t {
        name: cstr!("tnt.wad"),
        mission: d_mode::pack_tnt,
        mode: d_mode::commercial,
        description: cstr!("Final Doom: TNT: Evilution"),
    },
    iwad_t {
        name: cstr!("doom.wad"),
        mission: d_mode::doom,
        mode: d_mode::retail,
        description: cstr!("Doom"),
    },
    iwad_t {
        name: cstr!("doom1.wad"),
        mission: d_mode::doom,
        mode: d_mode::shareware,
        description: cstr!("Doom Shareware"),
    },
    iwad_t {
        name: cstr!("chex.wad"),
        mission: d_mode::pack_chex,
        mode: d_mode::shareware,
        description: cstr!("Chex Quest"),
    },
    iwad_t {
        name: cstr!("hacx.wad"),
        mission: d_mode::pack_hacx,
        mode: d_mode::commercial,
        description: cstr!("Hacx"),
    },
    iwad_t {
        name: cstr!("freedm.wad"),
        mission: d_mode::doom2,
        mode: d_mode::commercial,
        description: cstr!("FreeDM"),
    },
    iwad_t {
        name: cstr!("freedoom2.wad"),
        mission: d_mode::doom2,
        mode: d_mode::commercial,
        description: cstr!("Freedoom: Phase 2"),
    },
    iwad_t {
        name: cstr!("freedoom1.wad"),
        mission: d_mode::doom,
        mode: d_mode::retail,
        description: cstr!("Freedoom: Phase 1"),
    },
    iwad_t {
        name: cstr!("heretic.wad"),
        mission: d_mode::heretic,
        mode: d_mode::retail,
        description: cstr!("Heretic"),
    },
    iwad_t {
        name: cstr!("heretic1.wad"),
        mission: d_mode::heretic,
        mode: d_mode::shareware,
        description: cstr!("Heretic Shareware"),
    },
    iwad_t {
        name: cstr!("hexen.wad"),
        mission: d_mode::hexen,
        mode: d_mode::commercial,
        description: cstr!("Hexen"),
    },
    iwad_t {
        name: cstr!("strife1.wad"),
        mission: d_mode::strife,
        mode: d_mode::commercial,
        description: cstr!("Strife"),
    },
];

static mut iwad_dirs_built: bool = false;
static mut iwad_dirs: [*mut c_char; MAX_IWAD_DIRS] = [ptr::null_mut(); MAX_IWAD_DIRS];
static mut num_iwad_dirs: c_int = 0;

unsafe fn AddIWADDir(dir: *mut c_char) {
    if num_iwad_dirs < MAX_IWAD_DIRS as c_int {
        iwad_dirs[num_iwad_dirs as usize] = dir;
        num_iwad_dirs += 1;
    }
}

unsafe fn DirIsFile(path: *mut c_char, filename: *mut c_char) -> c_int {
    let path_len = strlen(path);
    let filename_len = strlen(filename);

    if path_len > filename_len
        && *path.add(path_len - filename_len - 1) == DIR_SEPARATOR
        && strcasecmp(path.add(path_len - filename_len), filename) == 0
    {
        1
    } else {
        0
    }
}

unsafe fn CheckDirectoryHasIWAD(dir: *mut c_char, iwadname: *mut c_char) -> *mut c_char {
    if DirIsFile(dir, iwadname) != 0 && M_FileExists(dir) != 0 {
        return strdup(dir);
    }

    let filename = if strcmp(dir, cstr!(".")) == 0 {
        strdup(iwadname)
    } else {
        let strs: [*const c_char; 4] = [
            dir as *const c_char,
            DIR_SEPARATOR_S.as_ptr() as *const c_char,
            iwadname as *const c_char,
            ptr::null(),
        ];
        // SAFETY: null-terminated pointer array; ownership transferred to caller via return.
        M_StringJoinA(strs.as_ptr())
    };

    printf(c"Trying IWAD file:%s\n".as_ptr(), filename);

    if M_FileExists(filename) != 0 {
        return filename;
    }

    free(filename as *mut c_void);
    ptr::null_mut()
}

unsafe fn SearchDirectoryForIWAD(
    dir: *mut c_char,
    mask: c_int,
    mission: *mut c_int,
) -> *mut c_char {
    for i in 0..IWADS.len() {
        if ((1 << IWADS[i].mission) & mask) == 0 {
            continue;
        }

        let filename = CheckDirectoryHasIWAD(dir, IWADS[i].name);

        if !filename.is_null() {
            *mission = IWADS[i].mission;
            return filename;
        }
    }

    ptr::null_mut()
}

unsafe fn IdentifyIWADByName(mut name: *mut c_char, mask: c_int) -> c_int {
    let p = strrchr(name, DIR_SEPARATOR as c_int);
    if !p.is_null() {
        name = p.add(1);
    }

    let mut mission = d_mode::none;

    for i in 0..IWADS.len() {
        if ((1 << IWADS[i].mission) & mask) == 0 {
            continue;
        }

        if strcasecmp(name, IWADS[i].name) == 0 {
            mission = IWADS[i].mission;
            break;
        }
    }

    mission
}

unsafe fn BuildIWADDirList() {
    AddIWADDir(cstr!("."));
    iwad_dirs_built = true;
}

#[no_mangle]
pub unsafe extern "C" fn D_FindWADByName(name: *mut c_char) -> *mut c_char {
    if M_FileExists(name) != 0 {
        return name;
    }

    BuildIWADDirList();

    for i in 0..num_iwad_dirs {
        if DirIsFile(iwad_dirs[i as usize], name) != 0 && M_FileExists(iwad_dirs[i as usize]) != 0 {
            return strdup(iwad_dirs[i as usize]);
        }

        let strs: [*const c_char; 4] = [
            iwad_dirs[i as usize] as *const c_char,
            DIR_SEPARATOR_S.as_ptr() as *const c_char,
            name as *const c_char,
            ptr::null(),
        ];
        // SAFETY: null-terminated pointer array; freed below on miss, transferred to caller on hit.
        let path = M_StringJoinA(strs.as_ptr());

        if M_FileExists(path) != 0 {
            return path;
        }

        free(path as *mut c_void);
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn D_TryFindWADByName(filename: *mut c_char) -> *mut c_char {
    let result = D_FindWADByName(filename);
    if !result.is_null() {
        result
    } else {
        filename
    }
}

#[no_mangle]
pub unsafe extern "C" fn D_FindIWAD(mask: c_int, mission: *mut c_int) -> *mut c_char {
    let iwadparm = M_CheckParmWithArgs(cstr!("-iwad"), 1);

    if iwadparm != 0 {
        let iwadfile = *myargv.offset((iwadparm + 1) as isize);
        let result = D_FindWADByName(iwadfile);

        if result.is_null() {
            i_error!(
                "IWAD file '{}' not found!",
                std::ffi::CStr::from_ptr(iwadfile).to_string_lossy()
            );
        }

        *mission = IdentifyIWADByName(result, mask);
        result
    } else {
        printf(c"-iwad not specified, trying a few iwad names\n".as_ptr());

        let mut result: *mut c_char = ptr::null_mut();

        BuildIWADDirList();

        for i in 0..num_iwad_dirs {
            if !result.is_null() {
                break;
            }
            result = SearchDirectoryForIWAD(iwad_dirs[i as usize], mask, mission);
        }

        result
    }
}

#[no_mangle]
pub unsafe extern "C" fn D_FindAllIWADs(mask: c_int) -> *mut *const iwad_t {
    let result =
        malloc(std::mem::size_of::<*const iwad_t>() * (IWADS.len() + 1)) as *mut *const iwad_t;
    let mut result_len: usize = 0;

    for i in 0..IWADS.len() {
        if ((1 << IWADS[i].mission) & mask) == 0 {
            continue;
        }

        let filename = D_FindWADByName(IWADS[i].name);

        if !filename.is_null() {
            *result.add(result_len) = &IWADS[i];
            result_len += 1;
        }
    }

    *result.add(result_len) = ptr::null();

    result
}

#[no_mangle]
pub unsafe extern "C" fn D_SaveGameIWADName(gamemission: c_int) -> *mut c_char {
    for i in 0..IWADS.len() {
        if gamemission == IWADS[i].mission {
            return IWADS[i].name;
        }
    }
    cstr!("unknown.wad")
}

#[no_mangle]
pub unsafe extern "C" fn D_SuggestIWADName(mission: c_int, mode: c_int) -> *mut c_char {
    for i in 0..IWADS.len() {
        if IWADS[i].mission == mission && IWADS[i].mode == mode {
            return IWADS[i].name;
        }
    }
    cstr!("unknown.wad")
}

#[no_mangle]
pub unsafe extern "C" fn D_SuggestGameName(mission: c_int, mode: c_int) -> *mut c_char {
    for i in 0..IWADS.len() {
        if IWADS[i].mission == mission && (mode == d_mode::indetermined || IWADS[i].mode == mode) {
            return IWADS[i].description;
        }
    }
    cstr!("Unknown game?")
}

#[no_mangle]
pub extern "C" fn D_CheckCorrectIWAD(_mission: c_int) {
    // Not implemented in original C codebase.
}
