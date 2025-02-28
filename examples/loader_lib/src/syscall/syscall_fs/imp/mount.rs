use crate::syscall::SyscallResult;

/// 功能:挂载文件系统；
/// # Arguments
/// * `special: *const u8`, 挂载设备
/// * `dir: *const u8`, 挂载点
/// * `fs_type: *const u8`, 挂载的文件系统类型
/// * `flags: usize`, 挂载参数
/// * `data: *const u8`, 传递给文件系统的字符串参数,可为NULL
/// 返回值:成功返回0,失败返回-1
pub fn syscall_mount(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 功能:卸载文件系统；
/// 输入:指定卸载目录,卸载参数；
/// 返回值:成功返回0,失败返回-1
/// # Arguments
/// * `dir: *const u8`, 指定卸载目录
/// * `flags: usize`, 卸载参数
pub fn syscall_umount(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}
