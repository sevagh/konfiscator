#![crate_type = "cdylib"]

extern crate libc;

use libc::{c_void, size_t};
use std::mem::drop;

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    (&mut vec![0u8; size]).as_mut_ptr() as *mut c_void
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    drop(ptr);
}
