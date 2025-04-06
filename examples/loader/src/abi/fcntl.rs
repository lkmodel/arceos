use crate::{ABI_TABLE, AbiEntry};
use abi_macro::abi;
use arceos_posix_api::ctypes::mode_t;
use arceos_posix_api::{sys_close, sys_fcntl, sys_open};
use core::ffi::{c_char, c_int};

#[abi(open)]
#[unsafe(no_mangle)]
extern "C" fn abi_open(filename: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    if filename.is_null() {
        return -1;
    }
    sys_open(filename, flags, mode)
}

#[abi(fcntl)]
#[unsafe(no_mangle)]
extern "C" fn abi_fcntl(fd: c_int, cmd: c_int, arg: usize) -> c_int {
    sys_fcntl(fd, cmd, arg)
}

#[abi(close)]
#[unsafe(no_mangle)]
extern "C" fn abi_close(fd: c_int) -> c_int {
    sys_close(fd)
}

