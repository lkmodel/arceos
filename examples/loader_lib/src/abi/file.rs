// use arceos_posix_api::{
//     ctypes, sys_fstat, sys_getcwd, sys_lseek, sys_lstat, sys_open, sys_rename, sys_stat,
// };
// use axlog::info;
// use core::ffi::{c_char, c_int};
//
// #[no_mangle]
// pub extern "C" fn abi_open(filename: *const c_char, flags: c_int, mode: ctypes::mode_t) -> i32 {
//     info!("[ABI:FS] Open a file {:?}", filename);
//     sys_open(filename, flags, mode)
// }
//
// #[no_mangle]
// pub extern "C" fn abi_lseek(fd: c_int, offset: ctypes::off_t, whence: c_int) -> ctypes::off_t {
//     info!("[ABI:FS] Use lseek");
//     sys_lseek(fd, offset, whence)
// }
//
// #[no_mangle]
// pub unsafe extern "C" fn abi_stat(path: *const c_char, buf: *mut ctypes::stat) -> c_int {
//     info!("[ABI:FS] Use stat");
//     sys_stat(path, buf)
// }
//
// #[no_mangle]
// pub unsafe extern "C" fn abi_fstat(fd: c_int, buf: *mut ctypes::stat) -> c_int {
//     info!("[ABI:FS] Use fstat");
//     sys_fstat(fd, buf)
// }
//
// #[no_mangle]
// pub unsafe extern "C" fn abi_lstat(path: *const c_char, buf: *mut ctypes::stat) -> ctypes::ssize_t {
//     info!("[ABI:FS] Use lstat");
//     sys_lstat(path, buf)
// }
//
// #[no_mangle]
// pub fn abi_getcwd(buf: *mut c_char, size: usize) -> *mut c_char {
//     info!("[ABI:FS] Use getcwd");
//     sys_getcwd(buf, size)
// }
//
// #[no_mangle]
// pub fn abi_rename(old: *const c_char, new: *const c_char) -> c_int {
//     info!("[ABI:FS] Use rename");
//     sys_rename(old, new)
// }
