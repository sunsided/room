//! Rust port of vendor/doomgeneric/dstrings.c.
//!
//! Globally defined quit messages.

#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::c_char;

// `*const c_char` does not implement `Sync`, so wrap it.
// `repr(transparent)` guarantees ABI compatibility with `*const c_char` for C interop.
#[repr(transparent)]
pub struct Ptr(pub *const c_char);
unsafe impl Sync for Ptr {}

impl Ptr {
    pub fn as_ptr(&self) -> *const c_char {
        self.0
    }
}

macro_rules! cstr {
    ($s:literal) => {
        Ptr(concat!($s, "\0").as_ptr() as *const c_char)
    };
}

#[no_mangle]
pub static doom1_endmsg: [Ptr; 8] = [
    cstr!("are you sure you want to\nquit this great game?"),
    cstr!("please don't leave, there's more\ndemons to toast!"),
    cstr!("let's beat it -- this is turning\ninto a bloodbath!"),
    cstr!("i wouldn't leave if i were you.\ndos is much worse."),
    cstr!("you're trying to say you like dos\nbetter than me, right?"),
    cstr!("don't leave yet -- there's a\ndemon around that corner!"),
    cstr!("ya know, next time you come in here\ni'm gonna toast ya."),
    cstr!("go ahead and leave. see if i care."),
];

#[no_mangle]
pub static doom2_endmsg: [Ptr; 8] = [
    cstr!("are you sure you want to\nquit this great game?"),
    cstr!("you want to quit?\nthen, thou hast lost an eighth!"),
    cstr!("don't go now, there's a \ndimensional shambler waiting\nat the dos prompt!"),
    cstr!("get outta here and go back\nto your boring programs."),
    cstr!("if i were your boss, i'd \n deathmatch ya in a minute!"),
    cstr!("look, bud. you leave now\nand you forfeit your body count!"),
    cstr!("just leave. when you come\nback, i'll be waiting with a bat."),
    cstr!("you're lucky i don't smack\nyou for thinking about leaving."),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doom1_table_length() {
        assert_eq!(doom1_endmsg.len(), 8);
    }

    #[test]
    fn doom2_table_length() {
        assert_eq!(doom2_endmsg.len(), 8);
    }

    #[test]
    fn doom1_first_entry_roundtrips() {
        use std::ffi::CStr;
        let s = unsafe { CStr::from_ptr(doom1_endmsg[0].0) };
        assert_eq!(
            s.to_str().unwrap(),
            "are you sure you want to\nquit this great game?"
        );
    }

    #[test]
    fn doom2_first_entry_roundtrips() {
        use std::ffi::CStr;
        let s = unsafe { CStr::from_ptr(doom2_endmsg[0].0) };
        assert_eq!(
            s.to_str().unwrap(),
            "are you sure you want to\nquit this great game?"
        );
    }

    /// Every entry in both tables must be NUL-terminated and non-empty.
    #[test]
    fn all_doom1_entries_are_valid_c_strings() {
        use std::ffi::CStr;
        for entry in doom1_endmsg.iter() {
            let s = unsafe { CStr::from_ptr(entry.0) };
            let text = s.to_str().expect("doom1_endmsg entry is invalid UTF-8");
            assert!(!text.is_empty(), "doom1_endmsg entry must not be empty");
        }
    }

    #[test]
    fn all_doom2_entries_are_valid_c_strings() {
        use std::ffi::CStr;
        for entry in doom2_endmsg.iter() {
            let s = unsafe { CStr::from_ptr(entry.0) };
            let text = s.to_str().expect("doom2_endmsg entry is invalid UTF-8");
            assert!(!text.is_empty(), "doom2_endmsg entry must not be empty");
        }
    }

    /// The two tables are independent: same index may differ between them.
    #[test]
    fn doom1_and_doom2_differ_at_index_1() {
        use std::ffi::CStr;
        let s1 = unsafe { CStr::from_ptr(doom1_endmsg[1].0) }
            .to_str()
            .unwrap();
        let s2 = unsafe { CStr::from_ptr(doom2_endmsg[1].0) }
            .to_str()
            .unwrap();
        assert_ne!(
            s1, s2,
            "index 1 messages should differ between doom1 and doom2"
        );
    }

    /// The last entry in each table is accessible.
    #[test]
    fn last_entries_are_accessible() {
        use std::ffi::CStr;
        let s1 = unsafe { CStr::from_ptr(doom1_endmsg[7].0) };
        let s2 = unsafe { CStr::from_ptr(doom2_endmsg[7].0) };
        assert!(!s1.to_str().unwrap().is_empty());
        assert!(!s2.to_str().unwrap().is_empty());
    }
}
