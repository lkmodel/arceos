use crate::{ABI_TABLE, AbiEntry};
use abi_macro::abi;
use core::ffi::c_char;

#[abi(getenv)]
#[unsafe(no_mangle)]
extern "C" fn abi_getenv(_name: *const c_char) -> *mut c_char {
    core::ptr::null_mut()
}
