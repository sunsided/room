//! Rust port of vendor/doomgeneric/i_video.c.
//!
//! Video output shim: allocates the palette-indexed `I_VideoBuffer`,
//! expands it through the gamma-corrected palette into BGRA `DG_ScreenBuffer`
//! on `I_FinishUpdate`, and forwards calls to the platform layer.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_float, c_int, c_void};
use std::mem;
use std::ptr;

use super::z_zone::PU_STATIC;

pub const SCREENWIDTH: c_int = 320;
pub const SCREENHEIGHT: c_int = 200;

#[repr(C)]
#[derive(Clone, Copy)]
struct Color {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}

struct FB_BitField {
    offset: u32,
    length: u32,
}

struct FB_ScreenInfo {
    xres: u32,
    yres: u32,
    xres_virtual: u32,
    yres_virtual: u32,
    bits_per_pixel: u32,
    red: FB_BitField,
    green: FB_BitField,
    blue: FB_BitField,
    transp: FB_BitField,
}

#[no_mangle]
pub static mut I_VideoBuffer: *mut u8 = ptr::null_mut();

#[no_mangle]
pub static mut screenvisible: c_int = 0;

#[no_mangle]
pub static mut screensaver_mode: c_int = 0;

#[no_mangle]
pub static mut usegamma: c_int = 0;

#[no_mangle]
pub static mut mouse_acceleration: c_float = 2.0;

#[no_mangle]
pub static mut mouse_threshold: c_int = 10;

#[no_mangle]
pub static mut fb_scaling: c_int = 1;

#[no_mangle]
pub static mut usemouse: c_int = 0;

static mut COLORS: [Color; 256] = [Color {
    b: 0,
    g: 0,
    r: 0,
    a: 0,
}; 256];

static mut s_Fb: FB_ScreenInfo = FB_ScreenInfo {
    xres: 0,
    yres: 0,
    xres_virtual: 0,
    yres_virtual: 0,
    bits_per_pixel: 0,
    red: FB_BitField {
        offset: 0,
        length: 0,
    },
    green: FB_BitField {
        offset: 0,
        length: 0,
    },
    blue: FB_BitField {
        offset: 0,
        length: 0,
    },
    transp: FB_BitField {
        offset: 0,
        length: 0,
    },
};

extern "C" {
    fn DG_DrawFrame();
    fn DG_SetWindowTitle(title: *const c_char);
    fn I_InitInput();
    fn I_GetEvent();
    fn M_CheckParmWithArgs(check: *const c_char, num_args: c_int) -> c_int;
    static gammatable: [[u8; 256]; 5];
    static mut myargv: *mut *mut c_char;
    fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void;
    fn Z_Free(ptr: *mut c_void);
    fn I_Error(format: *const c_char, ...);
}

unsafe fn cmap_to_fb(out: *mut u8, inp: *mut u8, in_pixels: c_int) {
    let bpp = s_Fb.bits_per_pixel;

    if bpp == 16 {
        cmap_to_rgb565(out, inp, in_pixels);
        return;
    }

    if bpp != 32 {
        // For safety, default to 32bpp path
    }

    let mut out_ptr = out;
    let mut inp_ptr = inp;

    for _ in 0..in_pixels {
        let idx = *inp_ptr as usize;
        let c = COLORS[idx];

        let pix = ((c.r as u32) << s_Fb.red.offset)
            | ((c.g as u32) << s_Fb.green.offset)
            | ((c.b as u32) << s_Fb.blue.offset);

        for _ in 0..fb_scaling {
            (out_ptr as *mut u32).write_unaligned(pix);
            out_ptr = out_ptr.add(4);
        }

        inp_ptr = inp_ptr.add(1);
    }
}

unsafe fn cmap_to_rgb565(out: *mut u8, inp: *mut u8, in_pixels: c_int) {
    let mut out_ptr = out as *mut u16;
    let mut inp_ptr = inp;

    for _ in 0..in_pixels {
        let idx = *inp_ptr as usize;
        let c = COLORS[idx];

        let p = (((c.r as u16) & 0xF8) << 8) | (((c.g as u16) & 0xFC) << 3) | ((c.b as u16) >> 3);

        for _ in 0..fb_scaling {
            *out_ptr = p;
            out_ptr = out_ptr.add(1);
        }

        inp_ptr = inp_ptr.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn I_InitGraphics() {
    s_Fb = mem::zeroed::<FB_ScreenInfo>();
    s_Fb.xres = doomgeneric_sys::DOOMGENERIC_RESX as u32;
    s_Fb.yres = doomgeneric_sys::DOOMGENERIC_RESY as u32;
    s_Fb.xres_virtual = s_Fb.xres;
    s_Fb.yres_virtual = s_Fb.yres;

    // Default to rgba8888
    s_Fb.bits_per_pixel = 32;
    s_Fb.blue.length = 8;
    s_Fb.green.length = 8;
    s_Fb.red.length = 8;
    s_Fb.transp.length = 8;
    s_Fb.blue.offset = 0;
    s_Fb.green.offset = 8;
    s_Fb.red.offset = 16;
    s_Fb.transp.offset = 24;

    // Check for -gfxmode arg
    let gfxmodeparm = M_CheckParmWithArgs(b"-gfxmode\0".as_ptr() as *const c_char, 1);
    if gfxmodeparm != 0 {
        let mode = *myargv.add((gfxmodeparm + 1) as usize);
        if !mode.is_null() {
            let mode_str = std::ffi::CStr::from_ptr(mode);
            if mode_str.to_bytes() == b"rgba8888" {
                s_Fb.bits_per_pixel = 32;
                s_Fb.blue.length = 8;
                s_Fb.green.length = 8;
                s_Fb.red.length = 8;
                s_Fb.transp.length = 8;
                s_Fb.blue.offset = 0;
                s_Fb.green.offset = 8;
                s_Fb.red.offset = 16;
                s_Fb.transp.offset = 24;
            } else if mode_str.to_bytes() == b"rgb565" {
                s_Fb.bits_per_pixel = 16;
                s_Fb.blue.length = 5;
                s_Fb.green.length = 6;
                s_Fb.red.length = 5;
                s_Fb.transp.length = 0;
                s_Fb.blue.offset = 11;
                s_Fb.green.offset = 5;
                s_Fb.red.offset = 0;
                s_Fb.transp.offset = 16;
            } else {
                I_Error(
                    b"Unknown gfxmode value: %s\n\0".as_ptr() as *const c_char,
                    mode,
                );
            }
        }
    }

    // Auto-scaling factor
    let scale_parm = M_CheckParmWithArgs(b"-scaling\0".as_ptr() as *const c_char, 1);
    if scale_parm != 0 {
        let val = *myargv.add((scale_parm + 1) as usize);
        if !val.is_null() {
            // Simple atoi-like conversion
            let mut n: c_int = 0;
            let mut p = val;
            while *p >= b'0' as c_char && *p <= b'9' as c_char {
                n = n * 10 + (*p as c_int - b'0' as c_int);
                p = p.add(1);
            }
            fb_scaling = n;
        }
    } else {
        fb_scaling = (s_Fb.xres / SCREENWIDTH as u32) as c_int;
        let y_scale = (s_Fb.yres / SCREENHEIGHT as u32) as c_int;
        if y_scale < fb_scaling {
            fb_scaling = y_scale;
        }
    }

    // Allocate video buffer
    I_VideoBuffer = Z_Malloc(SCREENWIDTH * SCREENHEIGHT, PU_STATIC, ptr::null_mut()) as *mut u8;

    screenvisible = 1;

    I_InitInput();
}

#[no_mangle]
pub unsafe extern "C" fn I_ShutdownGraphics() {
    Z_Free(I_VideoBuffer as *mut c_void);
    I_VideoBuffer = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn I_StartFrame() {}

#[no_mangle]
pub unsafe extern "C" fn I_StartTic() {
    I_GetEvent();
}

#[no_mangle]
pub unsafe extern "C" fn I_UpdateNoBlit() {}

#[no_mangle]
pub unsafe extern "C" fn I_FinishUpdate() {
    let y_offset = (((s_Fb.yres as i32 - (SCREENHEIGHT as c_int * fb_scaling))
        * (s_Fb.bits_per_pixel as c_int / 8))
        / 2)
    .max(0) as u32;
    let x_offset = (((s_Fb.xres as i32 - (SCREENWIDTH as c_int * fb_scaling))
        * (s_Fb.bits_per_pixel as c_int / 8))
        / 2)
    .max(0) as u32;
    let x_offset_end = ((s_Fb.xres as i32 - (SCREENWIDTH as c_int * fb_scaling))
        * (s_Fb.bits_per_pixel as c_int / 8)
        - x_offset as i32)
        .max(0) as u32;

    let mut line_in = I_VideoBuffer;
    let screen_ptr = doomgeneric_sys::DG_ScreenBuffer as *mut u8;
    let mut line_out = screen_ptr.add(y_offset as usize + x_offset as usize);

    for _ in 0..SCREENHEIGHT as usize {
        for _i in 0..fb_scaling {
            line_out = line_out.add(x_offset as usize);

            cmap_to_fb(line_out, line_in, SCREENWIDTH as c_int);

            line_out = line_out.add(
                (SCREENWIDTH as c_int * fb_scaling * (s_Fb.bits_per_pixel as c_int / 8)) as usize
                    + x_offset_end as usize,
            );
        }
        line_in = line_in.add(SCREENWIDTH as usize);
    }

    DG_DrawFrame();
}

#[no_mangle]
pub unsafe extern "C" fn I_ReadScreen(scr: *mut u8) {
    std::ptr::copy(I_VideoBuffer, scr, (SCREENWIDTH * SCREENHEIGHT) as usize);
}

#[no_mangle]
pub unsafe extern "C" fn I_SetPalette(palette: *mut u8) {
    let mut p = palette;
    for i in 0..256 {
        COLORS[i].a = 0;
        COLORS[i].r = gammatable[usegamma as usize][*p as usize];
        p = p.add(1);
        COLORS[i].g = gammatable[usegamma as usize][*p as usize];
        p = p.add(1);
        COLORS[i].b = gammatable[usegamma as usize][*p as usize];
        p = p.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn I_GetPaletteIndex(_r: c_int, _g: c_int, _b: c_int) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn I_BeginRead() {}

#[no_mangle]
pub unsafe extern "C" fn I_EndRead() {}

#[no_mangle]
pub unsafe extern "C" fn I_SetWindowTitle(title: *mut c_char) {
    DG_SetWindowTitle(title);
}

#[no_mangle]
pub unsafe extern "C" fn I_GraphicsCheckCommandLine() {}

#[no_mangle]
pub unsafe extern "C" fn I_SetGrabMouseCallback(_func: *mut c_void) {}

#[no_mangle]
pub unsafe extern "C" fn I_EnableLoadingDisk() {}

#[no_mangle]
pub unsafe extern "C" fn I_BindVideoVariables() {}

#[no_mangle]
pub unsafe extern "C" fn I_DisplayFPSDots(_dots_on: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn I_CheckIsScreensaver() {}

#[no_mangle]
pub unsafe extern "C" fn I_Video_Link_Anchor() {
    I_InitGraphics();
    I_ShutdownGraphics();
    I_StartFrame();
    I_StartTic();
    I_UpdateNoBlit();
    I_FinishUpdate();
    I_ReadScreen(ptr::null_mut());
    I_SetPalette(ptr::null_mut());
    I_GetPaletteIndex(0, 0, 0);
    I_BeginRead();
    I_EndRead();
    I_SetWindowTitle(ptr::null_mut());
    I_GraphicsCheckCommandLine();
    I_SetGrabMouseCallback(ptr::null_mut());
    I_EnableLoadingDisk();
    I_BindVideoVariables();
    I_DisplayFPSDots(0);
    I_CheckIsScreensaver();
}
