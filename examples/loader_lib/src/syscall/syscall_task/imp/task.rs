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

/// 当前不涉及多核情况
pub fn syscall_getpid() -> SyscallResult {
    unimplemented!();
    //    Ok(current_process().pid() as isize)
}

/// 获取有效用户 id，即相当于哪个用户的权限。在实现多用户权限前默认为最高权限
pub fn syscall_geteuid() -> SyscallResult {
    Ok(0)
}
