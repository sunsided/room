# Design: Fix UB from mutating immutable static animation tables in `wi_stuff`

**Issue:** [#20](https://github.com/sunsided/room/issues/20)
**Date:** 2026-05-15
**Status:** Approved

## Problem

`room/src/doom/wi_stuff.rs` declares three animation info tables as immutable `static`:

```rust
static EPSD0ANIMINFO: [anim_t; 10] = [...];
static EPSD1ANIMINFO: [anim_t; 9]  = [...];
static EPSD2ANIMINFO: [anim_t; 6]  = [...];
```

`ANIMS` stores `*const anim_t` pointers into these tables:

```rust
static mut ANIMS: [*const anim_t; NUMEPISODES] = [
    EPSD0ANIMINFO.as_ptr(),
    ...
];
```

Four functions cast those `*const` pointers to `*mut anim_t` and write through them:

- `WI_initAnimatedBack` - writes `ctr`, `nexttic`
- `WI_updateAnimatedBack` - writes `ctr`, `nexttic`
- `WI_drawAnimatedBack` - casts to `*mut` unnecessarily (read-only)
- `WI_loadUnloadData` - writes `p[i]` (patch pointers)

Writing through a pointer derived from an immutable `static` is undefined behavior. It may fault on read-only memory pages.

## Design

### Approach: `static mut` arrays + fix read-only cast

Minimal change, consistent with the rest of the file which has ~50 `static mut` globals and all-`unsafe` functions.

### Changes

**1. Promote tables to `static mut`:**

```rust
// TODO: Consider splitting into immutable config (type_, period, nanims, loc,
// data1, data2) and mutable runtime state (ctr, nexttic, lastdrawn, state, p)
// using two separate arrays, or wrapping in UnsafeCell for explicit interior
// mutability without static mut.
static mut EPSD0ANIMINFO: [anim_t; 10] = [...];
static mut EPSD1ANIMINFO: [anim_t; 9]  = [...];
static mut EPSD2ANIMINFO: [anim_t; 6]  = [...];
```

**2. Change `ANIMS` element type to `*mut anim_t`:**

```rust
static mut ANIMS: [*mut anim_t; NUMEPISODES] = [
    EPSD0ANIMINFO.as_mut_ptr(),
    EPSD1ANIMINFO.as_mut_ptr(),
    EPSD2ANIMINFO.as_mut_ptr(),
    ptr::null_mut(),
];
```

**3. Remove const-to-mut casts in write paths:**

In `WI_initAnimatedBack`, `WI_updateAnimatedBack`, `WI_loadUnloadData`:
```rust
// Before:
let base = ANIMS[epsd] as *mut anim_t;
// After:
let base = ANIMS[epsd];
```

**4. Fix `WI_drawAnimatedBack` to use read-only pointer:**

```rust
// Before (unnecessary mut cast):
let base = ANIMS[epsd] as *mut anim_t;
// After:
let base = ANIMS[epsd] as *const anim_t;
```

**5. Remove `unsafe impl Sync for anim_t`:**

`static mut` requires `Send`, not `Sync`. Raw pointers implement `Send` already, so the impl is no longer needed.

### Files changed

- `room/src/doom/wi_stuff.rs` only

### Non-changes

- No behavior change.
- No struct layout changes.
- No FFI surface changes (`anim_t` has no `#[repr(C)]`).
- All existing `unsafe` annotations remain.

## Future work (captured as TODO comments in code)

- Split `anim_t` into immutable config struct + mutable state struct to make the const/mutable boundary explicit at the type level.
- Replace `static mut` with `UnsafeCell`-based newtype once the file moves toward more idiomatic Rust.

## Testing

- `cargo build` must pass.
- `cargo test` must pass.
- No runtime behavior change expected.
