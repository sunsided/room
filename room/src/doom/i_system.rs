//! Rust port of vendor/doomgeneric/i_system.c.
//!
//! System interface: zone memory allocation, error handling, exit hooks,
//! startup banners, and DOS memory-dump emulation for compatibility.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use crate::doom::m_argv::{myargc, myargv, M_CheckParmWithArgs, M_ParmExists};
use crate::doom::m_misc::M_StrToInt;
use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::ptr;

use crate::i_error;
use crate::types::Boolean;

const DEFAULT_RAM: c_int = 6; // MiB
const MIN_RAM: c_int = 6; // MiB

type atexit_func_t = extern "C" fn();

#[repr(C)]
struct atexit_listentry_t {
    func: atexit_func_t,
    run_on_error: Boolean,
    next: *mut atexit_listentry_t,
}

static mut exit_funcs: *mut atexit_listentry_t = ptr::null_mut();

unsafe fn AutoAllocMemory(size: *mut c_int, mut default_ram: c_int, min_ram: c_int) -> *mut u8 {
    let mut zonemem: *mut u8 = ptr::null_mut();

    while zonemem.is_null() {
        if default_ram < min_ram {
            i_error!("Unable to allocate {} MiB of RAM for zone", default_ram);
        }

        *size = default_ram * 1024 * 1024;
        zonemem = libc::malloc(*size as usize) as *mut u8;

        if zonemem.is_null() {
            default_ram -= 1;
        }
    }

    zonemem
}

#[no_mangle]
pub extern "C" fn I_AtExit(func: atexit_func_t, run_on_error: Boolean) {
    unsafe {
        let entry =
            libc::malloc(std::mem::size_of::<atexit_listentry_t>()) as *mut atexit_listentry_t;
        (*entry).func = func;
        (*entry).run_on_error = run_on_error;
        (*entry).next = exit_funcs;
        exit_funcs = entry;
    }
}

#[no_mangle]
pub extern "C" fn I_Tactile(_on: c_int, _off: c_int, _total: c_int) {}

#[no_mangle]
pub extern "C" fn I_ZoneBase(size: *mut c_int) -> *mut u8 {
    unsafe {
        let mut default_ram = DEFAULT_RAM;
        let mut min_ram = MIN_RAM;

        let p = M_CheckParmWithArgs(c"-mb".as_ptr().cast_mut(), 1);
        if p > 0 {
            default_ram = libc::atoi(*myargv.offset((p + 1) as isize));
            min_ram = default_ram;
        }

        let zonemem = AutoAllocMemory(size, default_ram, min_ram);

        libc::printf(
            c"zone memory: %p, %x allocated for zone\n".as_ptr(),
            zonemem,
            *size,
        );

        zonemem
    }
}

#[no_mangle]
pub extern "C" fn I_PrintBanner(msg: *mut c_char) {
    unsafe {
        let len = libc::strlen(msg) as c_int;
        let spaces = 35 - len / 2;
        for _ in 0..spaces {
            libc::putchar(b' ' as c_int);
        }
        libc::puts(msg);
    }
}

#[no_mangle]
pub extern "C" fn I_PrintDivider() {
    unsafe {
        for _ in 0..75 {
            libc::putchar(b'=' as c_int);
        }
        libc::putchar(b'\n' as c_int);
    }
}

#[no_mangle]
pub extern "C" fn I_PrintStartupBanner(gamedescription: *mut c_char) {
    unsafe {
        I_PrintDivider();
        I_PrintBanner(gamedescription);
        I_PrintDivider();
        libc::printf(
            c" Room, like Doom Generic, is free software, covered by the GNU General Public\n License.  There is NO warranty; not even for MERCHANTABILITY or FITNESS\n FOR A PARTICULAR PURPOSE. You are welcome to change and distribute\n copies under certain conditions. See the source for more information.\n".as_ptr(),
        );
        I_PrintDivider();
    }
}

#[no_mangle]
pub extern "C" fn I_ConsoleStdout() -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn I_Quit() {
    unsafe {
        let mut entry = exit_funcs;
        while !entry.is_null() {
            ((*entry).func)();
            entry = (*entry).next;
        }
    }
}

unsafe fn ZenityAvailable() -> bool {
    std::process::Command::new("/usr/bin/zenity")
        .arg("--help")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

unsafe fn ZenityErrorBox(message: *const c_char) -> c_int {
    if !ZenityAvailable() {
        return 0;
    }

    let msg = CStr::from_ptr(message).to_string_lossy();
    let status = std::process::Command::new("/usr/bin/zenity")
        .arg("--error")
        .arg("--text")
        .arg(&*msg)
        .status();

    match status {
        Ok(s) => {
            if s.success() {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn I_Error(msg: *const c_char) {
    unsafe {
        static mut already_quitting: bool = false;

        if already_quitting {
            eprintln!("Warning: recursive call to I_Error detected.");
        } else {
            already_quitting = true;
        }

        let msg_cstr = CStr::from_ptr(msg);
        eprintln!("{}", msg_cstr.to_string_lossy());
        eprintln!();

        let mut entry = exit_funcs;
        while !entry.is_null() {
            if (*entry).run_on_error.is_truthy() {
                ((*entry).func)();
            }
            entry = (*entry).next;
        }

        let exit_gui_popup = M_ParmExists(c"-nogui".as_ptr().cast_mut()) == 0;
        if exit_gui_popup && I_ConsoleStdout() == 0 {
            ZenityErrorBox(msg);
        }

        std::process::exit(-1);
    }
}

/// Wrapper called by the C preprocessor macro for I_Error.
#[no_mangle]
pub extern "C" fn I_ErrorV(msg: *const c_char) {
    I_Error(msg);
}

const DOS_MEM_DUMP_SIZE: usize = 10;

static MEM_DUMP_DOS622: [u8; DOS_MEM_DUMP_SIZE] =
    [0x57, 0x92, 0x19, 0x00, 0xF4, 0x06, 0x70, 0x00, 0x16, 0x00];
static MEM_DUMP_WIN98: [u8; DOS_MEM_DUMP_SIZE] =
    [0x9E, 0x0F, 0xC9, 0x00, 0x65, 0x04, 0x70, 0x00, 0x16, 0x00];
static MEM_DUMP_DOSBOX: [u8; DOS_MEM_DUMP_SIZE] =
    [0x00, 0x00, 0x00, 0xF1, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00];
static mut MEM_DUMP_CUSTOM: [u8; DOS_MEM_DUMP_SIZE] = [0; DOS_MEM_DUMP_SIZE];

#[derive(Clone, Copy)]
enum DosMemDump {
    Dos622,
    Win98,
    Dosbox,
    Custom,
}

static mut dos_mem_dump: DosMemDump = DosMemDump::Win98;

#[no_mangle]
pub extern "C" fn I_GetMemoryValue(offset: c_uint, value: *mut c_void, size: c_int) -> c_int {
    unsafe {
        static mut firsttime: bool = true;

        if firsttime {
            firsttime = false;

            let p = M_CheckParmWithArgs(c"-setmem".as_ptr().cast_mut(), 1);
            if p > 0 {
                let arg = *myargv.offset((p + 1) as isize);
                if libc::strcasecmp(arg, c"dos622".as_ptr()) == 0 {
                    dos_mem_dump = DosMemDump::Dos622;
                } else if libc::strcasecmp(arg, c"dos71".as_ptr()) == 0 {
                    dos_mem_dump = DosMemDump::Win98;
                } else if libc::strcasecmp(arg, c"dosbox".as_ptr()) == 0 {
                    dos_mem_dump = DosMemDump::Dosbox;
                } else {
                    let mut idx: usize = 0;
                    let mut pp = (p + 1) as isize;
                    while idx < DOS_MEM_DUMP_SIZE {
                        pp += 1;
                        if pp >= myargc as isize || **myargv.offset(pp) == b'-' as c_char {
                            break;
                        }
                        let mut val: c_int = 0;
                        M_StrToInt(*myargv.offset(pp), &mut val);
                        MEM_DUMP_CUSTOM[idx] = val as u8;
                        idx += 1;
                    }
                    dos_mem_dump = DosMemDump::Custom;
                }
            }
        }

        let dump: *const u8 = match dos_mem_dump {
            DosMemDump::Dos622 => MEM_DUMP_DOS622.as_ptr(),
            DosMemDump::Win98 => MEM_DUMP_WIN98.as_ptr(),
            DosMemDump::Dosbox => MEM_DUMP_DOSBOX.as_ptr(),
            DosMemDump::Custom => std::ptr::addr_of!(MEM_DUMP_CUSTOM[0]),
        };

        let offset = offset as usize;
        if offset >= DOS_MEM_DUMP_SIZE {
            return 0;
        }

        match size {
            1 => {
                *(value as *mut u8) = *dump.add(offset);
                1
            }
            2 => {
                if offset + 1 >= DOS_MEM_DUMP_SIZE {
                    return 0;
                }
                *(value as *mut u16) =
                    (*dump.add(offset) as u16) | ((*dump.add(offset + 1) as u16) << 8);
                1
            }
            4 => {
                if offset + 3 >= DOS_MEM_DUMP_SIZE {
                    return 0;
                }
                *(value as *mut u32) = (*dump.add(offset) as u32)
                    | ((*dump.add(offset + 1) as u32) << 8)
                    | ((*dump.add(offset + 2) as u32) << 16)
                    | ((*dump.add(offset + 3) as u32) << 24);
                1
            }
            _ => 0,
        }
    }
}
extern "C" fn dummy_atexit() {}

#[no_mangle]
pub unsafe extern "C" fn I_System_Link_Anchor() {
    I_AtExit(dummy_atexit, Boolean::FALSE);
    I_Tactile(0, 0, 0);
    let mut size: c_int = 0;
    I_ZoneBase(&mut size);
    I_ConsoleStdout();
    I_Quit();
    I_GetMemoryValue(0, ptr::null_mut(), 0);
    I_PrintBanner(ptr::null_mut());
    I_PrintDivider();
    I_PrintStartupBanner(ptr::null_mut());
    I_ErrorV(ptr::null());
}
