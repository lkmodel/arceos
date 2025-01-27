use crate::syscall::SyscallResult;

/// 实现`ppoll`系统调用
///
/// 其中timeout是一段相对时间,需要计算出相对于当前时间戳的绝对时间戳
///
/// # Arguments
/// * `ufds - *mut PollFd`
/// * `nfds - usize`
/// * `timeout - *const TimeSecs`
/// * `mask - usize`
pub fn syscall_ppoll(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 实现`pselect6`系统调用
/// # Arguments
/// * `nfds - usize`
/// * `readfds - *mut usize`
/// * `writefds - *mut usize`
/// * `exceptfds - *mut usize`
/// * `timeout - *const TimeSecs`
/// * `mask - usize`
pub fn syscall_pselect6(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}
