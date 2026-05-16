//! Rust port of vendor/doomgeneric/r_draw.c.
//!
//! The actual span/column drawing functions. All drawing to the view buffer
//! is accomplished in this module; the other refresh files only know about
//! coordinates, not the architecture of the frame buffer.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::doom::c_ffi::{FUZZOFF, FUZZTABLE, SBARHEIGHT};
use crate::doom::d_mode::commercial;
use crate::doom::doomstat::gamemode;
use crate::doom::i_video::{SCREENHEIGHT, SCREENWIDTH};
use crate::doom::m_fixed::FRACBITS;
use crate::doom::v_video::{patch_t, V_DrawPatch, V_MarkRect, V_RestoreBuffer, V_UseBuffer};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const MAXWIDTH: usize = 1120;
const MAXHEIGHT: usize = 832;

// ---------------------------------------------------------------------------
// External symbols
// ---------------------------------------------------------------------------

extern "C" {
    fn I_Error(format: *const c_char, ...);
    fn W_CacheLumpName(name: *const c_char, tag: c_int) -> *mut c_void;
    fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void;
    fn Z_Free(ptr: *mut c_void);

    static mut I_VideoBuffer: *mut u8;
    static mut colormaps: *mut u8;
    static mut centery: c_int;
}

// ---------------------------------------------------------------------------
// View-buffer globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut viewimage: *mut u8 = ptr::null_mut();

#[no_mangle]
pub static mut viewwidth: c_int = 0;

#[no_mangle]
pub static mut scaledviewwidth: c_int = 0;

#[no_mangle]
pub static mut viewheight: c_int = 0;

#[no_mangle]
pub static mut viewwindowx: c_int = 0;

#[no_mangle]
pub static mut viewwindowy: c_int = 0;

#[no_mangle]
pub static mut ylookup: [*mut u8; MAXHEIGHT] = [ptr::null_mut(); MAXHEIGHT];

#[no_mangle]
pub static mut columnofs: [c_int; MAXWIDTH] = [0; MAXWIDTH];

#[no_mangle]
pub static mut translations: [[u8; 256]; 3] = [[0; 256]; 3];

// ---------------------------------------------------------------------------
// Background buffer (module-local)
// ---------------------------------------------------------------------------

static mut background_buffer: *mut u8 = ptr::null_mut();

// ---------------------------------------------------------------------------
// Column-drawing globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut dc_colormap: *mut u8 = ptr::null_mut();

#[no_mangle]
pub static mut dc_x: c_int = 0;

#[no_mangle]
pub static mut dc_yl: c_int = 0;

#[no_mangle]
pub static mut dc_yh: c_int = 0;

#[no_mangle]
pub static mut dc_iscale: c_int = 0;

#[no_mangle]
pub static mut dc_texturemid: c_int = 0;

/// First pixel in a column (possibly virtual).
#[no_mangle]
pub static mut dc_source: *mut u8 = ptr::null_mut();

/// Just for profiling.
#[no_mangle]
pub static mut dccount: c_int = 0;

// ---------------------------------------------------------------------------
// Fuzz / spectre effect
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut fuzzoffset: [c_int; FUZZTABLE] = [
    FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
    FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    -FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
    -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
];

#[no_mangle]
pub static mut fuzzpos: c_int = 0;

// ---------------------------------------------------------------------------
// Translation tables
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut dc_translation: *mut u8 = ptr::null_mut();

#[no_mangle]
pub static mut translationtables: *mut u8 = ptr::null_mut();

// ---------------------------------------------------------------------------
// Span-drawing globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut ds_y: c_int = 0;

#[no_mangle]
pub static mut ds_x1: c_int = 0;

#[no_mangle]
pub static mut ds_x2: c_int = 0;

#[no_mangle]
pub static mut ds_colormap: *mut u8 = ptr::null_mut();

#[no_mangle]
pub static mut ds_xfrac: c_int = 0;

#[no_mangle]
pub static mut ds_yfrac: c_int = 0;

#[no_mangle]
pub static mut ds_xstep: c_int = 0;

#[no_mangle]
pub static mut ds_ystep: c_int = 0;

/// Start of a 64×64 tile image.
#[no_mangle]
pub static mut ds_source: *mut u8 = ptr::null_mut();

/// Just for profiling.
#[no_mangle]
pub static mut dscount: c_int = 0;

// ---------------------------------------------------------------------------
// R_DrawColumn
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawColumn() {
    unsafe {
        let count = dc_yh - dc_yl;

        // Zero length, column does not exceed a pixel.
        if count < 0 {
            return;
        }

        debug_assert!(
            (dc_x as u32) < (SCREENWIDTH as u32) && dc_yl >= 0 && dc_yh < SCREENHEIGHT,
            "R_DrawColumn: {} to {} at {}",
            dc_yl,
            dc_yh,
            dc_x
        );

        // Framebuffer destination address.
        let mut dest = ylookup[dc_yl as usize].add(columnofs[dc_x as usize] as usize);

        // Determine scaling, which is the only mapping to be done.
        let fracstep = dc_iscale;
        let mut frac = dc_texturemid + (dc_yl - centery) * fracstep;

        // Inner loop that does the actual texture mapping.
        let mut count = count;
        loop {
            *dest = *dc_colormap.add(*dc_source.add(((frac >> FRACBITS) & 127) as usize) as usize);
            dest = dest.add(SCREENWIDTH as usize);
            frac += fracstep;
            if count == 0 {
                break;
            }
            count -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawColumnLow
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawColumnLow() {
    unsafe {
        let count = dc_yh - dc_yl;

        // Zero length.
        if count < 0 {
            return;
        }

        // Blocky mode, need to multiply by 2.
        let x = dc_x << 1;

        debug_assert!(
            (dc_x as u32) < (SCREENWIDTH as u32) && dc_yl >= 0 && dc_yh < SCREENHEIGHT,
            "R_DrawColumnLow: {} to {} at {}",
            dc_yl,
            dc_yh,
            dc_x
        );

        let mut dest = ylookup[dc_yl as usize].add(columnofs[x as usize] as usize);
        let mut dest2 = ylookup[dc_yl as usize].add(columnofs[(x + 1) as usize] as usize);

        let fracstep = dc_iscale;
        let mut frac = dc_texturemid + (dc_yl - centery) * fracstep;

        let mut count = count;
        loop {
            let pix =
                *dc_colormap.add(*dc_source.add(((frac >> FRACBITS) & 127) as usize) as usize);
            *dest = pix;
            *dest2 = pix;
            dest = dest.add(SCREENWIDTH as usize);
            dest2 = dest2.add(SCREENWIDTH as usize);
            frac += fracstep;
            if count == 0 {
                break;
            }
            count -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawFuzzColumn
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawFuzzColumn() {
    unsafe {
        // Adjust borders. Low...
        if dc_yl == 0 {
            dc_yl = 1;
        }

        // .. and high.
        if dc_yh == viewheight - 1 {
            dc_yh = viewheight - 2;
        }

        let count = dc_yh - dc_yl;

        // Zero length.
        if count < 0 {
            return;
        }

        debug_assert!(
            (dc_x as u32) < (SCREENWIDTH as u32) && dc_yl >= 0 && dc_yh < SCREENHEIGHT,
            "R_DrawFuzzColumn: {} to {} at {}",
            dc_yl,
            dc_yh,
            dc_x
        );

        let mut dest = ylookup[dc_yl as usize].add(columnofs[dc_x as usize] as usize);

        let fracstep = dc_iscale;
        let mut frac = dc_texturemid + (dc_yl - centery) * fracstep;

        let mut count = count;
        loop {
            // Lookup framebuffer, and retrieve a pixel that is either one
            // column left or right of the current one.  Add index from
            // colormap to index.
            let offset = fuzzoffset[fuzzpos as usize];
            let src_pix = *dest.offset(offset as isize);
            *dest = *colormaps.add(6 * 256 + src_pix as usize);

            // Clamp table lookup index.
            fuzzpos += 1;
            if fuzzpos == FUZZTABLE as c_int {
                fuzzpos = 0;
            }

            dest = dest.add(SCREENWIDTH as usize);
            frac += fracstep;
            if count == 0 {
                break;
            }
            count -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawFuzzColumnLow
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawFuzzColumnLow() {
    unsafe {
        // Adjust borders. Low...
        if dc_yl == 0 {
            dc_yl = 1;
        }

        // .. and high.
        if dc_yh == viewheight - 1 {
            dc_yh = viewheight - 2;
        }

        let count = dc_yh - dc_yl;

        // Zero length.
        if count < 0 {
            return;
        }

        // low detail mode, need to multiply by 2
        let x = dc_x << 1;

        debug_assert!(
            (x as u32) < (SCREENWIDTH as u32) && dc_yl >= 0 && dc_yh < SCREENHEIGHT,
            "R_DrawFuzzColumnLow: {} to {} at {}",
            dc_yl,
            dc_yh,
            dc_x
        );

        let mut dest = ylookup[dc_yl as usize].add(columnofs[x as usize] as usize);
        let mut dest2 = ylookup[dc_yl as usize].add(columnofs[(x + 1) as usize] as usize);

        let fracstep = dc_iscale;
        let mut frac = dc_texturemid + (dc_yl - centery) * fracstep;

        let mut count = count;
        loop {
            let offset = fuzzoffset[fuzzpos as usize];
            let src_pix = *dest.offset(offset as isize);
            let pix = *colormaps.add(6 * 256 + src_pix as usize);
            *dest = pix;
            *dest2 = pix;

            // Clamp table lookup index.
            fuzzpos += 1;
            if fuzzpos == FUZZTABLE as c_int {
                fuzzpos = 0;
            }

            dest = dest.offset(SCREENWIDTH as isize);
            dest2 = dest2.offset(SCREENWIDTH as isize);
            frac += fracstep;
            if count == 0 {
                break;
            }
            count -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawTranslatedColumn
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawTranslatedColumn() {
    unsafe {
        let count = dc_yh - dc_yl;
        if count < 0 {
            return;
        }

        debug_assert!(
            (dc_x as u32) < (SCREENWIDTH as u32) && dc_yl >= 0 && dc_yh < SCREENHEIGHT,
            "R_DrawTranslatedColumn: {} to {} at {}",
            dc_yl,
            dc_yh,
            dc_x
        );

        let mut dest = ylookup[dc_yl as usize].add(columnofs[dc_x as usize] as usize);

        let fracstep = dc_iscale;
        let mut frac = dc_texturemid + (dc_yl - centery) * fracstep;

        let mut count = count;
        loop {
            let src_idx = *dc_source.add((frac >> FRACBITS) as usize) as usize;
            let trans_idx = *dc_translation.add(src_idx) as usize;
            *dest = *dc_colormap.add(trans_idx);
            dest = dest.add(SCREENWIDTH as usize);
            frac += fracstep;
            if count == 0 {
                break;
            }
            count -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawTranslatedColumnLow
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawTranslatedColumnLow() {
    unsafe {
        let count = dc_yh - dc_yl;
        if count < 0 {
            return;
        }

        // low detail, need to scale by 2
        let x = dc_x << 1;

        debug_assert!(
            (x as u32) < (SCREENWIDTH as u32) && dc_yl >= 0 && dc_yh < SCREENHEIGHT,
            "R_DrawTranslatedColumnLow: {} to {} at {}",
            dc_yl,
            dc_yh,
            x
        );

        let mut dest = ylookup[dc_yl as usize].add(columnofs[x as usize] as usize);
        let mut dest2 = ylookup[dc_yl as usize].add(columnofs[(x + 1) as usize] as usize);

        let fracstep = dc_iscale;
        let mut frac = dc_texturemid + (dc_yl - centery) * fracstep;

        let mut count = count;
        loop {
            let src_idx = *dc_source.add((frac >> FRACBITS) as usize) as usize;
            let trans_idx = *dc_translation.add(src_idx) as usize;
            let pix = *dc_colormap.add(trans_idx);
            *dest = pix;
            *dest2 = pix;
            dest = dest.add(SCREENWIDTH as usize);
            dest2 = dest2.add(SCREENWIDTH as usize);
            frac += fracstep;
            if count == 0 {
                break;
            }
            count -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_InitTranslationTables
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_InitTranslationTables() {
    unsafe {
        translationtables = Z_Malloc(256 * 3, 1, ptr::null_mut()) as *mut u8; // PU_STATIC = 1

        let tt = translationtables;
        for i in 0..256 {
            if i >= 0x70 && i <= 0x7f {
                // map green ramp to gray, brown, red
                *tt.add(i) = (0x60 + (i & 0xf)) as u8;
                *tt.add(i + 256) = (0x40 + (i & 0xf)) as u8;
                *tt.add(i + 512) = (0x20 + (i & 0xf)) as u8;
            } else {
                // Keep all other colors as is.
                *tt.add(i) = i as u8;
                *tt.add(i + 256) = i as u8;
                *tt.add(i + 512) = i as u8;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawSpan
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawSpan() {
    unsafe {
        let mut position: u32;
        let step: u32;

        // Pack position and step variables into a single 32-bit integer,
        // with x in the top 16 bits and y in the bottom 16 bits.  For
        // each 16-bit part, the top 6 bits are the integer part and the
        // bottom 10 bits are the fractional part of the pixel position.
        position = ((ds_xfrac << 10) as u32 & 0xffff0000) | ((ds_yfrac >> 6) as u32 & 0x0000ffff);
        step = ((ds_xstep << 10) as u32 & 0xffff0000) | ((ds_ystep >> 6) as u32 & 0x0000ffff);

        let mut dest = ylookup[ds_y as usize].add(columnofs[ds_x1 as usize] as usize);

        // We do not check for zero spans here?
        let mut count = ds_x2 - ds_x1;

        debug_assert!(
            ds_x2 >= ds_x1
                && ds_x1 >= 0
                && ds_x2 < SCREENWIDTH
                && (ds_y as u32) <= (SCREENHEIGHT as u32),
            "R_DrawSpan: {} to {} at {}",
            ds_x1,
            ds_x2,
            ds_y
        );

        loop {
            // Calculate current texture index in u,v.
            let ytemp = ((position >> 4) & 0x0fc0) as usize;
            let xtemp = (position >> 26) as usize;
            let spot = xtemp | ytemp;

            // Lookup pixel from flat texture tile, re-index using light/colormap.
            *dest = *ds_colormap.add(*ds_source.add(spot) as usize);
            dest = dest.add(1);

            position = position.wrapping_add(step);
            if count == 0 {
                break;
            }
            count -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawSpanLow
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawSpanLow() {
    unsafe {
        let mut position: u32;
        let step: u32;

        position = ((ds_xfrac << 10) as u32 & 0xffff0000) | ((ds_yfrac >> 6) as u32 & 0x0000ffff);
        step = ((ds_xstep << 10) as u32 & 0xffff0000) | ((ds_ystep >> 6) as u32 & 0x0000ffff);

        let mut count = ds_x2 - ds_x1;

        // Blocky mode, need to multiply by 2.
        let ds_x1_low = ds_x1 << 1;

        debug_assert!(
            ds_x2 >= ds_x1
                && ds_x1 >= 0
                && ds_x2 < SCREENWIDTH
                && (ds_y as u32) <= (SCREENHEIGHT as u32),
            "R_DrawSpanLow: {} to {} at {}",
            ds_x1,
            ds_x2,
            ds_y
        );

        let mut dest = ylookup[ds_y as usize].add(columnofs[ds_x1_low as usize] as usize);

        loop {
            let ytemp = ((position >> 4) & 0x0fc0) as usize;
            let xtemp = (position >> 26) as usize;
            let spot = xtemp | ytemp;

            // Lowres/blocky mode does it twice,
            // while scale is adjusted appropriately.
            let pix = *ds_colormap.add(*ds_source.add(spot) as usize);
            *dest = pix;
            dest = dest.add(1);
            *dest = pix;
            dest = dest.add(1);

            position = position.wrapping_add(step);
            if count == 0 {
                break;
            }
            count -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// R_InitBuffer
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_InitBuffer(width: c_int, height: c_int) {
    unsafe {
        // Handle resize, e.g. smaller view windows with border and/or status bar.
        viewwindowx = (SCREENWIDTH - width) >> 1;

        // Column offset. For windows.
        for i in 0..width {
            columnofs[i as usize] = viewwindowx + i;
        }

        // Same with base row offset.
        if width == SCREENWIDTH {
            viewwindowy = 0;
        } else {
            viewwindowy = (SCREENHEIGHT - SBARHEIGHT - height) >> 1;
        }

        // Precalculate all row offsets.
        for i in 0..height {
            ylookup[i as usize] = I_VideoBuffer.add(((i + viewwindowy) * SCREENWIDTH) as usize);
        }
    }
}

// ---------------------------------------------------------------------------
// DEH_String shim — identity in this build
// ---------------------------------------------------------------------------

unsafe fn DEH_String(s: *const c_char) -> *const c_char {
    s
}

// ---------------------------------------------------------------------------
// R_FillBackScreen
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_FillBackScreen() {
    unsafe {
        // If we are running full screen, there is no need to do any of this,
        // and the background buffer can be freed if it was previously in use.
        if scaledviewwidth == SCREENWIDTH {
            if !background_buffer.is_null() {
                Z_Free(background_buffer as *mut c_void);
                background_buffer = ptr::null_mut();
            }
            return;
        }

        // Allocate the background buffer if necessary
        if background_buffer.is_null() {
            background_buffer = Z_Malloc(
                SCREENWIDTH * (SCREENHEIGHT - SBARHEIGHT),
                1, // PU_STATIC
                ptr::null_mut(),
            ) as *mut u8;
        }

        let name = if gamemode == commercial {
            DEH_String(b"GRNROCK\0".as_ptr() as *const c_char)
        } else {
            DEH_String(b"FLOOR7_2\0".as_ptr() as *const c_char)
        };

        let src = W_CacheLumpName(name, 8) as *mut u8; // PU_CACHE = 8
        let mut dest = background_buffer;

        for y in 0..(SCREENHEIGHT - SBARHEIGHT) {
            let row_src = src.add(((y & 63) << 6) as usize);
            for _ in 0..(SCREENWIDTH / 64) {
                ptr::copy_nonoverlapping(row_src, dest, 64);
                dest = dest.add(64);
            }
            let remainder = SCREENWIDTH & 63;
            if remainder != 0 {
                ptr::copy_nonoverlapping(row_src, dest, remainder as usize);
                dest = dest.add(remainder as usize);
            }
        }

        // Draw screen and bezel; this is done to a separate screen buffer.
        V_UseBuffer(background_buffer);

        let mut patch =
            W_CacheLumpName(DEH_String(b"brdr_t\0".as_ptr() as *const c_char), 8) as *mut patch_t;
        for x in (0..scaledviewwidth).step_by(8) {
            V_DrawPatch(viewwindowx + x, viewwindowy - 8, patch);
        }

        patch = W_CacheLumpName(DEH_String(b"brdr_b\0".as_ptr() as *const c_char), 8) as *mut patch_t;
        for x in (0..scaledviewwidth).step_by(8) {
            V_DrawPatch(viewwindowx + x, viewwindowy + viewheight, patch);
        }

        patch = W_CacheLumpName(DEH_String(b"brdr_l\0".as_ptr() as *const c_char), 8) as *mut patch_t;
        for y in (0..viewheight).step_by(8) {
            V_DrawPatch(viewwindowx - 8, viewwindowy + y, patch);
        }

        patch = W_CacheLumpName(DEH_String(b"brdr_r\0".as_ptr() as *const c_char), 8) as *mut patch_t;
        for y in (0..viewheight).step_by(8) {
            V_DrawPatch(viewwindowx + scaledviewwidth, viewwindowy + y, patch);
        }

        // Draw beveled edge.
        V_DrawPatch(
            viewwindowx - 8,
            viewwindowy - 8,
            W_CacheLumpName(DEH_String(b"brdr_tl\0".as_ptr() as *const c_char), 8) as *mut patch_t,
        );
        V_DrawPatch(
            viewwindowx + scaledviewwidth,
            viewwindowy - 8,
            W_CacheLumpName(DEH_String(b"brdr_tr\0".as_ptr() as *const c_char), 8) as *mut patch_t,
        );
        V_DrawPatch(
            viewwindowx - 8,
            viewwindowy + viewheight,
            W_CacheLumpName(DEH_String(b"brdr_bl\0".as_ptr() as *const c_char), 8) as *mut patch_t,
        );
        V_DrawPatch(
            viewwindowx + scaledviewwidth,
            viewwindowy + viewheight,
            W_CacheLumpName(DEH_String(b"brdr_br\0".as_ptr() as *const c_char), 8) as *mut patch_t,
        );

        V_RestoreBuffer();
    }
}

// ---------------------------------------------------------------------------
// R_VideoErase
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_VideoErase(ofs: u32, count: c_int) {
    unsafe {
        if !background_buffer.is_null() {
            ptr::copy_nonoverlapping(
                background_buffer.add(ofs as usize),
                I_VideoBuffer.add(ofs as usize),
                count as usize,
            );
        }
    }
}

// ---------------------------------------------------------------------------
// R_DrawViewBorder
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn R_DrawViewBorder() {
    unsafe {
        if scaledviewwidth == SCREENWIDTH {
            return;
        }

        let top = ((SCREENHEIGHT - SBARHEIGHT) - viewheight) / 2;
        let side = (SCREENWIDTH - scaledviewwidth) / 2;

        // copy top and one line of left side
        R_VideoErase(0, (top * SCREENWIDTH + side) as c_int);

        // copy one line of right side and bottom
        let ofs = ((viewheight + top) * SCREENWIDTH - side) as u32;
        R_VideoErase(ofs, (top * SCREENWIDTH + side) as c_int);

        // copy sides using wraparound
        let mut ofs = ((top * SCREENWIDTH) + SCREENWIDTH - side) as u32;
        let side_doubled = side << 1;

        for _ in 1..viewheight {
            R_VideoErase(ofs, side_doubled);
            ofs += SCREENWIDTH as u32;
        }

        V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT - SBARHEIGHT);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    // -----------------------------------------------------------------------
    // R_InitBuffer
    // -----------------------------------------------------------------------

    #[test]
    fn init_buffer_fullscreen() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            // Allocate a fake video buffer so ylookup doesn't deref null.
            let mut fake_buf = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let orig_buf = I_VideoBuffer;
            I_VideoBuffer = fake_buf.as_mut_ptr();

            R_InitBuffer(SCREENWIDTH, 168);

            assert_eq!(viewwindowx, 0);
            assert_eq!(viewwindowy, 0);
            for i in 0..SCREENWIDTH {
                assert_eq!(columnofs[i as usize], i, "columnofs[{i}] mismatch");
            }
            for i in 0..168 {
                assert_eq!(
                    ylookup[i as usize],
                    I_VideoBuffer.add((i * SCREENWIDTH) as usize),
                    "ylookup[{i}] mismatch"
                );
            }

            I_VideoBuffer = orig_buf;
        }
    }

    #[test]
    fn init_buffer_windowed_256x168() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut fake_buf = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let orig_buf = I_VideoBuffer;
            I_VideoBuffer = fake_buf.as_mut_ptr();

            R_InitBuffer(256, 168);

            assert_eq!(viewwindowx, (SCREENWIDTH - 256) >> 1); // 32
            assert_eq!(viewwindowy, (SCREENHEIGHT - SBARHEIGHT - 168) >> 1); // 0
            for i in 0..256 {
                assert_eq!(columnofs[i as usize], viewwindowx + i);
            }

            I_VideoBuffer = orig_buf;
        }
    }

    #[test]
    fn init_buffer_windowed_200x100() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut fake_buf = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let orig_buf = I_VideoBuffer;
            I_VideoBuffer = fake_buf.as_mut_ptr();

            R_InitBuffer(200, 100);

            assert_eq!(viewwindowx, (SCREENWIDTH - 200) >> 1); // 60
            assert_eq!(viewwindowy, (SCREENHEIGHT - SBARHEIGHT - 100) >> 1); // 34
            for i in 0..200 {
                assert_eq!(columnofs[i as usize], viewwindowx + i);
            }
            for i in 0..100 {
                let expected = I_VideoBuffer.add(((i + viewwindowy) * SCREENWIDTH) as usize);
                assert_eq!(ylookup[i as usize], expected, "ylookup[{i}] mismatch");
            }

            I_VideoBuffer = orig_buf;
        }
    }

    // -----------------------------------------------------------------------
    // R_InitTranslationTables
    // -----------------------------------------------------------------------

    #[test]
    fn init_translation_tables() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            // Initialize the zone allocator so Z_Malloc works.
            // It's fine to re-initialize in a unit test.
            crate::doom::z_zone::Z_Init();

            R_InitTranslationTables();

            let tt = translationtables;
            for i in 0..256usize {
                if i >= 0x70 && i <= 0x7f {
                    assert_eq!(
                        *tt.add(i),
                        (0x60 + (i & 0xf)) as u8,
                        "tt[{i}] gray mismatch"
                    );
                    assert_eq!(
                        *tt.add(i + 256),
                        (0x40 + (i & 0xf)) as u8,
                        "tt[{i}+256] brown mismatch"
                    );
                    assert_eq!(
                        *tt.add(i + 512),
                        (0x20 + (i & 0xf)) as u8,
                        "tt[{i}+512] red mismatch"
                    );
                } else {
                    assert_eq!(*tt.add(i), i as u8, "tt[{i}] identity mismatch");
                    assert_eq!(*tt.add(i + 256), i as u8, "tt[{i}+256] identity mismatch");
                    assert_eq!(*tt.add(i + 512), i as u8, "tt[{i}+512] identity mismatch");
                }
            }

            // Clean up
            crate::doom::z_zone::Z_Free(translationtables as *mut c_void);
            translationtables = ptr::null_mut();
        }
    }

    // -----------------------------------------------------------------------
    // R_DrawColumn
    // -----------------------------------------------------------------------

    #[test]
    fn draw_column_basic() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut framebuffer = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let mut source = vec![0u8; 128];
            for i in 0..128 {
                source[i] = i as u8;
            }
            let mut colormap = vec![0u8; 256];
            for i in 0..256 {
                colormap[i] = (i ^ 0x55) as u8;
            }

            // Set up ylookup and columnofs for a full-screen buffer
            for i in 0..SCREENHEIGHT {
                ylookup[i as usize] = framebuffer.as_mut_ptr().add((i * SCREENWIDTH) as usize);
            }
            for i in 0..SCREENWIDTH {
                columnofs[i as usize] = i;
            }

            dc_colormap = colormap.as_mut_ptr();
            dc_source = source.as_mut_ptr();
            dc_x = 10;
            dc_yl = 20;
            dc_yh = 30;
            dc_iscale = 0x10000; // 1.0 in fixed point
            dc_texturemid = 0;
            centery = 0;

            R_DrawColumn();

            // Verify each pixel
            for row in 20..=30 {
                let src_idx = row & 127;
                let expected = colormap[source[src_idx as usize] as usize];
                let actual = framebuffer[(row * SCREENWIDTH + 10) as usize];
                assert_eq!(actual, expected, "pixel mismatch at row {row}");
            }
        }
    }

    #[test]
    fn draw_column_negative_count_returns_early() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut framebuffer = vec![0xABu8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            for i in 0..SCREENHEIGHT {
                ylookup[i as usize] = framebuffer.as_mut_ptr().add((i * SCREENWIDTH) as usize);
            }
            for i in 0..SCREENWIDTH {
                columnofs[i as usize] = i;
            }

            dc_x = 5;
            dc_yl = 10;
            dc_yh = 5; // count = -5
            dc_colormap = ptr::null_mut();
            dc_source = ptr::null_mut();

            R_DrawColumn();

            // Framebuffer should be untouched
            for i in 0..framebuffer.len() {
                assert_eq!(framebuffer[i], 0xAB, "framebuffer[{i}] was modified");
            }
        }
    }

    // -----------------------------------------------------------------------
    // R_DrawColumnLow
    // -----------------------------------------------------------------------

    #[test]
    fn draw_column_low_doubles() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut framebuffer = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let mut source = vec![0u8; 128];
            source[0] = 42;
            let mut colormap = vec![0u8; 256];
            colormap[42] = 99;

            for i in 0..SCREENHEIGHT {
                ylookup[i as usize] = framebuffer.as_mut_ptr().add((i * SCREENWIDTH) as usize);
            }
            for i in 0..SCREENWIDTH {
                columnofs[i as usize] = i;
            }

            dc_colormap = colormap.as_mut_ptr();
            dc_source = source.as_mut_ptr();
            dc_x = 5;
            dc_yl = 10;
            dc_yh = 10;
            dc_iscale = 0;
            dc_texturemid = 0;
            centery = 0;

            R_DrawColumnLow();

            let x = dc_x << 1;
            assert_eq!(framebuffer[(10 * SCREENWIDTH + x) as usize], 99);
            assert_eq!(framebuffer[(10 * SCREENWIDTH + x + 1) as usize], 99);
        }
    }

    // -----------------------------------------------------------------------
    // R_DrawSpan
    // -----------------------------------------------------------------------

    #[test]
    fn draw_span_basic() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut framebuffer = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let mut source = vec![0u8; 64 * 64];
            for i in 0..(64 * 64) {
                source[i] = (i % 256) as u8;
            }
            let mut colormap = vec![0u8; 256];
            for i in 0..256 {
                colormap[i] = (i ^ 0xAA) as u8;
            }

            for i in 0..SCREENHEIGHT {
                ylookup[i as usize] = framebuffer.as_mut_ptr().add((i * SCREENWIDTH) as usize);
            }
            for i in 0..SCREENWIDTH {
                columnofs[i as usize] = i;
            }

            ds_colormap = colormap.as_mut_ptr();
            ds_source = source.as_mut_ptr();
            ds_y = 50;
            ds_x1 = 10;
            ds_x2 = 14;
            ds_xfrac = 0;
            ds_yfrac = 0;
            ds_xstep = 0x10000; // 1.0 in fixed point
            ds_ystep = 0;

            R_DrawSpan();

            // With xfrac=0, yfrac=0, xstep=1.0 (0x10000), ystep=0:
            // position = 0
            // step = (0x10000 << 10) & 0xffff0000 = 0x0400_0000
            // After each pixel, position increases by 0x0400_0000,
            // so xtemp (position >> 26) increments by 1 each time.
            // ds_x2 - ds_x1 = 4, so we draw 5 pixels (10..=14).
            let base = (50 * SCREENWIDTH + 10) as usize;
            assert_eq!(framebuffer[base], colormap[source[0] as usize]);
            assert_eq!(framebuffer[base + 1], colormap[source[1] as usize]);
            assert_eq!(framebuffer[base + 2], colormap[source[2] as usize]);
            assert_eq!(framebuffer[base + 3], colormap[source[3] as usize]);
            assert_eq!(framebuffer[base + 4], colormap[source[4] as usize]);
        }
    }

    // -----------------------------------------------------------------------
    // R_DrawSpanLow
    // -----------------------------------------------------------------------

    #[test]
    fn draw_span_low_doubles() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut framebuffer = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let mut source = vec![0u8; 64 * 64];
            source[0] = 77;
            let mut colormap = vec![0u8; 256];
            colormap[77] = 88;

            for i in 0..SCREENHEIGHT {
                ylookup[i as usize] = framebuffer.as_mut_ptr().add((i * SCREENWIDTH) as usize);
            }
            for i in 0..SCREENWIDTH {
                columnofs[i as usize] = i;
            }

            ds_colormap = colormap.as_mut_ptr();
            ds_source = source.as_mut_ptr();
            ds_y = 60;
            ds_x1 = 5;
            ds_x2 = 5;
            ds_xfrac = 0;
            ds_yfrac = 0;
            ds_xstep = 0;
            ds_ystep = 0;

            R_DrawSpanLow();

            // ds_x2 - ds_x1 = 0, so 1 iteration. Each iteration writes 2 pixels.
            // ds_x1_low = 10
            let base = (60 * SCREENWIDTH + 10) as usize;
            assert_eq!(framebuffer[base], 88);
            assert_eq!(framebuffer[base + 1], 88);
        }
    }

    // -----------------------------------------------------------------------
    // R_VideoErase
    // -----------------------------------------------------------------------

    #[test]
    fn video_erase_copies_from_background() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut video = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let mut bg = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            for i in 0..bg.len() {
                bg[i] = (i % 256) as u8;
            }

            let orig_video = I_VideoBuffer;
            I_VideoBuffer = video.as_mut_ptr();
            background_buffer = bg.as_mut_ptr();

            R_VideoErase(100, 10);

            for i in 100..110 {
                assert_eq!(video[i as usize], bg[i as usize]);
            }
            // Ensure surrounding bytes are untouched
            assert_eq!(video[99], 0);
            assert_eq!(video[110], 0);

            I_VideoBuffer = orig_video;
            background_buffer = ptr::null_mut();
        }
    }

    #[test]
    fn video_erase_null_background_does_nothing() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut video = vec![0xCCu8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let orig_video = I_VideoBuffer;
            I_VideoBuffer = video.as_mut_ptr();
            background_buffer = ptr::null_mut();

            R_VideoErase(0, 10);

            for i in 0..10 {
                assert_eq!(video[i], 0xCC);
            }

            I_VideoBuffer = orig_video;
        }
    }

    // -----------------------------------------------------------------------
    // R_DrawViewBorder
    // -----------------------------------------------------------------------

    #[test]
    fn draw_view_border_fullscreen_returns_early() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            scaledviewwidth = SCREENWIDTH;
            // Should return without doing anything (no panic)
            R_DrawViewBorder();
        }
    }

    #[test]
    fn draw_view_border_sets_correct_offsets() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            scaledviewwidth = 256;
            viewheight = 168;

            let top = ((SCREENHEIGHT - SBARHEIGHT) - viewheight) / 2;
            let side = (SCREENWIDTH - scaledviewwidth) / 2;

            assert_eq!(top, 0);
            assert_eq!(side, 32);
        }
    }

    // -----------------------------------------------------------------------
    // R_DrawTranslatedColumn
    // -----------------------------------------------------------------------

    #[test]
    fn draw_translated_column_maps_colors() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut framebuffer = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let mut source = vec![0u8; 256];
            source[0] = 5; // source pixel at frac=0
            let mut translation = vec![0u8; 256];
            translation[5] = 7; // map color 5 → 7
            let mut colormap = vec![0u8; 256];
            colormap[7] = 99;

            for i in 0..SCREENHEIGHT {
                ylookup[i as usize] = framebuffer.as_mut_ptr().add((i * SCREENWIDTH) as usize);
            }
            for i in 0..SCREENWIDTH {
                columnofs[i as usize] = i;
            }

            dc_colormap = colormap.as_mut_ptr();
            dc_source = source.as_mut_ptr();
            dc_translation = translation.as_mut_ptr();
            dc_x = 15;
            dc_yl = 25;
            dc_yh = 25;
            dc_iscale = 0;
            dc_texturemid = 0;
            centery = 0;

            R_DrawTranslatedColumn();

            assert_eq!(framebuffer[(25 * SCREENWIDTH + 15) as usize], 99);
        }
    }

    // -----------------------------------------------------------------------
    // R_DrawFuzzColumn
    // -----------------------------------------------------------------------

    #[test]
    fn draw_fuzz_column_reads_adjacent() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut framebuffer = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            // Pre-fill the framebuffer so the fuzz effect has something to read.
            // fuzzoffset[0] = FUZZOFF = SCREENWIDTH, so it reads one row below.
            framebuffer[((25 + 1) * SCREENWIDTH + 10) as usize] = 3;

            // Set up colormaps: colormaps[6*256 + 3] = 77
            let mut colormaps_buf = vec![0u8; 32 * 256];
            colormaps_buf[6 * 256 + 3] = 77;

            for i in 0..SCREENHEIGHT {
                ylookup[i as usize] = framebuffer.as_mut_ptr().add((i * SCREENWIDTH) as usize);
            }
            for i in 0..SCREENWIDTH {
                columnofs[i as usize] = i;
            }

            colormaps = colormaps_buf.as_mut_ptr();
            dc_x = 10;
            dc_yl = 25;
            dc_yh = 25;
            dc_iscale = 0;
            dc_texturemid = 0;
            centery = 0;
            fuzzpos = 0;
            viewheight = SCREENHEIGHT;

            R_DrawFuzzColumn();

            // fuzzpos should have advanced
            assert_eq!(fuzzpos, 1);
            // The pixel at (10, 25) should now be colormaps[6*256 + framebuffer[(10, 26)]]
            assert_eq!(framebuffer[(25 * SCREENWIDTH + 10) as usize], 77);
        }
    }

    #[test]
    fn draw_fuzz_column_clamps_borders() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            let mut framebuffer = vec![0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let mut colormaps_buf = vec![0u8; 32 * 256];

            for i in 0..SCREENHEIGHT {
                ylookup[i as usize] = framebuffer.as_mut_ptr().add((i * SCREENWIDTH) as usize);
            }
            for i in 0..SCREENWIDTH {
                columnofs[i as usize] = i;
            }

            colormaps = colormaps_buf.as_mut_ptr();
            dc_x = 0;
            dc_yl = 0; // will be clamped to 1
            dc_yh = SCREENHEIGHT - 1; // will be clamped to SCREENHEIGHT - 2
            dc_iscale = 0;
            dc_texturemid = 0;
            centery = 0;
            fuzzpos = 0;
            viewheight = SCREENHEIGHT;

            R_DrawFuzzColumn();

            // After clamping: dc_yl = 1, dc_yh = SCREENHEIGHT - 2
            assert_eq!(dc_yl, 1);
            assert_eq!(dc_yh, SCREENHEIGHT - 2);
        }
    }

    // -----------------------------------------------------------------------
    // Fuzz offset table
    // -----------------------------------------------------------------------

    #[test]
    fn fuzzoffset_exact_values() {
        let expected: [c_int; FUZZTABLE] = [
            FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF,
            -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
            -FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF,
            FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF,
            FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF,
            -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
        ];
        unsafe {
            for (i, (&got, &want)) in fuzzoffset.iter().zip(expected.iter()).enumerate() {
                assert_eq!(
                    got, want,
                    "fuzzoffset[{i}] mismatch: got {got}, want {want}"
                );
            }
        }
    }

    #[test]
    fn fuzzoffset_positive_count() {
        unsafe {
            let pos = fuzzoffset.iter().filter(|&&v| v > 0).count();
            let neg = fuzzoffset.iter().filter(|&&v| v < 0).count();
            assert_eq!(
                pos, 29,
                "expected 29 positive fuzzoffset entries, got {pos}"
            );
            assert_eq!(
                neg, 21,
                "expected 21 negative fuzzoffset entries, got {neg}"
            );
        }
    }
}
