#![crate_type = "cdylib"]

extern crate libc;

use libc::{c_void, size_t};

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    println!("Hello world from KONFISCATOR! {}", size);
    unsafe { libc::malloc(size) }
}
