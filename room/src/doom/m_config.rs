#![allow(non_upper_case_globals, non_snake_case, static_mut_refs)]
#![allow(clippy::manual_c_str_literals)]

use crate::i_error;
use std::ffi::{c_char, c_float, c_int, c_void, CStr};
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum DefaultType {
    Int = 0,
    IntHex,
    String,
    Float,
    Key,
}

#[repr(C)]
struct Default {
    name: *const c_char,
    location: *mut c_void,
    ty: DefaultType,
    untranslated: c_int,
    original_translated: c_int,
    bound: bool,
}

struct DefaultCollection {
    defaults: *mut Default,
    numdefaults: c_int,
    filename: *mut c_char,
}

const fn cfg(name: &'static [u8], ty: DefaultType) -> Default {
    Default {
        name: name.as_ptr() as *const c_char,
        location: ptr::null_mut(),
        ty,
        untranslated: 0,
        original_translated: 0,
        bound: false,
    }
}

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn atof(s: *const c_char) -> f64;
    fn malloc(n: usize) -> *mut c_void;
}

use crate::doom::m_argv::{myargv, M_CheckParmWithArgs};
use crate::doom::m_misc::{M_MakeDirectory, M_StringJoinA};

#[no_mangle]
pub static mut configdir: *mut c_char = ptr::null_mut();

static mut default_main_config: *mut c_char = ptr::null_mut();
static mut default_extra_config: *mut c_char = ptr::null_mut();

static mut DOOM_DEFAULTS_LIST: [Default; 76] = [
    cfg(b"mouse_sensitivity\0", DefaultType::Int),
    cfg(b"sfx_volume\0", DefaultType::Int),
    cfg(b"music_volume\0", DefaultType::Int),
    cfg(b"show_talk\0", DefaultType::Int),
    cfg(b"voice_volume\0", DefaultType::Int),
    cfg(b"show_messages\0", DefaultType::Int),
    cfg(b"key_right\0", DefaultType::Key),
    cfg(b"key_left\0", DefaultType::Key),
    cfg(b"key_up\0", DefaultType::Key),
    cfg(b"key_down\0", DefaultType::Key),
    cfg(b"key_strafeleft\0", DefaultType::Key),
    cfg(b"key_straferight\0", DefaultType::Key),
    cfg(b"key_useHealth\0", DefaultType::Key),
    cfg(b"key_jump\0", DefaultType::Key),
    cfg(b"key_flyup\0", DefaultType::Key),
    cfg(b"key_flydown\0", DefaultType::Key),
    cfg(b"key_flycenter\0", DefaultType::Key),
    cfg(b"key_lookup\0", DefaultType::Key),
    cfg(b"key_lookdown\0", DefaultType::Key),
    cfg(b"key_lookcenter\0", DefaultType::Key),
    cfg(b"key_invquery\0", DefaultType::Key),
    cfg(b"key_mission\0", DefaultType::Key),
    cfg(b"key_invPop\0", DefaultType::Key),
    cfg(b"key_invKey\0", DefaultType::Key),
    cfg(b"key_invHome\0", DefaultType::Key),
    cfg(b"key_invEnd\0", DefaultType::Key),
    cfg(b"key_invleft\0", DefaultType::Key),
    cfg(b"key_invright\0", DefaultType::Key),
    cfg(b"key_invLeft\0", DefaultType::Key),
    cfg(b"key_invRight\0", DefaultType::Key),
    cfg(b"key_useartifact\0", DefaultType::Key),
    cfg(b"key_invUse\0", DefaultType::Key),
    cfg(b"key_invDrop\0", DefaultType::Key),
    cfg(b"key_lookUp\0", DefaultType::Key),
    cfg(b"key_lookDown\0", DefaultType::Key),
    cfg(b"key_fire\0", DefaultType::Key),
    cfg(b"key_use\0", DefaultType::Key),
    cfg(b"key_strafe\0", DefaultType::Key),
    cfg(b"key_speed\0", DefaultType::Key),
    cfg(b"use_mouse\0", DefaultType::Int),
    cfg(b"mouseb_fire\0", DefaultType::Int),
    cfg(b"mouseb_strafe\0", DefaultType::Int),
    cfg(b"mouseb_forward\0", DefaultType::Int),
    cfg(b"mouseb_jump\0", DefaultType::Int),
    cfg(b"use_joystick\0", DefaultType::Int),
    cfg(b"joyb_fire\0", DefaultType::Int),
    cfg(b"joyb_strafe\0", DefaultType::Int),
    cfg(b"joyb_use\0", DefaultType::Int),
    cfg(b"joyb_speed\0", DefaultType::Int),
    cfg(b"joyb_jump\0", DefaultType::Int),
    cfg(b"screenblocks\0", DefaultType::Int),
    cfg(b"screensize\0", DefaultType::Int),
    cfg(b"detaillevel\0", DefaultType::Int),
    cfg(b"snd_channels\0", DefaultType::Int),
    cfg(b"snd_musicdevice\0", DefaultType::Int),
    cfg(b"snd_sfxdevice\0", DefaultType::Int),
    cfg(b"snd_sbport\0", DefaultType::Int),
    cfg(b"snd_sbirq\0", DefaultType::Int),
    cfg(b"snd_sbdma\0", DefaultType::Int),
    cfg(b"snd_mport\0", DefaultType::Int),
    cfg(b"usegamma\0", DefaultType::Int),
    cfg(b"savedir\0", DefaultType::String),
    cfg(b"messageson\0", DefaultType::Int),
    cfg(b"back_flat\0", DefaultType::String),
    cfg(b"nickname\0", DefaultType::String),
    cfg(b"chatmacro0\0", DefaultType::String),
    cfg(b"chatmacro1\0", DefaultType::String),
    cfg(b"chatmacro2\0", DefaultType::String),
    cfg(b"chatmacro3\0", DefaultType::String),
    cfg(b"chatmacro4\0", DefaultType::String),
    cfg(b"chatmacro5\0", DefaultType::String),
    cfg(b"chatmacro6\0", DefaultType::String),
    cfg(b"chatmacro7\0", DefaultType::String),
    cfg(b"chatmacro8\0", DefaultType::String),
    cfg(b"chatmacro9\0", DefaultType::String),
    cfg(b"comport\0", DefaultType::Int),
];

static mut doom_defaults: DefaultCollection = DefaultCollection {
    defaults: unsafe { &mut DOOM_DEFAULTS_LIST as *mut _ },
    numdefaults: 76,
    filename: ptr::null_mut(),
};

static mut EXTRA_DEFAULTS_LIST: [Default; 119] = [
    cfg(b"graphical_startup\0", DefaultType::Int),
    cfg(b"autoadjust_video_settings\0", DefaultType::Int),
    cfg(b"fullscreen\0", DefaultType::Int),
    cfg(b"aspect_ratio_correct\0", DefaultType::Int),
    cfg(b"startup_delay\0", DefaultType::Int),
    cfg(b"screen_width\0", DefaultType::Int),
    cfg(b"screen_height\0", DefaultType::Int),
    cfg(b"screen_bpp\0", DefaultType::Int),
    cfg(b"grabmouse\0", DefaultType::Int),
    cfg(b"novert\0", DefaultType::Int),
    cfg(b"mouse_acceleration\0", DefaultType::Float),
    cfg(b"mouse_threshold\0", DefaultType::Int),
    cfg(b"snd_samplerate\0", DefaultType::Int),
    cfg(b"snd_cachesize\0", DefaultType::Int),
    cfg(b"snd_maxslicetime_ms\0", DefaultType::Int),
    cfg(b"snd_musiccmd\0", DefaultType::String),
    cfg(b"opl_io_port\0", DefaultType::IntHex),
    cfg(b"show_endoom\0", DefaultType::Int),
    cfg(b"png_screenshots\0", DefaultType::Int),
    cfg(b"vanilla_savegame_limit\0", DefaultType::Int),
    cfg(b"vanilla_demo_limit\0", DefaultType::Int),
    cfg(b"vanilla_keyboard_mapping\0", DefaultType::Int),
    cfg(b"video_driver\0", DefaultType::String),
    cfg(b"window_position\0", DefaultType::String),
    cfg(b"joystick_index\0", DefaultType::Int),
    cfg(b"joystick_x_axis\0", DefaultType::Int),
    cfg(b"joystick_x_invert\0", DefaultType::Int),
    cfg(b"joystick_y_axis\0", DefaultType::Int),
    cfg(b"joystick_y_invert\0", DefaultType::Int),
    cfg(b"joystick_strafe_axis\0", DefaultType::Int),
    cfg(b"joystick_strafe_invert\0", DefaultType::Int),
    cfg(b"joystick_physical_button0\0", DefaultType::Int),
    cfg(b"joystick_physical_button1\0", DefaultType::Int),
    cfg(b"joystick_physical_button2\0", DefaultType::Int),
    cfg(b"joystick_physical_button3\0", DefaultType::Int),
    cfg(b"joystick_physical_button4\0", DefaultType::Int),
    cfg(b"joystick_physical_button5\0", DefaultType::Int),
    cfg(b"joystick_physical_button6\0", DefaultType::Int),
    cfg(b"joystick_physical_button7\0", DefaultType::Int),
    cfg(b"joystick_physical_button8\0", DefaultType::Int),
    cfg(b"joystick_physical_button9\0", DefaultType::Int),
    cfg(b"joyb_strafeleft\0", DefaultType::Int),
    cfg(b"joyb_straferight\0", DefaultType::Int),
    cfg(b"joyb_menu_activate\0", DefaultType::Int),
    cfg(b"joyb_prevweapon\0", DefaultType::Int),
    cfg(b"joyb_nextweapon\0", DefaultType::Int),
    cfg(b"mouseb_strafeleft\0", DefaultType::Int),
    cfg(b"mouseb_straferight\0", DefaultType::Int),
    cfg(b"mouseb_use\0", DefaultType::Int),
    cfg(b"mouseb_backward\0", DefaultType::Int),
    cfg(b"mouseb_prevweapon\0", DefaultType::Int),
    cfg(b"mouseb_nextweapon\0", DefaultType::Int),
    cfg(b"dclick_use\0", DefaultType::Int),
    cfg(b"key_pause\0", DefaultType::Key),
    cfg(b"key_menu_activate\0", DefaultType::Key),
    cfg(b"key_menu_up\0", DefaultType::Key),
    cfg(b"key_menu_down\0", DefaultType::Key),
    cfg(b"key_menu_left\0", DefaultType::Key),
    cfg(b"key_menu_right\0", DefaultType::Key),
    cfg(b"key_menu_back\0", DefaultType::Key),
    cfg(b"key_menu_forward\0", DefaultType::Key),
    cfg(b"key_menu_confirm\0", DefaultType::Key),
    cfg(b"key_menu_abort\0", DefaultType::Key),
    cfg(b"key_menu_help\0", DefaultType::Key),
    cfg(b"key_menu_save\0", DefaultType::Key),
    cfg(b"key_menu_load\0", DefaultType::Key),
    cfg(b"key_menu_volume\0", DefaultType::Key),
    cfg(b"key_menu_detail\0", DefaultType::Key),
    cfg(b"key_menu_qsave\0", DefaultType::Key),
    cfg(b"key_menu_endgame\0", DefaultType::Key),
    cfg(b"key_menu_messages\0", DefaultType::Key),
    cfg(b"key_menu_qload\0", DefaultType::Key),
    cfg(b"key_menu_quit\0", DefaultType::Key),
    cfg(b"key_menu_gamma\0", DefaultType::Key),
    cfg(b"key_spy\0", DefaultType::Key),
    cfg(b"key_menu_incscreen\0", DefaultType::Key),
    cfg(b"key_menu_decscreen\0", DefaultType::Key),
    cfg(b"key_menu_screenshot\0", DefaultType::Key),
    cfg(b"key_map_toggle\0", DefaultType::Key),
    cfg(b"key_map_north\0", DefaultType::Key),
    cfg(b"key_map_south\0", DefaultType::Key),
    cfg(b"key_map_east\0", DefaultType::Key),
    cfg(b"key_map_west\0", DefaultType::Key),
    cfg(b"key_map_zoomin\0", DefaultType::Key),
    cfg(b"key_map_zoomout\0", DefaultType::Key),
    cfg(b"key_map_maxzoom\0", DefaultType::Key),
    cfg(b"key_map_follow\0", DefaultType::Key),
    cfg(b"key_map_grid\0", DefaultType::Key),
    cfg(b"key_map_mark\0", DefaultType::Key),
    cfg(b"key_map_clearmark\0", DefaultType::Key),
    cfg(b"key_weapon1\0", DefaultType::Key),
    cfg(b"key_weapon2\0", DefaultType::Key),
    cfg(b"key_weapon3\0", DefaultType::Key),
    cfg(b"key_weapon4\0", DefaultType::Key),
    cfg(b"key_weapon5\0", DefaultType::Key),
    cfg(b"key_weapon6\0", DefaultType::Key),
    cfg(b"key_weapon7\0", DefaultType::Key),
    cfg(b"key_weapon8\0", DefaultType::Key),
    cfg(b"key_prevweapon\0", DefaultType::Key),
    cfg(b"key_nextweapon\0", DefaultType::Key),
    cfg(b"key_arti_all\0", DefaultType::Key),
    cfg(b"key_arti_health\0", DefaultType::Key),
    cfg(b"key_arti_poisonbag\0", DefaultType::Key),
    cfg(b"key_arti_blastradius\0", DefaultType::Key),
    cfg(b"key_arti_teleport\0", DefaultType::Key),
    cfg(b"key_arti_teleportother\0", DefaultType::Key),
    cfg(b"key_arti_egg\0", DefaultType::Key),
    cfg(b"key_arti_invulnerability\0", DefaultType::Key),
    cfg(b"key_message_refresh\0", DefaultType::Key),
    cfg(b"key_demo_quit\0", DefaultType::Key),
    cfg(b"key_multi_msg\0", DefaultType::Key),
    cfg(b"key_multi_msgplayer1\0", DefaultType::Key),
    cfg(b"key_multi_msgplayer2\0", DefaultType::Key),
    cfg(b"key_multi_msgplayer3\0", DefaultType::Key),
    cfg(b"key_multi_msgplayer4\0", DefaultType::Key),
    cfg(b"key_multi_msgplayer5\0", DefaultType::Key),
    cfg(b"key_multi_msgplayer6\0", DefaultType::Key),
    cfg(b"key_multi_msgplayer7\0", DefaultType::Key),
    cfg(b"key_multi_msgplayer8\0", DefaultType::Key),
];

static mut extra_defaults: DefaultCollection = DefaultCollection {
    defaults: unsafe { &mut EXTRA_DEFAULTS_LIST as *mut _ },
    numdefaults: 119,
    filename: ptr::null_mut(),
};

// scantokey mapping from DOS keyboard scan codes to internal key codes.
// Key constant values from doomkeys.h:
//   KEY_BACKSPACE=0x7f, KEY_RCTRL=0x9D, KEY_RSHIFT=0xB6, KEYP_MULTIPLY='*',
//   KEY_RALT=0xB8, KEY_CAPSLOCK=0xBA, KEY_F1=0xBB, KEY_F2=0xBC, KEY_F3=0xBD,
//   KEY_F4=0xBE, KEY_F5=0xBF, KEY_F6=0xC0, KEY_F7=0xC1, KEY_F8=0xC2,
//   KEY_F9=0xC3, KEY_F10=0xC4, KEY_PAUSE=0xFF, KEY_SCRLCK=0xC6,
//   KEY_HOME=0xC7, KEY_UPARROW=0xAD, KEY_PGUP=0xC9, KEY_MINUS=0x2D,
//   KEY_LEFTARROW=0xAC, KEYP_5='5', KEY_RIGHTARROW=0xAE, KEYP_PLUS='+',
//   KEY_END=0xCF, KEY_DOWNARROW=0xAF, KEY_PGDN=0xD1, KEY_INS=0xD2,
//   KEY_DEL=0xD3, KEY_F11=0xD7, KEY_F12=0xD8, KEY_PRTSCR=0xD9
const KEY_BACKSPACE: c_int = 0x7f;
const KEY_RCTRL: c_int = 0x9D;
const KEY_RSHIFT: c_int = 0xB6;
const KEYP_MULTIPLY: c_int = b'*' as c_int;
const KEY_RALT: c_int = 0xB8;
const KEY_CAPSLOCK: c_int = 0xBA;
const KEY_F1: c_int = 0xBB;
const KEY_F2: c_int = 0xBC;
const KEY_F3: c_int = 0xBD;
const KEY_F4: c_int = 0xBE;
const KEY_F5: c_int = 0xBF;
const KEY_F6: c_int = 0xC0;
const KEY_F7: c_int = 0xC1;
const KEY_F8: c_int = 0xC2;
const KEY_F9: c_int = 0xC3;
const KEY_F10: c_int = 0xC4;
const KEY_PAUSE: c_int = 0xFF;
const KEY_SCRLCK: c_int = 0xC6;
const KEY_HOME: c_int = 0xC7;
const KEY_UPARROW: c_int = 0xAD;
const KEY_PGUP: c_int = 0xC9;
const KEY_MINUS: c_int = 0x2D;
const KEY_LEFTARROW: c_int = 0xAC;
const KEYP_5: c_int = b'5' as c_int;
const KEY_RIGHTARROW: c_int = 0xAE;
const KEYP_PLUS: c_int = b'+' as c_int;
const KEY_END: c_int = 0xCF;
const KEY_DOWNARROW: c_int = 0xAF;
const KEY_PGDN: c_int = 0xD1;
const KEY_INS: c_int = 0xD2;
const KEY_DEL: c_int = 0xD3;
const KEY_F11: c_int = 0xD7;
const KEY_F12: c_int = 0xD8;
const KEY_PRTSCR: c_int = 0xD9;

const SCANTOKEY: [c_int; 128] = [
    0,
    27,
    b'1' as c_int,
    b'2' as c_int,
    b'3' as c_int,
    b'4' as c_int,
    b'5' as c_int,
    b'6' as c_int,
    b'7' as c_int,
    b'8' as c_int,
    b'9' as c_int,
    b'0' as c_int,
    b'-' as c_int,
    b'=' as c_int,
    KEY_BACKSPACE,
    9,
    b'q' as c_int,
    b'w' as c_int,
    b'e' as c_int,
    b'r' as c_int,
    b't' as c_int,
    b'y' as c_int,
    b'u' as c_int,
    b'i' as c_int,
    b'o' as c_int,
    b'p' as c_int,
    b'[' as c_int,
    b']' as c_int,
    13,
    KEY_RCTRL,
    b'a' as c_int,
    b's' as c_int,
    b'd' as c_int,
    b'f' as c_int,
    b'g' as c_int,
    b'h' as c_int,
    b'j' as c_int,
    b'k' as c_int,
    b'l' as c_int,
    b';' as c_int,
    b'\'' as c_int,
    b'`' as c_int,
    KEY_RSHIFT,
    b'\\' as c_int,
    b'z' as c_int,
    b'x' as c_int,
    b'c' as c_int,
    b'v' as c_int,
    b'b' as c_int,
    b'n' as c_int,
    b'm' as c_int,
    b',' as c_int,
    b'.' as c_int,
    b'/' as c_int,
    KEY_RSHIFT,
    KEYP_MULTIPLY,
    KEY_RALT,
    b' ' as c_int,
    KEY_CAPSLOCK,
    KEY_F1,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_F10,
    KEY_PAUSE,
    KEY_SCRLCK,
    KEY_HOME,
    KEY_UPARROW,
    KEY_PGUP,
    KEY_MINUS,
    KEY_LEFTARROW,
    KEYP_5,
    KEY_RIGHTARROW,
    KEYP_PLUS,
    KEY_END,
    KEY_DOWNARROW,
    KEY_PGDN,
    KEY_INS,
    KEY_DEL,
    0,
    0,
    0,
    KEY_F11,
    KEY_F12,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    KEY_PRTSCR,
    0,
];

const DIR_SEPARATOR_S: &[u8] = b"/\0";

unsafe fn search_collection(
    collection: *const DefaultCollection,
    name: *const c_char,
) -> *mut Default {
    let collection = &*collection;
    for i in 0..collection.numdefaults {
        let def = collection.defaults.add(i as usize);
        if strcmp(name, (*def).name) == 0 {
            return def;
        }
    }
    ptr::null_mut()
}

unsafe fn get_default_for_name(name: *const c_char) -> *mut Default {
    let mut result = search_collection(&doom_defaults, name);
    if result.is_null() {
        result = search_collection(&extra_defaults, name);
    }
    if result.is_null() {
        i_error!(
            "Unknown configuration variable: '{}'",
            std::ffi::CStr::from_ptr(name).to_string_lossy()
        );
    }
    result
}

fn parse_int_parameter(strparm: &CStr) -> c_int {
    let bytes = strparm.to_bytes();
    if bytes.len() >= 2 && bytes[0] == b'0' && (bytes[1] == b'x' || bytes[1] == b'X') {
        if let Ok(val) = i32::from_str_radix(std::str::from_utf8(&bytes[2..]).unwrap_or("0"), 16) {
            return val;
        }
    }
    if let Ok(val) = std::str::from_utf8(bytes).unwrap_or("0").parse::<c_int>() {
        return val;
    }
    0
}

unsafe fn set_variable(def: *mut Default, value: *const c_char) {
    let def = &mut *def;
    match def.ty {
        DefaultType::String => {
            *(def.location as *mut *mut c_char) = strdup(value);
        }
        DefaultType::Int | DefaultType::IntHex => {
            let strparm = CStr::from_ptr(value);
            *(def.location as *mut c_int) = parse_int_parameter(strparm);
        }
        DefaultType::Key => {
            let strparm = CStr::from_ptr(value);
            let intparm = parse_int_parameter(strparm);
            def.untranslated = intparm;
            let translated = if (0..128).contains(&intparm) {
                SCANTOKEY[intparm as usize]
            } else {
                0
            };
            def.original_translated = translated;
            *(def.location as *mut c_int) = translated;
        }
        DefaultType::Float => {
            *(def.location as *mut c_float) = atof(value) as c_float;
        }
    }
}

#[no_mangle]
pub extern "C" fn M_SetConfigFilenames(main_config: *mut c_char, extra_config: *mut c_char) {
    unsafe {
        default_main_config = main_config;
        default_extra_config = extra_config;
    }
}

#[no_mangle]
pub extern "C" fn M_SaveDefaults() {
    // No-op: ORIGCODE is undefined, so the file-I/O body is never compiled
    // in the original C either.
}

#[no_mangle]
pub extern "C" fn M_SaveDefaultsAlternate(main: *mut c_char, extra: *mut c_char) {
    unsafe {
        let orig_main = doom_defaults.filename;
        let orig_extra = extra_defaults.filename;
        doom_defaults.filename = main;
        extra_defaults.filename = extra;
        M_SaveDefaults();
        doom_defaults.filename = orig_main;
        extra_defaults.filename = orig_extra;
    }
}

#[no_mangle]
pub extern "C" fn M_LoadDefaults() {
    unsafe {
        let i = M_CheckParmWithArgs(b"-config\0".as_ptr() as *mut c_char, 1);
        if i != 0 {
            doom_defaults.filename = *myargv.offset((i + 1) as isize);
            printf(
                b"\tdefault file: %s\n\0".as_ptr() as *const c_char,
                doom_defaults.filename,
            );
        } else {
            let strs: [*const c_char; 3] = [configdir, default_main_config, std::ptr::null()];
            // SAFETY: null-terminated pointer array; result lives for the duration of the program.
            doom_defaults.filename = M_StringJoinA(strs.as_ptr());
        }

        printf(
            b"saving config in %s\n\0".as_ptr() as *const c_char,
            doom_defaults.filename,
        );

        let i = M_CheckParmWithArgs(b"-extraconfig\0".as_ptr() as *mut c_char, 1);
        if i != 0 {
            extra_defaults.filename = *myargv.offset((i + 1) as isize);
            printf(
                b"        extra configuration file: %s\n\0".as_ptr() as *const c_char,
                extra_defaults.filename,
            );
        } else {
            let strs: [*const c_char; 3] = [configdir, default_extra_config, std::ptr::null()];
            // SAFETY: null-terminated pointer array; result lives for the duration of the program.
            extra_defaults.filename = M_StringJoinA(strs.as_ptr());
        }

        // No-ops because ORIGCODE is undefined
        let _ = &doom_defaults;
        let _ = &extra_defaults;
    }
}

#[no_mangle]
pub extern "C" fn M_BindVariable(name: *mut c_char, location: *mut c_void) {
    unsafe {
        let variable = get_default_for_name(name as *const c_char);
        (*variable).location = location;
        (*variable).bound = true;
    }
}

#[no_mangle]
pub extern "C" fn M_SetVariable(name: *mut c_char, value: *mut c_char) -> bool {
    unsafe {
        let variable = get_default_for_name(name as *const c_char);
        if variable.is_null() || !(*variable).bound {
            return false;
        }
        set_variable(variable, value as *const c_char);
        true
    }
}

#[no_mangle]
pub extern "C" fn M_GetIntVariable(name: *mut c_char) -> c_int {
    unsafe {
        let variable = get_default_for_name(name as *const c_char);
        if variable.is_null()
            || !(*variable).bound
            || ((*variable).ty != DefaultType::Int && (*variable).ty != DefaultType::IntHex)
        {
            return 0;
        }
        *((*variable).location as *mut c_int)
    }
}

#[no_mangle]
pub extern "C" fn M_GetStrVariable(name: *mut c_char) -> *const c_char {
    unsafe {
        let variable = get_default_for_name(name as *const c_char);
        if variable.is_null() || !(*variable).bound || (*variable).ty != DefaultType::String {
            return ptr::null();
        }
        *((*variable).location as *mut *const c_char)
    }
}

#[no_mangle]
pub extern "C" fn M_GetFloatVariable(name: *mut c_char) -> c_float {
    unsafe {
        let variable = get_default_for_name(name as *const c_char);
        if variable.is_null() || !(*variable).bound || (*variable).ty != DefaultType::Float {
            return 0.0;
        }
        *((*variable).location as *mut c_float)
    }
}

unsafe fn GetDefaultConfigDir() -> *mut c_char {
    let result = malloc(2) as *mut c_char;
    *result = b'.' as c_char;
    *result.offset(1) = 0;
    result
}

#[no_mangle]
pub extern "C" fn M_SetConfigDir(dir: *mut c_char) {
    unsafe {
        if !dir.is_null() {
            configdir = dir;
        } else {
            configdir = GetDefaultConfigDir();
        }

        if strcmp(configdir, b"\0".as_ptr() as *const c_char) != 0 {
            printf(
                b"Using %s for configuration and saves\n\0".as_ptr() as *const c_char,
                configdir,
            );
        }

        M_MakeDirectory(configdir);
    }
}

#[no_mangle]
pub extern "C" fn M_GetSaveGameDir(_iwadname: *mut c_char) -> *mut c_char {
    unsafe {
        if strcmp(configdir, b"\0".as_ptr() as *const c_char) == 0 {
            strdup(b"\0".as_ptr() as *const c_char)
        } else {
            let strs: [*const c_char; 4] = [
                configdir,
                DIR_SEPARATOR_S.as_ptr() as *const c_char,
                b".savegame/\0".as_ptr() as *const c_char,
                std::ptr::null(),
            ];
            // SAFETY: null-terminated pointer array; ownership transferred to caller via return.
            let savegamedir = M_StringJoinA(strs.as_ptr());
            M_MakeDirectory(savegamedir);
            printf(
                b"Using %s for savegames\n\0".as_ptr() as *const c_char,
                savegamedir,
            );
            savegamedir
        }
    }
}
