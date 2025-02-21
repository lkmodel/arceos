mod mem;
mod init;
mod thread;
mod string;
mod setjmp;
mod fenv;
mod unistd;

use crate::ABI_TABLE;
use init::{
    abi_exit, abi_hello, abi_putchar
};

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_EXIT: usize = 3;

pub fn abi_entry(abi_num: usize, arg0: usize) {    
    match abi_num  {
        SYS_HELLO => abi_hello(),
        SYS_PUTCHAR => abi_putchar(arg0 as u8 as char),
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