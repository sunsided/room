#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::types::Boolean;

use crate::doom::m_misc::M_StringCopy;
use crate::doom::sha1::{
    sha1_digest_t, SHA1Context, SHA1_Final, SHA1_Init, SHA1_UpdateInt32, SHA1_UpdateString,
};
use crate::doom::w_wad::{lumpinfo, numlumps};

#[repr(C)]
pub struct LumpInfo {
    pub name: [c_char; 8],
    pub wad_file: *mut c_void,
    pub position: c_int,
    pub size: c_int,
    pub cache: *mut c_void,
    pub next: *mut LumpInfo,
}

unsafe fn get_file_number(handle: *mut c_void, open_wadfiles: &mut Vec<*mut c_void>) -> c_int {
    for (i, &wad) in open_wadfiles.iter().enumerate() {
        if wad == handle {
            return i as c_int;
        }
    }

    let result = open_wadfiles.len() as c_int;
    open_wadfiles.push(handle);
    result
}

unsafe fn checksum_add_lump(
    sha1_context: *mut SHA1Context,
    lump: *mut LumpInfo,
    open_wadfiles: &mut Vec<*mut c_void>,
) {
    let lump = &*lump;

    let mut buf: [c_char; 9] = [0; 9];
    M_StringCopy(buf.as_mut_ptr(), lump.name.as_ptr(), buf.len());

    SHA1_UpdateString(sha1_context, buf.as_mut_ptr());
    SHA1_UpdateInt32(
        sha1_context,
        get_file_number(lump.wad_file, open_wadfiles) as c_uint,
    );
    SHA1_UpdateInt32(sha1_context, lump.position as c_uint);
    SHA1_UpdateInt32(sha1_context, lump.size as c_uint);
}

#[no_mangle]
pub unsafe extern "C" fn W_Checksum(digest: *mut sha1_digest_t) {
    let mut sha1_context = std::mem::MaybeUninit::<SHA1Context>::uninit();
    SHA1_Init(sha1_context.as_mut_ptr());

    let mut open_wadfiles: Vec<*mut c_void> = Vec::new();

    for i in 0..numlumps {
        let lump = lumpinfo.add(i as usize) as *mut LumpInfo;
        checksum_add_lump(sha1_context.as_mut_ptr(), lump, &mut open_wadfiles);
    }

    SHA1_Final((*digest).as_mut_ptr(), sha1_context.as_mut_ptr());
}
