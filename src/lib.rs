#![crate_type = "cdylib"]

extern crate libc;

#[macro_use]
extern crate lazy_static;

pub mod vec;

#[cfg(feature = "vec")]
pub use vec::{free, malloc};
