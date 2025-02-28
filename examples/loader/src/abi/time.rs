use crate::{AbiEntry, ABI_TABLE};
use abi_macro::abi;
use core::ffi::c_int;

#[abi(utimes)]
#[unsafe(no_mangle)]
extern "C" fn abi_utimes() -> c_int {
    1
}