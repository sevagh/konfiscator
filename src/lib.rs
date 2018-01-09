#![crate_type = "cdylib"]
#![no_std]

extern crate libc;

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "libc_malloc")] {
        pub mod libc_malloc;
        pub use libc_malloc::*;
    } else if #[cfg(feature = "sbrk")] {
        extern crate sbrsk;
        pub mod sbrk;
        pub use sbrk::*;
    } else {
        unimplemented!("unsupported feature");
    }
}
