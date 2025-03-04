use crate::syscall::SyscallResult;

/// For `epoll_create`, Since Linux 2.6.8, the size argument is ignored, but must be greater than zero;
///
///
/// For `epoll_create1`, If flags is 0, then, other than the fact that the obsolete size argument is dropped, `epoll_create1()`
///  is the same as `epoll_create()`.
///
/// If flag equals to `EPOLL_CLOEXEC`, than set the `cloexec` flag for the `fd`
/// # Arguments
/// * `flag - usize`
pub fn syscall_epoll_create1(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 执行syscall_epoll_ctl，修改文件对应的响应事件
///
/// 需要一个`epoll`事件的`fd`，用来执行修改操作
///
/// # Arguments
/// * `epfd: i32, epoll`文件的`fd`
/// * `op: i32`, 修改操作的类型
/// * `fd: i32`, 接受事件的文件的`fd`
/// * `event: *const EpollEvent`, 接受的事件
pub fn syscall_epoll_ctl(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 执行syscall_epoll_wait系统调用
///
/// # Arguments
/// * `epfd: i32, epoll`文件的`fd`
/// * `event: *mut EpollEvent`, 接受事件的数组
/// * `max_event: i32`, 最大的响应事件数量,必须大于0
/// * `timeout: i32`, 超时时间，是一段相对时间，需要手动转化为绝对时间
///
/// `ret`: 实际写入的响应事件数目
pub fn syscall_epoll_wait(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}
