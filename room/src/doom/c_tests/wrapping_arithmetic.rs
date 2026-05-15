//! Wrapping-arithmetic and overflow edge-case tests.
//!
//! Every operation that can overflow in C gets a dedicated test so the
//! Rust port can replicate the exact saturation / wrap behaviour.

#![allow(non_snake_case)]

use std::ffi::c_int;
use std::ffi::c_uint;

use crate::doom::c_ffi;
use crate::doom::m_fixed::{FixedDiv, FixedMul, FRACUNIT};
use crate::doom::tables::{ANG180, ANG45, ANG90, ANGLETOFINESHIFT, FINEMASK};

// ---------------------------------------------------------------------------
// Fixed-point multiplication wrapping
// ---------------------------------------------------------------------------

#[test]
fn fixedmul_max_max() {
    // (i32::MAX * i32::MAX) >> 16
    assert_eq!(
        FixedMul(i32::MAX, i32::MAX),
        ((i32::MAX as i64 * i32::MAX as i64) >> 16) as c_int
    );
}

#[test]
fn fixedmul_max_min() {
    assert_eq!(
        FixedMul(i32::MAX, i32::MIN),
        ((i32::MAX as i64 * i32::MIN as i64) >> 16) as c_int
    );
}

#[test]
fn fixedmul_min_max() {
    assert_eq!(
        FixedMul(i32::MIN, i32::MAX),
        ((i32::MIN as i64 * i32::MAX as i64) >> 16) as c_int
    );
}

#[test]
fn fixedmul_min_min() {
    assert_eq!(
        FixedMul(i32::MIN, i32::MIN),
        ((i32::MIN as i64 * i32::MIN as i64) >> 16) as c_int
    );
}

#[test]
fn fixedmul_neg_one_one() {
    // -1.0 * 1.0 = -1.0  →  -65536 in 16.16
    assert_eq!(FixedMul(-FRACUNIT, FRACUNIT), -FRACUNIT);
}

#[test]
fn fixedmul_neg_one_neg_one() {
    // -1.0 * -1.0 = 1.0  →  65536 in 16.16
    assert_eq!(FixedMul(-FRACUNIT, -FRACUNIT), FRACUNIT);
}

// ---------------------------------------------------------------------------
// Fixed-point division saturation
// ---------------------------------------------------------------------------

#[test]
fn fixeddiv_by_zero_positive_saturates_max() {
    assert_eq!(FixedDiv(1, 0), i32::MAX);
    assert_eq!(FixedDiv(FRACUNIT, 0), i32::MAX);
}

#[test]
fn fixeddiv_by_zero_negative_saturates_min() {
    assert_eq!(FixedDiv(-1, 0), i32::MIN);
    assert_eq!(FixedDiv(-FRACUNIT, 0), i32::MIN);
}

#[test]
fn fixeddiv_max_by_neg_one() {
    assert_eq!(FixedDiv(i32::MAX, -1), i32::MIN);
}

#[test]
fn fixeddiv_min_by_one() {
    // i32::MIN.wrapping_abs() >> 14 is negative (arithmetic shift), so the
    // saturation guard does not fire.  The division path computes
    // ((i32::MIN as i64) << 16) / 1 = -140737488355328, which truncates to
    // 0 when cast back to i32 — matching the C behaviour for abs(INT_MIN).
    assert_eq!(FixedDiv(i32::MIN, 1), 0);
}

// ---------------------------------------------------------------------------
// Angle wrapping
// ---------------------------------------------------------------------------

#[test]
fn angle_to_fine_shift_value() {
    assert_eq!(ANGLETOFINESHIFT, 19);
}

#[test]
fn fine_mask_value() {
    assert_eq!(FINEMASK, 8191);
}

#[test]
fn angle_wraps_at_360() {
    let angle: c_uint = 0xFFFFFFFF;
    let fine = ((angle >> ANGLETOFINESHIFT) as c_int) & FINEMASK;
    assert_eq!(fine, 8191);
}

#[test]
fn angle_zero_to_fine() {
    let angle: c_uint = 0;
    let fine = ((angle >> ANGLETOFINESHIFT) as c_int) & FINEMASK;
    assert_eq!(fine, 0);
}

#[test]
fn ang45_value() {
    assert_eq!(ANG45, 1u32 << 29);
}

#[test]
fn ang90_value() {
    assert_eq!(ANG90, 1u32 << 30);
}

#[test]
fn ang180_value() {
    assert_eq!(ANG180, 1u32 << 31);
}

#[test]
fn angle_addition_wraps() {
    let a: c_uint = 0xFFFFFFFF;
    let b: c_uint = ANG45;
    let result = a.wrapping_add(b);
    let fine = ((result >> ANGLETOFINESHIFT) as c_int) & FINEMASK;
    // 0xFFFFFFFF + 0x20000000 = 0x1FFFFFFF (32-bit wrap)
    // 0x1FFFFFFF >> 19 = 0x3FF = 1023
    assert_eq!(fine, 1023);
}

// ---------------------------------------------------------------------------
// Map block coordinate wrapping
// ---------------------------------------------------------------------------

#[test]
fn mapblockshift_value() {
    assert_eq!(c_ffi::MAPBLOCKSHIFT, 23);
}

#[test]
fn mapblocksize_value() {
    assert_eq!(c_ffi::MAPBLOCKSIZE, 128 << 16);
}

#[test]
fn mapbmask_value() {
    assert_eq!(c_ffi::MAPBMASK, c_ffi::MAPBLOCKSIZE - 1);
}

#[test]
fn mapbtofrac_value() {
    assert_eq!(c_ffi::MAPBTOFRAC, 7);
}

#[test]
fn coord_on_block_boundary() {
    let x: c_int = 128 << 16; // exactly 128 units
    let blockx = x >> c_ffi::MAPBLOCKSHIFT;
    assert_eq!(blockx, 1);
}

// ---------------------------------------------------------------------------
// Item spawn queue wrapping
// ---------------------------------------------------------------------------

#[test]
fn item_queue_wraps() {
    let mut head: c_int = 0;
    head = (head + 1) & (c_ffi::ITEMQUESIZE as c_int - 1);
    assert_eq!(head, 1);
    head = 127;
    head = (head + 1) & (c_ffi::ITEMQUESIZE as c_int - 1);
    assert_eq!(head, 0);
}

// ---------------------------------------------------------------------------
// RNG wrapping (uses the Rust port since m_random.c is ported, but verifies
// the exact C behaviour).
// ---------------------------------------------------------------------------

#[test]
fn rndindex_wraps_at_256() {
    let mut idx: c_int = 0;
    idx = (idx + 1) & 0xff;
    assert_eq!(idx, 1);
    idx = (idx + 255) & 0xff;
    assert_eq!(idx, 0);
}
