//! Rust port of vendor/doomgeneric/m_bbox.c.

#![allow(non_snake_case)]

use super::m_fixed::fixed_t;

// Indices must match m_bbox.h:
//   BOXTOP=0, BOXBOTTOM=1, BOXLEFT=2, BOXRIGHT=3
pub const BOXTOP: usize = 0;
pub const BOXBOTTOM: usize = 1;
pub const BOXLEFT: usize = 2;
pub const BOXRIGHT: usize = 3;

/// `void M_ClearBox(fixed_t *box)` — initialises an inverted bbox so
/// subsequent `M_AddToBox` calls shrink it to fit.
///
/// # Safety
/// `bbox` must point to at least 4 contiguous `fixed_t` values.
#[no_mangle]
pub unsafe extern "C" fn M_ClearBox(bbox: *mut fixed_t) {
    let slice = std::slice::from_raw_parts_mut(bbox, 4);
    slice[BOXTOP] = i32::MIN;
    slice[BOXRIGHT] = i32::MIN;
    slice[BOXBOTTOM] = i32::MAX;
    slice[BOXLEFT] = i32::MAX;
}

/// `void M_AddToBox(fixed_t *box, fixed_t x, fixed_t y)`.
///
/// # Safety
/// `bbox` must point to at least 4 contiguous `fixed_t` values.
#[no_mangle]
pub unsafe extern "C" fn M_AddToBox(bbox: *mut fixed_t, x: fixed_t, y: fixed_t) {
    let slice = std::slice::from_raw_parts_mut(bbox, 4);
    if x < slice[BOXLEFT] {
        slice[BOXLEFT] = x;
    } else if x > slice[BOXRIGHT] {
        slice[BOXRIGHT] = x;
    }
    if y < slice[BOXBOTTOM] {
        slice[BOXBOTTOM] = y;
    } else if y > slice[BOXTOP] {
        slice[BOXTOP] = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_then_add_shrinks_to_point() {
        let mut bbox: [fixed_t; 4] = [0; 4];
        unsafe {
            M_ClearBox(bbox.as_mut_ptr());
            M_AddToBox(bbox.as_mut_ptr(), 10, 20);
        }
        // BOXTOP=0, BOXBOTTOM=1, BOXLEFT=2, BOXRIGHT=3
        // After M_ClearBox: [INT_MIN, INT_MAX, INT_MAX, INT_MIN]
        // M_AddToBox(10, 20):
        //   x=10 < BOXLEFT(INT_MAX) → true → BOXLEFT=10, skips else-if (BOXRIGHT stays INT_MIN)
        //   y=20 < BOXBOTTOM(INT_MAX) → true → BOXBOTTOM=20, skips else-if (BOXTOP stays INT_MIN)
        assert_eq!(bbox[0], i32::MIN); // top still inverted (else-if branch)
        assert_eq!(bbox[1], 20); // bottom = y
        assert_eq!(bbox[2], 10); // left = x
        assert_eq!(bbox[3], i32::MIN); // right still inverted (else-if branch)
    }

    #[test]
    fn add_multiple_points_expands_box() {
        let mut bbox: [fixed_t; 4] = [0; 4];
        unsafe {
            M_ClearBox(bbox.as_mut_ptr());
            M_AddToBox(bbox.as_mut_ptr(), 10, 20);
            M_AddToBox(bbox.as_mut_ptr(), -5, 100);
            M_AddToBox(bbox.as_mut_ptr(), 50, 5);
        }
        assert_eq!(bbox[0], 100); // top
        assert_eq!(bbox[1], 5); // bottom
        assert_eq!(bbox[2], -5); // left
        assert_eq!(bbox[3], 50); // right
    }
}
