use crate::{ABI_TABLE, AbiEntry};
use abi_macro::abi;
use axlog::error;
use core::ffi::{CStr, c_char, c_int};

#[abi(__assert_fail)]
#[unsafe(no_mangle)]
extern "C" fn abi__assert_fail(
    expr: *const c_char,
    file: *const c_char,
    line: c_int,
    func: *const c_char,
) -> ! {
    let expr = unsafe { CStr::from_ptr(expr).to_str().unwrap() };
    let file = unsafe { CStr::from_ptr(file).to_str().unwrap() };
    let line = line as usize;
    let func = unsafe { CStr::from_ptr(func).to_str().unwrap() };
    error!("Assertion failed: {} ({}:{}:{})", expr, file, line, func);
    panic!("Assertion failed: {} ({}:{}:{})", expr, file, line, func)
}
