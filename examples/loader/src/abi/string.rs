use crate::{AbiEntry, ABI_TABLE};
use abi_macro::abi;
use core::ffi::c_char;

#[abi(strlen)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_strlen(s: *const c_char) -> usize {
    let mut ptr = s;
    while !ptr.is_null() && unsafe { *ptr != 0 } {
        ptr = unsafe { ptr.add(1) };
    }
    (ptr as usize - s as usize) as usize
}