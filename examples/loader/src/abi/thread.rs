use axlog::info;
use arceos_posix_api::{self as api, ctypes, sys_pthread_mutex_init, sys_pthread_mutex_lock, sys_pthread_mutex_unlock};
use api::{sys_pthread_create, sys_pthread_exit, sys_pthread_join, sys_pthread_self};
use core::ffi::{c_int, c_void};
use crate::{AbiEntry, ABI_TABLE};
use abi_macro::abi;

#[abi(pthread_create)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_pthread_create(
    res: *mut ctypes::pthread_t,
    attr: *const ctypes::pthread_attr_t,
    start_routine: extern "C" fn(arg: *mut c_void) -> *mut c_void,
    arg: *mut c_void,   // void *__restrict
) -> i32 {
    info!("[ABI:Thread] Create a new thread!");

    info!("res: {:p}", res);
    info!("attr: {:p}", attr);
    info!("start_routine: {:p}", start_routine);
    info!("arg: {:p}", arg);

    unsafe {
        sys_pthread_create(
            res, 
            attr, 
            start_routine, 
            arg
        )
    }
}

#[abi(pthread_join)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_pthread_join(
    thread: ctypes::pthread_t, 
    retval: *mut *mut c_void
) -> i32 {
    info!("[ABI:Thread] Wait for the given thread to exit!");
    unsafe {
        sys_pthread_join(thread, retval)
    }
}

#[abi(pthread_exit)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_pthread_exit(retval: *mut c_void) -> ! {
    info!("[ABI:Thread] Exit the current thread!");
    sys_pthread_exit(retval);
}

#[abi(pthread_self)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_pthread_self() -> ctypes::pthread_t {
    info!("[ABI:Thread] Get the `pthread` struct of current thread!");
    sys_pthread_self()
}

#[abi(pthread_mutex_init)]
#[unsafe(no_mangle)]
pub fn abi_pthread_mutex_init(
    mutex: *mut ctypes::pthread_mutex_t,
    _attr: *const ctypes::pthread_mutexattr_t,
) -> c_int {
    info!("[ABI:Thread] Initialize a mutex!");
    sys_pthread_mutex_init(mutex, _attr)
}

#[abi(pthread_mutex_lock)]
#[unsafe(no_mangle)]
pub fn abi_pthread_mutex_lock(mutex: *mut ctypes::pthread_mutex_t) -> c_int {
    info!("[ABI:Thread] Lock the given mutex!");
    sys_pthread_mutex_lock(mutex)
}

#[abi(pthread_mutex_unlock)]
#[unsafe(no_mangle)]
pub fn abi_pthread_mutex_unlock(mutex: *mut ctypes::pthread_mutex_t) -> c_int {
    info!("[ABI:Thread] Unlock the given mutex!");
    sys_pthread_mutex_unlock(mutex)
}

#[abi(pthread_mutex_destroy)]
#[unsafe(no_mangle)]
pub fn abi_pthread_mutex_destroy() {
    info!("[ABI:Thread] Destroy the given mutex!");
}