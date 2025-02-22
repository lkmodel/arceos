use axlog::{debug, info};
use axstd::{
    print, println, process::exit, thread::sleep
};

use core::{
    ffi::{c_char, c_int}, mem, slice, str, sync::atomic::Ordering, time::Duration
};

use printf_compat::{format, output};

use alloc::string::String;

use crate::{config::KERNEL_PROCESS_ID, process::current_process, save_gp, switch_to_gp, APP_GP, FORK_WAIT, KERNEL_GP, MAIN_WAIT_QUEUE, PARENT_WAIT_QUEUE};

type MainFn = unsafe extern "C" fn(argc: i32, argv: *mut *mut i8, envp: *mut *mut i8) -> i32;

/// Description
/// The `__libc_start_main()` function shall initialize the process, call the main function with appropriate arguments, and handle the return from main().
/// `__libc_start_main()` is not in the source standard; it is only in the binary standard. 
#[unsafe(no_mangle)]
pub extern "C" fn abi_libc_start_main(
    main: MainFn,
    argc: i32,
    argv: *mut *mut i8,
    _init: usize,
    _fini: usize,
) {
    unsafe { 
        save_gp(&APP_GP);
        switch_to_gp(&KERNEL_GP);
    }

    info!("App GP: 0x{:x}", APP_GP.load(Ordering::SeqCst));

    let current_process = current_process();
    info!("Current process: {:?}", current_process.pid());
    
    info!("[ABI:Init]: abi_libc_start_main");
    info!("main: {:?}, argc: 0x{:x}, argv: {:x?}, _init: 0x{:x}, _fini: 0x{:x}", 
           main, argc, argv, _init, _fini);

    let main = unsafe {
        mem::transmute::<usize, MainFn>( main as usize)
    };

    unsafe {
        main(argc, argv, core::ptr::null_mut());
    }

    abi_fini();
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_init() {
    info!("[ABI:Init]: abi_init");
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_fini() {
	info!("[ABI:Fini]: abi_fini");
    // // 通知等待的 main 进程
    // MAIN_WAIT_QUEUE.notify_one(false);
    // FORK_WAIT.notify_one(false);
    // 获取当前进程
    let current = current_process();
    let pid = current.pid();
    
    // 检查是否为子进程
    if current.parent.load(Ordering::Relaxed) != 2 {
        // 是子进程，通知父进程
        info!("Child process {} exiting, notifying parent", pid);
        PARENT_WAIT_QUEUE.notify_one(false);
    } else {
        // 是父进程，通知主线程
        info!("Parent process {} exiting, notifying main thread", pid);
        MAIN_WAIT_QUEUE.notify_one(false);
    }
}

#[unsafe(no_mangle)]
pub fn abi_putchar(c: char) {
    info!("[ABI:Print] {c}");
    print!("{}", c);
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_hello() {
    info!("[ABI:Hello] Hello, Apps!");
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_exit(exit_code: i32) {
    info!("[ABI:Exit] Exit Apps by exit_code: {exit_code}!");
    exit(exit_code);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_printf(fat: *const c_char, mut args: ...) -> c_int {
    info!("[ABI:Print] Print a formatted string!");
    // 空指针检查
    if fat.is_null() {
        return -1;
    }

    let fat = ((fat as usize)) as *const c_char;

    info!("fat: {:p}", fat);

    let mut s = String::new();
    let bytes_written = unsafe { format(fat, args.as_va_list(), output::fmt_write(&mut s)) };
    print!("{}", s);
    bytes_written as c_int
}

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

#[unsafe(no_mangle)]
pub extern "C" fn abi_sleep(seconds: u32) {
    debug!("[ABI:Sleep] Sleep for {} seconds", seconds);
    sleep(Duration::from_secs(seconds as u64));
}
