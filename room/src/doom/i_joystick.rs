#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_void};

const NUM_VIRTUAL_BUTTONS: usize = 10;

static mut usejoystick: c_int = 0;
static mut joystick_index: c_int = -1;
static mut joystick_x_axis: c_int = 0;
static mut joystick_x_invert: c_int = 0;
static mut joystick_y_axis: c_int = 1;
static mut joystick_y_invert: c_int = 0;
static mut joystick_strafe_axis: c_int = -1;
static mut joystick_strafe_invert: c_int = 0;
static mut joystick_physical_buttons: [c_int; NUM_VIRTUAL_BUTTONS] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

extern "C" {
    fn M_BindVariable(name: *mut c_char, variable: *mut c_void);
}

use crate::c_write;

#[no_mangle]
pub extern "C" fn I_InitJoystick() {}

#[no_mangle]
pub extern "C" fn I_ShutdownJoystick() {}

#[no_mangle]
pub extern "C" fn I_UpdateJoystick() {}

#[no_mangle]
pub extern "C" fn I_BindJoystickVariables() {
    unsafe {
        M_BindVariable(
            b"use_joystick\0".as_ptr() as *mut c_char,
            &mut usejoystick as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"joystick_index\0".as_ptr() as *mut c_char,
            &mut joystick_index as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"joystick_x_axis\0".as_ptr() as *mut c_char,
            &mut joystick_x_axis as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"joystick_y_axis\0".as_ptr() as *mut c_char,
            &mut joystick_y_axis as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"joystick_strafe_axis\0".as_ptr() as *mut c_char,
            &mut joystick_strafe_axis as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"joystick_x_invert\0".as_ptr() as *mut c_char,
            &mut joystick_x_invert as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"joystick_y_invert\0".as_ptr() as *mut c_char,
            &mut joystick_y_invert as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"joystick_strafe_invert\0".as_ptr() as *mut c_char,
            &mut joystick_strafe_invert as *mut c_int as *mut c_void,
        );

        for i in 0..NUM_VIRTUAL_BUTTONS {
            let mut name: [c_char; 32] = [0; 32];
            c_write!(name, "joystick_physical_button{}", i);
            M_BindVariable(
                name.as_ptr() as *mut c_char,
                &mut joystick_physical_buttons[i] as *mut c_int as *mut c_void,
            );
        }
    }
}
