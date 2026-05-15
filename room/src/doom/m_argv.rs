#![allow(non_upper_case_globals, non_snake_case)]

use crate::i_error;
use std::ffi::{c_char, c_int, c_long, c_void};

enum FILE {}

const DIR_SEPARATOR: c_char = b'/' as c_char;
const MAXARGVS: usize = 100;

#[no_mangle]
pub static mut myargc: c_int = 0;

#[no_mangle]
pub static mut myargv: *mut *mut c_char = std::ptr::null_mut();

extern "C" {
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fclose(stream: *mut FILE) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn isspace(c: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

extern "C" {
    fn M_FileLength(handle: *mut c_void) -> c_long;
}

#[no_mangle]
pub extern "C" fn M_CheckParmWithArgs(check: *mut c_char, num_args: c_int) -> c_int {
    unsafe {
        let mut i: c_int = 1;
        while i < myargc - num_args {
            if strcasecmp(check, *myargv.offset(i as isize) as *const c_char) == 0 {
                return i;
            }
            i += 1;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn M_ParmExists(check: *mut c_char) -> c_int {
    unsafe { (M_CheckParm(check) != 0) as c_int }
}

#[no_mangle]
pub extern "C" fn M_CheckParm(check: *mut c_char) -> c_int {
    M_CheckParmWithArgs(check, 0)
}

unsafe fn LoadResponseFile(argv_index: c_int) {
    let response_filename: *mut c_char = (*myargv.offset(argv_index as isize)).offset(1);

    let handle: *mut FILE = fopen(
        response_filename as *const c_char,
        b"rb\0".as_ptr() as *const c_char,
    );
    if handle.is_null() {
        printf(b"\nNo such response file!\0".as_ptr() as *const c_char);
        return;
    }

    printf(
        b"Found response file %s!\n\0".as_ptr() as *const c_char,
        response_filename,
    );

    let size: c_long = M_FileLength(handle as *mut c_void);

    let file: *mut c_char = malloc(size as usize + 1) as *mut c_char;
    let mut i: usize = 0;
    while i < size as usize {
        let k: usize = fread(file.add(i) as *mut c_void, 1, size as usize - i, handle);
        if k == 0 {
            i_error!("Failed to read full contents of '{}'", std::ffi::CStr::from_ptr(response_filename as *const c_char).to_string_lossy());
        }
        i += k;
    }
    fclose(handle);
    *file.add(size as usize) = 0;

    let newargv: *mut *mut c_char =
        malloc(std::mem::size_of::<*mut c_char>() * MAXARGVS) as *mut *mut c_char;
    let mut newargc: c_int = 0;
    memset(
        newargv as *mut c_void,
        0,
        std::mem::size_of::<*mut c_char>() * MAXARGVS,
    );

    let mut idx: c_int = 0;
    while idx < argv_index {
        *newargv.offset(newargc as isize) = *myargv.offset(idx as isize);
        newargc += 1;
        idx += 1;
    }

    let infile: *mut c_char = file;
    let mut k: usize = 0;
    while (k as c_long) < size {
        while (k as c_long) < size && isspace(*infile.add(k) as c_int) != 0 {
            k += 1;
        }
        if (k as c_long) >= size {
            break;
        }

        if *infile.add(k) == b'"' as c_char {
            k += 1;
            *newargv.offset(newargc as isize) = infile.add(k);
            newargc += 1;
            while (k as c_long) < size
                && *infile.add(k) != b'"' as c_char
                && *infile.add(k) != b'\n' as c_char
            {
                k += 1;
            }
            if (k as c_long) >= size || *infile.add(k) == b'\n' as c_char {
                i_error!("Quotes unclosed in response file '{}'", std::ffi::CStr::from_ptr(response_filename as *const c_char).to_string_lossy());
            }
            *infile.add(k) = 0;
            k += 1;
        } else {
            *newargv.offset(newargc as isize) = infile.add(k);
            newargc += 1;
            while (k as c_long) < size && isspace(*infile.add(k) as c_int) == 0 {
                k += 1;
            }
            *infile.add(k) = 0;
            k += 1;
        }
    }

    idx = argv_index + 1;
    while idx < myargc {
        *newargv.offset(newargc as isize) = *myargv.offset(idx as isize);
        newargc += 1;
        idx += 1;
    }

    myargv = newargv;
    myargc = newargc;
}

#[no_mangle]
pub extern "C" fn M_FindResponseFile() {
    unsafe {
        let mut i: c_int = 1;
        while i < myargc {
            if **myargv.offset(i as isize) == b'@' as c_char {
                LoadResponseFile(i);
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn M_GetExecutableName() -> *mut c_char {
    unsafe {
        let sep: *mut c_char = strrchr(*myargv as *const c_char, DIR_SEPARATOR as c_int);
        if sep.is_null() {
            *myargv
        } else {
            sep.offset(1)
        }
    }
}
