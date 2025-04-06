use crate::{ABI_TABLE, AbiEntry};
use abi_macro::abi;
use core::ffi::c_int;

#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
pub static mut errno: c_int = 0;

pub fn set_errno(code: i32) {
    unsafe {
        errno = code;
    }
}

#[abi(__errno_location)]
#[unsafe(no_mangle)]
unsafe extern "C" fn abi_errno_location() -> *mut c_int {
    core::ptr::addr_of_mut!(errno)
}
