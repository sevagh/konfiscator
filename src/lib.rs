#![crate_type = "cdylib"]
#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;
extern crate libc;

pub mod vec;

#[cfg(feature = "vec")]
pub use vec::{free, malloc};
