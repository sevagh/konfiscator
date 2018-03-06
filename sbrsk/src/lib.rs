#![crate_type = "lib"]
#![no_std]

#[macro_use]
extern crate sc;

use core::result;

pub enum Error {
    ENOMEM
}

pub type Result<T> = result::Result<T, Error>;

pub fn brk(addr: usize) -> Result<()> {
    let existing_addr = unsafe {
        syscall!(BRK, 0)
    };
    let new_ptr = unsafe {
        syscall!(BRK, addr)
    };
    if new_ptr != addr {
        assert!(existing_addr == new_ptr);
        return Err(Error::ENOMEM);
    }
    Ok(())
}

pub fn sbrk(increment: usize) -> Result<usize> {
    let data_ptr = unsafe {
        syscall!(BRK, 0)
    };
    let new_ptr = unsafe {
        syscall!(BRK, data_ptr + increment)
    };
    if new_ptr == data_ptr {
        return Err(Error::ENOMEM);
    }
    Ok(new_ptr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbrk_valid() {
        let original_ptr = unsafe {
            syscall!(BRK, 0)
        };
        match sbrk(8) {
            Ok(x) => assert_eq!(x, original_ptr+8),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_sbrk_invalid() {
        match sbrk(0) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_brk_valid() {
        let original_ptr = unsafe {
            syscall!(BRK, 0)
        };
        match brk(original_ptr + 8) {
            Ok(_) => assert!(true),
            Err(Error::ENOMEM) => assert!(false),
        }
    }

    #[test]
    fn test_brk_invalid() {
        match brk(0) {
            Ok(_) => assert!(false),
            Err(Error::ENOMEM) => assert!(true),
        }
    }
}
