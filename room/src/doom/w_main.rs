#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::c_char;

use libc::printf;

use crate::types::Boolean;

use crate::doom::d_iwad::D_TryFindWADByName;
use crate::doom::m_argv::{myargc, myargv, M_CheckParmWithArgs};
use crate::doom::w_wad::W_AddFile;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

#[no_mangle]
pub extern "C" fn W_ParseCommandLine() -> Boolean {
    let mut modifiedgame: Boolean = Boolean::FALSE;

    unsafe {
        let p = M_CheckParmWithArgs(cstr!("-file"), 1);
        if p != 0 {
            let mut idx = p + 1;
            modifiedgame = Boolean::TRUE;
            while idx < myargc && **myargv.offset(idx as isize) != b'-' as c_char {
                let filename = D_TryFindWADByName(*myargv.offset(idx as isize));
                printf(cstr!(" adding %s\n"), filename);
                W_AddFile(filename);
                idx += 1;
            }
        }
    }

    modifiedgame
}
