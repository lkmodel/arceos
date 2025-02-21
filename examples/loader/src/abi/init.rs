use axlog::{debug, error, info};
use axtask::init_scheduler;

use axstd::{
    print, println, process::exit, thread::sleep
};

use core::{
    ffi::{c_char, c_int}, mem, slice, str, time::Duration
};

use printf_compat::{format, output};

use crate::{AbiEntry, ABI_TABLE};
use abi_macro::abi;
use alloc::string::String;
type MainFn = unsafe extern "C" fn(argc: i32, argv: *mut *mut i8, envp: *mut *mut i8) -> i32;

/// Description
/// The `__libc_start_main()` function shall initialize the process, call the main function with appropriate arguments, and handle the return from main().
/// `__libc_start_main()` is not in the source standard; it is only in the binary standard.
#[abi(__libc_start_main)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_libc_start_main(
	main: MainFn,
	argc: i32,
    argv: *mut *mut i8,
    _init: usize,
    _fini: usize,
) {
	info!("[ABI:Init]: abi_libc_start_main");
	info!("main: {:?}, argc: {}, argv: {:?}, _init: 0x{:x}, _fini: 0x{:x}", main, argc, argv, _init, _fini);

	init_scheduler();

    let main = unsafe {
		mem::transmute::<usize, MainFn>( main as usize)
	};

	unsafe {
        let status = main(argc, argv, core::ptr::null_mut());
        if status < 0 {
            error!("[ABI:Main]: main exit code: {}", status);
        }
	}

	abi_fini();
}

#[abi(_init)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_init() {
    info!("[ABI:Init]: abi_init");
}

#[abi(_fini)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_fini() {
	info!("[ABI:Init]: abi_fini");
}

#[abi(putchar)]
#[unsafe(no_mangle)]
pub fn abi_putchar(c: char) {
    // info!("[ABI:Print] {c}");
    print!("{}", c);
}

#[abi(hello)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_hello() {
    info!("[ABI:Hello] Hello, Apps!");
}

#[abi(exit)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_exit(exit_code: i32) {
    info!("[ABI:Exit] Exit Apps by exit_code: {exit_code}!");
    exit(exit_code);
}

#[abi(printf)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_printf(fat: *const c_char, mut args: ...) -> c_int {
    info!("[ABI:Print] Print a formatted string!");
    // 空指针检查
    if fat.is_null() {
        error!("[ABI:Print] Print NULL!");
        return -1;
    }

    let fat = ((fat as usize)) as *const c_char;

    info!("fat: {:p}", fat);

    let mut s = String::new();
    let bytes_written = unsafe { format(fat, args.as_va_list(), output::fmt_write(&mut s)) };
    print!("{}", s);
    bytes_written as c_int
}

#[abi(puts)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_puts(s: *const c_char) -> i32 {
    info!("[ABI:Print] Print a string!");
    if s.is_null() {
        return -1;
    }

    let res = ((s as usize)) as *const i8;
    if res.is_null() {
        return -1;
    }

    // 计算字符串长度并进行转换
    unsafe {
        let mut len = 0;
        let mut current = res;
        
        // 计算字符串长度
        while *current != 0 {
            len += 1;
            current = current.add(1);
        }

        // 创建字节切片
        let slice = slice::from_raw_parts(res as *const u8, len);
        
        // 转换为UTF-8字符串并处理
        match str::from_utf8(slice) {
            Ok(string) => {
                println!("{}", string);
                (len + 1) as i32
            }
            Err(_) => {
                // 如果不是有效的UTF-8，尝试按字节输出
                let bytes = slice.iter()
                    .map(|&b| b as char)
                    .collect::<String>();
                println!("{}", bytes);
                (len + 1) as i32
            }
        }
    }
}

#[abi(sleep)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_sleep(seconds: u32) {
    debug!("[ABI:Sleep] Sleep for {} seconds", seconds);
    sleep(Duration::from_secs(seconds as u64));
}
