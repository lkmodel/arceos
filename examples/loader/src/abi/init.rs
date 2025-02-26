use axlog::{debug, info};
use axstd::{
    print, println, process::exit, thread::sleep
};
use axtask::{current, TaskExtRef};

use core::{
    ffi::{c_char, c_int}, mem, slice, str, sync::atomic::Ordering, time::Duration
};

use printf_compat::{format, output};

use alloc::{string::String, sync::Arc};

use crate::{process::{current_process, PID2PC, TID2TASK}, save_gp, switch_to_gp, APP_GP, KERNEL_GP, MAIN_WAIT_QUEUE, PROCESS_COUNT};

type MainFn = unsafe extern "C" fn(argc: i32, argv: *mut *mut i8, envp: *mut *mut i8) -> i32;

/// Description
/// The `__libc_start_main()` function shall initialize the process, call the main function with appropriate arguments, and  handle the return from main().
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
        info!("Save app GP");
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

    info!("abi_fini : 0x{:x}", abi_fini as usize);

    abi_fini();

    let mut pc: usize;
    let mut ra: usize;

    unsafe {
        core::arch::asm!(
            // 获取当前 PC
            "auipc {}, 0",  // 将当前 PC 值加载到寄存器中
            "mv {}, ra",
            out(reg) pc,
            out(reg) ra,
        );
    }
    
    info!("Current PC: 0x{:x}, ra: 0x{:x}", pc, ra);
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_init() {
    info!("[ABI:Init]: abi_init");
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_fini() {
	info!("[ABI:Fini]: abi_fini");
    
    // 减少进程计数
    let remaining = PROCESS_COUNT.fetch_sub(1, Ordering::SeqCst) - 1;
    info!("Remaining processes: {}", remaining);
    
    let current_task = current();

    info!("Current task: {:?}", current_task.id());

    let pid = &current_task.task_ext().get_process_id();

    info!("Current task: {:?}", pid);
    
    // 获取当前任务
    let current_task = current();
    let task_id = current_task.id().as_u64();
    
    if remaining == 0 {
        info!("All processes finished");
        MAIN_WAIT_QUEUE.notify_one(false);
    }
    
    {
        // 先查找进程，但不要持有PID2PC的锁
        let process_to_clean = {
            let pid_map = PID2PC.lock();
            pid_map.get(&pid).cloned()
        };
        
        // // 如果找到进程，处理它
        // if let Some(process) = process_to_clean {
        //     // 从进程任务列表中移除当前任务
        //     let empty_tasks = {
        //         let mut tasks = process.tasks.lock();
        //         tasks.retain(|t| t.id().as_u64() != task_id);
        //         tasks.is_empty()
        //     };
            
        //     // 若为进程的最后一个任务，清理进程资源
        //     if empty_tasks {
        //         // 从父进程的子进程列表中移除
        //         let parent_id = process.parent.load(Ordering::Acquire);
                
        //         // 获取父进程但不持有 PID2PC 锁
        //         let parent_to_update = {
        //             let pid_map = PID2PC.lock();
        //             pid_map.get(&parent_id).cloned()
        //         };
                
        //         if let Some(parent) = parent_to_update {
        //             let mut parent_children = parent.children.lock();
        //             parent_children.retain(|c| c.pid() != *pid);
        //         }
                
        //         // 从全局表中移除进程（现在没有持有PID2PC锁）
        //         PID2PC.lock().remove(&pid);
        //     }
        // }
        
        // // 从全局表中移除任务
        // TID2TASK.lock().remove(&task_id);
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
