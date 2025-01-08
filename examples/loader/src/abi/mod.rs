mod init;
mod thread;

use axlog::debug;

use axstd::{
    print,
    println, 
    process::exit,
    thread::sleep,
};

use init::{abi_fini, abi_init, abi_libc_start_main};

use thread::{
    abi_pthread_create, abi_pthread_exit, 
    abi_pthread_join, abi_pthread_mutex_init, 
    abi_pthread_mutex_lock, abi_pthread_mutex_unlock, abi_pthread_self
};

use crate::load::EXEC_ZONE_START;

use core::ffi::{c_char, c_void};
use core::fmt::Write;
use core::time::Duration;
use core::ffi::CStr;

use alloc::string::String;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_EXIT: usize = 3;
const SYS_PTHREAD_CREATE: usize = 4;
const SYS_PTHREAD_JOIN: usize = 5;
const SYS_PTHREAD_EXIT: usize = 6;
const SYS_PTHREAD_SELF: usize = 7;
const SYS_PTHREAD_MUTEX_INIT: usize = 8;
const SYS_PTHREAD_MUTEX_LOCK: usize = 9;
const SYS_PTHREAD_MUTEX_UNLOCK: usize = 10;

#[unsafe(no_mangle)]
pub extern "C" fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_exit(exit_code: i32) {
    println!("[ABI:Exit] Exit Apps by exit_code: {exit_code}!");
    exit(exit_code);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_printf(format: *const c_char, mut args: ...) -> i32 {  
    if format.is_null() {
        return -1;
    }

    let format = ((format as usize) + EXEC_ZONE_START) as *const u8;

    unsafe {
        // 将 C 字符串转换为 Rust 字符串
        let fmt_str = match CStr::from_ptr(format).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };

        // 创建一个字符串缓冲区来存储输出
        let mut output = String::new();

        // 解析格式字符串
        let mut chars = fmt_str.chars().peekable();
        while let Some(c) = chars.next() {
            if c != '%' {
                output.push(c);
                continue;
            }

            // 处理格式说明符
            match chars.next() {
                Some('d') => {
                    let value = args.arg::<i32>();
                    write!(output, "{}", value).unwrap_or(());
                }
                Some('s') => {
                    let ptr = args.arg::<*const c_char>();
                    if !ptr.is_null() {
                        let s = CStr::from_ptr(ptr).to_string_lossy();
                        write!(output, "{}", s).unwrap_or(());
                    }
                }
                Some('c') => {
                    let value = args.arg::<i32>();
                    write!(output, "{}", value as u8 as char).unwrap_or(());
                }
                Some('p') => {
                    let ptr = args.arg::<*const c_void>();
                    // 打印指针地址，使用 0x 前缀和十六进制格式
                    write!(output, "{:p}", ptr).unwrap_or(());
                }
                Some('%') => {
                    output.push('%');
                }
                Some(c) => {
                    // 不支持的格式说明符，原样输出
                    output.push('%');
                    output.push(c);
                }
                None => {
                    output.push('%');
                }
            }
        }

        // 打印输出
        print!("{}", output);

        // 返回打印的字符数
        output.len() as i32
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_puts(s: *const u8) -> i32 {
    println!("[ABI:Print] Print a string!");
    println!("{:?}", s);
    let res = ((s as usize) + EXEC_ZONE_START) as *const u8;
    if res.is_null() {
        return -1;  // 错误处理：空指针
    }

    // 将 C 字符串转换为 Rust 字符串切片
    unsafe {
        let c_str = CStr::from_ptr(res);
        match c_str.to_str() {
            Ok(string) => {
                println!("{}", string);
                (string.len() + 1) as i32
            }
            Err(_) => -1  // 错误处理：无效的 UTF-8
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_sleep(seconds: u32) {
    debug!("[ABI:Sleep] Sleep for {} seconds", seconds);
    sleep(Duration::from_secs(seconds as u64));
}

pub fn abi_entry(abi_num: usize, arg0: usize) {    
    match abi_num  {
        SYS_HELLO => abi_hello(),
        SYS_PUTCHAR => abi_putchar(arg0 as u8 as char),
        SYS_EXIT => abi_exit(arg0 as i32),
        _ => panic!("[ABI:Unknown] Unknown ABI: 0x{abi_num:x}")
    }
}

macro_rules! define_abi_functions {
    ($(($name:expr, $func:expr)),*) => {
        pub fn get_abi_function(name: &str) -> Option<usize> {
            match name {
                $(
                    $name => Some($func as usize),
                )*
                _ => None
            }
        }
    }
}

define_abi_functions! {
    ("hello", abi_hello),
    ("putchar", abi_putchar),
    ("exit", abi_exit),
    ("pthread_create", abi_pthread_create),
    ("pthread_join", abi_pthread_join),
    ("pthread_exit", abi_pthread_exit),
    ("pthread_self", abi_pthread_self),
    // TODO
    ("pthread_mutex_init", abi_pthread_mutex_init),
    // TODO
    ("pthread_mutex_lock", abi_pthread_mutex_lock),
    // TODO
    ("pthread_mutex_unlock", abi_pthread_mutex_unlock),
    ("printf", abi_printf), 
    ("puts", abi_puts), 
    ("sleep", abi_sleep),
    ("_init", abi_init),
    ("_fini", abi_fini),
    ("__libc_start_main", abi_libc_start_main)
}
