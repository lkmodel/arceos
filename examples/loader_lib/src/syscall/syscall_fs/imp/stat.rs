use crate::syscall::SyscallResult;

/// 实现 stat 系列系统调用
/// # Arguments
/// * `fd - usize`
/// * `kst - *mut Kstat`
pub fn syscall_fstat(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 获取文件状态信息，但是给出的是目录`fd`和相对路径。
/// # Arguments
/// * `dir_fd - usize`
/// * `path - *const u8`
/// * `kst - *mut Kstat`
pub fn syscall_fstatat(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 获取文件系统的信息
/// # Arguments
/// * `path - *const u8`
/// * `stat - *mut FsStat`
pub fn syscall_statfs(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}
