use libc;

#[no_mangle]
pub extern "C" fn konfiscator_malloc(size: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::malloc(size) }
}

#[no_mangle]
pub extern "C" fn konfiscator_free(ptr: *mut libc::c_void) {
    unsafe { libc::free(ptr) }
}
