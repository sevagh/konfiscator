#![crate_type = "cdylib"]

extern crate libc;

use libc::{c_void, PT_NULL};

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
        println!("Hello world! {}", size);
        PT_NULL as *mut c_void
}
