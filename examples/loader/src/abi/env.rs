use core::ffi::c_char;
use abi_macro::abi;
use crate::{AbiEntry, ABI_TABLE};

#[abi(getenv)]
#[unsafe(no_mangle)]
extern "C" fn abi_getenv(_name:*const c_char)->*mut c_char{
    core::ptr::null_mut()
}