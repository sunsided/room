#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct wbstartstruct_t {
    pub epsd: c_int,
    pub last: c_int,
    pub partime: c_int,
    pub plyr: [wbplayerstruct_t; 4],
}

#[repr(C)]
pub struct wbplayerstruct_t {
    pub in_: c_int,
    pub skills: c_int,
    pub sitems: c_int,
    pub ssecret: c_int,
    pub stime: c_int,
    pub frags: [c_int; 4],
}

const MAX_CAPTURES: usize = 32;

static mut captured_stats: [wbstartstruct_t; MAX_CAPTURES] = {
    const DEFAULT: wbplayerstruct_t = wbplayerstruct_t {
        in_: 0,
        skills: 0,
        sitems: 0,
        ssecret: 0,
        stime: 0,
        frags: [0; 4],
    };
    const DEFAULT_WB: wbstartstruct_t = wbstartstruct_t {
        epsd: 0,
        last: 0,
        partime: 0,
        plyr: [DEFAULT; 4],
    };
    [DEFAULT_WB; MAX_CAPTURES]
};

static mut num_captured_stats: c_int = 0;

use crate::doom::i_timer::TICRATE;

extern "C" {
    fn M_ParmExists(check: *mut c_char) -> c_int;
    fn M_CheckParmWithArgs(check: *mut c_char, num_args: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    static mut myargv: *mut *mut c_char;
}

#[no_mangle]
pub extern "C" fn StatCopy(stats: *mut wbstartstruct_t) {
    unsafe {
        if M_ParmExists(b"-statdump\0".as_ptr() as *mut c_char) != 0
            && num_captured_stats < MAX_CAPTURES as c_int
        {
            memcpy(
                captured_stats
                    .as_mut_ptr()
                    .offset(num_captured_stats as isize) as *mut c_void,
                stats as *const c_void,
                std::mem::size_of::<wbstartstruct_t>(),
            );
            num_captured_stats += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn StatDump() {
    // All implementation is wrapped in #if ORIGCODE which is not defined
}

#[cfg(test)]
mod tests {
    use super::*;

    /// wbplayerstruct_t must be 36 bytes: 5 × i32 (20) + [i32; 4] (16).
    #[test]
    fn wbplayerstruct_t_size_matches_c() {
        // in_(4) + skills(4) + sitems(4) + ssecret(4) + stime(4) + frags[4](16) = 36
        assert_eq!(std::mem::size_of::<wbplayerstruct_t>(), 36);
    }

    /// wbstartstruct_t: 3 × sizeof(int) + 4 × sizeof(wbplayerstruct_t).
    #[test]
    fn wbstartstruct_t_size_matches_c() {
        // epsd(4) + last(4) + partime(4) + plyr[4] (4 × 36 = 144) = 156
        let expected =
            3 * std::mem::size_of::<c_int>() + 4 * std::mem::size_of::<wbplayerstruct_t>();
        assert_eq!(std::mem::size_of::<wbstartstruct_t>(), expected);
    }

    /// StatDump must not panic (it is currently a no-op stub).
    #[test]
    fn stat_dump_does_not_panic() {
        StatDump();
    }
}
