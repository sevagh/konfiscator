use libc::{self, c_void, size_t};

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    unsafe { libc::malloc(size) }
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    unsafe { libc::free(ptr) }
}
