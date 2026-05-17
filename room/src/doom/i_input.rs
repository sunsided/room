//! Rust port of vendor/doomgeneric/i_input.c.
//!
//! Keyboard input handling: reads keys from the doomgeneric platform layer
//! and posts them as Doom events.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::os::raw::c_int;

use crate::doom::d_event::event_t;
use crate::doom::doomkeys::KEY_RSHIFT;

#[no_mangle]
pub static mut vanilla_keyboard_mapping: c_int = 1;

static mut shiftdown: c_int = 0;

extern "C" {
    fn DG_GetKey(pressed: *mut c_int, key: *mut u8) -> c_int;
    fn D_PostEvent(ev: *const event_t);
}

static SHIFTXFORM: [u8; 128] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'"', // shift-'
    b'(', b')', b'*', b'+', b'<', // shift-,
    b'_', // shift--
    b'>', // shift-.
    b'?', // shift-/
    b')', // shift-0
    b'!', // shift-1
    b'@', // shift-2
    b'#', // shift-3
    b'$', // shift-4
    b'%', // shift-5
    b'^', // shift-6
    b'&', // shift-7
    b'*', // shift-8
    b'(', // shift-9
    b':', b':', // shift-;
    b'<', b'+', // shift-=
    b'>', b'?', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M',
    b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    b'[', // shift-[
    b'!', // shift-backslash
    b']', // shift-]
    b'"', b'_', b'\'', // shift-`
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'{', b'|', b'}', b'~', 127,
];

fn TranslateKey(key: u8) -> u8 {
    // The platform layer (DG_GetKey) already returns Doom key codes,
    // so this is an identity function (matching the active code in
    // the original C source).
    key
}

fn GetTypedChar(key: u8) -> u8 {
    let mut key = TranslateKey(key);
    unsafe {
        if shiftdown > 0 {
            if (key as usize) < SHIFTXFORM.len() {
                key = SHIFTXFORM[key as usize];
            } else {
                key = 0;
            }
        }
    }
    key
}

fn UpdateShiftStatus(pressed: c_int, key: u8) {
    let change = if pressed != 0 { 1 } else { -1 };
    if key == KEY_RSHIFT {
        unsafe {
            shiftdown += change;
        }
    }
}

#[no_mangle]
pub extern "C" fn I_InitInput() {}

#[no_mangle]
pub extern "C" fn I_GetEvent() {
    unsafe {
        let mut pressed: c_int = 0;
        let mut key: u8 = 0;

        while DG_GetKey(&mut pressed, &mut key) != 0 {
            UpdateShiftStatus(pressed, key);

            let mut event = event_t {
                type_: 0,
                data1: 0,
                data2: 0,
                data3: 0,
                data4: 0,
            };

            if pressed != 0 {
                event.type_ = 0; // ev_keydown
                event.data1 = TranslateKey(key) as c_int;
                event.data2 = GetTypedChar(key) as c_int;
                if event.data1 != 0 {
                    D_PostEvent(&event);
                }
            } else {
                event.type_ = 1; // ev_keyup
                event.data1 = TranslateKey(key) as c_int;
                event.data2 = 0;
                if event.data1 != 0 {
                    D_PostEvent(&event);
                }
                break;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn I_Input_Link_Anchor() {
    let _ = I_InitInput as *const () as usize;
    let _ = I_GetEvent as *const () as usize;
}
