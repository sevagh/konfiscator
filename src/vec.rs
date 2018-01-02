use libc::{c_void, size_t, PT_NULL};

#[derive(Debug)]
struct FatPtr(*mut [u8]);

unsafe impl Send for FatPtr {}
unsafe impl Sync for FatPtr {}

type ThinPtr = u8;

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    let fat_ptr = Box::into_raw((vec![0u8; size]).into_boxed_slice());
    let thin_ptr = fat_ptr as *mut c_void;
    thin_ptr
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    //let x = unsafe { Box::from_raw(MALLOC_HASHES.lock().unwrap().remove(&(ptr as u8)).unwrap().0) };
    //drop(x);
}
