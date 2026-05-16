#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

use libc::{fclose, fopen, fread, fseek, FILE, SEEK_SET};

#[repr(C)]
pub struct wad_file_class_t {
    pub OpenFile: unsafe extern "C" fn(*mut c_char) -> *mut wad_file_t,
    pub CloseFile: unsafe extern "C" fn(*mut wad_file_t),
    pub Read: unsafe extern "C" fn(*mut wad_file_t, c_uint, *mut c_void, usize) -> usize,
}

#[repr(C)]
pub struct wad_file_t {
    pub file_class: *mut wad_file_class_t,
    pub mapped: *mut u8,
    pub length: c_uint,
}

#[repr(C)]
struct stdc_wad_file_t {
    wad: wad_file_t,
    fstream: *mut FILE,
}

use crate::doom::m_misc::{M_FileLength, FILE as MiscFILE};
use crate::doom::z_zone::{Z_Free, Z_Malloc, PU_STATIC};

unsafe extern "C" fn W_StdC_OpenFile(path: *mut c_char) -> *mut wad_file_t {
    let fstream = fopen(path as *const c_char, b"rb\0".as_ptr() as *const c_char);
    if fstream.is_null() {
        return ptr::null_mut();
    }

    let result = Z_Malloc(
        std::mem::size_of::<stdc_wad_file_t>() as c_int,
        PU_STATIC,
        ptr::null_mut(),
    ) as *mut stdc_wad_file_t;

    if result.is_null() {
        fclose(fstream);
        return ptr::null_mut();
    }

    (*result).wad.file_class = ptr::addr_of_mut!(stdc_wad_file);
    (*result).wad.mapped = ptr::null_mut();
    (*result).wad.length = M_FileLength(fstream as *mut MiscFILE) as c_uint;
    (*result).fstream = fstream;

    &mut (*result).wad
}

unsafe extern "C" fn W_StdC_CloseFile(wad: *mut wad_file_t) {
    let stdc_wad = wad as *mut stdc_wad_file_t;
    fclose((*stdc_wad).fstream);
    Z_Free(stdc_wad as *mut c_void);
}

unsafe extern "C" fn W_StdC_Read(
    wad: *mut wad_file_t,
    offset: c_uint,
    buffer: *mut c_void,
    buffer_len: usize,
) -> usize {
    let stdc_wad = wad as *mut stdc_wad_file_t;
    fseek((*stdc_wad).fstream, offset as c_long, SEEK_SET);
    fread(buffer, 1, buffer_len, (*stdc_wad).fstream)
}

#[no_mangle]
pub static mut stdc_wad_file: wad_file_class_t = wad_file_class_t {
    OpenFile: W_StdC_OpenFile,
    CloseFile: W_StdC_CloseFile,
    Read: W_StdC_Read,
};

#[no_mangle]
pub extern "C" fn W_OpenFile(path: *mut c_char) -> *mut wad_file_t {
    unsafe { (stdc_wad_file.OpenFile)(path) }
}

#[no_mangle]
pub extern "C" fn W_CloseFile(wad: *mut wad_file_t) {
    unsafe {
        ((*(*wad).file_class).CloseFile)(wad);
    }
}

#[no_mangle]
pub extern "C" fn W_Read(
    wad: *mut wad_file_t,
    offset: c_uint,
    buffer: *mut c_void,
    buffer_len: usize,
) -> usize {
    unsafe { ((*(*wad).file_class).Read)(wad, offset, buffer, buffer_len) }
}
