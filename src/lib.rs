#![crate_type = "cdylib"]
#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;
extern crate libc;

pub mod vec;
pub mod libc_malloc;

#[cfg(feature = "vec")]
pub use vec::*;

#[cfg(feature = "libc_malloc")]
pub use libc_malloc::*;
