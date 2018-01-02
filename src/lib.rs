#![crate_type = "cdylib"]

extern crate libc;

#[cfg(feature = "vec")]
pub mod vec;

#[cfg(feature = "vec")]
pub use vec::*;

#[cfg(feature = "libc_malloc")]
pub mod libc_malloc;

#[cfg(feature = "libc_malloc")]
pub use libc_malloc::*;
