use core::ffi::{c_int, c_long};

use arceos_posix_api::ctypes;
use axhal::mem::PAGE_SIZE_4K;

pub extern "C" fn abi_sysconf(name: c_int) -> c_long {
    match name as u32 {
        ctypes::_SC_PAGE_SIZE => PAGE_SIZE_4K as c_long,
        ctypes::_SC_NPROCESSORS_ONLN => axconfig::SMP as c_long,
        // ctypes::_SC_AVPHYS_PAGES=>
        _ => 0
    }
}