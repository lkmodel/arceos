use axlog::{info, warn};
use axtask::yield_now;

use crate::{
    config::TASK_STACK_SIZE,
    linux_env::{
        linux_api::api::{exit_current_task, process_api},
        process_ext::api::current_task,
    },
    syscall::{RLIMIT_AS, RLIMIT_NOFILE, RLIMIT_STACK, RLimit, SyscallResult, WaitFlags},
};

/// # Arguments
/// * `exit_code` - i32
pub fn syscall_exit(args: [usize; 6]) -> ! {
    let exit_code = args[0] as i32;
    info!("exit: exit_code = {}", exit_code);

    exit_current_task(exit_code);
}

/// 设置任务资源限制
///
/// `pid`设为0时，表示应用于自己
///
/// # Arguments
/// * `pid - usize`
/// * `resource - i32`
/// * `new_limit - *const RLimit`
/// * `old_limit - *mut RLimit`
pub fn syscall_prlimit64(args: [usize; 6]) -> SyscallResult {
    let pid = args[0];
    let resource = args[1] as i32;
    let new_limit = args[2] as *const RLimit;
    let old_limit = args[3] as *mut RLimit;
    // 当pid不为0，其实没有权利去修改其他的进程的资源限制
    let curr_process = process_api();
    if pid == 0 || pid == curr_process.pid() as usize {
        match resource {
            RLIMIT_STACK => {
                if old_limit as usize != 0 {
                    unsafe {
                        *old_limit = RLimit {
                            rlim_cur: TASK_STACK_SIZE as u64,
                            rlim_max: TASK_STACK_SIZE as u64,
                        };
                    }
                }
            }
            RLIMIT_NOFILE => {
                // 仅支持修改最大文件数
                if old_limit as usize != 0 {
                    let limit = curr_process.fd_manager.get_limit();
                    unsafe {
                        *old_limit = RLimit {
                            rlim_cur: limit as u64,
                            rlim_max: limit as u64,
                        };
                    }
                }
                if new_limit as usize != 0 {
                    let new_limit = unsafe { (*new_limit).rlim_cur };
                    curr_process.fd_manager.set_limit(new_limit);
                }
            }
            RLIMIT_AS => {
                const USER_MEMORY_LIMIT: usize = 0xffff_ffff;
                if old_limit as usize != 0 {
                    unsafe {
                        *old_limit = RLimit {
                            rlim_cur: USER_MEMORY_LIMIT as u64,
                            rlim_max: USER_MEMORY_LIMIT as u64,
                        };
                    }
                }
            }
            _ => {}
        }
    }
    Ok(0)
}

/// 当前不涉及多核情况
pub fn syscall_getpid() -> SyscallResult {
    Ok(process_api().pid() as isize)
}

/// 获取有效用户 id，即相当于哪个用户的权限。在实现多用户权限前默认为最高权限
pub fn syscall_geteuid() -> SyscallResult {
    Ok(0)
}

/// 等待子进程完成任务，若子进程没有完成，则自身yield
/// 当前仅支持WNOHANG选项，即若未完成时则不予等待，直接返回0
/// # Arguments
/// * `pid` - isize
/// * `exit_code_ptr` - *mut i32
/// * `option` - WaitFlags
pub fn syscall_wait4(args: [usize; 6]) -> SyscallResult {
    let pid = args[0] as isize;
    let exit_code_ptr = args[1] as *mut i32;
    let option = WaitFlags::from_bits(args[2] as u32).unwrap();
    loop {
        unimplemented!();
        // let answer = unsafe { wait_pid(pid, exit_code_ptr) };
        // match answer {
        //     Ok(pid) => {
        //         return Ok(pid as isize);
        //     }
        //     Err(status) => {
        //         match status {
        //             WaitStatus::NotExist => {
        //                 return Err(SyscallError::EPERM);
        //             }
        //             WaitStatus::Running => {
        //                 if option.contains(WaitFlags::WNOHANG) {
        //                     // 不予等待，直接返回0
        //                     return Ok(0);
        //                 } else {
        //                     // wait回来之后，如果还需要wait，先检查是否有信号未处理
        //                     #[cfg(feature = "signal")]
        //                     if current_process().have_signals().is_some() {
        //                         return Err(SyscallError::EINTR);
        //                     }
        //                     // 执行yield操作，切换任务
        //                     yield_now_task();
        //                 }
        //             }
        //             _ => {
        //                 panic!("Shouldn't reach here!");
        //             }
        //         }
        //     }
        // };
    }
}

/// 获取用户组 id。在实现多用户权限前默认为最高权限
pub fn syscall_getgid() -> SyscallResult {
    Ok(0)
}

/// 获取有效用户组 id，即相当于哪个用户的权限。在实现多用户权限前默认为最高权限
pub fn syscall_getegid() -> SyscallResult {
    Ok(0)
}

/// 获取当前任务的线程 id
pub fn syscall_gettid() -> SyscallResult {
    Ok(current_task().id().as_u64() as isize)
}

/// # Arguments
/// * `path` - *const u8
/// * `argv` - *const usize
/// * `envp` - *const usize
pub fn syscall_exec(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    //     let path = args[0] as *const u8;
    //     let mut argv = args[1] as *const usize;
    //     let mut envp = args[2] as *const usize;
    //     let path = deal_with_path(AT_FDCWD, Some(path), false);
    //     if path.is_none() {
    //         return Err(SyscallError::EINVAL);
    //     }
    //     let path = path.unwrap();
    //     if path.is_dir() {
    //         return Err(SyscallError::EISDIR);
    //     }
    //     let path = path.path().to_string();
    //
    //     let mut args_vec = Vec::new();
    //     // args相当于argv，指向了参数所在的地址
    //     loop {
    //         let args_str_ptr = unsafe { *argv };
    //         if args_str_ptr == 0 {
    //             break;
    //         }
    //         args_vec.push(unsafe { raw_ptr_to_ref_str(args_str_ptr as *const u8) }.to_string());
    //         unsafe {
    //             argv = argv.add(1);
    //         }
    //     }
    //     let mut envs_vec = Vec::new();
    //     if envp as usize != 0 {
    //         loop {
    //             let envp_str_ptr = unsafe { *envp };
    //             if envp_str_ptr == 0 {
    //                 break;
    //             }
    //             envs_vec.push(unsafe { raw_ptr_to_ref_str(envp_str_ptr as *const u8) }.to_string());
    //             unsafe {
    //                 envp = envp.add(1);
    //             }
    //         }
    //     }
    //     // let testcase = if args_vec[0] == "./busybox".to_string()
    //     //     || args_vec[0] == "busybox".to_string()
    //     //     || args_vec[0] == "entry-static.exe".to_string()
    //     //     || args_vec[0] == "entry-dynamic.exe".to_string()
    //     //     || args_vec[0] == "lmbench_all".to_string()
    //     // {
    //     //     args_vec[1].clone()
    //     // } else {
    //     //     args_vec[0].clone()
    //     // };
    //     // if filter(testcase) == false {
    //     //     return -1;
    //     // }
    //     let curr_process = current_process();
    //
    //     // 设置 file_path
    //     curr_process.set_file_path(path.clone());
    //
    //     // 清空futex信号列表
    //     clear_wait(curr_process.pid(), true);
    //     let argc = args_vec.len();
    //     if curr_process.exec(path, args_vec, &envs_vec).is_err() {
    //         exit_current_task(0);
    //     }
    //     Ok(argc as isize)
}

/// 获取用户 id。在实现多用户权限前默认为最高权限
pub fn syscall_getuid() -> SyscallResult {
    Ok(0)
}

/// To get the parent process id
pub fn syscall_getppid() -> SyscallResult {
    #[cfg(feature = "uni_process")]
    {
        warn!("Pretending to successfully getppid.");
        Ok(0)
    }
    #[cfg(feature = "multi_process")]
    {
        Ok(process_api().get_parent() as isize)
    }
}

/// not support
pub fn syscall_getpgid() -> SyscallResult {
    Ok(0)
}

/// To yield the current task
pub fn syscall_yield() -> SyscallResult {
    yield_now();
    Ok(0)
}

/// # Arguments
/// * `pgid: usize`
pub fn syscall_setpgid(args: [usize; 6]) -> SyscallResult {
    let pgid = args[0];
    info!("not support setpgid, try to set {}", pgid);
    Ok(0)
}

/// 设置tid对应的指针
/// 返回值为当前的tid
/// # Arguments
/// * `tid` - usize
pub fn syscall_set_tid_address(args: [usize; 6]) -> SyscallResult {
    #[cfg(feature = "multi_process")]
    {
        let tid = args[0];
        set_child_tid(tid);
        Ok(current_task().id().as_u64() as isize)
    }
    #[cfg(feature = "uni_process")]
    {
        Ok(current_task().id().as_u64() as isize)
    }
}

/// # Arguments
/// * `new_mask - i32`
pub fn syscall_umask(args: [usize; 6]) -> SyscallResult {
    let new_mask = args[0] as i32;
    Ok(process_api().fd_manager.set_mask(new_mask) as isize)
}
