//! Rust port of vendor/doomgeneric/z_zone.c.
//!
//! Zone Memory Allocation — a doubly-linked-list allocator whose blocks
//! sit inside a buffer returned by `I_ZoneBase`.  Callers never inspect
//! the header, so only the user-visible pointer and behaviour matter.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::i_error;

pub const PU_STATIC: c_int = 1;
pub const PU_FREE: c_int = 4;
pub const PU_LEVEL: c_int = 5;
pub const PU_LEVSPEC: c_int = 6;
pub const PU_PURGELEVEL: c_int = 7;
pub const PU_CACHE: c_int = 8;

const ZONEID: u32 = 0x1d4a11;
const MINFRAGMENT: c_int = 64;
const MEM_ALIGN: usize = std::mem::size_of::<*mut ()>();

#[repr(C)]
#[derive(Clone, Copy)]
struct memblock_t {
    size: c_int,
    user: *mut *mut c_void,
    tag: c_int,
    id: c_int,
    next: *mut memblock_t,
    prev: *mut memblock_t,
}

#[repr(C)]
struct memzone_t {
    size: c_int,
    blocklist: memblock_t,
    rover: *mut memblock_t,
}

pub static mut mainzone: *mut memzone_t = ptr::null_mut();

use crate::doom::i_system::I_ZoneBase;

unsafe fn Z_ClearZone(zone: *mut memzone_t) {
    let block = (zone as *mut u8).add(std::mem::size_of::<memzone_t>()) as *mut memblock_t;

    (*zone).blocklist.next = block;
    (*zone).blocklist.prev = block;
    (*zone).blocklist.user = zone as *mut *mut c_void;
    (*zone).blocklist.tag = 1; // PU_STATIC
    (*zone).rover = block;

    (*block).prev = &mut (*zone).blocklist;
    (*block).next = &mut (*zone).blocklist;
    (*block).tag = PU_FREE;
    (*block).size = (*zone).size - std::mem::size_of::<memzone_t>() as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn Z_Init() {
    let mut size: c_int = 0;
    mainzone = I_ZoneBase(&mut size) as *mut memzone_t;
    (*mainzone).size = size;

    Z_ClearZone(mainzone);
}

#[no_mangle]
pub unsafe extern "C" fn Z_Free(ptr: *mut c_void) {
    let block = (ptr as *mut u8).sub(std::mem::size_of::<memblock_t>()) as *mut memblock_t;

    if (*block).id as u32 != ZONEID {
        i_error!("Z_Free: freed a pointer without ZONEID");
    }

    if (*block).tag != PU_FREE && !(*block).user.is_null() {
        *(*block).user = ptr::null_mut();
    }

    (*block).tag = PU_FREE;
    (*block).user = ptr::null_mut();
    (*block).id = 0;

    let mut block = block;
    let other = (*block).prev;
    if (*other).tag == PU_FREE {
        (*other).size += (*block).size;
        (*other).next = (*block).next;
        (*(*block).next).prev = other;

        if block == (*mainzone).rover {
            (*mainzone).rover = other;
        }
        block = other;
    }

    let other = (*block).next;
    if (*other).tag == PU_FREE {
        (*block).size += (*other).size;
        (*block).next = (*other).next;
        (*(*block).next).prev = block;

        if other == (*mainzone).rover {
            (*mainzone).rover = block;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void {
    if mainzone.is_null() {
        panic!("Z_Malloc: mainzone is null!");
    }
    let size = (size + MEM_ALIGN as c_int - 1) & !(MEM_ALIGN as c_int - 1);
    let size = size + std::mem::size_of::<memblock_t>() as c_int;

    let mut base = (*mainzone).rover;
    if base.is_null() {
        panic!("Z_Malloc: rover is null!");
    }
    if (*base).prev.is_null() {
        panic!(
            "Z_Malloc: rover->prev is null! base={:?} base.tag={} base.size={}",
            base,
            (*base).tag,
            (*base).size
        );
    }
    if (*(*base).prev).tag == PU_FREE {
        base = (*base).prev;
    }

    let mut rover = base;
    let start = (*base).prev;

    loop {
        if rover == start {
            i_error!("Z_Malloc: failed on allocation");
        }

        if (*rover).tag != PU_FREE {
            if (*rover).tag < PU_PURGELEVEL {
                base = rover;
                rover = (*rover).next;
            } else {
                base = (*base).prev;
                let rover_ptr = rover as *mut u8;
                Z_Free(rover_ptr.add(std::mem::size_of::<memblock_t>()) as *mut c_void);
                base = (*base).next;
                rover = (*base).next;
            }
        } else {
            rover = (*rover).next;
        }

        if (*base).tag == PU_FREE && (*base).size >= size {
            break;
        }
    }

    let extra = (*base).size - size;

    if extra > MINFRAGMENT {
        let newblock = (base as *mut u8).add(size as usize) as *mut memblock_t;
        (*newblock).size = extra;
        (*newblock).tag = PU_FREE;
        (*newblock).user = ptr::null_mut();
        (*newblock).prev = base;
        (*newblock).next = (*base).next;
        (*(*base).next).prev = newblock;
        (*base).next = newblock;
        (*base).size = size;
    }

    if user.is_null() && tag >= PU_PURGELEVEL {
        i_error!("Z_Malloc: an owner is required for purgable blocks");
    }

    (*base).user = user as *mut *mut c_void;
    (*base).tag = tag;

    let result = (base as *mut u8).add(std::mem::size_of::<memblock_t>()) as *mut c_void;

    // Zero the user data area using a byte-by-byte loop to avoid
    // memset writing past the end into the next block header.
    // (ptr::write_bytes / memset may use SIMD that overshoots on
    //  non-aligned sizes under ASan instrumentation.)
    let user_size = size - std::mem::size_of::<memblock_t>() as c_int;
    let result_bytes = result as *mut u8;
    for i in 0..user_size as usize {
        *result_bytes.add(i) = 0;
    }

    if !(*base).user.is_null() {
        *(*base).user = result;
    }

    (*mainzone).rover = (*base).next;
    (*base).id = ZONEID as c_int;

    result
}

#[no_mangle]
pub unsafe extern "C" fn Z_FreeTags(lowtag: c_int, hightag: c_int) {
    let zone = mainzone;
    let sentinel = std::ptr::addr_of_mut!((*zone).blocklist);
    let mut block = (*zone).blocklist.next;
    let mut freed = 0;
    let mut walked = 0;

    while block != sentinel && walked < 2000 {
        walked += 1;
        let next = (*block).next;

        if (*block).tag != PU_FREE && (*block).tag >= lowtag && (*block).tag <= hightag {
            let block_ptr = block as *mut u8;
            Z_Free(block_ptr.add(std::mem::size_of::<memblock_t>()) as *mut c_void);
            freed += 1;
        }

        block = next;
    }
    eprintln!(
        "[Z_FreeTags] lowtag={} hightag={} walked={} freed={}",
        lowtag, hightag, walked, freed
    );
}

#[no_mangle]
pub unsafe extern "C" fn Z_DumpHeap(lowtag: c_int, hightag: c_int) {
    let zone = mainzone;
    let sentinel = std::ptr::addr_of_mut!((*zone).blocklist);

    let msg = b"zone size: %i  location: %p\n\0";
    libc::printf(msg.as_ptr() as *const c_char, (*zone).size, zone);

    let msg2 = b"tag range: %i to %i\n\0";
    libc::printf(msg2.as_ptr() as *const c_char, lowtag, hightag);

    let mut block = (*zone).blocklist.next;
    loop {
        if (*block).tag >= lowtag && (*block).tag <= hightag {
            let msg3 = b"block:%p    size:%7i    user:%p    tag:%3i\n\0";
            libc::printf(
                msg3.as_ptr() as *const c_char,
                block,
                (*block).size,
                (*block).user,
                (*block).tag,
            );
        }

        if (*block).next == sentinel {
            break;
        }

        if (block as *mut u8).add((*block).size as usize) != (*block).next as *mut u8 {
            let msg4 = b"ERROR: block size does not touch the next block\n\0";
            libc::printf(msg4.as_ptr() as *const c_char);
        }

        if (*(*block).next).prev != block {
            let msg5 = b"ERROR: next block doesn't have proper back link\n\0";
            libc::printf(msg5.as_ptr() as *const c_char);
        }

        if (*block).tag == PU_FREE && (*(*block).next).tag == PU_FREE {
            let msg6 = b"ERROR: two consecutive free blocks\n\0";
            libc::printf(msg6.as_ptr() as *const c_char);
        }

        block = (*block).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Z_CheckHeap() {
    if !Z_CheckHeapQuiet() {
        // Already printed diagnostic info.
    }
    let zone = mainzone;
    let sentinel = std::ptr::addr_of_mut!((*zone).blocklist);
    let mut block = (*zone).blocklist.next;

    loop {
        if (*block).next == sentinel {
            break;
        }

        if (block as *mut u8).add((*block).size as usize) != (*block).next as *mut u8 {
            eprintln!(
                "Z_CheckHeap FAIL: block {:?} size={} next={:?} expected_next={:?}",
                block,
                (*block).size,
                (*block).next,
                (block as *mut u8).add((*block).size as usize)
            );
            i_error!("Z_CheckHeap: block size does not touch the next block\n");
        }

        if (*(*block).next).prev != block {
            eprintln!(
                "Z_CheckHeap FAIL: block {:?} next={:?} next.prev={:?}",
                block,
                (*block).next,
                (*(*block).next).prev
            );
            i_error!("Z_CheckHeap: next block doesn't have proper back link\n");
        }

        if (*block).tag == PU_FREE && (*(*block).next).tag == PU_FREE {
            i_error!("Z_CheckHeap: two consecutive free blocks\n");
        }

        block = (*block).next;
    }
}

/// Check heap integrity without aborting. Returns true if valid.
#[no_mangle]
pub unsafe extern "C" fn Z_CheckHeapQuiet() -> bool {
    let zone = mainzone;
    if zone.is_null() {
        return true;
    }
    let sentinel = std::ptr::addr_of_mut!((*zone).blocklist);
    let mut block = (*zone).blocklist.next;
    let mut valid = true;
    let mut count = 0;

    while block != sentinel && count < 2000 {
        let size = (*block).size;
        if size <= 0 || size > 10_000_000 {
            eprintln!(
                "Z_CheckHeapQuiet: block {:?} has invalid size {}",
                block, size
            );
            valid = false;
            break;
        }

        // The last block's next points to the sentinel (at the start of the zone),
        // not to block+size (at the end of the zone). This is by design.
        if (*block).next != sentinel {
            let expected_next = (block as *mut u8).add(size as usize) as *mut memblock_t;
            if (*block).next != expected_next {
                eprintln!(
                    "Z_CheckHeapQuiet: block {:?} size={} next={:?} expected={:?}",
                    block,
                    size,
                    (*block).next,
                    expected_next
                );
                valid = false;
            }
        }

        // Check prev link (works for all blocks including the last one whose next is sentinel)
        if !(*block).next.is_null() && (*(*block).next).prev != block {
            eprintln!(
                "Z_CheckHeapQuiet: block {:?} next={:?} next.prev={:?}",
                block,
                (*block).next,
                (*(*block).next).prev
            );
            valid = false;
        }

        // Check for consecutive free blocks (sentinel has tag=PU_STATIC, so skip it)
        if (*block).tag == PU_FREE && (*block).next != sentinel && (*(*block).next).tag == PU_FREE {
            eprintln!(
                "Z_CheckHeapQuiet: two consecutive free blocks at {:?} and {:?}",
                block,
                (*block).next
            );
            valid = false;
        }

        block = (*block).next;
        count += 1;
    }
    if count >= 2000 {
        eprintln!("Z_CheckHeapQuiet: too many blocks, possible loop!");
        valid = false;
    }
    valid
}

pub unsafe fn Z_CheckHeapAfter(name: &str) {
    if !Z_CheckHeapQuiet() {
        eprintln!("*** Heap corruption detected after {} ***", name);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Z_ChangeTag2(
    ptr: *mut c_void,
    tag: c_int,
    _file: *const c_char,
    _line: c_int,
) {
    let block = (ptr as *mut u8).sub(std::mem::size_of::<memblock_t>()) as *mut memblock_t;

    if (*block).id as u32 != ZONEID {
        i_error!("Z_ChangeTag: block without a ZONEID!");
    }

    if tag >= PU_PURGELEVEL && (*block).user.is_null() {
        i_error!("Z_ChangeTag: an owner is required for purgable blocks");
    }

    (*block).tag = tag;
}

#[no_mangle]
pub unsafe extern "C" fn Z_ChangeUser(ptr: *mut c_void, user: *mut *mut c_void) {
    let block = (ptr as *mut u8).sub(std::mem::size_of::<memblock_t>()) as *mut memblock_t;

    if (*block).id as u32 != ZONEID {
        i_error!("Z_ChangeUser: Tried to change user for invalid block!");
    }

    (*block).user = user;
    *user = ptr;
}

#[no_mangle]
pub unsafe extern "C" fn Z_FreeMemory() -> c_int {
    let zone = mainzone;
    let sentinel = std::ptr::addr_of_mut!((*zone).blocklist);
    let mut free: c_int = 0;
    let mut block = (*zone).blocklist.next;

    while block != sentinel {
        if (*block).tag == PU_FREE || (*block).tag >= PU_PURGELEVEL {
            free += (*block).size;
        }
        block = (*block).next;
    }

    free
}

#[no_mangle]
pub unsafe extern "C" fn Z_FileDumpHeap(_f: *mut libc::FILE) {
    // No Rust caller found; stubbed per plan.
}

#[no_mangle]
pub unsafe extern "C" fn Z_ZoneSize() -> u32 {
    (*mainzone).size as u32
}
#[no_mangle]
pub unsafe extern "C" fn Z_Zone_Link_Anchor() {
    Z_Init();
    Z_Free(ptr::null_mut());
    Z_Malloc(0, 0, ptr::null_mut());
    Z_FreeTags(0, 0);
    Z_DumpHeap(0, 0);
    Z_FileDumpHeap(ptr::null_mut());
    Z_CheckHeap();
    Z_ChangeTag2(ptr::null_mut(), 0, ptr::null(), 0);
    Z_ChangeUser(ptr::null_mut(), ptr::null_mut());
    Z_FreeMemory();
    Z_ZoneSize();
}
