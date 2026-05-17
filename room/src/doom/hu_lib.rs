//! Rust port of vendor/doomgeneric/hu_lib.c.
//!
//! Heads-up text and input code: text lines, scrolling text, and input widgets.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_uint};

use crate::doom::v_video::patch_t;

use crate::doom::i_video::SCREENWIDTH;

const HU_MAXLINES: usize = 4;
const HU_MAXLINELENGTH: usize = 80;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct hu_textline_t {
    pub x: c_int,
    pub y: c_int,
    pub f: *mut *mut patch_t,
    pub sc: c_int,
    pub l: [c_char; HU_MAXLINELENGTH + 1],
    pub len: c_int,
    pub needsupdate: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct hu_stext_t {
    pub l: [hu_textline_t; HU_MAXLINES],
    pub h: c_int,
    pub cl: c_int,
    pub on: *mut c_int,
    pub laston: c_int,
    _pad: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct hu_itext_t {
    pub l: hu_textline_t,
    pub lm: c_int,
    _pad0: [u8; 4],
    pub on: *mut c_int,
    pub laston: c_int,
    _pad1: [u8; 4],
}

#[cfg(target_pointer_width = "64")]
mod layout_checks {
    use super::*;
    const _: () = assert!(std::mem::size_of::<hu_textline_t>() == 112);
    const _: () = assert!(std::mem::offset_of!(hu_textline_t, x) == 0);
    const _: () = assert!(std::mem::offset_of!(hu_textline_t, y) == 4);
    const _: () = assert!(std::mem::offset_of!(hu_textline_t, f) == 8);
    const _: () = assert!(std::mem::offset_of!(hu_textline_t, sc) == 16);
    const _: () = assert!(std::mem::offset_of!(hu_textline_t, l) == 20);
    const _: () = assert!(std::mem::offset_of!(hu_textline_t, len) == 104);
    const _: () = assert!(std::mem::offset_of!(hu_textline_t, needsupdate) == 108);

    const _: () = assert!(std::mem::size_of::<hu_stext_t>() == 472);
    const _: () = assert!(std::mem::offset_of!(hu_stext_t, l) == 0);
    const _: () = assert!(std::mem::offset_of!(hu_stext_t, h) == 448);
    const _: () = assert!(std::mem::offset_of!(hu_stext_t, cl) == 452);
    const _: () = assert!(std::mem::offset_of!(hu_stext_t, on) == 456);
    const _: () = assert!(std::mem::offset_of!(hu_stext_t, laston) == 464);

    const _: () = assert!(std::mem::size_of::<hu_itext_t>() == 136);
    const _: () = assert!(std::mem::offset_of!(hu_itext_t, l) == 0);
    const _: () = assert!(std::mem::offset_of!(hu_itext_t, lm) == 112);
    const _: () = assert!(std::mem::offset_of!(hu_itext_t, on) == 120);
    const _: () = assert!(std::mem::offset_of!(hu_itext_t, laston) == 128);
}

extern "C" {
    fn V_DrawPatchDirect(x: c_int, y: c_int, patch: *mut patch_t);
    fn R_VideoErase(ofs: c_uint, count: c_int);
    static mut automapactive: c_int;
    static mut viewwindowx: c_int;
    static mut viewwindowy: c_int;
    static mut viewwidth: c_int;
    static mut viewheight: c_int;
}

/// Byte-swap for little-endian (SHORT macro from i_swap.h).
#[inline(always)]
fn short_swap(v: i16) -> i16 {
    v
}

#[no_mangle]
pub extern "C" fn HUlib_init() {}

#[no_mangle]
pub extern "C" fn HUlib_clearTextLine(t: *mut hu_textline_t) {
    unsafe {
        (*t).len = 0;
        (*t).l[0] = 0;
        (*t).needsupdate = 1;
    }
}

#[no_mangle]
pub extern "C" fn HUlib_initTextLine(
    t: *mut hu_textline_t,
    x: c_int,
    y: c_int,
    f: *mut *mut patch_t,
    sc: c_int,
) {
    unsafe {
        (*t).x = x;
        (*t).y = y;
        (*t).f = f;
        (*t).sc = sc;
        HUlib_clearTextLine(t);
    }
}

#[no_mangle]
pub extern "C" fn HUlib_addCharToTextLine(t: *mut hu_textline_t, ch: c_char) -> c_int {
    unsafe {
        if (*t).len == HU_MAXLINELENGTH as c_int {
            0
        } else {
            (*t).l[(*t).len as usize] = ch;
            (*t).len += 1;
            (*t).l[(*t).len as usize] = 0;
            (*t).needsupdate = 4;
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_delCharFromTextLine(t: *mut hu_textline_t) -> c_int {
    unsafe {
        if (*t).len == 0 {
            0
        } else {
            (*t).len -= 1;
            (*t).l[(*t).len as usize] = 0;
            (*t).needsupdate = 4;
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_drawTextLine(l: *mut hu_textline_t, drawcursor: c_int) {
    unsafe {
        let mut x = (*l).x;
        let f = (*l).f;
        let sc = (*l).sc;

        for i in 0..(*l).len as usize {
            let c = ((*l).l[i] as u8).to_ascii_uppercase();
            if c != b' ' && c >= sc as u8 && c <= b'_' {
                let patch = *f.add((c as c_int - sc) as usize);
                let w = short_swap((*patch).width) as c_int;
                if x + w > SCREENWIDTH {
                    break;
                }
                V_DrawPatchDirect(x, (*l).y, patch);
                x += w;
            } else {
                x += 4;
                if x >= SCREENWIDTH {
                    break;
                }
            }
        }

        if drawcursor != 0 {
            let cursor_patch = *f.add((b'_' as c_int - sc) as usize);
            let cursor_w = short_swap((*cursor_patch).width) as c_int;
            if x + cursor_w <= SCREENWIDTH {
                V_DrawPatchDirect(x, (*l).y, cursor_patch);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_eraseTextLine(l: *mut hu_textline_t) {
    unsafe {
        if automapactive == 0 && viewwindowx != 0 && (*l).needsupdate != 0 {
            let lh = short_swap((**(*l).f).height) as c_int + 1;
            for y in (*l).y..(*l).y + lh {
                let yoffset = y * SCREENWIDTH;
                if y < viewwindowy || y >= viewwindowy + viewheight {
                    R_VideoErase(yoffset as c_uint, SCREENWIDTH);
                } else {
                    R_VideoErase(yoffset as c_uint, viewwindowx);
                    R_VideoErase((yoffset + viewwindowx + viewwidth) as c_uint, viewwindowx);
                }
            }
        }

        if (*l).needsupdate != 0 {
            (*l).needsupdate -= 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_initSText(
    s: *mut hu_stext_t,
    x: c_int,
    y: c_int,
    h: c_int,
    font: *mut *mut patch_t,
    startchar: c_int,
    on: *mut c_int,
) {
    unsafe {
        (*s).h = h;
        (*s).on = on;
        (*s).laston = 1;
        (*s).cl = 0;
        let font_h = short_swap((**font).height) as c_int + 1;
        for i in 0..h as usize {
            HUlib_initTextLine(
                &mut (*s).l[i],
                x,
                y - (i as c_int) * font_h,
                font,
                startchar,
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_addLineToSText(s: *mut hu_stext_t) {
    unsafe {
        (*s).cl += 1;
        if (*s).cl == (*s).h {
            (*s).cl = 0;
        }
        HUlib_clearTextLine(&mut (*s).l[(*s).cl as usize]);

        for i in 0..(*s).h as usize {
            (*s).l[i].needsupdate = 4;
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_addMessageToSText(
    s: *mut hu_stext_t,
    prefix: *mut c_char,
    msg: *mut c_char,
) {
    unsafe {
        HUlib_addLineToSText(s);
        if !prefix.is_null() {
            let mut p = prefix;
            while *p != 0 {
                HUlib_addCharToTextLine(&mut (*s).l[(*s).cl as usize], *p);
                p = p.add(1);
            }
        }
        let mut m = msg;
        while *m != 0 {
            HUlib_addCharToTextLine(&mut (*s).l[(*s).cl as usize], *m);
            m = m.add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_drawSText(s: *mut hu_stext_t) {
    unsafe {
        if *(*s).on == 0 {
            return;
        }
        for i in 0..(*s).h as usize {
            let mut idx = (*s).cl as isize - i as isize;
            if idx < 0 {
                idx += (*s).h as isize;
            }
            HUlib_drawTextLine(&mut (*s).l[idx as usize], 0);
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_eraseSText(s: *mut hu_stext_t) {
    unsafe {
        for i in 0..(*s).h as usize {
            if (*s).laston != 0 && *(*s).on == 0 {
                (*s).l[i].needsupdate = 4;
            }
            HUlib_eraseTextLine(&mut (*s).l[i]);
        }
        (*s).laston = if *(*s).on != 0 { 1 } else { 0 };
    }
}

#[no_mangle]
pub extern "C" fn HUlib_initIText(
    it: *mut hu_itext_t,
    x: c_int,
    y: c_int,
    font: *mut *mut patch_t,
    startchar: c_int,
    on: *mut c_int,
) {
    unsafe {
        (*it).lm = 0;
        (*it).on = on;
        (*it).laston = 1;
        HUlib_initTextLine(&mut (*it).l, x, y, font, startchar);
    }
}

#[no_mangle]
pub extern "C" fn HUlib_delCharFromIText(it: *mut hu_itext_t) {
    unsafe {
        if (*it).l.len != (*it).lm {
            HUlib_delCharFromTextLine(&mut (*it).l);
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_eraseLineFromIText(it: *mut hu_itext_t) {
    unsafe {
        while (*it).lm != (*it).l.len {
            HUlib_delCharFromTextLine(&mut (*it).l);
        }
    }
}

#[no_mangle]
pub extern "C" fn HUlib_resetIText(it: *mut hu_itext_t) {
    unsafe {
        (*it).lm = 0;
        HUlib_clearTextLine(&mut (*it).l);
    }
}

#[no_mangle]
pub extern "C" fn HUlib_addPrefixToIText(it: *mut hu_itext_t, str: *mut c_char) {
    unsafe {
        let mut p = str;
        while *p != 0 {
            HUlib_addCharToTextLine(&mut (*it).l, *p);
            p = p.add(1);
        }
        (*it).lm = (*it).l.len;
    }
}

#[no_mangle]
pub extern "C" fn HUlib_keyInIText(it: *mut hu_itext_t, ch: u8) -> c_int {
    unsafe {
        let ch = ch.to_ascii_uppercase();
        if (b' '..=b'_').contains(&ch) {
            HUlib_addCharToTextLine(&mut (*it).l, ch as c_char);
        } else if ch == crate::doom::doomkeys::KEY_BACKSPACE {
            HUlib_delCharFromIText(it);
        } else if ch != crate::doom::doomkeys::KEY_ENTER {
            return 0;
        }
        1
    }
}

#[no_mangle]
pub extern "C" fn HUlib_drawIText(it: *mut hu_itext_t) {
    unsafe {
        if *(*it).on == 0 {
            return;
        }
        HUlib_drawTextLine(&mut (*it).l, 1);
    }
}

#[no_mangle]
pub extern "C" fn HUlib_eraseIText(it: *mut hu_itext_t) {
    unsafe {
        if (*it).laston != 0 && *(*it).on == 0 {
            (*it).l.needsupdate = 4;
        }
        HUlib_eraseTextLine(&mut (*it).l);
        (*it).laston = if *(*it).on != 0 { 1 } else { 0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    const HU_TEXTLINE_T_SIZEOF: usize = 112;
    const HU_STEXT_T_SIZEOF: usize = 472;
    const HU_ITEXT_T_SIZEOF: usize = 136;

    #[test]
    fn hu_textline_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<hu_textline_t>(), HU_TEXTLINE_T_SIZEOF);
        assert_eq!(std::mem::offset_of!(hu_textline_t, x), 0);
        assert_eq!(std::mem::offset_of!(hu_textline_t, y), 4);
        assert_eq!(std::mem::offset_of!(hu_textline_t, f), 8);
        assert_eq!(std::mem::offset_of!(hu_textline_t, sc), 16);
        assert_eq!(std::mem::offset_of!(hu_textline_t, l), 20);
        assert_eq!(std::mem::offset_of!(hu_textline_t, len), 104);
        assert_eq!(std::mem::offset_of!(hu_textline_t, needsupdate), 108);
    }

    #[test]
    fn hu_stext_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<hu_stext_t>(), HU_STEXT_T_SIZEOF);
        assert_eq!(std::mem::offset_of!(hu_stext_t, l), 0);
        assert_eq!(std::mem::offset_of!(hu_stext_t, h), 448);
        assert_eq!(std::mem::offset_of!(hu_stext_t, cl), 452);
        assert_eq!(std::mem::offset_of!(hu_stext_t, on), 456);
        assert_eq!(std::mem::offset_of!(hu_stext_t, laston), 464);
    }

    #[test]
    fn hu_itext_t_layout_matches_c() {
        let _g = LOCK.lock().unwrap();
        assert_eq!(std::mem::size_of::<hu_itext_t>(), HU_ITEXT_T_SIZEOF);
        assert_eq!(std::mem::offset_of!(hu_itext_t, l), 0);
        assert_eq!(std::mem::offset_of!(hu_itext_t, lm), 112);
        assert_eq!(std::mem::offset_of!(hu_itext_t, on), 120);
        assert_eq!(std::mem::offset_of!(hu_itext_t, laston), 128);
    }

    /// Build a zero-initialised hu_textline_t without a valid font pointer.
    /// Safe to use with clear/add/del because those don't dereference `f`.
    fn make_textline() -> hu_textline_t {
        hu_textline_t {
            x: 0,
            y: 0,
            f: std::ptr::null_mut(),
            sc: 0,
            l: [0; HU_MAXLINELENGTH + 1],
            len: 0,
            needsupdate: 0,
        }
    }

    #[test]
    fn clear_text_line_resets_fields() {
        let _g = LOCK.lock().unwrap();
        let mut tl = make_textline();
        tl.len = 5;
        tl.needsupdate = 0;
        tl.l[0] = b'X' as c_char;

        HUlib_clearTextLine(&mut tl);

        assert_eq!(tl.len, 0, "len must be reset to 0");
        assert_eq!(tl.l[0], 0, "first char must be nul after clear");
        assert_eq!(tl.needsupdate, 1, "needsupdate must be set to 1");
    }

    #[test]
    fn add_char_increments_len_and_nul_terminates() {
        let _g = LOCK.lock().unwrap();
        let mut tl = make_textline();
        HUlib_clearTextLine(&mut tl);

        let ret = HUlib_addCharToTextLine(&mut tl, b'A' as c_char);

        assert_eq!(ret, 1, "successful add must return 1");
        assert_eq!(tl.len, 1);
        assert_eq!(tl.l[0], b'A' as c_char);
        assert_eq!(tl.l[1], 0, "character after the last must be nul");
        assert_eq!(tl.needsupdate, 4);
    }

    #[test]
    fn add_multiple_chars_builds_string() {
        let _g = LOCK.lock().unwrap();
        let mut tl = make_textline();
        HUlib_clearTextLine(&mut tl);

        for ch in b"HI" {
            HUlib_addCharToTextLine(&mut tl, *ch as c_char);
        }
        assert_eq!(tl.len, 2);
        assert_eq!(tl.l[0], b'H' as c_char);
        assert_eq!(tl.l[1], b'I' as c_char);
        assert_eq!(tl.l[2], 0);
    }

    #[test]
    fn add_char_at_max_capacity_returns_zero() {
        let _g = LOCK.lock().unwrap();
        let mut tl = make_textline();
        HUlib_clearTextLine(&mut tl);

        // Fill to HU_MAXLINELENGTH
        for _ in 0..HU_MAXLINELENGTH {
            HUlib_addCharToTextLine(&mut tl, b'X' as c_char);
        }
        assert_eq!(tl.len, HU_MAXLINELENGTH as c_int);

        // One more must be rejected
        let ret = HUlib_addCharToTextLine(&mut tl, b'Y' as c_char);
        assert_eq!(ret, 0, "add beyond max length must return 0");
        assert_eq!(tl.len, HU_MAXLINELENGTH as c_int, "len must not change");
    }

    #[test]
    fn del_char_decrements_len_and_nul_terminates() {
        let _g = LOCK.lock().unwrap();
        let mut tl = make_textline();
        HUlib_clearTextLine(&mut tl);
        HUlib_addCharToTextLine(&mut tl, b'A' as c_char);
        HUlib_addCharToTextLine(&mut tl, b'B' as c_char);

        let ret = HUlib_delCharFromTextLine(&mut tl);

        assert_eq!(ret, 1, "successful delete must return 1");
        assert_eq!(tl.len, 1);
        assert_eq!(tl.l[1], 0, "position after new end must be nul");
        assert_eq!(tl.needsupdate, 4);
    }

    #[test]
    fn del_char_on_empty_line_returns_zero() {
        let _g = LOCK.lock().unwrap();
        let mut tl = make_textline();
        HUlib_clearTextLine(&mut tl);

        let ret = HUlib_delCharFromTextLine(&mut tl);
        assert_eq!(ret, 0, "delete on empty line must return 0");
        assert_eq!(tl.len, 0, "len must stay 0");
    }

    /// HUlib_keyInIText with a printable character in [' ', '_'] must add it.
    #[test]
    fn key_in_itext_printable_adds_char() {
        let _g = LOCK.lock().unwrap();
        let mut on: c_int = 1;
        let mut it = hu_itext_t {
            l: make_textline(),
            lm: 0,
            _pad0: [0; 4],
            on: &mut on,
            laston: 0,
            _pad1: [0; 4],
        };
        HUlib_clearTextLine(&mut it.l);

        let ret = HUlib_keyInIText(&mut it, b'a'); // lowercase → uppercased to 'A'
        assert_eq!(ret, 1);
        assert_eq!(it.l.len, 1);
        assert_eq!(it.l.l[0], b'A' as c_char);
    }

    /// HUlib_keyInIText with KEY_BACKSPACE removes the last character.
    #[test]
    fn key_in_itext_backspace_removes_char() {
        use crate::doom::doomkeys::KEY_BACKSPACE;
        let _g = LOCK.lock().unwrap();
        let mut on: c_int = 1;
        let mut it = hu_itext_t {
            l: make_textline(),
            lm: 0,
            _pad0: [0; 4],
            on: &mut on,
            laston: 0,
            _pad1: [0; 4],
        };
        HUlib_clearTextLine(&mut it.l);
        HUlib_addCharToTextLine(&mut it.l, b'Z' as c_char);
        assert_eq!(it.l.len, 1);

        let ret = HUlib_keyInIText(&mut it, KEY_BACKSPACE);
        assert_eq!(ret, 1);
        assert_eq!(it.l.len, 0);
    }

    /// Characters outside [' ', '_'] (except Enter/Backspace) return 0.
    #[test]
    fn key_in_itext_unknown_key_returns_zero() {
        let _g = LOCK.lock().unwrap();
        let mut on: c_int = 1;
        let mut it = hu_itext_t {
            l: make_textline(),
            lm: 0,
            _pad0: [0; 4],
            on: &mut on,
            laston: 0,
            _pad1: [0; 4],
        };
        HUlib_clearTextLine(&mut it.l);

        // 0x01 is below ' ' (0x20) and is not Enter or Backspace
        let ret = HUlib_keyInIText(&mut it, 0x01);
        assert_eq!(ret, 0, "unknown control char must return 0");
        assert_eq!(it.l.len, 0, "no char should be added");
    }
}
