//! Rust port of vendor/doomgeneric/i_scale.c.
//!
//! Screen scale-up code: 1x,2x,3x,4x,5x pixel doubling and
//! aspect ratio-correcting stretch/squash functions.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use crate::doom::c_ffi::screen_mode_t;
use crate::doom::m_argv::M_CheckParm;
use crate::doom::z_zone::{Z_Free, Z_Malloc};
use std::ffi::{c_int, c_void};
use std::ptr;

extern "C" {
    static mut stdout: *mut libc::FILE;
}

use crate::doom::i_video::{SCREENHEIGHT, SCREENWIDTH};
use crate::doom::z_zone::PU_STATIC;

static mut src_buffer: *mut u8 = ptr::null_mut();
static mut dest_buffer: *mut u8 = ptr::null_mut();
static mut dest_pitch: c_int = 0;
static mut stretch_tables: [*mut u8; 2] = [ptr::null_mut(); 2];
static mut half_stretch_table: *mut u8 = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn I_InitScale(
    _src_buffer: *mut u8,
    _dest_buffer: *mut u8,
    _dest_pitch: c_int,
) {
    src_buffer = _src_buffer;
    dest_buffer = _dest_buffer;
    dest_pitch = _dest_pitch;
}

// ---------------------------------------------------------------------------
// Pixel-doubling scale-up functions
// ---------------------------------------------------------------------------

unsafe extern "C" fn i_scale_1x(x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int {
    let w = (x2 - x1) as usize;
    let mut bufp = src_buffer.add((y1 * SCREENWIDTH + x1) as usize);
    let mut screenp = dest_buffer.add((y1 * dest_pitch + x1) as usize);
    for _ in y1..y2 {
        std::ptr::copy_nonoverlapping(bufp, screenp, w);
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe fn scale_nx<const N: usize>(x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int {
    let multi_pitch = dest_pitch * N as c_int;
    let mut bufp = src_buffer.add((y1 * SCREENWIDTH + x1) as usize);
    let base = dest_buffer.add(((y1 * dest_pitch + x1) * N as c_int) as usize);
    let mut rows: [*mut u8; 5] = [ptr::null_mut(); 5];
    for i in 0..N {
        rows[i] = base.add((dest_pitch * i as c_int) as usize);
    }
    let w = (x2 - x1) as usize;

    for _ in y1..y2 {
        let mut bp = bufp;
        for _ in 0..w {
            let c = *bp;
            for i in 0..N {
                let sp = rows[i];
                for j in 0..N {
                    *sp.add(j) = c;
                }
                rows[i] = sp.add(N);
            }
            bp = bp.add(1);
        }
        for i in 0..N {
            rows[i] = rows[i].add((multi_pitch as usize).wrapping_sub(w * N));
        }
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_scale_2x(x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int {
    scale_nx::<2>(x1, y1, x2, y2)
}
unsafe extern "C" fn i_scale_3x(x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int {
    scale_nx::<3>(x1, y1, x2, y2)
}
unsafe extern "C" fn i_scale_4x(x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int {
    scale_nx::<4>(x1, y1, x2, y2)
}
unsafe extern "C" fn i_scale_5x(x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int {
    scale_nx::<5>(x1, y1, x2, y2)
}

// ---------------------------------------------------------------------------
// Stretch / squash line helpers
// ---------------------------------------------------------------------------

unsafe fn write_line_nx<const N: usize>(dest: *mut u8, src: *mut u8) {
    let mut d = dest;
    let mut s = src;
    for _ in 0..SCREENWIDTH {
        let c = *s;
        for i in 0..N {
            *d.add(i) = c;
        }
        d = d.add(N);
        s = s.add(1);
    }
}

unsafe fn write_blended_line_nx<const N: usize>(
    dest: *mut u8,
    src1: *mut u8,
    src2: *mut u8,
    stretch_table: *mut u8,
) {
    let mut d = dest;
    let mut s1 = src1;
    let mut s2 = src2;
    for _ in 0..SCREENWIDTH {
        let val = *stretch_table.add((*s1 as usize) * 256 + (*s2 as usize));
        for i in 0..N {
            *d.add(i) = val;
        }
        d = d.add(N);
        s1 = s1.add(1);
        s2 = s2.add(1);
    }
}

unsafe fn write_blended_line_1x(
    dest: *mut u8,
    src1: *mut u8,
    src2: *mut u8,
    stretch_table: *mut u8,
) {
    let mut d = dest;
    let mut s1 = src1;
    let mut s2 = src2;
    for _ in 0..SCREENWIDTH {
        *d = *stretch_table.add((*s1 as usize) * 256 + (*s2 as usize));
        d = d.add(1);
        s1 = s1.add(1);
        s2 = s2.add(1);
    }
}

// ---------------------------------------------------------------------------
// Lookup-table generation
// ---------------------------------------------------------------------------

unsafe fn find_nearest_color(palette: *mut u8, r: c_int, g: c_int, b: c_int) -> c_int {
    let mut best: c_int = 0;
    let mut best_diff = c_int::MAX;
    for i in 0..256 {
        let col = palette.add(i * 3);
        let dr = r - *col as c_int;
        let dg = g - *col.add(1) as c_int;
        let db = b - *col.add(2) as c_int;
        let diff = dr * dr + dg * dg + db * db;
        if diff == 0 {
            return i as c_int;
        }
        if diff < best_diff {
            best = i as c_int;
            best_diff = diff;
        }
    }
    best
}

unsafe fn generate_stretch_table(palette: *mut u8, pct: c_int) -> *mut u8 {
    let result = Z_Malloc(256 * 256, PU_STATIC, ptr::null_mut()) as *mut u8;
    for x in 0..256 {
        for y in 0..256 {
            let col1 = palette.add(x * 3);
            let col2 = palette.add(y * 3);
            let r = ((*col1 as c_int) * pct + (*col2 as c_int) * (100 - pct)) / 100;
            let g = ((*col1.add(1) as c_int) * pct + (*col2.add(1) as c_int) * (100 - pct)) / 100;
            let b = ((*col1.add(2) as c_int) * pct + (*col2.add(2) as c_int) * (100 - pct)) / 100;
            *result.add(x * 256 + y) = find_nearest_color(palette, r, g, b) as u8;
        }
    }
    result
}

unsafe extern "C" fn i_init_stretch_tables(palette: *mut u8) {
    if !stretch_tables[0].is_null() {
        return;
    }
    libc::printf(c"I_InitStretchTables: Generating lookup tables..".as_ptr());
    libc::fflush(stdout);
    stretch_tables[0] = generate_stretch_table(palette, 20);
    libc::printf(c"..".as_ptr());
    libc::fflush(stdout);
    stretch_tables[1] = generate_stretch_table(palette, 40);
    libc::puts(c"".as_ptr());
}

unsafe extern "C" fn i_init_squash_table(palette: *mut u8) {
    if !half_stretch_table.is_null() {
        return;
    }
    libc::printf(c"I_InitSquashTable: Generating lookup table..".as_ptr());
    libc::fflush(stdout);
    half_stretch_table = generate_stretch_table(palette, 50);
    libc::puts(c"".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn I_ResetScaleTables(palette: *mut u8) {
    if !stretch_tables[0].is_null() {
        Z_Free(stretch_tables[0] as *mut c_void);
        Z_Free(stretch_tables[1] as *mut c_void);
        libc::printf(c"I_ResetScaleTables: Regenerating lookup tables..\n".as_ptr());
        stretch_tables[0] = generate_stretch_table(palette, 20);
        stretch_tables[1] = generate_stretch_table(palette, 40);
    }
    if !half_stretch_table.is_null() {
        Z_Free(half_stretch_table as *mut c_void);
        libc::printf(c"I_ResetScaleTables: Regenerating lookup table..\n".as_ptr());
        half_stretch_table = generate_stretch_table(palette, 50);
    }
}

// ---------------------------------------------------------------------------
// Aspect ratio correcting stretch functions
// ---------------------------------------------------------------------------

unsafe extern "C" fn i_stretch_1x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in (0..SCREENHEIGHT).step_by(5) {
        std::ptr::copy_nonoverlapping(bufp, screenp, SCREENWIDTH as usize);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_1x(
            screenp,
            bufp,
            bufp.add(SCREENWIDTH as usize),
            stretch_tables[0],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_blended_line_1x(
            screenp,
            bufp,
            bufp.add(SCREENWIDTH as usize),
            stretch_tables[1],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_blended_line_1x(
            screenp,
            bufp.add(SCREENWIDTH as usize),
            bufp,
            stretch_tables[1],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_blended_line_1x(
            screenp,
            bufp.add(SCREENWIDTH as usize),
            bufp,
            stretch_tables[0],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        std::ptr::copy_nonoverlapping(bufp, screenp, SCREENWIDTH as usize);
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_stretch_2x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in (0..SCREENHEIGHT).step_by(5) {
        write_line_nx::<2>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<2>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<2>(
            screenp,
            bufp,
            bufp.add(SCREENWIDTH as usize),
            stretch_tables[1],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<2>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<2>(
            screenp,
            bufp.add(SCREENWIDTH as usize),
            bufp,
            stretch_tables[0],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<2>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<2>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<2>(
            screenp,
            bufp,
            bufp.add(SCREENWIDTH as usize),
            stretch_tables[0],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<2>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<2>(
            screenp,
            bufp.add(SCREENWIDTH as usize),
            bufp,
            stretch_tables[1],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<2>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<2>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_stretch_3x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in (0..SCREENHEIGHT).step_by(5) {
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<3>(
            screenp,
            bufp.add(SCREENWIDTH as usize),
            bufp,
            stretch_tables[1],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<3>(
            screenp,
            bufp,
            bufp.add(SCREENWIDTH as usize),
            stretch_tables[0],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<3>(
            screenp,
            bufp.add(SCREENWIDTH as usize),
            bufp,
            stretch_tables[0],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<3>(
            screenp,
            bufp,
            bufp.add(SCREENWIDTH as usize),
            stretch_tables[1],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<3>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_stretch_4x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in (0..SCREENHEIGHT).step_by(5) {
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<4>(
            screenp,
            bufp.add(SCREENWIDTH as usize),
            bufp,
            stretch_tables[0],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<4>(
            screenp,
            bufp.add(SCREENWIDTH as usize),
            bufp,
            stretch_tables[1],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<4>(
            screenp,
            bufp,
            bufp.add(SCREENWIDTH as usize),
            stretch_tables[1],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_blended_line_nx::<4>(
            screenp,
            bufp,
            bufp.add(SCREENWIDTH as usize),
            stretch_tables[0],
        );
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<4>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_stretch_5x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in 0..SCREENHEIGHT {
        write_line_nx::<5>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<5>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<5>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<5>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<5>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        write_line_nx::<5>(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    if M_CheckParm(c"-scanline".as_ptr().cast_mut()) > 0 {
        let mut screenp = dest_buffer.add((2 * dest_pitch) as usize);
        let mut y = 0;
        while y < 1198 {
            std::ptr::write_bytes(screenp, 0, 1600);
            screenp = screenp.add((dest_pitch * 3) as usize);
            y += 3;
        }
    }
    1
}

// ---------------------------------------------------------------------------
// Aspect ratio correcting squash functions
// ---------------------------------------------------------------------------

macro_rules! draw_pixel2 {
    ($dest:expr, $dest2:expr, $c:expr) => {
        *$dest = $c;
        $dest = $dest.add(1);
        *$dest2 = $c;
        $dest2 = $dest2.add(1);
    };
}

macro_rules! draw_pixel3 {
    ($dest:expr, $dest2:expr, $dest3:expr, $c:expr) => {
        *$dest = $c;
        $dest = $dest.add(1);
        *$dest2 = $c;
        $dest2 = $dest2.add(1);
        *$dest3 = $c;
        $dest3 = $dest3.add(1);
    };
}

macro_rules! draw_pixel4 {
    ($dest:expr, $dest2:expr, $dest3:expr, $dest4:expr, $c:expr) => {
        *$dest = $c;
        $dest = $dest.add(1);
        *$dest2 = $c;
        $dest2 = $dest2.add(1);
        *$dest3 = $c;
        $dest3 = $dest3.add(1);
        *$dest4 = $c;
        $dest4 = $dest4.add(1);
    };
}

macro_rules! draw_pixel5 {
    ($dest:expr, $dest2:expr, $dest3:expr, $dest4:expr, $dest5:expr, $c:expr) => {
        *$dest = $c;
        $dest = $dest.add(1);
        *$dest2 = $c;
        $dest2 = $dest2.add(1);
        *$dest3 = $c;
        $dest3 = $dest3.add(1);
        *$dest4 = $c;
        $dest4 = $dest4.add(1);
        *$dest5 = $c;
        $dest5 = $dest5.add(1);
    };
}

unsafe fn write_squashed_line_1x(dest: *mut u8, src: *mut u8) {
    let mut d = dest;
    let mut s = src;
    for _ in 0..(SCREENWIDTH / 5) {
        *d = *stretch_tables[0].add((*s.add(1) as usize) * 256 + (*s as usize));
        d = d.add(1);
        *d = *stretch_tables[1].add((*s.add(2) as usize) * 256 + (*s.add(1) as usize));
        d = d.add(1);
        *d = *stretch_tables[1].add((*s.add(2) as usize) * 256 + (*s.add(3) as usize));
        d = d.add(1);
        *d = *stretch_tables[0].add((*s.add(3) as usize) * 256 + (*s.add(4) as usize));
        d = d.add(1);
        s = s.add(5);
    }
}

unsafe fn write_squashed_line_2x(dest: *mut u8, src: *mut u8) {
    let mut d = dest;
    let mut d2 = dest.add(dest_pitch as usize);
    let mut s = src;
    for _ in 0..(SCREENWIDTH / 5) {
        let mut c = *s;
        draw_pixel2!(d, d2, c);
        c = *stretch_tables[1].add((*s.add(1) as usize) * 256 + (*s as usize));
        draw_pixel2!(d, d2, c);
        c = *s.add(1);
        draw_pixel2!(d, d2, c);
        c = *stretch_tables[0].add((*s.add(1) as usize) * 256 + (*s.add(2) as usize));
        draw_pixel2!(d, d2, c);
        c = *stretch_tables[0].add((*s.add(3) as usize) * 256 + (*s.add(2) as usize));
        draw_pixel2!(d, d2, c);
        c = *s.add(3);
        draw_pixel2!(d, d2, c);
        c = *stretch_tables[1].add((*s.add(3) as usize) * 256 + (*s.add(4) as usize));
        draw_pixel2!(d, d2, c);
        c = *s.add(4);
        draw_pixel2!(d, d2, c);
        s = s.add(5);
    }
}

unsafe fn write_squashed_line_3x(dest: *mut u8, src: *mut u8) {
    let mut d = dest;
    let mut d2 = dest.add(dest_pitch as usize);
    let mut d3 = dest.add((dest_pitch * 2) as usize);
    let mut s = src;
    for _ in 0..(SCREENWIDTH / 2) {
        let mut c = *s;
        draw_pixel3!(d, d2, d3, c);
        draw_pixel3!(d, d2, d3, c);
        c = *half_stretch_table.add((*s as usize) * 256 + (*s.add(1) as usize));
        draw_pixel3!(d, d2, d3, c);
        c = *s.add(1);
        draw_pixel3!(d, d2, d3, c);
        draw_pixel3!(d, d2, d3, c);
        s = s.add(2);
    }
}

unsafe fn write_squashed_line_4x(dest: *mut u8, src: *mut u8) {
    let mut d = dest;
    let mut d2 = dest.add(dest_pitch as usize);
    let mut d3 = dest.add((dest_pitch * 2) as usize);
    let mut d4 = dest.add((dest_pitch * 3) as usize);
    let mut s = src;
    for _ in 0..(SCREENWIDTH / 5) {
        let mut c = *s;
        draw_pixel4!(d, d2, d3, d4, c);
        draw_pixel4!(d, d2, d3, d4, c);
        draw_pixel4!(d, d2, d3, d4, c);
        c = *stretch_tables[0].add((*s as usize) * 256 + (*s.add(1) as usize));
        draw_pixel4!(d, d2, d3, d4, c);
        c = *s.add(1);
        draw_pixel4!(d, d2, d3, d4, c);
        draw_pixel4!(d, d2, d3, d4, c);
        c = *stretch_tables[1].add((*s.add(1) as usize) * 256 + (*s.add(2) as usize));
        draw_pixel4!(d, d2, d3, d4, c);
        c = *s.add(2);
        draw_pixel4!(d, d2, d3, d4, c);
        draw_pixel4!(d, d2, d3, d4, c);
        c = *stretch_tables[1].add((*s.add(3) as usize) * 256 + (*s.add(2) as usize));
        draw_pixel4!(d, d2, d3, d4, c);
        c = *s.add(3);
        draw_pixel4!(d, d2, d3, d4, c);
        draw_pixel4!(d, d2, d3, d4, c);
        c = *stretch_tables[0].add((*s.add(4) as usize) * 256 + (*s.add(3) as usize));
        draw_pixel4!(d, d2, d3, d4, c);
        c = *s.add(4);
        draw_pixel4!(d, d2, d3, d4, c);
        draw_pixel4!(d, d2, d3, d4, c);
        draw_pixel4!(d, d2, d3, d4, c);
        s = s.add(5);
    }
}

unsafe fn write_squashed_line_5x(dest: *mut u8, src: *mut u8) {
    let mut d = dest;
    let mut d2 = dest.add(dest_pitch as usize);
    let mut d3 = dest.add((dest_pitch * 2) as usize);
    let mut d4 = dest.add((dest_pitch * 3) as usize);
    let mut d5 = dest.add((dest_pitch * 4) as usize);
    let mut s = src;
    for _ in 0..SCREENWIDTH {
        let c = *s;
        draw_pixel5!(d, d2, d3, d4, d5, c);
        draw_pixel5!(d, d2, d3, d4, d5, c);
        draw_pixel5!(d, d2, d3, d4, d5, c);
        draw_pixel5!(d, d2, d3, d4, d5, c);
        s = s.add(1);
    }
}

unsafe extern "C" fn i_squash_1x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in 0..SCREENHEIGHT {
        write_squashed_line_1x(screenp, bufp);
        screenp = screenp.add(dest_pitch as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_squash_2x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in 0..SCREENHEIGHT {
        write_squashed_line_2x(screenp, bufp);
        screenp = screenp.add((dest_pitch * 2) as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_squash_3x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in 0..SCREENHEIGHT {
        write_squashed_line_3x(screenp, bufp);
        screenp = screenp.add((dest_pitch * 3) as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_squash_4x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in 0..SCREENHEIGHT {
        write_squashed_line_4x(screenp, bufp);
        screenp = screenp.add((dest_pitch * 4) as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

unsafe extern "C" fn i_squash_5x(_x1: c_int, _y1: c_int, _x2: c_int, _y2: c_int) -> c_int {
    if _x1 != 0 || _y1 != 0 || _x2 != SCREENWIDTH || _y2 != SCREENHEIGHT {
        return 0;
    }
    let mut bufp = src_buffer;
    let mut screenp = dest_buffer;
    for _ in 0..SCREENHEIGHT {
        write_squashed_line_5x(screenp, bufp);
        screenp = screenp.add((dest_pitch * 5) as usize);
        bufp = bufp.add(SCREENWIDTH as usize);
    }
    1
}

// ---------------------------------------------------------------------------
// Screen mode descriptors
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut mode_scale_1x: screen_mode_t = screen_mode_t {
    width: 320,
    height: 200,
    init_mode: None,
    draw_screen: Some(i_scale_1x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_scale_2x: screen_mode_t = screen_mode_t {
    width: 640,
    height: 400,
    init_mode: None,
    draw_screen: Some(i_scale_2x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_scale_3x: screen_mode_t = screen_mode_t {
    width: 960,
    height: 600,
    init_mode: None,
    draw_screen: Some(i_scale_3x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_scale_4x: screen_mode_t = screen_mode_t {
    width: 1280,
    height: 800,
    init_mode: None,
    draw_screen: Some(i_scale_4x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_scale_5x: screen_mode_t = screen_mode_t {
    width: 1600,
    height: 1000,
    init_mode: None,
    draw_screen: Some(i_scale_5x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_stretch_1x: screen_mode_t = screen_mode_t {
    width: 320,
    height: 240,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_stretch_1x),
    poor_quality: 1,
};

#[no_mangle]
pub static mut mode_stretch_2x: screen_mode_t = screen_mode_t {
    width: 640,
    height: 480,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_stretch_2x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_stretch_3x: screen_mode_t = screen_mode_t {
    width: 960,
    height: 720,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_stretch_3x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_stretch_4x: screen_mode_t = screen_mode_t {
    width: 1280,
    height: 960,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_stretch_4x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_stretch_5x: screen_mode_t = screen_mode_t {
    width: 1600,
    height: 1200,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_stretch_5x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_squash_1x: screen_mode_t = screen_mode_t {
    width: 256,
    height: 200,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_squash_1x),
    poor_quality: 1,
};

#[no_mangle]
pub static mut mode_squash_2x: screen_mode_t = screen_mode_t {
    width: 512,
    height: 400,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_squash_2x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_squash_3x: screen_mode_t = screen_mode_t {
    width: 800,
    height: 600,
    init_mode: Some(i_init_squash_table),
    draw_screen: Some(i_squash_3x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_squash_4x: screen_mode_t = screen_mode_t {
    width: 1024,
    height: 800,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_squash_4x),
    poor_quality: 0,
};

#[no_mangle]
pub static mut mode_squash_5x: screen_mode_t = screen_mode_t {
    width: 1280,
    height: 1000,
    init_mode: Some(i_init_stretch_tables),
    draw_screen: Some(i_squash_5x),
    poor_quality: 0,
};
