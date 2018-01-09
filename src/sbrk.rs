use sbrsk;
use libc::{c_void, size_t, PT_NULL};

#[no_mangle]
pub extern "C" fn konfiscator_malloc(size: size_t) -> *mut c_void {
    let ret = match sbrsk::sbrk(size) {
        Ok(x) => x as u32,
        Err(sbrsk::Error::ENOMEM) => PT_NULL,
    };
    ret as *mut c_void
}

#[no_mangle]
pub extern "C" fn konfiscator_free(ptr: *mut c_void) {
    let _ = ptr;
    ()
}
