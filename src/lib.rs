#![crate_type = "cdylib"]
#![no_std]

extern crate libc;

#[cfg(feature = "sbrk")]
pub mod sbrk;

#[cfg(feature = "sbrk")]
pub use sbrk::*;

#[cfg(feature = "mmap")]
pub mod mmap;

#[cfg(feature = "mmap")]
pub use mmap::*;
