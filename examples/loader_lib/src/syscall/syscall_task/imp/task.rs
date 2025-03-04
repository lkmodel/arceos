use crate::syscall::SyscallResult;

/// # Arguments
/// * `exit_code` - i32
pub fn syscall_exit(_args: [usize; 6]) -> ! {
    unimplemented!();
    //    let exit_code = args[0] as i32;
    //    info!("exit: exit_code = {}", exit_code);
    // let cases = ["fcanf", "fgetwc_buffering", "lat_pipe"];
    // let mut test_filter = TEST_FILTER.lock();
    // for case in cases {
    //     let case = case.to_string();
    //     if test_filter.contains_key(&case) {
    //         test_filter.remove(&case);
    //     }
    // }
    // drop(test_filter);
    //    exit_current_task(exit_code)
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
    unimplemented!();
    //    let pid = args[0];
    //    let resource = args[1] as i32;
    //    let new_limit = args[2] as *const RLimit;
    //    let old_limit = args[3] as *mut RLimit;
    //    // 当pid不为0，其实没有权利去修改其他的进程的资源限制
    //    let curr_process = current_process();
    //    if pid == 0 || pid == curr_process.pid() as usize {
    //        match resource {
    //            RLIMIT_STACK => {
    //                if old_limit as usize != 0 {
    //                    unsafe {
    //                        *old_limit = RLimit {
    //                            rlim_cur: TASK_STACK_SIZE as u64,
    //                            rlim_max: TASK_STACK_SIZE as u64,
    //                        };
    //                    }
    //                }
    //            }
    //            RLIMIT_NOFILE => {
    //                // 仅支持修改最大文件数
    //                if old_limit as usize != 0 {
    //                    let limit = curr_process.fd_manager.get_limit();
    //                    unsafe {
    //                        *old_limit = RLimit {
    //                            rlim_cur: limit as u64,
    //                            rlim_max: limit as u64,
    //                        };
    //                    }
    //                }
    //                if new_limit as usize != 0 {
    //                    let new_limit = unsafe { (*new_limit).rlim_cur };
    //                    curr_process.fd_manager.set_limit(new_limit);
    //                }
    //            }
    //            RLIMIT_AS => {
    //                const USER_MEMORY_LIMIT: usize = 0xffff_ffff;
    //                if old_limit as usize != 0 {
    //                    unsafe {
    //                        *old_limit = RLimit {
    //                            rlim_cur: USER_MEMORY_LIMIT as u64,
    //                            rlim_max: USER_MEMORY_LIMIT as u64,
    //                        };
    //                    }
    //                }
    //            }
    //            _ => {}
    //        }
    //    }
    //    Ok(0)
}

/// 当前不涉及多核情况
pub fn syscall_getpid() -> SyscallResult {
    unimplemented!();
    //    Ok(current_process().pid() as isize)
}

/// 获取有效用户 id，即相当于哪个用户的权限。在实现多用户权限前默认为最高权限
pub fn syscall_geteuid() -> SyscallResult {
    Ok(0)
}
