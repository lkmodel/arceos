mod mem;
mod init;
mod thread;
mod string;
mod process;
mod fenv;
mod setjmp;
mod time;
mod unistd;
mod env;
mod fcntl;

use core::ffi::c_int;
use init::{
    abi_exit, abi_hello, abi_putchar
};

pub use crate::ABI_TABLE;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_EXIT: usize = 3;

pub fn abi_entry(abi_num: usize, arg0: usize) {    
    match abi_num  {
        SYS_HELLO => abi_hello(),
        SYS_PUTCHAR => abi_putchar(arg0 as c_int),
        SYS_EXIT => abi_exit(arg0 as i32),
        _ => panic!("[ABI:Unknown] Unknown ABI: 0x{abi_num:x}")
    }
}


pub fn lookup_abi_call(name: &str) -> Option<usize> {
    ABI_TABLE
        .iter()
        .find(|a| a.name == name)
        .map(|a| a.addr as usize)
}
