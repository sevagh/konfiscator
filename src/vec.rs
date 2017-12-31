use libc::{c_void, size_t};
use std::mem::drop;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<u8, usize>> = Mutex::new(HashMap::new());
}

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    let ret = (&mut vec![0u8; size]).as_mut_ptr() as *mut c_void;
    HASHMAP.lock().unwrap().insert(ret as u8, size);
    ret
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    let size = HASHMAP.lock().unwrap().remove(&(ptr as u8)).unwrap();
    drop(unsafe { Vec::from_raw_parts(ptr, size, size) });
}
