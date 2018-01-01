use libc::{c_void, size_t};
use std::mem::drop;
use alloc::raw_vec::RawVec;
use alloc::heap::Heap;

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    let vec: RawVec<u8, Heap> = RawVec::with_capacity(size);
    vec.ptr() as *mut c_void
}

#[no_mangle]
pub extern "C" fn free(ptr: c_void) {
    drop(ptr)
}
