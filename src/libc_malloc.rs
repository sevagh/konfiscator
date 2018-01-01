use libc::{self, c_void, size_t};

#[no_mangle]
#[export_name = "malloc"]
pub extern "C" fn _malloc(size: size_t) -> *mut c_void {
    unsafe { libc::malloc(size) }
}

#[no_mangle]
#[export_name = "free"]
pub extern "C" fn _free(ptr: *mut c_void) {
    unsafe { libc::free(ptr) }
}
