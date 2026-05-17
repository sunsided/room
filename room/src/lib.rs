// TODO: Add `# Safety` documentation to all unsafe functions.
#![allow(clippy::missing_safety_doc)]
// This crate is a C-to-Rust port. Many symbols are defined for FFI completeness
// but not yet called from Rust code; libc functions are redeclared with
// compatible-but-different types; and internal pointer types are intentionally
// less visible than their containing statics.
#![allow(dead_code, clashing_extern_declarations, private_interfaces)]
// Doom thinker dispatch compares function pointers for identity within a single
// binary, which is a well-understood and reliable pattern in this context.
#![allow(unpredictable_function_pointer_comparisons)]
// This is a C-to-Rust port: extern "C" functions routinely take raw pointer
// arguments that are dereferenced in the body. The C callers ensure validity.
#![allow(clippy::not_unsafe_ptr_arg_deref)]
// C-style indexed loops are idiomatic in this port; many arrays are static mut,
// so converting to iterators would require from_raw_parts complexity.
#![allow(clippy::needless_range_loop, clippy::explicit_counter_loop)]

pub(crate) mod audio;
pub mod doom;
pub mod headless;
pub mod types;

#[cfg(feature = "dhat-heap")]
pub use dhat;
