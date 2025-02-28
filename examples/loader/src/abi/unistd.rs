use crate::process::current_process;
use crate::{AbiEntry, ABI_TABLE};
use abi_macro::abi;
use arceos_posix_api::ctypes;
use axhal::mem::PAGE_SIZE_4K;
use core::ffi::{c_int, c_long};
use core::sync::atomic::Ordering;

#[abi(sysconf)]
#[unsafe(no_mangle)]
extern "C" fn abi_sysconf(name: c_int) -> c_long {
    match name as u32 {
        ctypes::_SC_PAGE_SIZE => PAGE_SIZE_4K as c_long,
        ctypes::_SC_NPROCESSORS_ONLN => axconfig::SMP as c_long,
        // ctypes::_SC_AVPHYS_PAGES=>
        _ => 0
    }
}

#[abi(getpid)]
#[unsafe(no_mangle)]
extern "C" fn abi_getpid() -> c_int {
    current_process().pid.load(Ordering::SeqCst) as c_int
}

#[abi(abort)]
#[unsafe(no_mangle)]
extern "C" fn abi_abort() -> ! {
    panic!()
}